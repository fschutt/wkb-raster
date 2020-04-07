use crate::BoolParseError;

// --- writing

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

// --- parsing

/// Converts two hex characters into a single byte (big endian)
#[inline]
pub fn hex_chars_to_byte_be([msb, lsb]: [u8;2]) -> u8 {

    // convert to byte
    let (msb, _) = if msb > 64 { msb.overflowing_sub(55) } else { msb.overflowing_sub(48) };
    let (lsb, _) = if lsb > 64 { lsb.overflowing_sub(55) } else { lsb.overflowing_sub(48) };
    
    lsb + (msb << 4)
}

#[inline]
pub fn parse_bool_be(input: [u8;2]) -> Result<bool, BoolParseError> {
    let byte = hex_chars_to_byte_be(input);
    match byte {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(BoolParseError(input, byte)),
    }
}

#[inline]
pub fn parse_i8_be(input: [u8;2]) -> i8 {
    i8::from_be_bytes([hex_chars_to_byte_be(input)])
}


#[inline]
pub fn parse_u8_be(input: [u8;2]) -> u8 {
    u8::from_be_bytes([hex_chars_to_byte_be(input)])
}

#[inline]
pub fn parse_i16_be(input: [u8;4]) -> i16 {
    i16::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
    ])
}

#[inline]
pub fn parse_u16_be(input: [u8;4]) -> u16 {
    u16::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
    ])
}

#[inline]
pub fn parse_i32_be(input: [u8;8]) -> i32 {
    i32::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
        hex_chars_to_byte_be([input[4], input[5]]),
        hex_chars_to_byte_be([input[6], input[7]]),
    ])
}

#[inline]
pub fn parse_u32_be(input: [u8;8]) -> u32 {
    u32::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
        hex_chars_to_byte_be([input[4], input[5]]),
        hex_chars_to_byte_be([input[6], input[7]]),
    ])
}

#[inline]
pub fn parse_i64_be(input: [u8;16]) -> i64 {
    i64::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
        hex_chars_to_byte_be([input[4], input[5]]),
        hex_chars_to_byte_be([input[6], input[7]]),
        
        hex_chars_to_byte_be([input[8], input[9]]),
        hex_chars_to_byte_be([input[10], input[11]]),
        hex_chars_to_byte_be([input[12], input[13]]),
        hex_chars_to_byte_be([input[14], input[15]]),
    ])
}

#[inline]
pub fn parse_u64_be(input: [u8;16]) -> u64 {
    u64::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
        hex_chars_to_byte_be([input[4], input[5]]),
        hex_chars_to_byte_be([input[6], input[7]]),
        
        hex_chars_to_byte_be([input[8], input[9]]),
        hex_chars_to_byte_be([input[10], input[11]]),
        hex_chars_to_byte_be([input[12], input[13]]),
        hex_chars_to_byte_be([input[14], input[15]]),
    ])
}

#[inline]
pub fn parse_f32_be(input: [u8;8]) -> f32 {
    f32::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
        hex_chars_to_byte_be([input[4], input[5]]),
        hex_chars_to_byte_be([input[6], input[7]]),
    ])
}

#[inline]
pub fn parse_f64_be(input: [u8;16]) -> f64 {
    f64::from_be_bytes([
        hex_chars_to_byte_be([input[0], input[1]]),
        hex_chars_to_byte_be([input[2], input[3]]),
        hex_chars_to_byte_be([input[4], input[5]]),
        hex_chars_to_byte_be([input[6], input[7]]),
        
        hex_chars_to_byte_be([input[8], input[9]]),
        hex_chars_to_byte_be([input[10], input[11]]),
        hex_chars_to_byte_be([input[12], input[13]]),
        hex_chars_to_byte_be([input[14], input[15]]),
    ])
}

