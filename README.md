[![Build Status](https://travis-ci.org/teovoinea/steganography.svg?branch=master)](https://travis-ci.org/teovoinea/steganography)
[![Crates.io](https://img.shields.io/crates/v/steganography.svg)]()
[![Crates.io](https://img.shields.io/crates/d/steganography.svg)]()
[![Docs.rs](https://docs.rs/steganography/badge.svg)]()

# steganography
A simple steganography library written in rust

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

//Use steganography::
```

## Writing a buffer to a file

```rust
let buffer = [0x00_u8];

let outputFile = write_to_file(&buffer, "test.jpg".to_string());

let ref mut outputPath = &Path::new("test.png");

let _ = out.save(outputPath).unwrap();
```

## Reading a buffer from a file

```rust
let out_buf = read_from_file("test.png".to_string());
```

## Testing

```bash
cargo test -- --test-threads=1
```
