macro_rules! parse_memory_data_fns {(
    $endian_path:path, // crate::big_endian

    $fn_parse_memory_data_bool_endian:ident,
    $fn_parse_memory_data_uint2_endian:ident,
    $fn_parse_memory_data_uint4_endian:ident,
    $fn_parse_memory_data_int8_endian:ident,
    $fn_parse_memory_data_uint8_endian:ident,
    $fn_parse_memory_data_int16_endian:ident,
    $fn_parse_memory_data_uint16_endian:ident,
    $fn_parse_memory_data_int32_endian:ident,
    $fn_parse_memory_data_uint32_endian:ident,
    $fn_parse_memory_data_f32_endian:ident,
    $fn_parse_memory_data_f64_endian:ident,

    $parse_bool_endian:ident,
    $parse_uint2_endian:ident,
    $parse_uint4_endian:ident,
    $parse_int8_endian:ident,
    $parse_uint8_endian:ident,
    $parse_int16_endian:ident,
    $parse_uint16_endian:ident,
    $parse_int32_endian:ident,
    $parse_uint32_endian:ident,
    $parse_f32_endian:ident,
    $parse_f64_endian:ident,
) => (

    fn $fn_parse_memory_data_bool_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<bool>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 1;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }
        
        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![false;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 1) {
            match b {
                &[b0, b1] => {
                    let pixel_value = $parse_bool_endian([b0, b1])?;
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_uint2_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<u8>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 1;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }
        
        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 1) {
            match b {
                &[b0, b1] => {
                    let pixel_value = $parse_uint8_endian([b0, b1]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_uint4_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<u8>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 1;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }
        
        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 1) {
            match b {
                &[b0, b1] => {
                    let pixel_value = $parse_uint8_endian([b0, b1]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_int8_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<i8>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 1;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }
        
        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 1) {
            match b {
                &[b0, b1] => {
                    let pixel_value = $parse_int8_endian([b0, b1]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_uint8_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<u8>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 1;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }
        
        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 1) {
            match b {
                &[b0, b1] => {
                    let pixel_value = $parse_uint8_endian([b0, b1]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_int16_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<i16>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;

        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 2;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 2) {
            match b {
                &[b0, b1, b2, b3] => {
                    let pixel_value = $parse_int16_endian([b0, b1, b2, b3]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_uint16_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<u16>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 2;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 2) {
            match b {
                &[b0, b1, b2, b3] => {
                    let pixel_value = $parse_uint16_endian([b0, b1, b2, b3]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_int32_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<i32>>, &'a [u8]), ParseError<'a>> {

        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 4;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 4) {
            match b {
                &[b0, b1, b2, b3, b4, b5, b6, b7] => {
                    let pixel_value = $parse_int32_endian([b0, b1, b2, b3, b4, b5, b6, b7]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_uint32_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<u32>>, &'a [u8]), ParseError<'a>> {
        
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 4;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 4) {
            match b {
                &[b0, b1, b2, b3, b4, b5, b6, b7] => {
                    let pixel_value = $parse_uint32_endian([b0, b1, b2, b3, b4, b5, b6, b7]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_f32_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<f32>>, &'a [u8]), ParseError<'a>> {

        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 4;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0.0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 4) {
            match b {
                &[b0, b1, b2, b3, b4, b5, b6, b7] => {
                    let pixel_value = $parse_f32_endian([b0, b1, b2, b3, b4, b5, b6, b7]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }

    fn $fn_parse_memory_data_f64_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Vec<Vec<f64>>, &'a [u8]), ParseError<'a>> {
        use $endian_path::*;

        let width = width as usize;
        let height = height as usize;
        
        // total bytes necessary = 2*width*height*pix_depth
        let total_bytes_necessary = 2 * width as usize * height as usize * 8;
        if input.len() < total_bytes_necessary { return Err(ParseError::WrongInputSize { expected_len: total_bytes_necessary, got: input }); }

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_vec = vec![vec![0.0;width];height];

        for b in input[0..total_bytes_necessary].chunks_exact(2 * 8) {
            match b {
                &[b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15] => {
                    let pixel_value = $parse_f64_endian([b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15]);
                    unsafe { *total_vec.get_unchecked_mut(current_row).get_unchecked_mut(current_col) = pixel_value; }
                    current_col += 1;
                    if current_col > (width - 1) {
                        current_col = 0;
                        current_row += 1;
                    }
                },
                _ => continue,
            }
        }

        Ok((total_vec, &input[total_bytes_necessary..]))
    }
)}

macro_rules! gen_parse_function_body {
    (
        $fn_parse_memory_data_bool_endian:ident,
        $fn_parse_memory_data_uint2_endian:ident,
        $fn_parse_memory_data_uint4_endian:ident,
        $fn_parse_memory_data_int8_endian:ident,
        $fn_parse_memory_data_uint8_endian:ident,
        $fn_parse_memory_data_int16_endian:ident,
        $fn_parse_memory_data_uint16_endian:ident,
        $fn_parse_memory_data_int32_endian:ident,
        $fn_parse_memory_data_uint32_endian:ident,
        $fn_parse_memory_data_f32_endian:ident,
        $fn_parse_memory_data_f64_endian:ident,

        $input:expr,
        $pixtype:expr,
        $width:expr,
        $height:expr,
) => ({

        let mut input = $input;
        let pixtype = $pixtype;
        let width = $width;
        let height = $height;

        let data = match pixtype {
            PixType::Bool1Bit(nodata)   => {
                let (data, new_memory_input) = $fn_parse_memory_data_bool_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::Bool1Bit { data, nodata }
            },
            PixType::UInt2(nodata)      => {
                let (data, new_memory_input) = $fn_parse_memory_data_uint2_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::UInt2 { data, nodata }
            },
            PixType::UInt4(nodata)      => {
                let (data, new_memory_input) = $fn_parse_memory_data_uint4_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::UInt4 { data, nodata }
            },
            PixType::Int8(nodata)       => {
                let (data, new_memory_input) = $fn_parse_memory_data_int8_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::Int8 { data, nodata }
            },
            PixType::UInt8(nodata)      => {
                let (data, new_memory_input) = $fn_parse_memory_data_uint8_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::UInt8 { data, nodata }
            },
            PixType::Int16(nodata)      => {
                let (data, new_memory_input) = $fn_parse_memory_data_int16_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::Int16 { data, nodata }
            },
            PixType::UInt16(nodata)     => {
                let (data, new_memory_input) = $fn_parse_memory_data_uint16_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::UInt16 { data, nodata }
            },
            PixType::Int32(nodata)      => {
                let (data, new_memory_input) = $fn_parse_memory_data_int32_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::Int32 { data, nodata }
            },
            PixType::UInt32(nodata)     => {
                let (data, new_memory_input) = $fn_parse_memory_data_uint32_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::UInt32 { data, nodata }
            },
            PixType::Float32(nodata)    => {
                let (data, new_memory_input) = $fn_parse_memory_data_f32_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::Float32 { data, nodata }
            },
            PixType::Float64(nodata)    => {
                let (data, new_memory_input) = $fn_parse_memory_data_f64_endian(input, width, height)?;
                input = new_memory_input;
                InMemoryRasterData::Float64 { data, nodata }
            },
        };

        Ok((RasterDataSource::InMemory(data), input))
    })
}