extern crate steganography;

use steganography::encoder::*;
use steganography::util::*;

fn main() {
    //Define a secret message to hide in out picture
    let message = "This is a steganography demo!".to_string();
    //Convert our string to bytes
    let payload = str_to_bytes(&message);
    //Load the image where we want to embed our secret message
    let destination_image = file_as_dynamic_image("examples/example.jpg".to_string());
    //Create an encoder
    let enc = Encoder::new(payload, destination_image);
    //Encode our message into the alpha channel of the image
    let result = enc.encode_alpha();
    //Save the new image
    save_image_buffer(result, "examples/hidden_message.png".to_string());
}