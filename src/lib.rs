extern crate image;
extern crate rayon;

use std::path::Path;

use std::fs;
use std::fs::File;
use std::fs::Metadata;
use rayon::prelude::*;
use std::io::prelude::*;

use image::{
	GenericImage,
	DynamicImage,
	ImageBuffer,
	Rgba
};


#[test]
fn test_buffer_read_write() {
	let buffer = [0x00_u8];

	let out = write_to_file(&buffer, "test.jpg".to_string());

    let ref mut pout = &Path::new("test.png");
    // Write the contents of this image to the Writer in PNG format.
    let _ = out.save(pout).unwrap();

	let out_buf = read_from_file("test.png".to_string());

	assert_eq!(0 as u8, *out_buf.get(0).unwrap());
}

#[test]
fn test_string_read_write() {
	let message = "test".to_string();
	
	let out = message_to_file(message, "test.jpg".to_string());
		
    let ref mut pout = &Path::new("test.png");
    // Write the contents of this image to the Writer in PNG format.
    let _ = out.save(pout).unwrap();

	let out_buf = message_from_file("test.png".to_string(), 4 as usize);

	assert_eq!("test".to_string(), out_buf);
}

#[test]
fn test_file_read_write() {
	let out = file_to_file("test.txt".to_string(), "test.jpg".to_string());
	
    let ref mut pout = &Path::new("test.png");
    // Write the contents of this image to the Writer in PNG format.
    let _ = out.save(pout).unwrap();
	
	let out_buf = message_from_file("test.png".to_string(), 14 as usize);
	assert_eq!("this is a test".to_string(), out_buf);
}

#[test]
fn test_will_fit() {
	let test = fs::metadata("test.txt").unwrap();
	let readme = fs::metadata("README.md").unwrap();
	let mut files = Vec::new();
	let mut images = Vec::new();

	files.push(&test);
	files.push(&readme);
	let image = Path::new("test.png");
	images.push(image);
	assert_eq!(true, will_fit(files, images));
}

pub fn write_to_file(input: &[u8], filename: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let img = image::open(&Path::new(&filename)).unwrap();
	return write_to_image(input, img);
}

pub fn read_from_file(filename: String) -> Vec<u8> {
	let img = image::open(&Path::new(&filename)).unwrap().to_rgba();
	return read_from_image(img);	
}

pub fn write_to_image(input: &[u8], img: DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let (width, height) = img.dimensions();
	let bytes = width * height;

	if input.len() > bytes as usize{
		panic!("Input is too large for image size");
	}

	let mut out = ImageBuffer::new(width, height);
	
	for (x, y, pixel) in img.pixels() {
		let mut tmp_pixel = pixel;
		
		let inputIndex = x + (y * width);
		
		if inputIndex < input.len() as u32{
			tmp_pixel.data[3] = input[inputIndex as usize];
		}

		out.put_pixel(x, y, tmp_pixel);
	}

	return out;
}

pub fn read_from_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {
	let mut out: Vec<u8> = Vec::new();

	for (x, y, pixel) in img.enumerate_pixels() {
		out.push(pixel.data[3]);
	}

	return out;
}

pub fn message_to_image(src: String, img: DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	return write_to_image(src.as_bytes(), img);
}

pub fn message_to_file(src: String, filename: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let img = image::open(&Path::new(&filename)).unwrap();
	return message_to_image(src, img);
}

pub fn message_from_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>, len: usize) -> String {
	let mut out = read_from_image(img);
	out.truncate(len);
	return String::from_utf8_lossy(&out).into_owned();	
}

pub fn message_from_file(filename: String, len: usize) -> String {
	let img = image::open(&Path::new(&filename)).unwrap().to_rgba();
	return message_from_image(img, len);
}

pub fn file_to_image(src: String, img: DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let mut f = File::open(src).expect("Could not open file");
	let mut buffer = Vec::new();
	f.read_to_end(&mut buffer).expect("Could not read file");
	return write_to_image(buffer.as_slice(), img);
}

pub fn file_to_file(src: String, filename: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let mut f = File::open(src).expect("Could not open file");
	let mut buffer = Vec::new();
	f.read_to_end(&mut buffer).expect("Could not read file");
	return write_to_file(buffer.as_slice(), filename);
}

pub fn will_fit(files: Vec<&Metadata>, images: Vec<&Path>) -> bool {
	let file_size: u64 = files.par_iter()
								.map(|&f| f.len())
								.sum();
	let image_space: u32 = images.par_iter()
									.map(|&p|{
										let img = image::open(p).unwrap();
										img.width() * img.height()
									})
									.sum();

	println!("File size: {:?} Image size: {:?}", file_size, image_space);
	image_space as u64 >= file_size
}

/*
pub fn file_from_image() {

}

pub fn file_from_file() {

}*/
