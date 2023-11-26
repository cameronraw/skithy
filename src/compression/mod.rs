use std::{
    env, fs,
    io::{Error, Write},
    path::PathBuf,
};

use crate::huffman::{create_byte_vec_from, HuffmanNode};

pub fn compress_file(file_path: String) {
    create_file_path(file_path).map_or_else(
        |err| panic!("Failed to compress file: {}", err),
        |file_path| match fs::read(file_path.clone()) {
            Ok(contents) => {
                let mut output_path = file_path.clone();
                output_path.set_extension("skithy");
                let huffman_tree = HuffmanNode::from_buffer(contents.clone());
                let mut encoded: Vec<bool> = vec![];
                for byte in contents {
                    let encoding = huffman_tree.find_encoding(byte).unwrap();
                    encoded.extend(encoding);
                }

                let mut file = fs::File::create(&output_path).unwrap();

                create_byte_vec_from(huffman_tree.into())
                    .into_iter()
                    .for_each(|byte| {
                        file.write_all(&[byte]).unwrap();
                    });

                // 00000000
                let mut byte = 0u8;

                let mut bit_index = 0;

                for bit in encoded {
                    if bit {
                        // 1 means 00000001
                        // << means move to the left
                        // bit_index is how many times
                        // so... if bit_index is 5, you'd get 00100000
                        // then | is bitwise OR, whilst |= is an automatic assignment
                        // if in either binary expression, one of the units is one, it will be 1 in
                        // the resulting binary expression:
                        //
                        // 00000101 |= 00101101 = 00101101
                        byte |= 1 << bit_index;
                    }
                    bit_index += 1;

                    if bit_index == 8 {
                        file.write_all(&[byte]).unwrap();
                        byte = 0;
                        bit_index = 0;
                    }
                }

                // Write any remaining bits
                if bit_index != 0 {
                    file.write_all(&[byte]).unwrap();
                }
            }
            Err(err) => {
                println!("Failed to read file: {}", err);
            }
        },
    );
}

fn create_file_path(file_path: String) -> Result<PathBuf, Error> {
    let path_buf = env::current_dir()?.join(file_path);
    Ok(path_buf)
}
