use std::fs::File;
use std::io::prelude::*;
use std::str;

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

/*
pub fn bytes_to_files(bytes: &[u8]) -> &[&File] {
    //TODO
}
*/
