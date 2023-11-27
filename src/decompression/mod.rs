use std::fs;

pub fn decompress_file(file_path: String) {
    let contents = fs::read(file_path.clone())
        .map_or_else(|err| panic!("Could not read file: {:?}", err), |f| f);
    for byte in contents {
        let _newbit = byte >> 1;
        // This is where we need the Huffman Tree. 0 would mean to traverse left, and 1 to traverse
        // right.
    }
}
