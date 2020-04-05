use wkb_raster::big_endian as be;

fn main() {

    println!("written scaleX:\t{:?}", be::parse_f64_be(b"4F1BE8B4814E4B3F"));
    println!("written scaleY:\t{:?}", be::parse_f64_be(b"4F1BE8B4814E4BBF"));
    println!("written ipX:\t{:?}", be::parse_f64_be(b"602CF9C592FF1F40"));
    println!("written ipY:\t{:?}", be::parse_f64_be(b"74DA40A70D004940"));
    
    println!("\nshould be:\n");
    
    println!("scaleX:\t{:?}", be::parse_f64_be(b"030F984F285A4B3F"));
    println!("scaleY:\t{:?}", be::parse_f64_be(b"EF0E984F285A4B3F"));
    println!("ipX:\t{:?}", be::parse_f64_be(b"875E8A23B8FE1F40"));
    println!("ipY:\t{:?}", be::parse_f64_be(b"2FB48EFB28004940"));
}