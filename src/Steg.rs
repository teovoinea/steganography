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
	ouput: ImageBuffer<Rgba<u8>, Vec<u8>>,
	method: Method,
}

pub enum Method {
	Alpha,
}

impl Steg {

	pub fn new() -> Steg {
		Steg {
			self.method = Method::Alpha;
			self.input = [0x00_u8];
		}
	}

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