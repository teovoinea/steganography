extern crate steganography;
extern crate image;

use steganography::encoder::*;
use steganography::decoder::*;
use std::path::Path;

#[test]
fn test_encode_decode_alpha() {
    let buffer = [0x00_u8];
    let img = image::open(
                &Path::new(
                    &"tests/test.jpg".to_string()
                )
            ).unwrap();
    
    let enc = Encoder::new(&buffer, img);
    let res = enc.encode_alpha();
    
    let dec = Decoder::new(res);
    let dec_vec = dec.decode_alpha();

    assert!(dec_vec[0] == buffer[0]);
}

#[test]
fn test_encode_decode_image() {
    
}