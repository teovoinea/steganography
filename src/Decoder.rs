use image::{
	ImageBuffer
};

pub struct Decoder {
	img: ImageBuffer
}

impl Decoder {

	pub fn new(img: ImageBuffer) -> Decoder {
		Decoder {
			img
		}
	}

	pub fn decode_alpha(&self) -> &[u8] {
		let mut out: Vec<u8> = Vec::new();

		for (x, y, pixel) in self.img.enumerate_pixels() {
			out.push(pixel.data[3]);
		}

		return out.as_slice();
	}
}
