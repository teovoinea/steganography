use image::{
	ImageBuffer,
	GenericImage,
	DynamicImage,
	Rgba
};
	
pub struct Encoder<'a> {
	img: DynamicImage,
	input: &'a [u8]
}

impl<'a> Encoder<'a> {
	/// Creates a new encoder with a buffer to write and an image to write it to
	pub fn new(input: &[u8], img: DynamicImage) -> Encoder {
		Encoder{
			img,
			input
		}
	}

	/// Encodes the buffer into the alpha channel of the destination image
	pub fn encode_alpha(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
		let (width, height) = self.img.dimensions();
		let bytes = width * height;

		if self.input.len() > bytes as usize{
			panic!("Input is too large for image size");
		}

		let mut out = ImageBuffer::new(width, height);
		
		for (x, y, pixel) in self.img.pixels() {
			let mut tmp_pixel = pixel;
			
			let input_index = x + (y * width);
			
			if input_index < self.input.len() as u32{
				tmp_pixel.data[3] = self.input[input_index as usize];
			}

			out.put_pixel(x, y, tmp_pixel);
		}

		return out;
	}

	// Encodes the buffer into its own image using RGBA channels
	pub fn encode_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
		//4 bytes per pixel
		let mut pixels = self.input.len() / 4;
		let padding = 4 - (self.input.len() % 4);
		pixels = pixels + padding;

		//make it as close to a square as possible
		let width = (pixels as f64).sqrt().floor() as u32;
		let height = (pixels as f64 / width as f64).ceil() as u32;

		//create all the pixels
		let mut out = ImageBuffer::new(width, height);
		let mut out_pixels: Vec<(u32, u32, Rgba<u8>)> = Vec::new();
		for (x, y, pixel) in out.enumerate_pixels() {
			let tmp_pixel: &Rgba<u8> = pixel;
			let mut out_pixel = tmp_pixel.clone();
			let input_index = (x + (y * width)) * 4;
			if  input_index < self.input.len() as u32{
				let r: u8 = self.input[input_index as usize];
				let g: u8 = self.input[input_index as usize + 1];
				let b: u8 = self.input[input_index as usize + 2];
				let a: u8 = self.input[input_index as usize + 3];
				out_pixel.data = [r, g, b, a];
			}
			else {
				out_pixel.data = [0, 0, 0, 0];
			}
			out_pixels.push((x, y, out_pixel));
		}

		//write them to the output buffer
		for p in out_pixels {
			out.put_pixel(p.0, p.1, p.2);
		}

		out
	}
}
