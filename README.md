# wkb-raster

[Well Known Binary format for PostGIS RASTER type](https://github.com/postgis/postgis/blob/master/raster/doc/RFC2-WellKnownBinaryFormat)

The WKB format for RASTER is meant for transport and
takes into account endianness and avoids any padding.
Still, beside padding and endianness, it matches the
internal serialized format (see RFC1), for quick
input/output.

## Example

### Raster to WKB string

```rust
use wkb_raster::{Raster, RasterBand, RasterDataSource, InMemoryRasterData, Endian};

// 2x2 image bytes, u8 format
let bytes = vec![
    vec![0, 1],
    vec![1, 0],
];

let raster = Raster {
    endian: Endian::Big,    // note: currently Endian::Little is not supported in PostGIS
    version: 0,             // always set to 0
    scale_x: 1.0,           // pixel width in degrees
    scale_y: 1.0,           // pixel height in degrees
    ip_x: 0.0,              // upper left corner longitude in degrees
    ip_y: 0.0,              // upper left corner latitude in degrees
    skew_x: 0.0,            // rotation in degrees (0 to 360)
    skew_y: 0.0,            // rotation in degrees (0 to 360)
    srid: 4326,             // SRID EPSG identifier
    width: 2,               // pixel columns
    height: 2,              // rows
    bands: vec![RasterBand {
        is_nodata_value: false           // true only if entire band is NODATA
        data: RasterDataSource::InMemory(
            InMemoryRasterData::UInt8 {
                data: bytes,
                nodata
            }
        ),
    }],
};

assert_eq!(
    raster.to_wkb_string(),
    String::from("00000000013FF00000000000003FF00000000000000000000000000000000000000000000000000000000000000000000000000000000010E600020002040000010100")
);
```

### WKB string to raster

```rust
use wkb_raster::{Raster, RasterBand, RasterDataSource, InMemoryRasterData, Endian};

let parsed_raster = Raster::from_wkb_string(b"00000000013FF00000000000003FF00000000000000000000000000000000000000000000000000000000000000000000000000000000010E600020002040000010100").unwrap();

// 2x2 image bytes, u8 format
let bytes = vec![
    vec![0, 1],
    vec![1, 0],
];

assert_eq!(parsed_raster, Raster {
    endian: Endian::Big,
    version: 0,
    scale_x: 1.0,
    scale_y: 1.0,
    ip_x: 0.0,
    ip_y: 0.0,
    skew_x: 0.0,
    skew_y: 0.0,
    srid: 4326,
    width: 2,
    height: 2,
    bands: vec![RasterBand {
        is_nodata_value: false,
        data: RasterDataSource::InMemory(
            InMemoryRasterData::UInt8 {
                data: bytes,
                nodata
            }
        ),
    }],
});
```

License: MIT
