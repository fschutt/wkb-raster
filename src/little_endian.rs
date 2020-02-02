
/// Converts a single byte into two hex chars (little endian)
#[inline]
pub fn byte_to_hex_chars_le(byte: u8) -> [u8;2] {
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
pub fn write_bool_le(s: &mut Vec<u8>, input: bool) {
    s.extend_from_slice(&byte_to_hex_chars_le(input as u8));
}

#[inline]
pub fn write_i8_le(s: &mut Vec<u8>, input: i8) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}

#[inline]
pub fn write_u8_le(s: &mut Vec<u8>, input: u8) {
    s.extend_from_slice(&byte_to_hex_chars_le(input));
}

#[inline]
pub fn write_i16_le(s: &mut Vec<u8>, input: i16) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}

#[inline]
pub fn write_u16_le(s: &mut Vec<u8>, input: u16) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}

#[inline]
pub fn write_i32_le(s: &mut Vec<u8>, input: i32) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}

#[inline]
pub fn write_u32_le(s: &mut Vec<u8>, input: u32) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}

#[inline]
pub fn write_f32_le(s: &mut Vec<u8>, input: f32) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}

#[inline]
pub fn write_f64_le(s: &mut Vec<u8>, input: f64) {
    for b in &input.to_le_bytes() { s.extend_from_slice(&byte_to_hex_chars_le(*b)); }
}