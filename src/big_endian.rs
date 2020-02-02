/// Converts a single byte into two hex chars (big endian)
#[inline]
pub fn byte_to_hex_chars_be(byte: u8) -> [u8;2] {
    // byte = 230 = 0xE6
    // msb = 0xE = 14
    // lsb = 0x6 = 6
    let msb = byte >> 4;
    let lsb = byte & 0b00001111;

    // convert to ascii char
    let (msb, _) = if msb > 9 { msb.overflowing_add(55) } else { msb.overflowing_add(48) };
    let (lsb, _) = if lsb > 9 { lsb.overflowing_add(55) } else { lsb.overflowing_add(48) };

    [msb, lsb]
}

#[inline]
pub fn write_bool_be(s: &mut Vec<u8>, input: bool) {
    s.extend_from_slice(&byte_to_hex_chars_be(input as u8));
}

#[inline]
pub fn write_i8_be(s: &mut Vec<u8>, input: i8) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}

#[inline]
pub fn write_u8_be(s: &mut Vec<u8>, input: u8) {
    s.extend_from_slice(&byte_to_hex_chars_be(input));
}

#[inline]
pub fn write_i16_be(s: &mut Vec<u8>, input: i16) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}

#[inline]
pub fn write_u16_be(s: &mut Vec<u8>, input: u16) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}

#[inline]
pub fn write_i32_be(s: &mut Vec<u8>, input: i32) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}

#[inline]
pub fn write_u32_be(s: &mut Vec<u8>, input: u32) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}

#[inline]
pub fn write_f32_be(s: &mut Vec<u8>, input: f32) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}

#[inline]
pub fn write_f64_be(s: &mut Vec<u8>, input: f64) {
    for b in &input.to_be_bytes() { s.extend_from_slice(&byte_to_hex_chars_be(*b)); }
}