///
/// Author: Andres Sanchez
/// Module to encode and decode PBM images
/// 
use std::str;
use std::{fs, vec};

pub enum PbmType {
    P1,
    P4,
}

pub struct ImageMeta {
    format: PbmType,
    width: usize,
    height: usize,
    bytes: Vec<u8>,
}

pub struct PbmParser;

impl PbmParser {

    fn parse_dimensions(line_bytes: &[u8]) -> (usize, usize) {
        let line_str = str::from_utf8(line_bytes).expect("Cannot parse dimensions line");

        let parsed_dimensions: Vec<u32> = line_str
            .split(' ')
            .map(|n_str| n_str.parse::<u32>().expect("Cannot parse number"))
            .collect();

        match parsed_dimensions.as_slice() {
            [cols, rows] => (*cols as usize, *rows as usize),
            _ => panic!("Can't parse dimensions from PBM"),
        }
    }

    fn get_image_meta(file_path: &str) -> ImageMeta {
        let file_bytes = fs::read(file_path).expect("Can't Read file");

        let mut line_start = 0;
        let mut is_comment = false;

        // Read header of the PBM file
        let mut components: Vec<&[u8]> = Vec::new();
        for (byte_index, byte) in file_bytes.iter().enumerate() {
            // If magic number and dimensions have been read, the rest of the bytes
            // correspond to the image
            if components.len() == 2 {
                components.push(&file_bytes[line_start..]);
                break;
            }

            // Flag comment lines to be ignored
            if byte_index == line_start && *byte as char == '#' {
                is_comment = true;
            }

            if *byte as char == '\n' {
                if !is_comment {
                    let current_line = &file_bytes[line_start..byte_index];
                    components.push(current_line);
                }
                line_start = byte_index + 1;
                is_comment = false;
            }
        }

        let magic_number = *components.get(0).expect("Can't parse PBM magic number");
        let magic_number = str::from_utf8(magic_number)
            .expect("Can't parse magic number")
            .to_string();

        let magic_number = match magic_number.as_str() {
            "P1" => PbmType::P1,
            "P4" => PbmType::P4,
            _ => panic!("Invalid PBM format"),
        };

        let (cols, rows) =
            Self::parse_dimensions(*components.get(1).expect("Can't parse PBM dimensions"));

        let flat_image = Vec::from(&file_bytes[line_start..]);

        ImageMeta {
            format: magic_number,
            width: cols,
            height: rows,
            bytes: flat_image,
        }
    }

    fn decode_p1(image_meta: ImageMeta) -> Vec<Vec<u8>> {
        let ImageMeta {
            bytes,
            format: _,
            width: cols,
            height: _,
        } = image_meta;

        // Replace ascii values of '0's and '1's to actual 0s and 1s
        let ascii_to_bits: Vec<u8> = bytes
            .iter()
            .filter(|ch| **ch as char != '\n')
            .map(|ch| ch - '0' as u8)
            .collect();

        // Reshape sequence of bits to a matrix with corresponding width, height
        // assumes height matches the specified in the pbm header
        ascii_to_bits
            .chunks(cols)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<u8>>>()
    }

    fn decode_p4(image_meta: ImageMeta) -> Vec<Vec<u8>> {
        let ImageMeta {
            bytes,
            format: _,
            width: cols,
            height: rows,
        } = image_meta;

        let flat_image = bytes;

        let mut image = vec![vec![0u8; cols]; rows];

        let mut current_row = 0;
        let mut current_col = 0;

        for byte in flat_image {
            for i in (0u8..8u8).rev() {
                if current_col >= cols {
                    current_row += 1;
                    current_col = 0;
                    break;
                }

                let bit = if (1 << i) & byte != 0 { 1 } else { 0 };
                image[current_row][current_col] = bit;
                current_col += 1;
            }
        }

        image
    }

    /// Decodes an PBM image into a matrix of bits, each corresponding a pixel 
    /// in the image
    pub fn decode(file_path: &str) -> Vec<Vec<u8>> {
        let image_meta = Self::get_image_meta(file_path);

        match image_meta.format {
            PbmType::P1 => Self::decode_p1(image_meta),
            PbmType::P4 => Self::decode_p4(image_meta),
        }
    }
}
