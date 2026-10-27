//! Source allocation capacities for HTTP []string and reader byte buffers.
// Adapted from Go 1.26.5 runtime/{slice,msize}.go and internal/runtime/gc.
// Copyright The Go Authors. BSD-3-Clause; see ../GO-LICENSE.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeaderAllocationProfile { word_bits: u8, heap_address_bits: u8 }
impl HeaderAllocationProfile {
    pub const GO126_64: Self = Self { word_bits: 64, heap_address_bits: 48 };
    pub const GO126_32: Self = Self { word_bits: 32, heap_address_bits: 32 };
    pub const fn go126(word_bits: u8, heap_address_bits: u8) -> Self {
        assert!(word_bits == 32 || word_bits == 64);
        assert!(heap_address_bits > 0 && heap_address_bits < 64 && heap_address_bits <= word_bits);
        Self { word_bits, heap_address_bits }
    }
    fn word(self, value: u64) -> i64 { if self.word_bits == 32 { value as u32 as i32 as i64 } else { value as i64 } }
    fn add(self, left: i64, right: i64) -> i64 { self.word((left as u64).wrapping_add(right as u64)) }
    fn unsigned(self, value: i64) -> u64 { if self.word_bits == 32 { value as u32 as u64 } else { value as u64 } }
    fn max_alloc(self) -> u64 { (1u64 << self.heap_address_bits) - u64::from(self.word_bits == 32) }
    pub(super) fn clone_length(self, lengths: impl Iterator<Item = usize>) -> usize {
        let mut length = 0;
        for next in lengths { length = self.add(length, next as i64); }
        if length < 0 || length as u64 > self.max_alloc() / (self.word_bits / 4) as u64 { panic!("makeslice: len out of range"); }
        usize::try_from(length).unwrap_or_else(|_| panic!("makeslice: len out of range"))
    }
    pub(super) fn validate(self, length: usize) {
        let size = (self.word_bits / 4) as u64; // two source words per string
        if length as u128 > ((1u128 << (self.word_bits - 1)) - 1) || length as u128 * size as u128 > self.max_alloc() as u128 {
            panic!("growslice: len out of range");
        }
    }
    pub(super) fn grow(self, old: usize, length: usize) -> usize {
        self.validate(length);
        let capacity = self.next_capacity(old, length);
        let string_size = (self.word_bits / 4) as u64;
        if capacity as u64 > self.max_alloc() / string_size { panic!("growslice: len out of range"); }
        let rounded = self.roundup(capacity as u64 * string_size, false);
        if rounded > self.max_alloc() { panic!("growslice: len out of range"); }
        usize::try_from(rounded / string_size).unwrap_or_else(|_| panic!("growslice: len out of range"))
    }
    pub(crate) fn grow_bytes(self, old: usize, length: usize) -> usize {
        self.validate_byte_capacity(length as u128, "growslice: len out of range");
        let capacity = self.next_capacity(old, length);
        if capacity as u64 > self.max_alloc() { panic!("growslice: len out of range"); }
        let rounded = self.roundup(capacity as u64, true);
        self.validate_byte_capacity(rounded as u128, "growslice: len out of range");
        usize::try_from(rounded).unwrap_or_else(|_| panic!("growslice: len out of range"))
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
        if capacity > ((1u128 << (self.word_bits - 1)) - 1) || capacity > self.max_alloc() as u128 { panic!("{message}"); }
    }
    fn next_capacity(self, old: usize, length: usize) -> i64 {
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
