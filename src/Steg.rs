extern crate image;

use std::path::Path;

use std::io::prelude::*;
use std::fs::File;

use image::{
	GenericImage,
	DynamicImage,
	ImageBuffer,
	Rgba
};

pub struct Steg {
	input: [u8],
	container: DynamicImage,
	method: Method,
	action: Action,
}

pub enum Method {
	Alpha,
}

pub enum Action {
	Read,
	Write,
}

impl Steg {

	pub fn new(a: Action) -> Steg {
		Steg {
			//self.method = Method::Alpha;
			//self.input = [0x00_u8];
			self.action = a;
		}
	}

	pub fn input_as_file<'a>(&'a mut self, filename: String) -> &'a mut Steg {
		let mut f = File::open(src).expect("Could not open file");
		let mut buffer = Vec::new();
		f.read_to_end(&mut buffer).expect("Could not read file");
		self.input = buffer.as_slice();
		self
	}

	pub fn input_as_string<'a>(&'a mut self, text: String) -> &'a mut Steg {
		self.input = text.as_bytes();
		self
	}

	pub fn input_as_bytes<'a>(&'a mut self, buffer: &[u8]) -> &'a mut Steg {
		self.input = buffer;
		self
	}

	pub fn encrypt<'a>(&'a mut self, key: String) -> &'a mut Steg {
		self
	}

	pub fn decrypt<'a>(&'a mut self, ley: String) -> &'a mut Steg {
		self
	}

	pub fn container_as_file<'a>(&'a mut self, filename: String) -> &'a mut Steg {
		self.container = image::open(&Path::new(&filename)).unwrap();
		self
	}

	pub fn container_as_img<'a>(&'a mut self, img: DynamicImage) -> &'a mut Steg {
		self.container = img;
		self
	}

	pub fn execute(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
		match self.action {
			Action::Write => {

			}

			Action::Read => {
				
			}
		}
	}

/*
	pub fn input<'a>(&'a mut self, bytes: &[u8]) -> &'a mut Steg{
		self.input = bytes;
		self
	}

	pub fn method<'a>(&'a mut self, method: Method) -> &'a mut Steg{
		self.input = method;
		self
	}

	pub fn execute(&self) -> Steg::output {
		match self.method {
			Method::Alpha => apply_alpha(),
			None => expr,
		}
	}

	fn apply_alpha(&self) -> Steg::output {
		if self.method != Method::Alpha {
			panic!("Method is not alpha");
		}
	}
}

pub fn string_to_bytes(message: String) -> &[u8] {

}

pub fn file_to_bytes(filename: String) -> &[u8] {

}

var s = Steg::new(Action::Write).input_as_file("my_file.png").encrypt("some_key").execute().output_as_bytes();
var ns = Steg::new(Action::Read).input_as_bytes(Vec<u8>).decrypt("some_key").execute().output_as_file("my_file.png");

input_as_x => turns things into bytes
encrypt/decrypt applies the functions to the input bytes

*/