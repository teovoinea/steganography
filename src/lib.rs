extern crate image;

use std::path::Path;

use image::{
	GenericImage,
	DynamicImage,
	ImageBuffer,
	Rgba
};


#[test]
fn it_works() {
	let buffer = [0x00_u8];

	let out = write_to_file(&buffer, "test.jpg".to_string());

    let ref mut pout = &Path::new("test.png");
    // Write the contents of this image to the Writer in PNG format.
    let _ = out.save(pout).unwrap();

	let out_buf = read_from_file("test.png".to_string());

	assert_eq!(0 as u8, *out_buf.get(0).unwrap());
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
		
		if x+y < input.len() as u32{
			tmp_pixel.data[3] = input[(x + (y*width)) as usize];
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
