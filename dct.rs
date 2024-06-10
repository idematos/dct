extern crate image;
extern crate imageproc;
extern crate nalgebra as na;

use image::{GenericImageView, GrayImage, Luma};
use imageproc::gradients::sobel_gradients;
use na::DMatrix;
use std::f64::consts::PI;

fn main() {
    // Load the image
    let img = image::open("example.jpg").unwrap().to_luma8();

    // Convert image to DMatrix
    let (width, height) = img.dimensions();
    let mut matrix = DMatrix::zeros(height as usize, width as usize);
    for y in 0..height {
        for x in 0..width {
            matrix[(y as usize, x as usize)] = img.get_pixel(x, y).0[0] as f64;
        }
    }

    // Apply DCT
    let dct_matrix = dct2(&matrix);

    // Quantize DCT coefficients
    let quant_matrix = quantize(&dct_matrix);

    // Apply inverse DCT
    let idct_matrix = idct2(&quant_matrix);

    // Convert DMatrix back to image
    let mut compressed_img = GrayImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let pixel_value = idct_matrix[(y as usize, x as usize)].round().clamp(0.0, 255.0) as u8;
            compressed_img.put_pixel(x, y, Luma([pixel_value]));
        }
    }

    // Save the compressed image
    compressed_img.save("compressed_example.png").unwrap();
}

// Apply 2D DCT
fn dct2(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let n = matrix.nrows();
    let m = matrix.ncols();
    let mut dct_matrix = DMatrix::zeros(n, m);
    for u in 0..n {
        for v in 0..m {
            let mut sum = 0.0;
            for x in 0..n {
                for y in 0..m {
                    sum += matrix[(x, y)]
                        * ((PI * u as f64 * (2.0 * x as f64 + 1.0) / (2.0 * n as f64)).cos())
                        * ((PI * v as f64 * (2.0 * y as f64 + 1.0) / (2.0 * m as f64)).cos());
                }
            }
            let c_u = if u == 0 { (1.0 / (n as f64).sqrt()) } else { (2.0 / (n as f64)).sqrt() };
            let c_v = if v == 0 { (1.0 / (m as f64).sqrt()) } else { (2.0 / (m as f64)).sqrt() };
            dct_matrix[(u, v)] = c_u * c_v * sum;
        }
    }
    dct_matrix
}

// Apply 2D inverse DCT
fn idct2(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let n = matrix.nrows();
    let m = matrix.ncols();
    let mut idct_matrix = DMatrix::zeros(n, m);
    for x in 0..n {
        for y in 0..m {
            let mut sum = 0.0;
            for u in 0..n {
                for v in 0..m {
                    let c_u = if u == 0 { (1.0 / (n as f64).sqrt()) } else { (2.0 / (n as f64)).sqrt() };
                    let c_v = if v == 0 { (1.0 / (m as f64).sqrt()) } else { (2.0 / (m as f64)).sqrt() };
                    sum += c_u
                        * c_v
                        * matrix[(u, v)]
                        * ((PI * u as f64 * (2.0 * x as f64 + 1.0) / (2.0 * n as f64)).cos())
                        * ((PI * v as f64 * (2.0 * y as f64 + 1.0) / (2.0 * m as f64)).cos());
                }
            }
            idct_matrix[(x, y)] = sum;
        }
    }
    idct_matrix
}

// Quantize DCT coefficients
fn quantize(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let quantization_table = [
        [16, 11, 10, 16, 24, 40, 51, 61],
        [12, 12, 14, 19, 26, 58, 60, 55],
        [14, 13, 16, 24, 40, 57, 69, 56],
        [14, 17, 22, 29, 51, 87, 80, 62],
        [18, 22, 37, 56, 68, 109, 103, 77],
        [24, 35, 55, 64, 81, 104, 113, 92],
        [49, 64, 78, 87, 103, 121, 120, 101],
        [72, 92, 95, 98, 112, 100, 103, 99],
    ];
    let mut quant_matrix = matrix.clone();
    let (n, m) = matrix.shape();
    for i in 0..n {
        for j in 0..m {
            quant_matrix[(i, j)] = (matrix[(i, j)] / quantization_table[i % 8][j % 8] as f64).round();
        }
    }
    quant_matrix
}
