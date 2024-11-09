# Discrete Cosine Transform (DCT in Rust)
This is a image compression algorithm implemented in Rust using the Discrete Cosine Transform (DCT), inspired by the JPEG compression technique.

## Features

- Load an image and convert it to grayscale
- Apply the Discrete Cosine Transform (DCT) to the image
- Quantize the DCT coefficients
- Apply the inverse DCT to reconstruct the image
- Save the compressed image

## Prerequisites

Rust and Cargo installed. You can download them from [here](https://www.rust-lang.org/tools/install).

## Usage

1. Place the image you want to compress in the project directory and rename it to `example.jpg`.
2. Run the project:
   
   ```
   cargo run
   ```
   
3. The compressed image will be saved in the project directory.
