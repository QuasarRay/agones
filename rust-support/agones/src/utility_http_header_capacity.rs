//! Source allocation capacities shared by HTTP buffers and gateway patterns.
// Adapted from Go 1.26.5 runtime/{slice,msize}.go and internal/runtime/gc.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceSliceAllocationError { MakeLength, MakeCapacity, GrowLength, OutOfMemory, NativeAddressSpace }
impl fmt::Display for SourceSliceAllocationError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(match self {
            Self::MakeLength => "runtime error: makeslice: len out of range",
            Self::MakeCapacity => "runtime error: makeslice: cap out of range",
            Self::GrowLength => "runtime error: growslice: len out of range",
            Self::OutOfMemory => "out of memory",
            Self::NativeAddressSpace => "source allocation exceeds native Rust address space",
        })
    }
}
impl Error for SourceSliceAllocationError {}
/// Unsigned capacity bits and the byte request passed to source mallocgc.
/// For Go32 byte slices, the capacity's high bit can be signed in the Go
/// slice header; it must not disappear in a native Rust conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSliceAllocationLayout { pub capacity: u64, pub allocation_bytes: u64 }

pub(crate) fn reserve_source_array<T>(capacity: u64) -> Result<Vec<T>, SourceSliceAllocationError> {
    if capacity as u128 > usize::MAX as u128 || capacity as u128 * std::mem::size_of::<T>() as u128 > isize::MAX as u128 {
        return Err(SourceSliceAllocationError::NativeAddressSpace);
    }
    let mut result = Vec::new();
    result.try_reserve_exact(capacity as usize).map_err(|_| SourceSliceAllocationError::OutOfMemory)?;
    Ok(result)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderAllocationProfile { word_bits: u8, heap_address_bits: u8 }
impl HeaderAllocationProfile {
    pub const GO126_64: Self = Self { word_bits: 64, heap_address_bits: 48 };
    pub const GO126_32: Self = Self { word_bits: 32, heap_address_bits: 32 };
    pub const fn word_bits(self) -> u8 { self.word_bits }
    pub const fn go126(word_bits: u8, heap_address_bits: u8) -> Self {
        assert!(word_bits == 32 || word_bits == 64);
        assert!(heap_address_bits > 0 && heap_address_bits < 64 && heap_address_bits <= word_bits);
        Self { word_bits, heap_address_bits }
    }
    fn word(self, value: u64) -> i64 { if self.word_bits == 32 { value as u32 as i32 as i64 } else { value as i64 } }
    fn add(self, left: i64, right: i64) -> i64 { self.word((left as u64).wrapping_add(right as u64)) }
    fn unsigned(self, value: i64) -> u64 { if self.word_bits == 32 { value as u32 as u64 } else { value as u64 } }
    pub(crate) fn max_alloc(self) -> u64 { (1u64 << self.heap_address_bits) - u64::from(self.word_bits == 32) }
    pub fn make_slice_layout(self, length: i64, capacity: i64, element_size: u64) -> Result<SourceSliceAllocationLayout, SourceSliceAllocationError> {
        let maximum = (1u128 << (self.word_bits - 1)) - 1;
        let invalid_length = length < 0 || length as u128 > maximum || length as u128 * element_size as u128 > self.max_alloc() as u128;
        if invalid_length { return Err(SourceSliceAllocationError::MakeLength); }
        if capacity < 0 || capacity as u128 > maximum || length > capacity || capacity as u128 * element_size as u128 > self.max_alloc() as u128 {
            return Err(SourceSliceAllocationError::MakeCapacity);
        }
        Ok(SourceSliceAllocationLayout { capacity: capacity as u64, allocation_bytes: capacity as u64 * element_size })
    }
    pub fn grow_slice_layout(self, old: u64, length: u64, element_size: u64, no_scan: bool) -> Result<SourceSliceAllocationLayout, SourceSliceAllocationError> {
        if length as u128 > (1u128 << (self.word_bits - 1)) - 1 { return Err(SourceSliceAllocationError::GrowLength); }
        if element_size == 0 { return Ok(SourceSliceAllocationLayout { capacity: length, allocation_bytes: 0 }); }
        let capacity = self.unsigned(self.next_capacity(old, length));
        if capacity as u128 * element_size as u128 > self.max_alloc() as u128 { return Err(SourceSliceAllocationError::GrowLength); }
        let rounded = self.roundup(capacity * element_size, no_scan);
        let capacity = rounded / element_size;
        let allocation_bytes = capacity * element_size;
        if allocation_bytes > self.max_alloc() { return Err(SourceSliceAllocationError::GrowLength); }
        Ok(SourceSliceAllocationLayout { capacity, allocation_bytes })
    }
    pub(super) fn try_clone_length(self, lengths: impl Iterator<Item = usize>) -> Result<usize, SourceSliceAllocationError> {
        let mut length = 0;
        for next in lengths { length = self.add(length, next as i64); }
        self.make_slice_layout(length, length, (self.word_bits / 4) as u64)?;
        usize::try_from(length).map_err(|_| SourceSliceAllocationError::NativeAddressSpace)
    }
    pub(super) fn validate(self, length: usize) {
        let size = (self.word_bits / 4) as u64; // two source words per string
        if length as u128 > ((1u128 << (self.word_bits - 1)) - 1) || length as u128 * size as u128 > self.max_alloc() as u128 {
            panic!("growslice: len out of range");
        }
    }
    pub(super) fn grow(self, old: usize, length: usize) -> usize {
        let layout = self.grow_slice_layout(old as u64, length as u64, (self.word_bits / 4) as u64, false)
            .unwrap_or_else(|_| panic!("growslice: len out of range"));
        usize::try_from(layout.capacity).unwrap_or_else(|_| panic!("growslice: len out of range"))
    }
    pub(crate) fn grow_bytes(self, old: usize, length: usize) -> usize {
        self.try_grow_bytes(old, length).unwrap_or_else(|| panic!("growslice: len out of range"))
    }
    /// Shared source calculation; callers retain their own panic/effect
    /// representation instead of catching a Rust panic to inspect its text.
    pub(crate) fn try_grow_bytes(self, old: usize, length: usize) -> Option<usize> {
        let rounded = self.source_grow_byte_capacity(old as u64, length as u64)?;
        // Retain the existing HTTP buffer projection. Complete signed Go32
        // slice-header capacities remain a separate required boundary.
        if !self.valid_byte_capacity(rounded as u128) { return None; }
        usize::try_from(rounded).ok()
    }
    /// Source-sized result precedes conversion to any native Rust capacity.
    pub(crate) fn source_grow_byte_capacity(self, old: u64, length: u64) -> Option<u64> {
        self.grow_slice_layout(old, length, 1, true).ok().map(|layout| layout.capacity)
    }
    /// A fresh strings.Builder.Grow calls MakeNoZero(n). Its requested
    /// length check precedes allocator rounding; the returned capacity is
    /// the rounded no-pointer size, not n or Rust Vec::capacity().
    pub(crate) fn make_no_zero_byte_capacity(self, length: u128) -> Option<u64> {
        if !self.valid_byte_capacity(length) { return None; }
        Some(self.roundup(length as u64, true))
    }
    /// encoding/json.refill uses make([]byte, len, 2*cap+512); make retains
    /// the requested capacity rather than growslice's allocator size class.
    pub(crate) fn json_buffer_capacity(self, old: usize) -> usize {
        let capacity = self.add(self.add(old as i64, old as i64), 512);
        if capacity < 0 { panic!("makeslice: cap out of range"); }
        self.validate_byte_capacity(capacity as u128, "makeslice: cap out of range");
        usize::try_from(capacity).unwrap_or_else(|_| panic!("makeslice: cap out of range"))
    }
    fn validate_byte_capacity(self, capacity: u128, message: &str) {
        if !self.valid_byte_capacity(capacity) { panic!("{message}"); }
    }
    fn valid_byte_capacity(self, capacity: u128) -> bool {
        capacity <= ((1u128 << (self.word_bits - 1)) - 1) && capacity <= self.max_alloc() as u128
    }
    fn next_capacity(self, old: u64, length: u64) -> i64 {
        let requested = length as i64;
        let mut capacity = old as i64;
        let double = self.add(capacity, capacity);
        if requested > double { capacity = requested; }
        else if old < 256 { capacity = double; }
        else {
            loop {
                capacity = self.add(capacity, self.add(capacity, 3 * 256) >> 2);
                if self.unsigned(capacity) >= requested as u64 { break; }
            }
            if capacity <= 0 { capacity = requested; }
        }
        capacity
    }
    fn roundup(self, bytes: u64, no_scan: bool) -> u64 {
        if bytes <= 32768 - 8 {
            let header = if !no_scan && bytes > (self.word_bits as u64 / 8) * self.word_bits as u64 { 8 } else { 0 };
            let needed = bytes + header;
            SIZE_CLASSES.iter().copied().find(|size| *size >= needed).expect("source small size class") - header
        } else {
            let advanced = if self.word_bits == 32 { (bytes as u32).wrapping_add(8191) as u64 } else { bytes.wrapping_add(8191) };
            if advanced < bytes { bytes } else { advanced & !8191 }
        }
    }
}
const SIZE_CLASSES: &[u64] = &[0,8,16,24,32,48,64,80,96,112,128,144,160,176,192,208,224,240,256,288,320,352,384,416,448,480,512,576,640,704,768,896,1024,1152,1280,1408,1536,1792,2048,2304,2688,3072,3200,3456,4096,4864,5376,6144,6528,6784,6912,8192,9472,9728,10240,10880,12288,13568,14336,16384,18432,19072,20480,21760,24576,27264,28672,32768];
