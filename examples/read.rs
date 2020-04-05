use wkb_raster::big_endian as be;

fn main() {

    println!("N48E008:\n");

    println!("written scaleX:\t{:?}", be::parse_f64_be(b"37AFEAA3AF484B3F"));
    println!("written scaleY:\t{:?}", be::parse_f64_be(b"37AFEAA3AF484B3F"));
    println!("written ipX:\t{:?}", be::parse_f64_be(b"0000000000002040"));
    println!("written ipY:\t{:?}", be::parse_f64_be(b"0000000000804840"));
    
    println!("\nshould be:\n");
    
    println!("scaleX:\t{:?}", be::parse_f64_be(b"4F1BE8B4814E4B3F"));
    println!("scaleY:\t{:?}", be::parse_f64_be(b"4F1BE8B4814E4BBF"));
    println!("ipX:\t{:?}", be::parse_f64_be(b"602CF9C592FF1F40"));
    println!("ipY:\t{:?}", be::parse_f64_be(b"74DA40A70D804840"));
}