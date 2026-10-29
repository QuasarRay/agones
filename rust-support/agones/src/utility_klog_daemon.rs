//! Source klog flush-daemon state and native real-clock task effect.
// klog/v2/klog.go, Google Inc./Kubernetes Authors, Apache-2.0.
// Native clock/select/close/panic/shutdown profiles remain required. The
// daemon transfers work to the existing CGP task owner; it is not a manager.
use std::{sync::{Mutex, mpsc::{sync_channel, SyncSender, Receiver, RecvTimeoutError}}, time::{Duration, Instant}};
use cgp::prelude::*;
use crate::{utility_runtime::UtilityRuntimeContext, utility_scope::{CanSpawnUtilityTask, UtilityBlockingTask},
    utility_klog_dispatch::{FormatResult, SourceKlogFormatError}};

struct Running { stop: SyncSender<()>, done: Receiver<()> }
struct FlushCompletion(SyncSender<()>);
impl Drop for FlushCompletion {
    fn drop(&mut self) { let _ = self.0.send(()); }
}
#[derive(Default)]
pub struct SourceKlogFlushDaemon { running: Mutex<Option<Running>> }
impl SourceKlogFlushDaemon {
    pub fn is_running(&self) -> bool { self.running.lock().unwrap_or_else(|e| e.into_inner()).is_some() }
    pub fn run(&self, runtime: &UtilityRuntimeContext, nanoseconds: i64) -> FormatResult {
        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        if running.is_some() { return Ok(()); }
        let (stop, stopped) = sync_channel(1);
        let (done, completed) = sync_channel(1);
        if nanoseconds <= 0 {
            // NewTicker rejects a nonpositive interval synchronously, before
            // spawning. The complete post-panic channel-state profile remains
            // required; no successful running daemon is fabricated here.
            crate::utility_format_methods::SourcePrintfPanic::raise("non-positive interval for NewTicker".to_owned());
        }
        let interval = Duration::from_nanos(nanoseconds as u64);
        let first_tick = Instant::now().checked_add(interval).ok_or_else(|| missing("klog ticker native Instant range"))?;
        let parent = runtime.clone();
        let work: UtilityBlockingTask = Box::new(move || {
            // Source sends stopDone in a defer, including unwinding Flush.
            // Its buffered send does not require stop to be waiting already.
            let _completion = FlushCompletion(done);
            let mut next_tick = first_tick;
            loop {
                let wait = next_tick.saturating_duration_since(Instant::now());
                match stopped.recv_timeout(wait) {
                    Ok(()) | Err(RecvTimeoutError::Disconnected) => {
                        parent.source_klog_state.output.flush(&parent);
                        return Ok(());
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        // Advance from the ticker's deadline, not from the end
                        // of Flush. Keep one ready tick when Flush is slow;
                        // don't replay an unbounded history of missed ticks.
                        next_tick = next_tick.checked_add(interval)
                            .unwrap_or_else(|| std::panic::panic_any(missing("klog ticker next native Instant range")));
                        let now = Instant::now();
                        if next_tick <= now {
                            let lag = now.duration_since(next_tick).as_nanos();
                            let steps = lag / interval.as_nanos() + 1;
                            let advance = interval.as_nanos() * steps;
                            let advance = Duration::new((advance / 1_000_000_000) as u64, (advance % 1_000_000_000) as u32);
                            next_tick = next_tick.checked_add(advance)
                                .unwrap_or_else(|| std::panic::panic_any(missing("klog ticker missed native Instant range")));
                        }
                        parent.source_klog_state.output.flush(&parent);
                    }
                }
            }
        });
        if let Err(_unstarted) = runtime.spawn_blocking_utility_task_or_return("klog flushDaemon", work) {
            return Err(missing("klog flushDaemon live utility task/runtime owner"));
        }
        *running = Some(Running { stop, done: completed });
        Ok(())
    }
    /// Source stop holds its own mutex while sending the buffered stop request
    /// and waiting for the final Flush; it clears channels only afterward.
    pub fn stop(&self) -> FormatResult {
        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(worker) = running.as_ref() {
            worker.stop.send(()).map_err(|_| missing("klog flushDaemon stop receiver/panic profile"))?;
            worker.done.recv().map_err(|_| missing("klog flushDaemon completion/panic profile"))?;
            *running = None;
        }
        Ok(())
    }
    /// Native drop requests a buffered stop without waiting for completion.
    /// Only explicit parent shutdown performs the source stop-and-wait path.
    pub(crate) fn request_stop(&self) {
        if let Some(worker) = self.running.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
            let _ = worker.stop.try_send(());
        }
    }
}
#[cgp_component(SourceKlogFlushDaemonController)]
pub trait CanControlSourceKlogFlushDaemon {
    fn run_source_klog_flush_daemon(&self, nanoseconds: i64) -> FormatResult;
    fn stop_source_klog_flush_daemon(&self) -> FormatResult;
    fn source_klog_flush_daemon_is_running(&self) -> bool;
}
#[cgp_new_provider]
impl SourceKlogFlushDaemonController<UtilityRuntimeContext> for ControlNativeSourceKlogFlushDaemon {
    fn run_source_klog_flush_daemon(runtime: &UtilityRuntimeContext, nanoseconds: i64) -> FormatResult {
        runtime.source_klog_state.output.flush_daemon.run(runtime, nanoseconds)
    }
    fn stop_source_klog_flush_daemon(runtime: &UtilityRuntimeContext) -> FormatResult {
        runtime.source_klog_state.output.flush_daemon.stop()
    }
    fn source_klog_flush_daemon_is_running(runtime: &UtilityRuntimeContext) -> bool {
        runtime.source_klog_state.output.flush_daemon.is_running()
    }
}
pub fn stop_flush_daemon(runtime: &UtilityRuntimeContext) -> FormatResult { runtime.stop_source_klog_flush_daemon() }
pub fn start_flush_daemon(runtime: &UtilityRuntimeContext, nanoseconds: i64) -> FormatResult {
    stop_flush_daemon(runtime)?;
    runtime.run_source_klog_flush_daemon(nanoseconds)
}
fn missing(operation: &'static str) -> SourceKlogFormatError { SourceKlogFormatError::Unbound { source_type: None, operation } }
