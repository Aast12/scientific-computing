//! 
//! Reads an image (PBM format) of a grid of 16 x 30 of
//! character samples (0-9 A-F).
//! 
//! Prints out the geometric mean of the bits of each digit and saves 
//! an image for each character sample and labels them accordingly 
//! ({character}{sample_number}.pbm).
//! 
//! Arguments: 
//!     - Input file of the image, defaults to ./digits.pbm 
//!     - Output directory for the character images, defaults to ./output
//! 
//! 
use std::env;

mod image;
mod pbm;

use image::Image;
use pbm::PbmParser;

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).unwrap_or("./digits.pbm".to_string());
    let output_dir = args.nth(2).unwrap_or("./output".to_string());

    let image_bits = PbmParser::decode(filename.as_str());
    let image = Image::from_bits(image_bits);

    const ROWS: usize = 16;
    const COLS: usize = 30;
    const GRID_SIZE: usize = 128;

    let resized_image = image.nearest_neighbor_resize(GRID_SIZE * COLS, GRID_SIZE * ROWS);

    // Map image row indices (0 - 15) to ther corresponding digits (1-9 0 A-F)
    let digits = (0..ROWS).map(|row| {
        if row < 10 {
            ('0' as u8 + (row as u8 + 1) % 10) as char
        } else {
            (('A' as u8) + (row as u8 % 10)) as char
        }
    });

    digits.enumerate().for_each(|(row, digit)| {
        let curr_digit_black_pixels = (0..COLS).map(|col| {
            let sample =
                resized_image.slice(col * GRID_SIZE, row * GRID_SIZE, GRID_SIZE, GRID_SIZE);

            let repetition = format!("{:0>2}", col + 1);

            let output_path = format!("{}/{}{}.pbm", output_dir, digit, repetition);

            Image::raw_write(&sample, output_path.as_str()).expect("Could not write output file");

            sample
                .iter()
                .map(|row| row.iter().filter(|x| **x != 0u8).count() as f64)
                .sum::<f64>()
        });

        let geometric_mean =
            (curr_digit_black_pixels.product::<f64>() as f64).powf(1.0 / COLS as f64);

        println!("{}: {:?}", digit, geometric_mean);
    })
}
