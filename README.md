[![Build Status](https://travis-ci.org/teovoinea/steganography.svg?branch=master)](https://travis-ci.org/teovoinea/steganography)
[![Crates.io](https://img.shields.io/crates/v/steganography.svg)](https://crates.io/crates/steganography)
[![Crates.io](https://img.shields.io/crates/d/steganography.svg)](https://crates.io/crates/steganography)
[![Docs.rs](https://docs.rs/steganography/badge.svg)](https://docs.rs/steganography)

# steganography
A stable steganography library written in rust

[Crates.io](https://crates.io/crates/steganography)

## Usage

Add the following to the Cargo.toml in your project:

```toml
[dependencies]
steganography = "*"
```

and import using ```extern crate```:

```rust
extern crate steganography;

/*
use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;
*/
```

## Writing a message to a file

```rust
//Define a secret message to hide in out picture
let message = "This is a steganography demo!".to_string();
//Convert our string to bytes
let payload = str_to_bytes(&message);
//Load the image where we want to embed our secret message
let destination_image = file_as_dynamic_image("example.jpg".to_string());
//Create an encoder
let enc = Encoder::new(payload, destination_image);
//Encode our message into the alpha channel of the image
let result = enc.encode_alpha();
//Save the new image
save_image_buffer(result, "hidden_message.png".to_string();
```

## Reading a message from a file

```rust
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
```

## Running the examples
```bash
cargo build --example example_encode
cargo run --example example_encode
```

```bash
cargo build --example example_decode
cargo run --example example_decode
```

## Testing

```bash
cargo test -- --test-threads=1
```
