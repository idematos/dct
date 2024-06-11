# Discrete Cosine Transform (DCT in Rust)
This is a image compression algorithm implemented in Rust using the Discrete Cosine Transform (DCT), inspired by the JPEG compression technique.

## Features

- Load an image and convert it to grayscale
- Apply the Discrete Cosine Transform (DCT) to the image
- Quantize the DCT coefficients
- Apply the inverse DCT to reconstruct the image
- Save the compressed image

## Getting Started

### Prerequisites

- Rust and Cargo installed. You can download them from [here](https://www.rust-lang.org/tools/install).

### Dependencies

This project uses the following dependencies:

- `image`: For image processing and handling
- `imageproc`: For additional image processing functions
- `nalgebra`: For matrix operations

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
image = "0.23"
nalgebra = "0.29"

