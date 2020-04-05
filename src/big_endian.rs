use crate::ParseError;

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
pub fn parse_bool_be<'a>(input: &'a [u8]) -> Result<bool, ParseError<'a>> {
    let [msb, lsb] = match input {
        [msb, lsb] => [*msb, *lsb],
        _ => return Err(ParseError::WrongInputSize { expected_len: 2, got: input }),
    };
    let byte = hex_chars_to_byte_be([msb, lsb]);
    
    match byte {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ParseError::UnableToParseBool([msb, lsb], byte)),
    }
}

#[inline]
pub fn parse_i8_be<'a>(input: &'a [u8]) -> Result<i8, ParseError<'a>> {
    let sized_input: [[u8;2];1] = match input {
        [msb, lsb] => [[*msb, *lsb]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 2, got: input }),
    };
    let bytes = [hex_chars_to_byte_be(sized_input[0])];
    Ok(i8::from_be_bytes(bytes))
}


#[inline]
pub fn parse_u8_be<'a>(input: &'a [u8]) -> Result<u8, ParseError<'a>> {
    let sized_input: [[u8;2];1] = match input {
        [msb, lsb] => [[*msb, *lsb]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 2, got: input }),
    };
    let bytes = [hex_chars_to_byte_be(sized_input[0])];
    Ok(u8::from_be_bytes(bytes))
}

#[inline]
pub fn parse_i16_be<'a>(input: &'a [u8]) -> Result<i16, ParseError<'a>> {
    let sized_input: [[u8;2];2] = match input {
        [msb1, lsb1, msb2, lsb2] => [[*msb1, *lsb1], [*msb2, *lsb2]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 4, got: input }),
    };
    let bytes = [hex_chars_to_byte_be(sized_input[0]), hex_chars_to_byte_be(sized_input[1])];
    Ok(i16::from_be_bytes(bytes))
}

#[inline]
pub fn parse_u16_be<'a>(input: &'a [u8]) -> Result<u16, ParseError<'a>> {
    let sized_input: [[u8;2];2] = match input {
        [msb1, lsb1, msb2, lsb2] => [[*msb1, *lsb1], [*msb2, *lsb2]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 4, got: input }),
    };
    let bytes = [hex_chars_to_byte_be(sized_input[0]), hex_chars_to_byte_be(sized_input[1])];
    Ok(u16::from_be_bytes(bytes))
}

#[inline]
pub fn parse_i32_be<'a>(input: &'a [u8]) -> Result<i32, ParseError<'a>> {
    let sized_input: [[u8;2];4] = match input {
        [msb1, lsb1, msb2, lsb2, msb3, lsb3, msb4, lsb4] => [[*msb1, *lsb1], [*msb2, *lsb2], [*msb3, *lsb3], [*msb4, *lsb4]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 8, got: input }),
    };
    let bytes = [
        hex_chars_to_byte_be(sized_input[0]), 
        hex_chars_to_byte_be(sized_input[1]),
        hex_chars_to_byte_be(sized_input[2]),
        hex_chars_to_byte_be(sized_input[3]),
    ];
    Ok(i32::from_be_bytes(bytes))
}

#[inline]
pub fn parse_u32_be<'a>(input: &'a [u8]) -> Result<u32, ParseError<'a>> {
    let sized_input: [[u8;2];4] = match input {
        [msb1, lsb1, msb2, lsb2, msb3, lsb3, msb4, lsb4] => [[*msb1, *lsb1], [*msb2, *lsb2], [*msb3, *lsb3], [*msb4, *lsb4]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 8, got: input }),
    };
    let bytes = [
        hex_chars_to_byte_be(sized_input[0]), 
        hex_chars_to_byte_be(sized_input[1]),
        hex_chars_to_byte_be(sized_input[2]),
        hex_chars_to_byte_be(sized_input[3]),
    ];
    Ok(u32::from_be_bytes(bytes))
}

#[inline]
pub fn parse_i64_be<'a>(input: &'a [u8]) -> Result<i64, ParseError<'a>> {
    let sized_input: [[u8;2];8] = match input {
        [
         msb1, lsb1, msb2, lsb2, msb3, lsb3, msb4, lsb4,
         msb5, lsb5, msb6, lsb6, msb7, lsb7, msb8, lsb8,
        ] => [
            [*msb1, *lsb1], [*msb2, *lsb2], [*msb3, *lsb3], [*msb4, *lsb4],
            [*msb5, *lsb5], [*msb6, *lsb6], [*msb7, *lsb7], [*msb8, *lsb8]
        ],
        _ => return Err(ParseError::WrongInputSize { expected_len: 16, got: input }),
    };
    let bytes = [
        hex_chars_to_byte_be(sized_input[0]), 
        hex_chars_to_byte_be(sized_input[1]),
        hex_chars_to_byte_be(sized_input[2]),
        hex_chars_to_byte_be(sized_input[3]),
        hex_chars_to_byte_be(sized_input[4]),
        hex_chars_to_byte_be(sized_input[5]),
        hex_chars_to_byte_be(sized_input[6]),
        hex_chars_to_byte_be(sized_input[7]),
    ];
    Ok(i64::from_be_bytes(bytes))
}

#[inline]
pub fn parse_u64_be<'a>(input: &'a [u8]) -> Result<u64, ParseError<'a>> {
    let sized_input: [[u8;2];8] = match input {
        [
         msb1, lsb1, msb2, lsb2, msb3, lsb3, msb4, lsb4,
         msb5, lsb5, msb6, lsb6, msb7, lsb7, msb8, lsb8,
        ] => [
            [*msb1, *lsb1], [*msb2, *lsb2], [*msb3, *lsb3], [*msb4, *lsb4],
            [*msb5, *lsb5], [*msb6, *lsb6], [*msb7, *lsb7], [*msb8, *lsb8]
        ],
        _ => return Err(ParseError::WrongInputSize { expected_len: 16, got: input }),
    };
    let bytes = [
        hex_chars_to_byte_be(sized_input[0]), 
        hex_chars_to_byte_be(sized_input[1]),
        hex_chars_to_byte_be(sized_input[2]),
        hex_chars_to_byte_be(sized_input[3]),
        hex_chars_to_byte_be(sized_input[4]),
        hex_chars_to_byte_be(sized_input[5]),
        hex_chars_to_byte_be(sized_input[6]),
        hex_chars_to_byte_be(sized_input[7]),
    ];
    Ok(u64::from_be_bytes(bytes))
}

#[inline]
pub fn parse_f32_be<'a>(input: &'a [u8]) -> Result<f32, ParseError<'a>> {
    let sized_input: [[u8;2];4] = match input {
        [msb1, lsb1, msb2, lsb2, msb3, lsb3, msb4, lsb4] => [[*msb1, *lsb1], [*msb2, *lsb2], [*msb3, *lsb3], [*msb4, *lsb4]],
        _ => return Err(ParseError::WrongInputSize { expected_len: 8, got: input }),
    };
    let bytes = [
        hex_chars_to_byte_be(sized_input[0]), 
        hex_chars_to_byte_be(sized_input[1]),
        hex_chars_to_byte_be(sized_input[2]),
        hex_chars_to_byte_be(sized_input[3]),
    ];
    Ok(f32::from_be_bytes(bytes))
}

#[inline]
pub fn parse_f64_be<'a>(input: &'a [u8]) -> Result<f64, ParseError<'a>> {
    let sized_input: [[u8;2];8] = match input {
        [
         msb1, lsb1, msb2, lsb2, msb3, lsb3, msb4, lsb4,
         msb5, lsb5, msb6, lsb6, msb7, lsb7, msb8, lsb8,
        ] => [
            [*msb1, *lsb1], [*msb2, *lsb2], [*msb3, *lsb3], [*msb4, *lsb4],
            [*msb5, *lsb5], [*msb6, *lsb6], [*msb7, *lsb7], [*msb8, *lsb8]
        ],
        _ => return Err(ParseError::WrongInputSize { expected_len: 16, got: input }),
    };
    let bytes = [
        hex_chars_to_byte_be(sized_input[0]), 
        hex_chars_to_byte_be(sized_input[1]),
        hex_chars_to_byte_be(sized_input[2]),
        hex_chars_to_byte_be(sized_input[3]),
        hex_chars_to_byte_be(sized_input[4]),
        hex_chars_to_byte_be(sized_input[5]),
        hex_chars_to_byte_be(sized_input[6]),
        hex_chars_to_byte_be(sized_input[7]),
    ];
    Ok(f64::from_be_bytes(bytes))
}
