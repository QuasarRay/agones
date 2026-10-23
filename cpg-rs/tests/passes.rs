use cpg_rs::{Error, Result, graph::*, passes::*};
use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};

struct AccumulatingPass { events: Mutex<Vec<&'static str>>, count: usize, fail: Option<&'static str>, visited: AtomicUsize }
impl AccumulatingPass {
    fn new(count: usize, fail: Option<&'static str>) -> Self { Self { events: Mutex::new(vec![]), count, fail, visited: AtomicUsize::new(0) } }
    fn event(&self, event: &'static str) -> Result<()> {
        self.events.lock().unwrap().push(event);
        if self.fail == Some(event) { Err(Error::Invalid(event.into())) } else { Ok(()) }
    }
}
impl CpgPassDefinition for AccumulatingPass {
    type Part = usize;
    type Accumulator = Vec<usize>;
    fn init(&self) -> Result<()> { self.event("init") }
    fn finish(&self) -> Result<()> { self.event("finish") }
    fn generate_parts(&self, _graph: &Graph) -> Result<Vec<usize>> { self.event("generate")?; Ok((0..self.count).collect()) }
    fn create_accumulator(&self) -> Result<Vec<usize>> { self.event("accumulator")?; Ok(vec![]) }
    fn run_on_part(&self, graph: &Graph, builder: &mut DiffGraphBuilder, part: &usize, accumulator: &mut Vec<usize>) -> Result<()> {
        self.event("part")?;
        assert_eq!(graph.node_count(), 0, "every worker reads the graph before any changes are applied");
        self.visited.fetch_add(1, Ordering::SeqCst);
        accumulator.push(*part);
        let node = DetachedNode::new("METHOD"); node.set("NAME", Value::string(part.to_string()))?;
        builder.add_node(node); Ok(())
    }
    fn merge_accumulator(&self, left: &mut Vec<usize>, right: Vec<usize>) -> Result<()> { self.event("merge")?; left.extend(right); Ok(()) }
    fn on_accumulator_complete(&self, builder: &mut DiffGraphBuilder, accumulator: Vec<usize>) -> Result<()> {
        self.event("complete")?;
        assert_eq!(accumulator, (0..self.count).collect::<Vec<_>>());
        let node = DetachedNode::new("META_DATA"); node.set("VERSION", Value::string(self.count.to_string()))?;
        builder.add_node(node); Ok(())
    }
}

#[test]
fn parallel_pass_merges_in_source_order_and_finishes_before_applying_one_batch() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    pool.install(|| {
        let pass = AccumulatingPass::new(64, None);
        let mut cpg = Cpg::empty().unwrap();
        let report = CpgContext.create_and_apply(&pass, cpg.graph_mut()).unwrap();
        assert_eq!(report.parts, 64);
        assert_eq!(report.generated_changes, 65);
        assert_eq!(report.applied.applied_changes, 65);
        assert_eq!(pass.visited.load(Ordering::SeqCst), 64);
        let names = cpg.graph().nodes("METHOD").map(|node| cpg.graph().property(node.handle, "NAME").unwrap().unwrap()).collect::<Vec<_>>();
        assert_eq!(names, (0..64).map(|part| Value::string(part.to_string())).collect::<Vec<_>>());
        let events = pass.events.lock().unwrap();
        assert_eq!(events.first(), Some(&"init")); assert_eq!(events.last(), Some(&"finish"));
        assert_eq!(events.iter().filter(|event| **event == "complete").count(), 1);
        assert_eq!(events.iter().filter(|event| **event == "finish").count(), 1);
    });
}

#[test]
fn lifecycle_failures_finish_once_and_do_not_apply_or_absorb_partial_changes() {
    for phase in ["init", "generate", "accumulator", "part", "complete"] {
        let pass = AccumulatingPass::new(2, Some(phase));
        let cpg = Cpg::empty().unwrap();
        let mut external = DiffGraphBuilder::new(); external.add_node(DetachedNode::new("FILE"));
        assert!(CpgContext.run_with_builder(&pass, cpg.graph(), &mut external).is_err());
        assert_eq!(external.size(), 1);
        assert_eq!(cpg.graph().node_count(), 0);
        let events = pass.events.lock().unwrap();
        assert_eq!(events.last(), Some(&"finish"));
        assert_eq!(events.iter().filter(|event| **event == "finish").count(), 1);
    }
    let pass = AccumulatingPass::new(0, Some("finish"));
    let mut cpg = Cpg::empty().unwrap();
    assert!(CpgContext.create_and_apply(&pass, cpg.graph_mut()).is_err());
    assert_eq!(cpg.graph().node_count(), 0);
}

#[test]
fn empty_pass_completes_its_accumulator_and_composable_passes_leave_application_to_the_caller() {
    let pass = AccumulatingPass::new(0, None);
    let mut cpg = Cpg::empty().unwrap();
    let mut external = DiffGraphBuilder::new();
    assert_eq!(CpgContext.run_with_builder(&pass, cpg.graph(), &mut external).unwrap(), 0);
    assert_eq!(cpg.graph().node_count(), 0);
    assert_eq!(external.size(), 1);
    let sequential = SingleCpgPass { name: "one".into(), run: |_graph: &Graph, builder: &mut DiffGraphBuilder| {
        builder.add_node(DetachedNode::new("METHOD")); Ok(())
    }};
    assert_eq!(CpgContext.run_with_builder(&sequential, cpg.graph(), &mut external).unwrap(), 1);
    assert_eq!(external.size(), 2);
    let report = CpgContext.apply_cpg_diff(cpg.graph_mut(), external).unwrap();
    assert_eq!(report.applied_changes, 2);
    assert_eq!(cpg.graph().node_count(), 2);
}

#[test]
fn logging_wrapper_reports_failures_and_panic_cleanup_still_runs() {
    let pass = AccumulatingPass::new(1, Some("part"));
    let cpg = Cpg::empty().unwrap();
    assert_eq!(run_with_builder_logged(&CpgContext, &pass, cpg.graph(), &mut DiffGraphBuilder::new()), -1);
    struct PanickingPass<'a>(&'a AtomicUsize);
    impl ParallelCpgPassDefinition for PanickingPass<'_> {
        type Part = ();
        fn generate_parts(&self, _graph: &Graph) -> Result<Vec<()>> { Ok(vec![()]) }
        fn run_on_part(&self, _graph: &Graph, _builder: &mut DiffGraphBuilder, _part: &()) -> Result<()> { panic!("source callback failure") }
        fn finish(&self) -> Result<()> { self.0.fetch_add(1, Ordering::SeqCst); Ok(()) }
    }
    let finished = AtomicUsize::new(0);
    let pass = WithoutAccumulator(PanickingPass(&finished));
    let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        CpgContext.run_with_builder(&pass, cpg.graph(), &mut DiffGraphBuilder::new())
    }));
    assert!(failure.is_err()); assert_eq!(finished.load(Ordering::SeqCst), 1);
}

#[test]
fn naming_duration_and_borrowed_collection_adapters_preserve_source_operations() {
    assert_eq!(generate_out_file_name("pre", "", "MyPass", -2), "pre_MyPass_-2");
    assert_eq!(generate_out_file_name("", "custom", "MyPass", 0), "_custom_0");
    assert_eq!(coarsest_duration(1000), "1 second");
    assert_eq!(coarsest_duration(1500), "1500 milliseconds");
    assert_eq!(coarsest_duration(0), "0 days");
    let mut values = [1, 2];
    assert!(std::ptr::eq(as_list(&values).as_ptr(), values.as_ptr()));
    as_list_mut(&mut values)[1] = 3; assert_eq!(values, [1, 3]);
    assert_eq!(with_start_end_times_logged("return", || 42), 42);
}
