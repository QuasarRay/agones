//! Static source collection-failure effects used by native convenience APIs.
// Adapted from Go runtime error/panic and slice/map failure behavior.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use super::SourceSliceAllocationError;
use crate::{utility_tls::CanExitSourceProcess, utility_declarations::{
    derive_deftly_template_SourceErrorOrigin, derive_deftly_template_NativeSourceCollectionProviders}};
use cgp::prelude::*;
use derive_deftly::Deftly;
use std::{any::Any, error::Error, fmt, io::Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "runtime", source_struct = "errorString")]
pub struct SourceRuntimeAllocationError { message: &'static str }
impl fmt::Display for SourceRuntimeAllocationError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { write!(out, "runtime error: {}", self.message) }
}
impl Error for SourceRuntimeAllocationError {}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deftly)]
#[derive_deftly(SourceErrorOrigin)]
#[deftly(source_package = "runtime", source_struct = "plainError")]
pub struct SourceNilMapAssignment;
impl fmt::Display for SourceNilMapAssignment {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result { out.write_str("assignment to entry in nil map") }
}
impl Error for SourceNilMapAssignment {}

/// A source recoverable panic must reach its owning recovery/fatal policy
/// before output. resume_unwind skips Rust's eager panic hook. Source
/// traceback/caller/abort-profile behavior still requires its runtime owner.
pub(crate) fn unwind_source_panic<T: Any + Send>(payload: T) -> ! { std::panic::resume_unwind(Box::new(payload)) }

#[cgp_component(SourceCollectionFailureRaiser)]
pub trait CanRaiseSourceCollectionFailure {
    fn raise_source_allocation_failure(&self, error: SourceSliceAllocationError) -> !;
    fn raise_source_nil_map_assignment(&self) -> !;
}
#[cgp_new_provider]
impl<Context: CanExitSourceProcess> SourceCollectionFailureRaiser<Context> for RaiseSourceCollectionFailure {
    fn raise_source_allocation_failure(context: &Context, error: SourceSliceAllocationError) -> ! { raise_source_allocation_failure(context, error) }
    fn raise_source_nil_map_assignment(_context: &Context) -> ! { unwind_source_panic(SourceNilMapAssignment) }
}
pub(crate) fn raise_source_allocation_failure(context: &impl CanExitSourceProcess, error: SourceSliceAllocationError) -> ! {
    let message = match error {
        SourceSliceAllocationError::MakeLength => "makeslice: len out of range",
        SourceSliceAllocationError::MakeCapacity => "makeslice: cap out of range",
        SourceSliceAllocationError::GrowLength => "growslice: len out of range",
        SourceSliceAllocationError::NativeAddressSpace => unwind_source_panic(error),
        SourceSliceAllocationError::OutOfMemory => {
            let _ = std::io::stderr().write_all(b"fatal error: out of memory\n");
            context.exit_source_process(2);
            std::process::abort();
        }
    };
    unwind_source_panic(SourceRuntimeAllocationError { message })
}

/// Stateless native projection of the same provider used by the existing
/// runtime context. It owns no resources, task, registry or application
/// lifecycle; callers choosing another static context use the fallible API.
#[derive(Deftly)]
#[derive_deftly(NativeSourceCollectionProviders)]
pub struct NativeSourceCollectionContext;
pub fn complete_source_collection<T>(context: &impl CanRaiseSourceCollectionFailure, result: Result<T, SourceSliceAllocationError>) -> T {
    match result { Ok(value) => value, Err(error) => context.raise_source_allocation_failure(error) }
}
pub(crate) fn complete_native_source_collection<T>(result: Result<T, SourceSliceAllocationError>) -> T {
    complete_source_collection(&NativeSourceCollectionContext, result)
}
pub(crate) fn nil_source_map_assignment() -> ! { NativeSourceCollectionContext.raise_source_nil_map_assignment() }
