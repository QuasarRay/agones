//! Native CpgPass/ForkJoinParallelCpgPass lifecycle, composed through CGP.
use crate::{Result, execution::CanRunInExecutionContext, graph::{CanApplyCpgDiff, DiffGraphBuilder, Graph, ApplyReport}};
use cgp::prelude::*;
use rayon::prelude::*;
use std::{panic::{AssertUnwindSafe, catch_unwind, resume_unwind}, time::{Instant, SystemTime, UNIX_EPOCH}};

/// The callbacks implement pass behavior; the CGP provider owns their execution order,
/// fork/join policy, finalization, builder absorption and mutation application.
pub trait CpgPassDefinition: Sync {
    type Part: Send + Sync;
    type Accumulator: Send;
    fn name(&self) -> &str { std::any::type_name::<Self>() }
    fn init(&self) -> Result<()> { Ok(()) }
    fn finish(&self) -> Result<()> { Ok(()) }
    fn generate_parts(&self, graph: &Graph) -> Result<Vec<Self::Part>>;
    fn is_parallel(&self) -> bool { true }
    fn create_accumulator(&self) -> Result<Self::Accumulator>;
    fn run_on_part(&self, graph: &Graph, builder: &mut DiffGraphBuilder, part: &Self::Part, accumulator: &mut Self::Accumulator) -> Result<()>;
    fn merge_accumulator(&self, left: &mut Self::Accumulator, right: Self::Accumulator) -> Result<()>;
    fn on_accumulator_complete(&self, builder: &mut DiffGraphBuilder, accumulator: Self::Accumulator) -> Result<()>;
}

pub struct SingleCpgPass<F> { pub name: String, pub run: F }
impl<F> CpgPassDefinition for SingleCpgPass<F>
where F: Fn(&Graph, &mut DiffGraphBuilder) -> Result<()> + Sync {
    type Part = ();
    type Accumulator = ();
    fn name(&self) -> &str { &self.name }
    fn generate_parts(&self, _graph: &Graph) -> Result<Vec<()>> { Ok(vec![()]) }
    fn is_parallel(&self) -> bool { false }
    fn create_accumulator(&self) -> Result<()> { Ok(()) }
    fn run_on_part(&self, graph: &Graph, builder: &mut DiffGraphBuilder, _part: &(), _accumulator: &mut ()) -> Result<()> { (self.run)(graph, builder) }
    fn merge_accumulator(&self, _left: &mut (), _right: ()) -> Result<()> { Ok(()) }
    fn on_accumulator_complete(&self, _builder: &mut DiffGraphBuilder, _accumulator: ()) -> Result<()> { Ok(()) }
}
/// Source compatibility alias for the deprecated SimpleCpgPass class.
pub type SimpleCpgPass<F> = SingleCpgPass<F>;

/// Equivalent of ForkJoinParallelCpgPass, whose source accumulator is explicitly Null.
pub trait ParallelCpgPassDefinition: Sync {
    type Part: Send + Sync;
    fn name(&self) -> &str { std::any::type_name::<Self>() }
    fn init(&self) -> Result<()> { Ok(()) }
    fn finish(&self) -> Result<()> { Ok(()) }
    fn generate_parts(&self, graph: &Graph) -> Result<Vec<Self::Part>>;
    fn is_parallel(&self) -> bool { true }
    fn run_on_part(&self, graph: &Graph, builder: &mut DiffGraphBuilder, part: &Self::Part) -> Result<()>;
}
pub struct WithoutAccumulator<P>(pub P);
impl<P: ParallelCpgPassDefinition> CpgPassDefinition for WithoutAccumulator<P> {
    type Part = P::Part;
    type Accumulator = ();
    fn name(&self) -> &str { self.0.name() }
    fn init(&self) -> Result<()> { self.0.init() }
    fn finish(&self) -> Result<()> { self.0.finish() }
    fn generate_parts(&self, graph: &Graph) -> Result<Vec<Self::Part>> { self.0.generate_parts(graph) }
    fn is_parallel(&self) -> bool { self.0.is_parallel() }
    fn create_accumulator(&self) -> Result<()> { Ok(()) }
    fn run_on_part(&self, graph: &Graph, builder: &mut DiffGraphBuilder, part: &Self::Part, _accumulator: &mut ()) -> Result<()> { self.0.run_on_part(graph, builder, part) }
    fn merge_accumulator(&self, _left: &mut (), _right: ()) -> Result<()> { Ok(()) }
    fn on_accumulator_complete(&self, _builder: &mut DiffGraphBuilder, _accumulator: ()) -> Result<()> { Ok(()) }
}

#[cgp_component(CpgPassRunner)]
pub trait CanRunCpgPass {
    fn run_with_builder<P: CpgPassDefinition>(&self, pass: &P, graph: &Graph, external: &mut DiffGraphBuilder) -> Result<usize>;
    fn create_and_apply<P: CpgPassDefinition>(&self, pass: &P, graph: &mut Graph) -> Result<PassReport>;
}
#[derive(Debug)]
pub struct PassReport { pub parts: usize, pub generated_changes: usize, pub applied: ApplyReport }

fn run_chunk<P: CpgPassDefinition>(pass: &P, graph: &Graph, parts: &[P::Part]) -> Result<(DiffGraphBuilder, P::Accumulator)> {
    let mut builder = DiffGraphBuilder::new();
    let mut accumulator = pass.create_accumulator()?;
    for part in parts { pass.run_on_part(graph, &mut builder, part, &mut accumulator)?; }
    Ok((builder, accumulator))
}
fn run_parts<Context: CanRunInExecutionContext, P: CpgPassDefinition>(context: &Context, pass: &P, graph: &Graph, parts: &[P::Part]) -> Result<(DiffGraphBuilder, P::Accumulator)> {
    if !pass.is_parallel() || parts.is_empty() { return run_chunk(pass, graph, parts); }
    let dispatcher = tracing::dispatcher::get_default(Clone::clone);
    let span = tracing::Span::current();
    context.with_execution_context(|| {
        let chunk_size = parts.len().div_ceil(rayon::current_num_threads());
        // Indexed collection retains the source array's encounter order, independent
        // of worker completion order. Each partition owns its accumulator and diff.
        let chunks = parts.par_chunks(chunk_size).map(|parts| {
            tracing::dispatcher::with_default(&dispatcher, || {
                let _entered = span.enter(); run_chunk(pass, graph, parts)
            })
        }).collect::<Vec<_>>();
        let mut chunks = chunks.into_iter();
        let (mut left, mut accumulator) = chunks.next().expect("nonempty parts have a nonempty partition")?;
        for right in chunks {
            let (right, right_accumulator) = right?;
            left.absorb(right); pass.merge_accumulator(&mut accumulator, right_accumulator)?;
        }
        Ok((left, accumulator))
    })
}

#[cgp_new_provider]
impl<Context> CpgPassRunner<Context> for RunNativeCpgPass
where Context: CanRunInExecutionContext + CanApplyCpgDiff {
    fn run_with_builder<P: CpgPassDefinition>(context: &Context, pass: &P, graph: &Graph, external: &mut DiffGraphBuilder) -> Result<usize> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            pass.init()?;
            let parts = pass.generate_parts(graph)?;
            let (mut builder, accumulator) = run_parts(context, pass, graph, &parts)?;
            pass.on_accumulator_complete(&mut builder, accumulator)?;
            external.absorb(builder);
            Ok(parts.len())
        }));
        // The original finally block runs even when init, generateParts, collection,
        // completion or builder absorption fails. A finish failure takes precedence.
        pass.finish()?;
        match result { Ok(result) => result, Err(panic) => resume_unwind(panic) }
    }
    fn create_and_apply<P: CpgPassDefinition>(context: &Context, pass: &P, graph: &mut Graph) -> Result<PassReport> {
        let start = Instant::now();
        let span = tracing::info_span!("cpg_pass", name = pass.name());
        let _entered = span.enter();
        tracing::info!("Start of pass");
        let mut built = None;
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut builder = DiffGraphBuilder::new();
            let parts = <Self as CpgPassRunner<Context>>::run_with_builder(context, pass, graph, &mut builder)?;
            built = Some(Instant::now());
            let generated_changes = builder.size();
            let applied = context.apply_cpg_diff(graph, builder)?;
            Ok(PassReport { parts, generated_changes, applied })
        }));
        let elapsed = start.elapsed();
        let mutation_fraction = built.map_or(0.0, |built| built.elapsed().as_nanos() as f64 * 100.0 / (elapsed.as_nanos() + 1) as f64);
        match &result {
            Ok(Ok(report)) => tracing::info!(elapsed_ms = elapsed.as_millis() as u64, mutation_fraction,
                generated_changes = report.generated_changes, applied_changes = report.applied.applied_changes, parts = report.parts, "Pass completed"),
            Ok(Err(error)) => tracing::error!(%error, elapsed_ms = elapsed.as_millis() as u64, mutation_fraction, "Pass failed"),
            Err(_) => tracing::error!(elapsed_ms = elapsed.as_millis() as u64, mutation_fraction, "Pass panicked"),
        }
        match result { Ok(result) => result, Err(panic) => resume_unwind(panic) }
    }
}

pub fn run_with_builder_logged<Context: CanRunCpgPass, P: CpgPassDefinition>(context: &Context, pass: &P, graph: &Graph, builder: &mut DiffGraphBuilder) -> i64 {
    let start = Instant::now();
    let original_size = builder.size();
    tracing::info!(name = pass.name(), "Start of pass");
    match context.run_with_builder(pass, graph, builder) {
        Ok(parts) => {
            tracing::info!(name = pass.name(), elapsed_ms = start.elapsed().as_millis() as u64,
                generated_changes = builder.size() - original_size, parts, "Pass completed");
            parts as i64
        }
        Err(error) => { tracing::warn!(name = pass.name(), %error, elapsed_ms = start.elapsed().as_millis() as u64, "Pass failed"); -1 }
    }
}
pub fn generate_out_file_name(prefix: &str, out_name: &str, simple_class_name: &str, index: i32) -> String {
    format!("{prefix}_{}_{index}", if out_name.is_empty() { simple_class_name } else { out_name })
}
fn wall_clock_millis() -> i128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) { Ok(time) => time.as_millis() as i128, Err(time) => -(time.duration().as_millis() as i128) }
}
pub fn coarsest_duration(milliseconds: i128) -> String {
    for (factor, name) in [(86_400_000, "day"), (3_600_000, "hour"), (60_000, "minute"), (1000, "second"), (1, "millisecond")] {
        if milliseconds % factor == 0 {
            let value = milliseconds / factor;
            return format!("{value} {name}{}", if value == 1 { "" } else { "s" });
        }
    }
    unreachable!("one millisecond divides every integer duration")
}
/// Scope-owned tracing fields provide the source's temporary MDC duration context.
pub fn with_start_end_times_logged<T>(name: &str, work: impl FnOnce() -> T) -> T {
    struct Completion<'a> { start: i128, name: &'a str }
    impl Drop for Completion<'_> {
        fn drop(&mut self) {
            let duration = coarsest_duration(wall_clock_millis() - self.start);
            tracing::info!(name = self.name, time = duration.as_str(), "Pass completed");
        }
    }
    tracing::info!(name, "Running pass");
    let _completion = Completion { start: wall_clock_millis(), name };
    work()
}
/// Both deprecated source operations intentionally ignore serialization arguments:
/// createApplySerializeAndStore delegates to createAndApply; store's upstream body is empty.
#[deprecated(note = "use create_and_apply")]
pub fn create_apply_serialize_and_store<Context: CanRunCpgPass, P: CpgPassDefinition, Serialized>(context: &Context, pass: &P, graph: &mut Graph, _serialized: &mut Serialized, _prefix: &str) -> Result<PassReport> {
    context.create_and_apply(pass, graph)
}
#[deprecated(note = "the pinned CpgPassBase.store body explicitly has no effect")]
pub fn store<Overlay, Serialized>(_overlay: &Overlay, _name: &str, _serialized: &mut Serialized) {}

/// Rust's borrowed slice is the native collection view corresponding to Converters.toJava.
pub fn as_list<T>(sequence: &[T]) -> &[T] { sequence }
pub fn as_list_mut<T>(sequence: &mut [T]) -> &mut [T] { sequence }
