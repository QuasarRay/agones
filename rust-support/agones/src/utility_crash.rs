//! Source apimachinery HandleCrash state, ordered callbacks and worker boundary.
// Adapted from selected apimachinery util/runtime and util/wait (Apache-2.0).
// Copyright 2014 The Kubernetes Authors. Apache-2.0.
use std::{fmt::Write as _, io::Write as _, sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}}};
use cgp::prelude::*;
use tokio_util::sync::CancellationToken;
use crate::{utility_runtime::UtilityRuntimeContext, utility_panic::{SourceRecoveredPanic,
    SourceRuntimeErrorString, SourceBoundsCode, SourceBoundsError, unwind_source_runtime_error},
    utility_http::{HeaderAllocationProfile, SourceSliceAllocationError, NativeSourceCollectionContext,
        CanRaiseSourceCollectionFailure, HasSourceUnicodeProfile, complete_source_collection, reserve_source_array}};

#[derive(Clone)]
pub struct SourceCrashContext { cancelled: CancellationToken, pub call_depth: i64 }
impl SourceCrashContext {
    pub fn new(cancelled: CancellationToken) -> Self { Self { cancelled, call_depth: 0 } }
    pub fn is_cancelled(&self) -> bool { self.cancelled.is_cancelled() }
    pub async fn cancelled(&self) { self.cancelled.cancelled().await; }
}
/// Source callback/interface data. Selection of the crash effect stays in CGP.
pub type SourcePanicCallback = Arc<dyn for<'a, 'b, 'c> Fn(&'a UtilityRuntimeContext, &'b SourceCrashContext, SourceCrashPanic<'c>) + Send + Sync>;
#[derive(Clone, Copy)]
pub struct SourceCrashPanic<'a> { value: &'a SourceRecoveredPanic }
struct RethrowSourceCrash { owner: usize }
impl std::ops::Deref for SourceCrashPanic<'_> {
    type Target = SourceRecoveredPanic;
    fn deref(&self) -> &SourceRecoveredPanic { self.value }
}
impl std::fmt::Display for SourceCrashPanic<'_> {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.value, out) }
}
impl SourceCrashPanic<'_> {
    /// Transfer the existing recovered payload through its active callback
    /// boundary. No Error/String formatting or replacement value is involved.
    pub fn repanic(self) -> ! {
        crate::utility_panic::unwind_source_panic(RethrowSourceCrash { owner: self.value as *const SourceRecoveredPanic as usize })
    }
}
#[derive(Clone)]
pub enum SourcePanicHandler { Nil, LogPanic, Callback(SourcePanicCallback) }
impl SourcePanicHandler {
    fn call(&self, runtime: &UtilityRuntimeContext, context: &SourceCrashContext, panic: &SourceRecoveredPanic) {
        match self {
            Self::Nil => unwind_source_runtime_error(SourceRuntimeErrorString::new("invalid memory address or nil pointer dereference")),
            Self::LogPanic => runtime.report_source_crash_panic(context, panic),
            Self::Callback(callback) => callback(runtime, context, SourceCrashPanic { value: panic }),
        }
    }
}
#[derive(Clone)]
enum CallbackBacking { Nil, Empty, Slots(Arc<RwLock<Vec<SourcePanicHandler>>>) }
#[derive(Clone)]
pub struct SourcePanicHandlers { backing: CallbackBacking, offset: usize, length: usize, capacity: usize }
impl SourcePanicHandlers {
    pub fn nil() -> Self { Self { backing: CallbackBacking::Nil, offset: 0, length: 0, capacity: 0 } }
    pub fn is_nil(&self) -> bool { matches!(&self.backing, CallbackBacking::Nil) }
    pub fn len(&self) -> usize { self.length }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn make(length: i64, capacity: i64, profile: HeaderAllocationProfile) -> Self {
        let layout = complete_source_collection(&NativeSourceCollectionContext,
            profile.make_slice_layout(length, capacity, u64::from(profile.word_bits() / 8)));
        let backing = if layout.capacity == 0 { CallbackBacking::Empty } else {
            let mut slots = complete_source_collection(&NativeSourceCollectionContext, reserve_source_array::<SourcePanicHandler>(layout.capacity));
            slots.resize_with(layout.capacity as usize, || SourcePanicHandler::Nil);
            CallbackBacking::Slots(Arc::new(RwLock::new(slots)))
        };
        Self { backing, offset: 0, length: length as usize, capacity: layout.capacity as usize }
    }
    pub fn get(&self, index: usize) -> SourcePanicHandler {
        self.check_index(index);
        match &self.backing {
            CallbackBacking::Slots(slots) => slots.read().unwrap_or_else(|error| error.into_inner())[self.offset + index].clone(),
            _ => unreachable!("nonempty source callback backing"),
        }
    }
    pub fn set(&self, index: usize, handler: SourcePanicHandler) {
        self.check_index(index);
        if let CallbackBacking::Slots(slots) = &self.backing {
            slots.write().unwrap_or_else(|error| error.into_inner())[self.offset + index] = handler;
        }
    }
    fn check_index(&self, index: usize) {
        if index >= self.length { unwind_source_runtime_error(SourceBoundsError {
            x: index as i64, y: self.length as i64, signed: false, code: SourceBoundsCode::Index }); }
    }
    pub fn reslice(&self, low: usize, high: usize, maximum: usize) -> Self {
        for (x, y, code) in [(maximum, self.capacity, SourceBoundsCode::Slice3Capacity),
            (high, maximum, SourceBoundsCode::Slice3High), (low, high, SourceBoundsCode::Slice3Low)] {
            if x > y { unwind_source_runtime_error(SourceBoundsError { x: x as i64, y: y as i64, signed: false, code }); }
        }
        Self { backing: self.backing.clone(), offset: self.offset + low, length: high - low, capacity: maximum - low }
    }
    pub fn append(&mut self, handler: SourcePanicHandler, profile: HeaderAllocationProfile) {
        let length = complete_source_collection(&NativeSourceCollectionContext,
            self.length.checked_add(1).ok_or(SourceSliceAllocationError::GrowLength));
        if length > self.capacity {
            let layout = complete_source_collection(&NativeSourceCollectionContext,
                profile.grow_slice_layout(self.capacity as u64, length as u64, u64::from(profile.word_bits() / 8), false));
            let mut slots = complete_source_collection(&NativeSourceCollectionContext, reserve_source_array::<SourcePanicHandler>(layout.capacity));
            for index in 0..self.length { slots.push(self.get(index)); }
            slots.resize_with(layout.capacity as usize, || SourcePanicHandler::Nil);
            self.backing = CallbackBacking::Slots(Arc::new(RwLock::new(slots)));
            self.offset = 0; self.capacity = layout.capacity as usize;
        }
        if let CallbackBacking::Slots(slots) = &self.backing {
            slots.write().unwrap_or_else(|error| error.into_inner())[self.offset + self.length] = handler;
        }
        self.length = length;
    }
}

/// One existing-runtime-owned resource for the source ReallyCrash variable and
/// PanicHandlers slice header. User callbacks execute without either lock held.
pub struct SourceCrashState { really_crash: AtomicBool, handlers: Mutex<SourcePanicHandlers> }
impl Default for SourceCrashState {
    fn default() -> Self {
        let handlers = SourcePanicHandlers::make(1, 1, HeaderAllocationProfile::GO126_64);
        handlers.set(0, SourcePanicHandler::LogPanic);
        Self { really_crash: AtomicBool::new(true), handlers: Mutex::new(handlers) }
    }
}
impl SourceCrashState {
    pub fn really_crash(&self) -> bool { self.really_crash.load(Ordering::SeqCst) }
    pub fn set_really_crash(&self, value: bool) { self.really_crash.store(value, Ordering::SeqCst); }
    pub fn panic_handlers(&self) -> SourcePanicHandlers { self.handlers.lock().unwrap_or_else(|error| error.into_inner()).clone() }
    pub fn set_panic_handlers(&self, value: SourcePanicHandlers) { *self.handlers.lock().unwrap_or_else(|error| error.into_inner()) = value; }
}
#[cgp_getter]
pub trait HasSourceCrashState { fn source_crash_state(&self) -> &Arc<SourceCrashState>; }
#[cgp_component(SourceCrashHandler)]
pub trait CanHandleSourceCrash {
    fn handle_source_crash(&self, context: SourceCrashContext, panic: SourceRecoveredPanic, additional: &SourcePanicHandlers);
}
#[cgp_new_provider]
impl SourceCrashHandler<UtilityRuntimeContext> for HandleSourceCrash {
    fn handle_source_crash(runtime: &UtilityRuntimeContext, mut context: SourceCrashContext, panic: SourceRecoveredPanic, additional: &SourcePanicHandlers) {
        let additional = additional.clone();
        context.call_depth = context.call_depth.wrapping_add(4);
        let handlers = runtime.source_crash_state().panic_handlers();
        // Copy only the slice header. Each next function value is read from
        // its retained backing, so mutations of later slots remain visible.
        for callbacks in [handlers, additional] {
            for index in 0..callbacks.len() {
                let handler = callbacks.get(index);
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler.call(runtime, &context, &panic)));
                if let Err(payload) = result {
                    if payload.downcast_ref::<RethrowSourceCrash>().is_some_and(|request|
                        request.owner == &panic as *const SourceRecoveredPanic as usize) { panic.resume(); }
                    std::panic::resume_unwind(payload);
                }
            }
        }
        if runtime.source_crash_state().really_crash() { panic.resume(); }
    }
}
#[cgp_component(SourceCrashPanicReporter)]
pub trait CanReportSourceCrashPanic {
    fn report_source_crash_panic(&self, context: &SourceCrashContext, panic: &SourceRecoveredPanic);
}
#[cgp_new_provider]
impl<Context: CanRaiseSourceCollectionFailure + HasSourceUnicodeProfile> SourceCrashPanicReporter<Context> for ReportNativeSourceCrashPanic {
    fn report_source_crash_panic(runtime: &Context, _context: &SourceCrashContext, panic: &SourceRecoveredPanic) {
        if panic.is_abort_handler() { return; }
        let stack = crate::utility_runtime_panic::native_recovery_stack(runtime);
        let profile = *runtime.source_unicode_profile();
        let quote = |value: &str| complete_source_collection(runtime, crate::utility_go_quote::try_source_go_quote(value.as_bytes(), profile));
        let typed = panic.payload().downcast_ref::<crate::utility_format_methods::SourcePrintfPanic>();
        let panic_value = match typed {
            Some(value) => complete_source_collection(runtime, value.render(profile, false)),
            None => panic.to_string().into_bytes(),
        };
        let panic_quoted = complete_source_collection(runtime, crate::utility_go_quote::try_source_go_quote(&panic_value, profile));
        let mut record = format!("\"Observed a panic\" panic={panic_quoted}");
        if panic.source_type() != Some("string") {
            if let Some(value) = typed {
                let bytes = complete_source_collection(runtime, value.render(profile, true));
                let value = complete_source_collection(runtime, crate::utility_go_quote::try_source_go_quote(&bytes, profile));
                let _ = write!(&mut record, " panicGoValue={value}");
            } else {
                let value = match panic.error() {
                    Some(error) => complete_source_collection(runtime, crate::utility_go_quote::try_source_error_go_syntax(error, profile))
                        .unwrap_or_else(|| format!("{error:?}")),
                    None => format!("{:?}", panic.payload()),
                };
                // Noncanonical values retain an explicit native Debug projection.
                // Full GoStringer/reflection/klog/context/caller profiles remain.
                let _ = write!(&mut record, " panicGoValue={}", quote(&value));
            }
        }
        let _ = writeln!(&mut record, " stacktrace={}", quote(&stack));
        let _ = std::io::stderr().lock().write_all(record.as_bytes());
    }
}
