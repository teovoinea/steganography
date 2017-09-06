use image::{
	ImageBuffer,
	DynamicImage,
	Rgba
};
	
pub struct Encoder {
	img: DynamicImage,
	input: &[u8]
}

impl Encoder {
	
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
			
			let inputIndex = x + (y * width);
			
			if inputIndex < self.input.len() as u32{
				tmp_pixel.data[3] = self.input[inputIndex as usize];
			}

			out.put_pixel(x, y, tmp_pixel);
		}

		return out;
	}
}
