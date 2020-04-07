use wkb_raster::{Raster, RasterBand, RasterDataSource, InMemoryRasterData, Endian};

fn main() {
    // 2x2 image bytes, u8 format
    let bytes = vec![
        vec![0, 1],
        vec![1, 0],
    ];

    let raster = Raster {
        endian: Endian::Big,    // note: currently Endian::Little is not supported in PostGIS
        version: 0,             // always set to 0
        scale_x: 500.0,           // pixel width in degrees
        scale_y: 1.0,           // pixel height in degrees
        ip_x: 0.0,              // upper left corner longitude in degrees
        ip_y: 0.0,              // upper left corner latitude in degrees
        skew_x: 0.0,            // rotation in degrees (0 to 360)
        skew_y: 0.0,            // rotation in degrees (0 to 360)
        srid: 4326,             // SRID EPSG identifier
        width: 2,               // pixel columns
        height: 2,              // rows
        bands: vec![RasterBand {
            is_nodata_value: false,                     // See documentation, usually false
            data: RasterDataSource::InMemory(
                InMemoryRasterData::UInt8 {
                    data: bytes,
                    nodata: None,
                }
            ),
        }],
    };

    println!("{}", raster.to_wkb_string());
}