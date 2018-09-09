#![feature(test)]

extern crate test;
extern crate steganography;
extern crate image;

use steganography::encoder::*;
use steganography::decoder::*;
use std::path::Path;

use test::Bencher;

#[bench]
fn bench_encode_decode_alpha(b: &mut Bencher) {
    let img = image::open(
                    &Path::new(
                        &"tests/test.jpg".to_string()
                    )
                ).unwrap();
    b.iter(|| {
        let buffer = [0x00_u8];
        
        let enc = Encoder::new(&buffer, img.clone());
        let res = enc.encode_alpha();
        
        let dec = Decoder::new(res);
        let dec_vec = dec.decode_alpha();
    });
}

#[bench]
fn bench_encode_decode_image(b: &mut Bencher) {
    let img = image::open(
                &Path::new(
                    &"tests/test.jpg".to_string()
                )
            ).unwrap();

    b.iter(|| {
        let buffer = [0x00_u8, 0x01_u8, 0x02_u8, 0x03_u8,
                  0x04_u8, 0x05_u8, 0x06_u8, 0x07_u8,
                  0x08_u8, 0x09_u8, 0x0A_u8, 0x0B_u8,
                  0x0C_u8, 0x0D_u8, 0x0E_u8, 0x0F_u8];

        let enc = Encoder::new(&buffer, img.clone());
        let res = enc.encode_image();

        let dec = Decoder::new(res);
        let dec_vec = dec.decode_image();
    });
}