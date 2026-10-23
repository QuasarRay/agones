//! CGP execution capability with Rayon's current-pool-or-global fork/join behavior.
//! Native counterpart of ExecutionContextProvider and ParallelIteratorExecutor.
use cgp::prelude::*;
use rayon::prelude::*;

#[cgp_component(ExecutionContextRunner)]
pub trait CanRunInExecutionContext {
    fn with_execution_context<F, T>(&self, work: F) -> T
    where F: FnOnce() -> T + Send, T: Send;
}
#[cgp_new_provider]
impl<Context> ExecutionContextRunner<Context> for UseCurrentOrGlobalRayonPool {
    fn with_execution_context<F, T>(_context: &Context, work: F) -> T
    where F: FnOnce() -> T + Send, T: Send {
        let dispatcher = tracing::dispatcher::get_default(Clone::clone);
        let span = tracing::Span::current();
        rayon::join(move || tracing::dispatcher::with_default(&dispatcher, || {
            let _entered = span.enter();
            work()
        }), || ()).0
    }
}
pub struct NativeCpgRuntimeContext;
delegate_and_check_components! {
    NativeCpgRuntimeContext { ExecutionContextRunnerComponent: UseCurrentOrGlobalRayonPool }
}

pub struct ParallelIteratorExecutor<I> { iterator: I }
impl<I: IntoIterator> ParallelIteratorExecutor<I> {
    pub fn new(iterator: I) -> Self { Self { iterator } }
    /// Eagerly waits for the complete ordered result, as Future.traverse/Await in the source.
    /// Panics propagate through the pool and Result values retain their original input positions.
    pub fn map<Context, F, T>(self, context: &Context, function: F) -> std::vec::IntoIter<T>
    where Context: CanRunInExecutionContext, I::Item: Send, F: Fn(I::Item) -> T + Send + Sync, T: Send {
        let elements: Vec<_> = self.iterator.into_iter().collect();
        let dispatcher = tracing::dispatcher::get_default(Clone::clone);
        let span = tracing::Span::current();
        context.with_execution_context(move || elements.into_par_iter().map(move |element| {
            tracing::dispatcher::with_default(&dispatcher, || {
                let _entered = span.enter();
                function(element)
            })
        }).collect::<Vec<_>>()).into_iter()
    }
}
