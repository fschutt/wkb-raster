//! [Well Known Binary format for PostGIS RASTER type](https://github.com/postgis/postgis/blob/master/raster/doc/RFC2-WellKnownBinaryFormat)
//! 
//! The WKB format for RASTER is meant for transport and
//! takes into account endianness and avoids any padding.
//! Still, beside padding and endianness, it matches the
//! internal serialized format (see RFC1), for quick
//! input/output.
//!
//! # Example
//! 
//! ```rust
//! use wkb_raster::{Raster, RasterBand, RasterDataSource, InMemoryRasterData, Endian};
//! 
//! // 2x2 image bytes, u8 format
//! let bytes = vec![
//!     vec![0, 1], 
//!     vec![1, 0],
//! ];
//!
//! let raster = Raster {
//!     endian: Endian::Big,    // note: currently Endian::Little is not supported in PostGIS
//!     version: 0,             // always set to 0
//!     scale_x: 1.0,           // pixel width in degrees
//!     scale_y: 1.0,           // pixel height in degrees
//!     ip_x: 0.0,              // upper left corner longitude in degrees
//!     ip_y: 0.0,              // upper left corner latitude in degrees
//!     skew_x: 0.0,            // rotation in degrees (0 to 360)
//!     skew_y: 0.0,            // rotation in degrees (0 to 360)
//!     srid: 4326,             // SRID EPSG identifier
//!     width: 2,               // pixel columns
//!     height: 2,              // rows
//!     bands: vec![RasterBand {
//!         is_nodata_value: false,                     // See documentation, usually false
//!         data: RasterDataSource::InMemory(
//!             InMemoryRasterData::UInt8 {
//!                 data: bytes,
//!                 nodata             }
//!         ),
//!     }],
//! };
//! 
//! assert_eq!(
//!     raster.to_wkb_string(), 
//!     String::from("00000000013FF00000000000003FF00000000000000000000000000000000000000000000000000000000000000000000000000000000010E600020002040000010100")
//! );
//! ```

// ```ignore
// // Basic Type definitions
// // byte : 1 byte
// // uint16 : 16 bit unsigned integer (2 bytes)
// // uint32 : 32 bit unsigned integer (4 bytes)
// // float64 : double precision floating point number (8 bytes)
// 
//  +------------------------------------------------------------+
//  | RASTER                                                     |
//  +---------------+-------------+------------------------------+
//  | - name -      |  - type -   | - meaning -                  |
//  +---------------+-------------+------------------------------+
//  | endiannes     | byte        | 1:ndr/little endian          |
//  |               |             | 0:xdr/big endian             |
//  +---------------+-------------+------------------------------+
//  | version       | uint16      | format version (0 for this   |
//  |               |             | structure)                   |
//  +---------------+-------------+------------------------------+
//  | nBands        | uint16      | Number of bands              |
//  +---------------+-------------+------------------------------+
//  | scaleX        | float64     | pixel width                  |
//  |               |             | in geographical units        |
//  +---------------+-------------+------------------------------+
//  | scaleY        | float64     | pixel height                 |
//  |               |             | in geographical units        |
//  +---------------+-------------+------------------------------+
//  | ipX           | float64     | X ordinate of upper-left     |
//  |               |             | pixel's upper-left corner    |
//  |               |             | in geographical units        |
//  +---------------+-------------+------------------------------+
//  | ipY           | float64     | Y ordinate of upper-left     |
//  |               |             | pixel's upper-left corner    |
//  |               |             | in geographical units        |
//  +---------------+-------------+------------------------------+
//  | skewX         | float64     | rotation about Y-axis        |
//  +---------------+-------------+------------------------------+
//  | skewY         | float64     | rotation about X-axis        |
//  +---------------+-------------+------------------------------+
//  | srid          | int32       | Spatial reference id         |
//  +---------------+-------------+------------------------------+
//  | width         | uint16      | number of pixel columns      |
//  +---------------+-------------+------------------------------+
//  | height        | uint16      | number of pixel rows         |
//  +---------------+-------------+------------------------------+
//  | bands[nBands] | RASTERBAND  | Bands data                   |
//  +---------------+-------------+------------------------------+
// 
// 
//  +------------------------------------------------------------------+
//  | RASTERBAND                                                       |
//  +---------------+--------------+-----------------------------------+
//  | - name -      |  - type -    | - meaning -                       |
//  +---------------+--------------+-----------------------------------+
//  | isOffline     | 1bit         | If true, data is to be found      |
//  |               |              | on the filesystem, trought the    |
//  |               |              | path specified in RASTERDATA      |
//  +---------------+--------------+-----------------------------------+
//  | hasNodataValue| 1bit         | If true, stored nodata value is   |
//  |               |              | a true nodata value. Otherwise    |
//  |               |              | the value stored as a nodata      |
//  |               |              | value should be ignored.          | 
//  +---------------+--------------+-----------------------------------+
//  | isNodataValue | 1bit         | If true, all the values of the    |
//  |               |              | band are expected to be nodata    |
//  |               |              | values. This is a dirty flag.     |
//  |               |              | To set the flag to its real value |
//  |               |              | the function st_bandisnodata must |
//  |               |              | must be called for the band with  | 
//  |               |              | 'TRUE' as last argument.          |
//  +---------------+--------------+-----------------------------------+
//  | reserved      | 1bit         | unused in this version            |
//  +---------------+--------------+-----------------------------------+
//  | pixtype       | 4bits        | 0: 1-bit boolean                  |
//  |               |              | 1: 2-bit unsigned integer         |
//  |               |              | 2: 4-bit unsigned integer         |
//  |               |              | 3: 8-bit signed integer           |
//  |               |              | 4: 8-bit unsigned integer         |
//  |               |              | 5: 16-bit signed integer          |
//  |               |              | 6: 16-bit unsigned signed integer |
//  |               |              | 7: 32-bit signed integer          |
//  |               |              | 8: 32-bit unsigned signed integer |
//  |               |              | 10: 32-bit float                  |
//  |               |              | 11: 64-bit float                  |
//  +---------------+--------------+-----------------------------------+
//  | nodata        | 1 to 8 bytes | Nodata value                      |
//  |               | depending on |                                   |
//  |               | pixtype [1]  |                                   |
//  +---------------+--------------+-----------------------------------+
//  | data          | RASTERDATA   | Raster band data (see below)      |
//  +---------------+--------------+-----------------------------------+
// 
//  +------------------------------------------------------------------+
//  | RASTERDATA (isOffline flag clear)                                |
//  +---------------+--------------+-----------------------------------+
//  | pix[w*h]      | 1 to 8 bytes | Pixels values, row after row,     |
//  |               | depending on | so pix[0] is upper-left, pix[w-1] |
//  |               | pixtype [1]  | is upper-right.                   |
//  |               |              |                                   |
//  |               |              | As for endiannes, it is specified |
//  |               |              | at the start of WKB, and implicit |
//  |               |              | up to 8bits (bit-order is most    |
//  |               |              | significant first)                |
//  |               |              |                                   |
//  +---------------+--------------+-----------------------------------+
// 
//  [1] 1,2 and 4 bit pixtypes are still encoded as 1-byte per value
// 
//  +-----------------------------------------------------------------+
//  | RASTERDATA (isOffline flag set)                                 |
//  +---------------+-------------+-----------------------------------+
//  | bandNumber    | int8        | 0-based band number to use from   |
//  |               |             | the set available in the external |
//  |               |             | file                              |
//  +---------------+-------------+-----------------------------------+
//  | path          | string      | null-terminated path to data file |
//  +---------------+-------------+-----------------------------------+
// ```

use std::path::PathBuf;
use std::ffi::CString;

#[macro_use]
mod parse_memory_data;
pub mod big_endian;
pub mod little_endian;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub enum ParseError<'a> {
    WrongInputSize { expected_len: usize, got: &'a [u8] },
    UnableToParseBool(BoolParseError),
    NoEndiannessGiven([u8;2]),
    InvalidPixelType(u8),
    FromBytesWithNulError(Vec<u8>),
    PathContainsNonUTF8Chars(CString),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct BoolParseError([u8;2], u8);

impl<'a> From<BoolParseError> for ParseError<'a> {
    fn from(e: BoolParseError) -> Self {
        ParseError::UnableToParseBool(e)
    }
}

/// Raster data 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Raster {
    /// Endinanness, 1:ndr/little endian, 0:xdr/big endian
    pub endian: Endian,
    /// format version (0 for this structure)                 
    pub version: u16,
    /// pixel width in geographical units  
    pub scale_x: f64,
    /// pixel height in geographical units  
    pub scale_y: f64,
    /// X ordinate of upper-left pixel's upper-left corner in geographical units        
    pub ip_x: f64,
    /// Y ordinate of upper-left pixel's upper-left corner in geographical units        
    pub ip_y: f64,
    /// rotation about Y-axis
    pub skew_x: f64,
    /// rotation about X-axis
    pub skew_y: f64,
    /// Spatial reference id
    pub srid: i32,
    /// Number of pixel columns
    pub width: u16,
    /// Number of pixel rows
    pub height: u16,
    /// Bands data
    pub bands: Vec<RasterBand>,
}

#[inline]
pub fn take_slice_1_byte<'a>(input: &'a [u8]) -> Result<([u8;2], &'a [u8]), ParseError<'a>> {
    use self::ParseError::*;
    const MIN_LEN: usize = 2;
    if input.len() < MIN_LEN { 
        return Err(WrongInputSize { expected_len: MIN_LEN, got: input });
    }
    unsafe {
        let a = input.get_unchecked(0);
        let b = input.get_unchecked(1);
        Ok(([*a, *b], &input[MIN_LEN..]))
    }
}

#[inline]
pub fn take_slice_2_bytes<'a>(input: &'a [u8]) -> Result<([u8;4], &'a [u8]), ParseError<'a>> {
    use self::ParseError::*;
    const MIN_LEN: usize = 4;
    if input.len() < MIN_LEN { 
        return Err(WrongInputSize { expected_len: MIN_LEN, got: input });
    }
    unsafe {
        let a = input.get_unchecked(0);
        let b = input.get_unchecked(1);
        let c = input.get_unchecked(2);
        let d = input.get_unchecked(3);
        Ok(([*a, *b, *c, *d], &input[MIN_LEN..]))
    }
}

#[inline]
pub fn take_slice_4_bytes<'a>(input: &'a [u8]) -> Result<([u8;8], &'a [u8]), ParseError<'a>> {
    use self::ParseError::*;
    const MIN_LEN: usize = 8;
    if input.len() < MIN_LEN { 
        return Err(WrongInputSize { expected_len: MIN_LEN, got: input });
    }
    unsafe {
        let a = input.get_unchecked(0);
        let b = input.get_unchecked(1);
        let c = input.get_unchecked(2);
        let d = input.get_unchecked(3);
        let e = input.get_unchecked(4);
        let f = input.get_unchecked(5);
        let g = input.get_unchecked(6);
        let h = input.get_unchecked(7);
        Ok(([*a, *b, *c, *d, *e, *f, *g, *h], &input[MIN_LEN..]))
    }
}

#[inline]
pub fn take_slice_8_bytes<'a>(input: &'a [u8]) -> Result<([u8;16], &'a [u8]), ParseError<'a>> {
    use self::ParseError::*;
    const MIN_LEN: usize = 16;
    if input.len() < MIN_LEN { 
        return Err(WrongInputSize { expected_len: MIN_LEN, got: input });
    }
    unsafe {
        let a = input.get_unchecked(0);
        let b = input.get_unchecked(1);
        let c = input.get_unchecked(2);
        let d = input.get_unchecked(3);
        let e = input.get_unchecked(4);
        let f = input.get_unchecked(5);
        let g = input.get_unchecked(6);
        let h = input.get_unchecked(7);

        let i = input.get_unchecked(8);
        let j = input.get_unchecked(9);
        let k = input.get_unchecked(10);
        let l = input.get_unchecked(11);
        let m = input.get_unchecked(12);
        let n = input.get_unchecked(13);
        let o = input.get_unchecked(14);
        let p = input.get_unchecked(15);

        Ok((
            [*a, *b, *c, *d, *e, *f, *g, *h,
             *i, *j, *k, *l, *m, *n, *o, *p], 
        &input[MIN_LEN..]
        ))
    }
}

impl Raster {

    /// Outputs the raster as a Well-Known-Binary string, ready to be used in SQL statements
    pub fn to_wkb_string(self) -> String {
        match self.endian {
            Endian::Big => self.to_wkb_string_big_endian(),
            Endian::Little => self.to_wkb_string_little_endian(),
        }
    }

    pub fn from_wkb_string<'a>(string_bytes: &'a [u8]) -> Result<Self, ParseError<'a>> {
        use self::ParseError::*;
        match take_slice_1_byte(string_bytes)? {
            ([b'0', b'0'], input) => Self::from_wkb_string_big_endian(input),
            ([b'0', b'1'], input) => Self::from_wkb_string_little_endian(input),
            (other, _) => Err(NoEndiannessGiven(other)),
        }
    }

    fn from_wkb_string_big_endian<'a>(input: &'a [u8]) -> Result<Self, ParseError<'a>> {

        use crate::big_endian::*;
        
        let (version_bytes, input) = take_slice_2_bytes(input)?;
        let version = parse_u16_be(version_bytes);

        let (nbands_bytes, input) = take_slice_2_bytes(input)?;
        let nbands = parse_u16_be(nbands_bytes);

        let (scale_x_bytes, input) = take_slice_8_bytes(input)?;
        let scale_x = parse_f64_be(scale_x_bytes);

        let (scale_y_bytes, input) = take_slice_8_bytes(input)?;
        let scale_y = parse_f64_be(scale_y_bytes);

        let (ip_x_bytes, input) = take_slice_8_bytes(input)?;
        let ip_x = parse_f64_be(ip_x_bytes);

        let (ip_y_bytes, input) = take_slice_8_bytes(input)?;
        let ip_y = parse_f64_be(ip_y_bytes);

        let (skew_x_bytes, input) = take_slice_8_bytes(input)?;
        let skew_x = parse_f64_be(skew_x_bytes);

        let (skew_y_bytes, input) = take_slice_8_bytes(input)?;
        let skew_y = parse_f64_be(skew_y_bytes);

        let (srid_bytes, input) = take_slice_4_bytes(input)?;
        let srid = parse_i32_be(srid_bytes);
        
        let (width_bytes, input) = take_slice_2_bytes(input)?;
        let width = parse_u16_be(width_bytes);

        let (height_bytes, mut input) = take_slice_2_bytes(input)?;
        let height = parse_u16_be(height_bytes);

        let mut raster_bands = Vec::with_capacity(nbands as usize);

        for _ in 0..(nbands as usize) {
            let (raster_band, rt_input) = RasterBand::from_wkb_string_big_endian(input, width, height)?;
            input = rt_input;
            raster_bands.push(raster_band);
        }

        Ok(Raster {
            endian: Endian::Big,
            version,
            scale_x,
            scale_y,
            ip_x,
            ip_y,
            skew_x,
            skew_y,
            srid,
            width,
            height,
            bands: raster_bands,
        })
    }

    fn from_wkb_string_little_endian<'a>(input: &'a [u8]) -> Result<Self, ParseError<'a>> {
        use crate::little_endian::*;
        
        let (version_bytes, input) = take_slice_2_bytes(input)?;
        let version = parse_u16_le(version_bytes);

        let (nbands_bytes, input) = take_slice_2_bytes(input)?;
        let nbands = parse_u16_le(nbands_bytes);

        let (scale_x_bytes, input) = take_slice_8_bytes(input)?;
        let scale_x = parse_f64_le(scale_x_bytes);

        let (scale_y_bytes, input) = take_slice_8_bytes(input)?;
        let scale_y = parse_f64_le(scale_y_bytes);

        let (ip_x_bytes, input) = take_slice_8_bytes(input)?;
        let ip_x = parse_f64_le(ip_x_bytes);

        let (ip_y_bytes, input) = take_slice_8_bytes(input)?;
        let ip_y = parse_f64_le(ip_y_bytes);

        let (skew_x_bytes, input) = take_slice_8_bytes(input)?;
        let skew_x = parse_f64_le(skew_x_bytes);

        let (skew_y_bytes, input) = take_slice_8_bytes(input)?;
        let skew_y = parse_f64_le(skew_y_bytes);

        let (srid_bytes, input) = take_slice_4_bytes(input)?;
        let srid = parse_i32_le(srid_bytes);
        
        let (width_bytes, input) = take_slice_2_bytes(input)?;
        let width = parse_u16_le(width_bytes);

        let (height_bytes, mut input) = take_slice_2_bytes(input)?;
        let height = parse_u16_le(height_bytes);

        let mut raster_bands = Vec::with_capacity(nbands as usize);

        for _ in 0..(nbands as usize) {
            let (raster_band, rt_input) = RasterBand::from_wkb_string_little_endian(input, width, height)?;
            input = rt_input;
            raster_bands.push(raster_band);
        }

        Ok(Raster {
            endian: Endian::Little,
            version,
            scale_x,
            scale_y,
            ip_x,
            ip_y,
            skew_x,
            skew_y,
            srid,
            width,
            height,
            bands: raster_bands,
        })
    }

    fn to_wkb_string_big_endian(self) -> String {

        use crate::big_endian::*;

        let mut string_bytes = Vec::new();

        // endianness, byte, 1 byte
        write_u8_be(&mut string_bytes, self.endian as u8);
        // version, uint16, two bytes
        write_u16_be(&mut string_bytes, self.version);            
        // nBands, uint16, two bytes
        write_u16_be(&mut string_bytes, self.bands.len() as u16);            
        // write extents, 6x8 bytes       
        write_f64_be(&mut string_bytes, self.scale_x);
        write_f64_be(&mut string_bytes, self.scale_y);
        write_f64_be(&mut string_bytes, self.ip_x);
        write_f64_be(&mut string_bytes, self.ip_y);
        write_f64_be(&mut string_bytes, self.skew_x);
        write_f64_be(&mut string_bytes, self.skew_y);

        // write srid
        write_i32_be(&mut string_bytes, self.srid);
        // write width
        write_u16_be(&mut string_bytes, self.width);
        // write height
        write_u16_be(&mut string_bytes, self.height);

        for band in self.bands {

            // write band config (1 byte)    
            let config = 0 |
                (band.data.is_offline() as u8) << 7 |
                (band.data.get_pixtype().has_nodata_value() as u8) << 6 |
                (band.is_nodata_value as u8) << 5 |
                (0_u8 << 4) |
                band.data.get_pixtype().get_type() & 0b00001111;

            write_u8_be(&mut string_bytes, config);

            // write nodata value
            string_bytes.append(&mut band.data.get_pixtype().get_nodata_value_as_string_big_endian());

            // write raster data
            string_bytes.append(&mut band.data.to_wkb_string_big_endian());
        }

        unsafe { String::from_utf8_unchecked(string_bytes) }
    }

    fn to_wkb_string_little_endian(self) -> String {
        
        use self::little_endian::*;

        let mut string_bytes = Vec::new();

        // endianness, byte, 1 byte
        write_u8_le(&mut string_bytes, self.endian as u8);
        // version, uint16, two bytes
        write_u16_le(&mut string_bytes, self.version);            
        // nBands, uint16, two bytes
        write_u16_le(&mut string_bytes, self.bands.len() as u16);            
        // write extents, 6x8 bytes       
        write_f64_le(&mut string_bytes, self.scale_x);
        write_f64_le(&mut string_bytes, self.scale_y);
        write_f64_le(&mut string_bytes, self.ip_x);
        write_f64_le(&mut string_bytes, self.ip_y);
        write_f64_le(&mut string_bytes, self.skew_x);
        write_f64_le(&mut string_bytes, self.skew_y);

        // write srid
        write_i32_le(&mut string_bytes, self.srid);
        // write width
        write_u16_le(&mut string_bytes, self.width);
        // write height
        write_u16_le(&mut string_bytes, self.height);

        for band in self.bands {

            // expected: 0x45 = 69 = 01000101
            // got: 00 = 0 = 01000101

            // write band config (1 byte)    
            let config = 0 |
                (band.data.is_offline() as u8) << 7 |
                (band.data.get_pixtype().has_nodata_value() as u8) << 6 |
                (band.is_nodata_value as u8) << 5 |
                (0_u8 << 4) |
                band.data.get_pixtype().get_type() & 0b00001111;

            write_u8_le(&mut string_bytes, config);
            
            // write nodata value
            string_bytes.append(&mut band.data.get_pixtype().get_nodata_value_as_string_little_endian());

            // write raster data
            string_bytes.append(&mut band.data.to_wkb_string_little_endian());
        }

        unsafe { String::from_utf8_unchecked(string_bytes) }
    }
}

/// Endianness of the output string
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Endian {
    Big = 0,
    Little = 1,
}

/// Single band of raster data
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RasterBand {
    /// If true, all the values of band are expected to be values. This is a dirty flagTo set the flag to its real
    /// the function `st_bandisnodatamust` be called for the band 'TRUE' as last argument.          
    pub is_nodata_value: bool,
    /// The actual data of the 
    pub data: RasterDataSource,
}

impl RasterBand {
    
    fn from_wkb_string_big_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Self, &'a [u8]), ParseError<'a>> {
        use crate::big_endian::*;
        use self::ParseError::*;


        let (pixinfo_bytes, mut input) = take_slice_1_byte(input)?;
        let pixinfo = parse_u8_be(pixinfo_bytes);
        let is_offline = ((pixinfo  & 0b10000000) >> 7) != 0;
        let has_nodata_value = ((pixinfo  & 0b01000000) >> 6) != 0;
        let is_nodata_value = ((pixinfo  & 0b00100000) >> 5) != 0; // ??
        let pixtype = pixinfo & 0b00001111;


        let pixtype = match pixtype {
            0 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_bool_be(nodata_bytes)?) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Bool1Bit(nodata)
            },
            1 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_u8_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt2(nodata)
            },
            2 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_u8_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt4(nodata)
            },
            3 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_i8_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Int8(nodata)
            },
            4 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_u8_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt8(nodata)
            },
            5 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_2_bytes(input)?;
                    input = pt_input;
                    Some(parse_i16_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Int16(nodata)
            },
            6 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_2_bytes(input)?;
                    input = pt_input;
                    Some(parse_u16_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt16(nodata)
            },
            7 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_4_bytes(input)?;
                    input = pt_input;
                    Some(parse_i32_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Int32(nodata)
            },
            8 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_4_bytes(input)?;
                    input = pt_input;
                    Some(parse_u32_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt32(nodata)
            },
            10 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_4_bytes(input)?;
                    input = pt_input;
                    Some(parse_f32_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Float32(nodata)
            },
            11 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_8_bytes(input)?;
                    input = pt_input;
                    Some(parse_f64_be(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Float64(nodata)
            }, 
            other => return Err(InvalidPixelType(other)),
        };


        let (raster_data_source, input) = if is_offline {
            RasterDataSource::parse_offline_big_endian(input, pixtype)?
        } else {
            RasterDataSource::parse_memory_big_endian(input, pixtype, width, height)?
        };


        Ok((RasterBand {
            is_nodata_value,
            data: raster_data_source,
        }, input))
    }

    fn from_wkb_string_little_endian<'a>(input: &'a [u8], width: u16, height: u16) -> Result<(Self, &'a [u8]), ParseError<'a>> {
        use crate::little_endian::*;
        use self::ParseError::*;


        let (pixinfo_bytes, mut input) = take_slice_1_byte(input)?;
        let pixinfo = parse_u8_le(pixinfo_bytes);
        let is_offline = ((pixinfo  & 0b10000000) >> 7) != 0;
        let has_nodata_value = ((pixinfo  & 0b01000000) >> 6) != 0;
        let is_nodata_value = ((pixinfo  & 0b00100000) >> 5) != 0; // ??
        let pixtype = pixinfo & 0b00001111;


        let pixtype = match pixtype {
            0 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_bool_le(nodata_bytes)?) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Bool1Bit(nodata)
            },
            1 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_u8_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt2(nodata)
            },
            2 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_u8_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt4(nodata)
            },
            3 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_i8_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Int8(nodata)
            },
            4 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    Some(parse_u8_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt8(nodata)
            },
            5 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_2_bytes(input)?;
                    input = pt_input;
                    Some(parse_i16_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Int16(nodata)
            },
            6 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_2_bytes(input)?;
                    input = pt_input;
                    Some(parse_u16_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt16(nodata)
            },
            7 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_4_bytes(input)?;
                    input = pt_input;
                    Some(parse_i32_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Int32(nodata)
            },
            8 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_4_bytes(input)?;
                    input = pt_input;
                    Some(parse_u32_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::UInt32(nodata)
            },
            10 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_4_bytes(input)?;
                    input = pt_input;
                    Some(parse_f32_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Float32(nodata)
            },
            11 => {
                let nodata = if has_nodata_value { 
                    let (nodata_bytes, pt_input) = take_slice_8_bytes(input)?;
                    input = pt_input;
                    Some(parse_f64_le(nodata_bytes)) 
                } else { 
                    let (_, pt_input) = take_slice_1_byte(input)?;
                    input = pt_input;
                    None 
                };
                PixType::Float64(nodata)
            }, 
            other => return Err(InvalidPixelType(other)),
        };

        let (raster_data_source, input) = if is_offline {
            RasterDataSource::parse_offline_little_endian(input, pixtype)?
        } else {
            RasterDataSource::parse_memory_little_endian(input, pixtype, width, height)?
        };


        Ok((RasterBand {
            is_nodata_value,
            data: raster_data_source,
        }, input))
    }
}

impl RasterDataSource {
    
    /// Returns `true` if the item is a `Offline { .. }` item
    pub fn is_offline(&self) -> bool {
        use self::RasterDataSource::*;
        match self {
            Offline(_) => true,
            InMemory(_) => false,
        }
    }

    pub fn get_pixtype(&self) -> PixType {
        use self::RasterDataSource::*;
        match &self {
            Offline(o) => o.pixtype,
            InMemory(i) => i.get_pixtype(),
        }
    }

    fn parse_offline_big_endian<'a>(input: &'a [u8], pixtype: PixType) -> Result<(Self, &'a [u8]), ParseError<'a>> {
        use crate::big_endian::*;
        use std::ffi::CStr;

        let (band_bytes, mut input) = take_slice_1_byte(input)?;
        let band = parse_i8_be(band_bytes);

        let mut path_bytes = Vec::new();
        loop {

            let (byte, new_input) = take_slice_1_byte(input)?;
            input = new_input;
            let parsed = parse_u8_be(byte);
            path_bytes.push(parsed);

            if parsed == b'\0' {
                break; 
            }
        }

        let path = CStr::from_bytes_with_nul(&path_bytes)
        .map_err(|_| ParseError::FromBytesWithNulError(path_bytes.clone()))?;
        let path = CString::from(path);
        let path = PathBuf::from(path.clone().into_string()
            .map_err(|_| ParseError::PathContainsNonUTF8Chars(path.to_owned()))?);
        
        Ok((RasterDataSource::Offline(OfflineRasterData {
            band,
            path,
            pixtype,
        }), input))

    }

    fn parse_offline_little_endian<'a>(input: &'a [u8], pixtype: PixType) -> Result<(Self, &'a [u8]), ParseError<'a>> {

        use crate::little_endian::*;
        use std::ffi::CStr;

        let (band_bytes, mut input) = take_slice_1_byte(input)?;
        let band = parse_i8_le(band_bytes);

        let mut path_bytes = Vec::new();
        loop {

            let (byte, new_input) = take_slice_1_byte(input)?;
            input = new_input;
            let parsed = parse_u8_le(byte);
            path_bytes.push(parsed);

            if parsed == b'\0' {
                break; 
            }
        }

        let path = CStr::from_bytes_with_nul(&path_bytes)
        .map_err(|_| ParseError::FromBytesWithNulError(path_bytes.clone()))?;
        let path = CString::from(path);
        let path = PathBuf::from(path.clone().into_string()
            .map_err(|_| ParseError::PathContainsNonUTF8Chars(path.to_owned()))?);
        
        Ok((RasterDataSource::Offline(OfflineRasterData {
            band,
            path,
            pixtype,
        }), input))
    }

    fn parse_memory_big_endian<'a>(input: &'a [u8], pixtype: PixType, width: u16, height: u16) -> Result<(Self, &'a [u8]), ParseError<'a>> {


        parse_memory_data_fns!(
            crate::big_endian,

            parse_memory_data_bool_be,
            parse_memory_data_uint2_be,
            parse_memory_data_uint4_be,
            parse_memory_data_int8_be,
            parse_memory_data_uint8_be,
            parse_memory_data_int16_be,
            parse_memory_data_uint16_be,
            parse_memory_data_int32_be,
            parse_memory_data_uint32_be,
            parse_memory_data_f32_be,
            parse_memory_data_f64_be,

            parse_bool_be,
            parse_uint2_be,
            parse_uint4_be,
            parse_i8_be,
            parse_u8_be,
            parse_i16_be,
            parse_u16_be,
            parse_i32_be,
            parse_u32_be,
            parse_f32_be,
            parse_f64_be,
        );

        let ret = gen_parse_function_body!(
            parse_memory_data_bool_be,
            parse_memory_data_uint2_be,
            parse_memory_data_uint4_be,
            parse_memory_data_int8_be,
            parse_memory_data_uint8_be,
            parse_memory_data_int16_be,
            parse_memory_data_uint16_be,
            parse_memory_data_int32_be,
            parse_memory_data_uint32_be,
            parse_memory_data_f32_be,
            parse_memory_data_f64_be,

            input,
            pixtype,
            width,
            height,
        );

        ret
    }

    fn parse_memory_little_endian<'a>(input: &'a [u8], pixtype: PixType, width: u16, height: u16) -> Result<(Self, &'a [u8]), ParseError<'a>> {
        

        parse_memory_data_fns!(
            crate::little_endian,

            parse_memory_data_bool_le,
            parse_memory_data_uint2_le,
            parse_memory_data_uint4_le,
            parse_memory_data_int8_le,
            parse_memory_data_uint8_le,
            parse_memory_data_int16_le,
            parse_memory_data_uint16_le,
            parse_memory_data_int32_le,
            parse_memory_data_uint32_le,
            parse_memory_data_f32_le,
            parse_memory_data_f64_le,

            parse_bool_le,
            parse_uint2_le,
            parse_uint4_le,
            parse_i8_le,
            parse_u8_le,
            parse_i16_le,
            parse_u16_le,
            parse_i32_le,
            parse_u32_le,
            parse_f32_le,
            parse_f64_le,
        );

        let ret = gen_parse_function_body!(
            parse_memory_data_bool_le,
            parse_memory_data_uint2_le,
            parse_memory_data_uint4_le,
            parse_memory_data_int8_le,
            parse_memory_data_uint8_le,
            parse_memory_data_int16_le,
            parse_memory_data_uint16_le,
            parse_memory_data_int32_le,
            parse_memory_data_uint32_le,
            parse_memory_data_f32_le,
            parse_memory_data_f64_le,
            
            input,
            pixtype,
            width,
            height,
        );

        ret
    }
}

/// Pixel type + optional nodata value
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum PixType {
    Bool1Bit(Option<bool>),
    UInt2(Option<u8>),
    UInt4(Option<u8>),
    Int8(Option<i8>),
    UInt8(Option<u8>),
    Int16(Option<i16>),
    UInt16(Option<u16>),
    Int32(Option<i32>),
    UInt32(Option<u32>),
    Float32(Option<f32>),
    Float64(Option<f64>),
}

impl PixType {
    fn get_type(&self) -> u8 {
        use self::PixType::*;
        match self {
            Bool1Bit(_) => 0,
            UInt2(_) => 1,
            UInt4(_) => 2,
            Int8(_) => 3,
            UInt8(_) => 4,
            Int16(_) => 5,
            UInt16(_) => 6,
            Int32(_) => 7,
            UInt32(_) => 8, // no 9!
            Float32(_) => 10,
            Float64(_) => 11,  
        }
    }

    pub fn has_nodata_value(&self) -> bool {
        use self::PixType::*;
        match self {
            | Bool1Bit(Some(_))
            | UInt2(Some(_))
            | UInt4(Some(_))
            | Int8(Some(_))
            | UInt8(Some(_))
            | Int16(Some(_))
            | UInt16(Some(_))
            | Int32(Some(_))
            | UInt32(Some(_))
            | Float32(Some(_))
            | Float64(Some(_)) => true,
            _ => false,
        }
    }

    #[inline]
    fn get_nodata_value_as_string_big_endian(&self) -> Vec<u8> {
        
        use crate::big_endian::*;
        use self::PixType::*;

        let mut s = Vec::new();

        match self {
            Bool1Bit(Some(b)) =>    { write_bool_be(&mut s, *b); },
            UInt2(Some(b)) =>       { write_u8_be(&mut s, *b); /* TODO: u2! */ }, 
            UInt4(Some(b)) =>       { write_u8_be(&mut s, *b); /* TODO: u4! */ },
            Int8(Some(b)) =>        { write_i8_be(&mut s, *b); },
            UInt8(Some(b)) =>       { write_u8_be(&mut s, *b); }
            Int16(Some(b)) =>       { write_i16_be(&mut s, *b); }
            UInt16(Some(b)) =>      { write_u16_be(&mut s, *b); }
            Int32(Some(b)) =>       { write_i32_be(&mut s, *b); }
            UInt32(Some(b)) =>      { write_u32_be(&mut s, *b); }
            Float32(Some(b)) =>     { write_f32_be(&mut s, *b); }
            Float64(Some(b)) =>     { write_f64_be(&mut s, *b); }
            _ =>                    { write_u8_be(&mut s, 0); },
        }

        s
    }

    #[inline]
    fn get_nodata_value_as_string_little_endian(&self) -> Vec<u8> {
        
        use self::little_endian::*;
        use self::PixType::*;

        let mut s = Vec::new();

        match self {
            Bool1Bit(Some(b)) =>    { write_bool_le(&mut s, *b); },
            UInt2(Some(b)) =>       { write_u8_le(&mut s, *b); /* TODO: u2! */ }, 
            UInt4(Some(b)) =>       { write_u8_le(&mut s, *b); /* TODO: u4! */ },
            Int8(Some(b)) =>        { write_i8_le(&mut s, *b); },
            UInt8(Some(b)) =>       { write_u8_le(&mut s, *b); }
            Int16(Some(b)) =>       { write_i16_le(&mut s, *b); }
            UInt16(Some(b)) =>      { write_u16_le(&mut s, *b); }
            Int32(Some(b)) =>       { write_i32_le(&mut s, *b); }
            UInt32(Some(b)) =>      { write_u32_le(&mut s, *b); }
            Float32(Some(b)) =>     { write_f32_le(&mut s, *b); }
            Float64(Some(b)) =>     { write_f64_le(&mut s, *b); }
            _ =>                    { write_u8_le(&mut s, 0); },
        }

        s
    }
}

/// Source of the raster data bytes + pixel type definition
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RasterDataSource {
    /// Pixel values are stored in an file
    Offline(OfflineRasterData),
    /// Pixels values, row after row, so pix[0] is upper-left, pix[w-1],is upper-right.
    /// As for endiannes, it is specified at the start of WKB, and up to 8bits 
    /// (bit-order is most significant first)
    InMemory(InMemoryRasterData),
}

/// Raster data file source
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OfflineRasterData {
    /// 0-based band number to use from the set available in the external file                             
    pub band: i8, 
    /// Path to data file
    pub path: PathBuf,
    /// Type of the pixels to read
    pub pixtype: PixType,
}

/// In-memory raster data with nodata value and 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InMemoryRasterData {
    Bool1Bit { data: Vec<Vec<bool>>, nodata: Option<bool> },
    UInt2 { data: Vec<Vec<u8>>, nodata: Option<u8> },
    UInt4 { data: Vec<Vec<u8>>, nodata: Option<u8> },
    Int8 { data: Vec<Vec<i8>>, nodata: Option<i8> },
    UInt8 { data: Vec<Vec<u8>>, nodata: Option<u8> },
    Int16 { data: Vec<Vec<i16>>, nodata: Option<i16> },
    UInt16 { data: Vec<Vec<u16>>, nodata: Option<u16> },
    Int32 { data: Vec<Vec<i32>>, nodata: Option<i32> },
    UInt32 { data: Vec<Vec<u32>>, nodata: Option<u32> },
    Float32 { data: Vec<Vec<f32>>, nodata: Option<f32> },
    Float64 { data: Vec<Vec<f64>>, nodata: Option<f64> },
}

impl InMemoryRasterData {
    /// Returns the pixtype of the `InMemoryRasterData`
    pub fn get_pixtype(&self) -> PixType {
        match &self {
            InMemoryRasterData::Bool1Bit { nodata, .. } => PixType::Bool1Bit(*nodata),
            InMemoryRasterData::UInt2 { nodata, .. } => PixType::UInt2(*nodata),
            InMemoryRasterData::UInt4 { nodata, .. } => PixType::UInt4(*nodata),
            InMemoryRasterData::Int8 { nodata, .. } => PixType::Int8(*nodata),
            InMemoryRasterData::UInt8 { nodata, .. } => PixType::UInt8(*nodata),
            InMemoryRasterData::Int16 { nodata, .. } => PixType::Int16(*nodata),
            InMemoryRasterData::UInt16 { nodata, .. } => PixType::UInt16(*nodata),
            InMemoryRasterData::Int32 { nodata, .. } => PixType::Int32(*nodata),
            InMemoryRasterData::UInt32 { nodata, .. } => PixType::UInt32(*nodata),
            InMemoryRasterData::Float32 { nodata, .. } => PixType::Float32(*nodata),
            InMemoryRasterData::Float64 { nodata, .. } => PixType::Float64(*nodata),
        }
    }
}

impl RasterDataSource {
    
    /// Outputs the string to put in the SQL query (big endian)
    fn to_wkb_string_big_endian(self) -> Vec<u8> {

        use self::RasterDataSource::*;
        use crate::big_endian::*;

        let mut s = Vec::new();

        match self {
            Offline(OfflineRasterData { band, path, .. }) => {
                // write band id
                write_i8_be(&mut s, band);
                // write file path
                let path: Vec<u8> = path.as_os_str().to_string_lossy().as_bytes().to_vec();
                let cstring = unsafe { CString::from_vec_unchecked(path) };
                for byte in cstring.to_bytes_with_nul() {
                    write_u8_be(&mut s, *byte);
                }
            },
            InMemory(data) => {
                match data {
                    InMemoryRasterData::Bool1Bit { data, .. }   => { for row in data { for byte in row { write_bool_be(&mut s, byte); } } },
                    InMemoryRasterData::UInt2 { data, .. }      => { for row in data { for byte in row { write_u8_be(&mut s, byte); } } },
                    InMemoryRasterData::UInt4 { data, .. }      => { for row in data { for byte in row { write_u8_be(&mut s, byte); } } },
                    InMemoryRasterData::Int8 { data, .. }       => { for row in data { for byte in row { write_i8_be(&mut s, byte); } } },
                    InMemoryRasterData::UInt8 { data, .. }      => { for row in data { for byte in row { write_u8_be(&mut s, byte); } } },
                    InMemoryRasterData::Int16 { data, .. }      => { for row in data { for byte in row { write_i16_be(&mut s, byte); } } },
                    InMemoryRasterData::UInt16 { data, .. }     => { for row in data { for byte in row { write_u16_be(&mut s, byte); } } },
                    InMemoryRasterData::Int32 { data, .. }      => { for row in data { for byte in row { write_i32_be(&mut s, byte); } } },
                    InMemoryRasterData::UInt32 { data, .. }     => { for row in data { for byte in row { write_u32_be(&mut s, byte); } } },
                    InMemoryRasterData::Float32 { data, .. }    => { for row in data { for byte in row { write_f32_be(&mut s, byte); } } },
                    InMemoryRasterData::Float64 { data, .. }    => { for row in data { for byte in row { write_f64_be(&mut s, byte); } } },
                }
            },
        }

        s
    }

    /// Outputs the string to put in the SQL query (little endian)
    fn to_wkb_string_little_endian(self) -> Vec<u8> {

        use self::RasterDataSource::*;
        use self::little_endian::*;

        let mut s = Vec::new();

        match self {
            Offline(OfflineRasterData { band, path, .. }) => {
                // write band id
                write_i8_le(&mut s, band);
                // write file path
                let path: Vec<u8> = path.as_os_str().to_string_lossy().as_bytes().to_vec();
                let cstring = unsafe { CString::from_vec_unchecked(path) };
                for byte in cstring.into_bytes_with_nul() {
                    let byte: u8 = byte; // rustc is dumb
                    write_u8_le(&mut s, byte);
                }
            },
            InMemory(data) => {
                match data {
                    InMemoryRasterData::Bool1Bit { data, .. }   => { for row in data { for byte in row { write_bool_le(&mut s, byte); } } },
                    InMemoryRasterData::UInt2 { data, .. }      => { for row in data { for byte in row { write_u8_le(&mut s, byte); } } },
                    InMemoryRasterData::UInt4 { data, .. }      => { for row in data { for byte in row { write_u8_le(&mut s, byte); } } },
                    InMemoryRasterData::Int8 { data, .. }       => { for row in data { for byte in row { write_i8_le(&mut s, byte); } } },
                    InMemoryRasterData::UInt8 { data, .. }      => { for row in data { for byte in row { write_u8_le(&mut s, byte); } } },
                    InMemoryRasterData::Int16 { data, .. }      => { for row in data { for byte in row { write_i16_le(&mut s, byte); } } },
                    InMemoryRasterData::UInt16 { data, .. }     => { for row in data { for byte in row { write_u16_le(&mut s, byte); } } },
                    InMemoryRasterData::Int32 { data, .. }      => { for row in data { for byte in row { write_i32_le(&mut s, byte); } } },
                    InMemoryRasterData::UInt32 { data, .. }     => { for row in data { for byte in row { write_u32_le(&mut s, byte); } } },
                    InMemoryRasterData::Float32 { data, .. }    => { for row in data { for byte in row { write_f32_le(&mut s, byte); } } },
                    InMemoryRasterData::Float64 { data, .. }    => { for row in data { for byte in row { write_f64_le(&mut s, byte); } } },
                }
            },
        }

        s
    }
}
