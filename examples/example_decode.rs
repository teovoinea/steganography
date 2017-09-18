extern crate steganography;

use steganography::decoder::*;
use steganography::util::*;

fn main() {
    //Load the image with the secret message
    let encoded_image = file_as_image_buffer("examples/decode_message.png".to_string());
    //Create a decoder
    let dec = Decoder::new(encoded_image);
    //Decode the image by reading the alpha channel
    let out_buffer = dec.decode_alpha();
    //If there is no alpha, it's set to 255 by default so we filter those out
    let clean_buffer: Vec<u8> = out_buffer.into_iter()
                                        .filter(|b| {
                                            *b != 0xff_u8
                                        })
                                        .collect();
    //Convert those bytes into a string we can read
    let message = bytes_to_str(clean_buffer.as_slice());
    //Print it out!
    println!("{:?}", message);
}