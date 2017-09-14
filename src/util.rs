use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::str;
use image::{
    DynamicImage,
    ImageBuffer,
    Rgba,
    open
};

pub fn str_to_bytes<'a>(msg: &'a String) -> &'a [u8] {
    msg.as_bytes()
}

pub fn file_to_bytes<'a>(mut file: File) -> Vec<u8> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}

pub fn files_to_bytes(files: &mut [&File]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for file in files {
        file.read_to_end(&mut buffer).expect("Could not read file");
        // Add a unique code to identify the end of one file and the beginning of the next
    }
    buffer
}

pub fn bytes_to_str(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).unwrap()
}

pub fn bytes_to_file<'a>(bytes: &[u8], mut file: &'a File) -> &'a File {
    let _ = file.write_all(bytes);
    &file
}

pub fn file_as_dynamic_image(filename: String) -> DynamicImage {
    let img = open(&Path::new(&filename)).unwrap();
    img
}

pub fn file_as_image_buffer(filename: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let img = open(&Path::new(&filename)).unwrap();
    img.to_rgba()
}

pub fn save_image_buffer(img: ImageBuffer<Rgba<u8>, Vec<u8>>, filename: String) {
    let out_path = &Path::new(&filename);
    let _ = img.save(out_path).unwrap();
}

/*
pub fn bytes_to_files(bytes: &[u8]) -> &[&File] {
    //TODO
}




*/
