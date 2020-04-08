// Runs tests for encoding / decoding for each type
use wkb_raster::{Raster, RasterBand, RasterDataSource, InMemoryRasterData, InMemoryRasterData::*, Endian};

fn run_test_inner(endian: Endian, input: InMemoryRasterData, width: u16, height: u16) {
    let setup = Raster {
        endian,
        version: 0,
        scale_x: 500.0,
        scale_y: 1.0,
        ip_x: 0.0,
        ip_y: 0.0,
        skew_x: 0.0,
        skew_y: 0.0,
        srid: 4326,
        width,
        height,
        bands: vec![RasterBand {
            is_nodata_value: false,
            data: RasterDataSource::InMemory(input),
        }],
    };

    let encoded = setup.clone().to_wkb_string();
    let decoded = Raster::from_wkb_string(&encoded.as_bytes()).unwrap();
    if decoded != setup {
        use std::process::exit;
        println!("expected: {:#?}\n\ngot:{:#?}", setup, decoded);
        exit(1);
    }
}

fn run_test(input: InMemoryRasterData, width: u16, height: u16) {
    run_test_inner(Endian::Big, input.clone(), width, height);
    run_test_inner(Endian::Little, input, width, height);
}

fn main() {

    run_test(Bool1Bit { data: vec![
        vec![true, false, true],
        vec![false, false, false],
        vec![false, true, false],
    ], nodata: None }, 3, 3);

    run_test(UInt2 { data: vec![
        vec![50, 38, 58],
        vec![20, 10, 90],
        vec![78, 5, 0],
    ], nodata: None }, 3, 3);

    run_test(UInt4 { data: vec![
        vec![9, 89, 23],
        vec![49, 8, 17],
        vec![90, 83, 48],
    ], nodata: None }, 3, 3);

    run_test(Int8 { data: vec![
        vec![-9, -25, 23],
        vec![49, -50, 19],
        vec![4, 18, 4],
    ], nodata: None }, 3, 3);

    run_test(UInt8 { data: vec![
        vec![45, 255, 0],
        vec![49, 39, 77],
        vec![3, 15, 10],
    ], nodata: None }, 3, 3);

    run_test(Int16 { data: vec![
        vec![3939, 255, 0],
        vec![49, 3939, 15],
        vec![405, 15, -2927],
    ], nodata: None }, 3, 3);
}