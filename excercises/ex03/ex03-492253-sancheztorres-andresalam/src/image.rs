///
/// Author: Andres Sanchez
/// Module to represent (bitmap) images and perform
/// basic operations, e.g. resize, slice, write
/// 
use std::fs::File;
use std::io::{Error, Write};
use std::str;
use std::vec;

pub struct Image {
    bits: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Image {
    /// Constructs a bitmap image from a matrix of pixels
    pub fn from_bits(bits: Vec<Vec<u8>>) -> Image {
        let height = bits.len();
        let width = bits[0].len();

        Image {
            bits,
            width,
            height,
        }
    }

    /// Creates an image, resized exactly to the given dimensions.
    /// Original proportions are not kept.
    pub fn nearest_neighbor_resize(&self, target_width: usize, target_height: usize) -> Image {
        let Image {
            bits: image,
            width: source_width,
            height: source_height,
        } = &self;
        let mut target_image = vec![vec![0; target_width]; target_height];

        // Maps a pixel from the target image to a pixel in the source image using nearest neighbor strategy
        let translate_target_pixel_from_src =
            |value: usize, target_dim: usize, source_dim: usize| {
                ((value as f32 / target_dim as f32 * source_dim as f32).round() as usize)
                    .min(source_dim - 1)
            };

        for x in 0..target_width {
            for y in 0..target_height {
                let source_x =
                    translate_target_pixel_from_src(x, target_width, *source_width) as usize;
                let source_y =
                    translate_target_pixel_from_src(y, target_height, *source_height) as usize;
                target_image[y][x] = image[source_y][source_x];
            }
        }

        Image {
            bits: target_image,
            width: target_width,
            height: target_height,
        }
    }

    /// Returns a slice of the image bits
    pub fn slice(&self, x: usize, y: usize, width: usize, height: usize) -> Vec<&[u8]> {
        let mut image_slice: Vec<&[u8]> = Vec::new();
        let image = &self.bits;

        for i in y..(y + height) {
            image_slice.push(&image[i][x..(x + width)]);
        }

        image_slice
    }

    /// Writes a matrix of bits into a PBM (ASCII) image
    pub fn raw_write(image: &Vec<&[u8]>, output_path: &str) -> Result<usize, Error> {
        let height = image.len();
        let width = image.get(0).expect("Invalid: Empty image").len();

        let mut output = File::create(output_path).expect("Cannot create output path");

        let image_str = image
            .iter()
            .map(|row| {
                let row_utf8 = row.iter().map(|ch| '0' as u8 + *ch).collect::<Vec<u8>>();
                str::from_utf8(row_utf8.as_slice())
                    .expect("Cannot parse bits into string")
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let output_str = format!("P1\n{} {}\n{}", width, height, image_str);

        output.write(output_str.as_bytes())
    }

    /// Writes the current image into a PBM (ASCII) image
    pub fn write(&self, output_path: &str) -> Result<usize, Error> {
        let image = &self.bits.iter().map(|row| row.as_slice()).collect();
        Self::raw_write(&image, output_path)
    }
}
