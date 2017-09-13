use image::{
	ImageBuffer,
	Rgba
};

pub struct Decoder {
	img: ImageBuffer<Rgba<u8>, Vec<u8>>
}

impl Decoder {
	/// Creates a new decoder with an image to read from
	pub fn new(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Decoder {
		Decoder {
			img
		}
	}

	/// Decodes the image by reading the alpha channel of each pixel
	pub fn decode_alpha(&self) -> Vec<u8> {
		let mut out: Vec<u8> = Vec::new();

		for (_, _, pixel) in self.img.enumerate_pixels() {
			out.push(pixel.data[3]);
		}

		out
	}
}
