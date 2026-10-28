//! Pattern opcode backing and explicit Rust-to-source length domains.
// Source slice growth follows Go runtime/slice.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. Pattern ownership follows grpc-gateway;
// Copyright gRPC authors. Apache-2.0; see ../GRPC-LICENSE.
use super::PatternSourceProfile;
use crate::utility_http::{HeaderValues, SourceSliceAllocationError, reserve_source_array};
use std::{error::Error, fmt, sync::Arc};

/// This is a rejected Rust bridge input, not a Go error, bounds panic or
/// overflow outcome: a Go header or address-sized data span could not
/// represent this length under the selected source word profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatternInputDomainError { pub parameter: &'static str, pub length: usize, pub maximum: u64 }
impl fmt::Display for PatternInputDomainError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "native bridge: {} length {} exceeds source Go input limit {}", self.parameter, self.length, self.maximum)
    }
}
impl Error for PatternInputDomainError {}
pub(super) fn input_length(profile: PatternSourceProfile, parameter: &'static str, length: usize) -> Result<(), PatternInputDomainError> {
    let maximum = profile.word().max_int();
    if length as u128 > maximum as u128 { return Err(PatternInputDomainError { parameter, length, maximum }); }
    Ok(())
}
pub(super) fn input_array(profile: PatternSourceProfile, parameter: &'static str, length: usize, element_size: u64) -> Result<(), PatternInputDomainError> {
    let unsigned = if profile.allocation().word_bits() == 32 { u32::MAX as u64 } else { u64::MAX };
    let maximum = profile.word().max_int().min(if element_size == 0 { unsigned } else { unsigned / element_size });
    if length as u128 > maximum as u128 { return Err(PatternInputDomainError { parameter, length, maximum }); }
    Ok(())
}
pub(super) fn input_strings(profile: PatternSourceProfile, parameter: &'static str, values: &HeaderValues) -> Result<(), PatternInputDomainError> {
    input_length(profile, parameter, values.len())?;
    input_array(profile, parameter, values.capacity(), (profile.allocation().word_bits() / 4) as u64)?;
    for value in values.iter() { input_length(profile, parameter, value.len())?; }
    Ok(())
}

/// Returned Pattern copies share the immutable typed-op backing, as source
/// slice-header copies do. Cloning a route must not deep-copy every opcode.
#[derive(Clone, Debug, Default)]
pub(super) struct PatternOperations { backing: Option<Arc<Vec<(i64, i64)>>>, source_capacity: u64 }
impl PatternOperations {
    pub(super) fn iter(&self) -> impl Iterator<Item = &(i64, i64)> {
        self.backing.iter().flat_map(|values| values.iter())
    }
}
#[derive(Default)]
pub(super) struct PatternOperationBuilder { values: Vec<(i64, i64)>, source_capacity: u64 }
impl PatternOperationBuilder {
    pub(super) fn push(&mut self, profile: PatternSourceProfile, operation: (i64, i64)) -> Result<(), SourceSliceAllocationError> {
        let length = self.values.len().checked_add(1).ok_or(SourceSliceAllocationError::GrowLength)?;
        if length as u64 > self.source_capacity {
            // op contains two source ints and no GC pointers. []string has
            // the same word count but a different scanning/header profile.
            let size = (profile.allocation().word_bits() / 4) as u64;
            let layout = profile.allocation().grow_slice_layout(self.source_capacity, length as u64, size, true)?;
            let mut grown = reserve_source_array::<(i64, i64)>(layout.capacity)?;
            grown.append(&mut self.values); self.values = grown; self.source_capacity = layout.capacity;
        }
        self.values.push(operation); Ok(())
    }
    pub(super) fn finish(self) -> PatternOperations {
        PatternOperations { backing: if self.values.is_empty() { None } else { Some(Arc::new(self.values)) }, source_capacity: self.source_capacity }
    }
}
