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
	
	pub fn new(input: &[u8], img: DynamicImage) -> Encoder {
		Encoder{
			img,
			input
		}
	}

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
}
