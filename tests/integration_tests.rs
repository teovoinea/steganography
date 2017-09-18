extern crate steganography;
extern crate image;

use steganography::encoder::*;
use steganography::decoder::*;
use std::path::Path;

/// Tests encoding and decoding alpha. Check if they read/write the same buffer.
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

/// Tests encoding and decoding an image. Check if they read/write the same buffer.
#[test]
fn test_encode_decode_image() {
    let buffer = [0x00_u8, 0x01_u8, 0x02_u8, 0x03_u8,
                  0x04_u8, 0x05_u8, 0x06_u8, 0x07_u8,
                  0x08_u8, 0x09_u8, 0x0A_u8, 0x0B_u8,
                  0x0C_u8, 0x0D_u8, 0x0E_u8, 0x0F_u8];

    let img = image::open(
                &Path::new(
                    &"tests/test.jpg".to_string()
                )
            ).unwrap();
    
    let enc = Encoder::new(&buffer, img);
    let res = enc.encode_image();

    let dec = Decoder::new(res);
    let dec_vec = dec.decode_image();
    
    assert!(dec_vec.as_slice() == buffer);
}