//! Bounded multiprecision decimal used by source reference float conversion.
// Adapted from Go internal/strconv/decimal.go. Copyright The Go Authors.
// BSD-3-Clause; see ../GO-LICENSE. This is formatter arithmetic, not a general
// numeric runtime. Optimized Dragonbox/fixed paths remain separate work.
pub(crate) struct SourceDecimal {
    pub digits: [u8; 800],
    pub count: i32,
    pub point: i32,
    pub truncated: bool,
}
impl Default for SourceDecimal {
    fn default() -> Self { Self { digits: [0; 800], count: 0, point: 0, truncated: false } }
}
impl SourceDecimal {
    pub fn assign(&mut self, mut value: u64) {
        let mut reversed = [0; 24]; let mut length = 0;
        while value != 0 { reversed[length] = b'0' + (value % 10) as u8; length += 1; value /= 10; }
        self.count = length as i32;
        for index in 0..length { self.digits[index] = reversed[length - index - 1]; }
        self.point = self.count; self.trim();
    }
    fn trim(&mut self) {
        while self.count > 0 && self.digits[(self.count - 1) as usize] == b'0' { self.count -= 1; }
        if self.count == 0 { self.point = 0; }
    }
    pub fn shift(&mut self, mut bits: i32) {
        if self.count == 0 { return; }
        // This selected arithmetic path uses Go64's source uint chunk bound.
        // Full Go32/native allocation/word/configuration profiles stay open.
        while bits > 60 { self.left_shift(60); bits -= 60; }
        while bits < -60 { self.right_shift(60); bits += 60; }
        if bits > 0 { self.left_shift(bits as usize); }
        else if bits < 0 { self.right_shift((-bits) as u32); }
    }
    fn right_shift(&mut self, bits: u32) {
        let mut read = 0; let mut write = 0; let mut value = 0u64;
        while value >> bits == 0 {
            if read >= self.count {
                if value == 0 { self.count = 0; return; }
                while value >> bits == 0 { value *= 10; read += 1; }
                break;
            }
            value = value * 10 + u64::from(self.digits[read as usize] - b'0'); read += 1;
        }
        self.point -= read - 1;
        let mask = (1u64 << bits) - 1;
        while read < self.count {
            let digit = value >> bits; value &= mask;
            self.digits[write] = b'0' + digit as u8; write += 1;
            value = value * 10 + u64::from(self.digits[read as usize] - b'0'); read += 1;
        }
        while value > 0 {
            let digit = value >> bits; value &= mask;
            if write < self.digits.len() { self.digits[write] = b'0' + digit as u8; write += 1; }
            else if digit > 0 { self.truncated = true; }
            value *= 10;
        }
        self.count = write as i32; self.trim();
    }
    fn left_shift(&mut self, bits: usize) {
        let (mut delta, cutoff) = LEFT_CHEATS[bits];
        if prefix_less(&self.digits[..self.count as usize], cutoff) { delta -= 1; }
        let mut read = self.count; let mut write = self.count + delta; let mut value = 0u64;
        while read > 0 {
            read -= 1; value += u64::from(self.digits[read as usize] - b'0') << bits;
            let quotient = value / 10; let remainder = value - quotient * 10; write -= 1;
            if write < self.digits.len() as i32 { self.digits[write as usize] = b'0' + remainder as u8; }
            else if remainder != 0 { self.truncated = true; }
            value = quotient;
        }
        while value > 0 {
            let quotient = value / 10; let remainder = value - quotient * 10; write -= 1;
            if write < self.digits.len() as i32 { self.digits[write as usize] = b'0' + remainder as u8; }
            else if remainder != 0 { self.truncated = true; }
            value = quotient;
        }
        self.count = (self.count + delta).min(self.digits.len() as i32);
        self.point += delta; self.trim();
    }
    fn should_round_up(&self, count: i32) -> bool {
        if count < 0 || count >= self.count { return false; }
        if self.digits[count as usize] == b'5' && count + 1 == self.count {
            return self.truncated || (count > 0 && (self.digits[(count - 1) as usize] - b'0') % 2 != 0);
        }
        self.digits[count as usize] >= b'5'
    }
    pub fn round(&mut self, count: i32) {
        if count < 0 || count >= self.count { return; }
        if self.should_round_up(count) { self.round_up(count); } else { self.round_down(count); }
    }
    pub fn round_down(&mut self, count: i32) {
        if count < 0 || count >= self.count { return; }
        self.count = count; self.trim();
    }
    pub fn round_up(&mut self, count: i32) {
        if count < 0 || count >= self.count { return; }
        let mut index = count;
        while index > 0 {
            index -= 1;
            if self.digits[index as usize] < b'9' {
                self.digits[index as usize] += 1; self.count = index + 1; return;
            }
        }
        self.digits[0] = b'1'; self.count = 1; self.point += 1;
    }
}
fn prefix_less(bytes: &[u8], cutoff: &[u8]) -> bool {
    for (index, &digit) in cutoff.iter().enumerate() {
        if index >= bytes.len() { return true; }
        if bytes[index] != digit { return bytes[index] < digit; }
    }
    false
}
// Source decimal digit-growth/cutoff data, indexed by binary shift count.
const LEFT_CHEATS: &[(i32, &[u8])] = &[
    (0,b""),(1,b"5"),(1,b"25"),(1,b"125"),(2,b"625"),(2,b"3125"),(2,b"15625"),
    (3,b"78125"),(3,b"390625"),(3,b"1953125"),(4,b"9765625"),(4,b"48828125"),
    (4,b"244140625"),(4,b"1220703125"),(5,b"6103515625"),(5,b"30517578125"),
    (5,b"152587890625"),(6,b"762939453125"),(6,b"3814697265625"),(6,b"19073486328125"),
    (7,b"95367431640625"),(7,b"476837158203125"),(7,b"2384185791015625"),(7,b"11920928955078125"),
    (8,b"59604644775390625"),(8,b"298023223876953125"),(8,b"1490116119384765625"),
    (9,b"7450580596923828125"),(9,b"37252902984619140625"),(9,b"186264514923095703125"),
    (10,b"931322574615478515625"),(10,b"4656612873077392578125"),(10,b"23283064365386962890625"),
    (10,b"116415321826934814453125"),(11,b"582076609134674072265625"),(11,b"2910383045673370361328125"),
    (11,b"14551915228366851806640625"),(12,b"72759576141834259033203125"),(12,b"363797880709171295166015625"),
    (12,b"1818989403545856475830078125"),(13,b"9094947017729282379150390625"),(13,b"45474735088646411895751953125"),
    (13,b"227373675443232059478759765625"),(13,b"1136868377216160297393798828125"),
    (14,b"5684341886080801486968994140625"),(14,b"28421709430404007434844970703125"),
    (14,b"142108547152020037174224853515625"),(15,b"710542735760100185871124267578125"),
    (15,b"3552713678800500929355621337890625"),(15,b"17763568394002504646778106689453125"),
    (16,b"88817841970012523233890533447265625"),(16,b"444089209850062616169452667236328125"),
    (16,b"2220446049250313080847263336181640625"),(16,b"11102230246251565404236316680908203125"),
    (17,b"55511151231257827021181583404541015625"),(17,b"277555756156289135105907917022705078125"),
    (17,b"1387778780781445675529539585113525390625"),(18,b"6938893903907228377647697925567626953125"),
    (18,b"34694469519536141888238489627838134765625"),(18,b"173472347597680709441192448139190673828125"),
    (19,b"867361737988403547205962240695953369140625"),
];
