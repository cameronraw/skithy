mod compression;
mod huffman;

use clap::Parser;
use compression::compress_file;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    file_path: Option<String>,
    #[arg(short, long)]
    skithy_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(file_path) = args.file_path {
        compress_file(file_path);
    } else if let Some(skithy_file) = args.skithy_file {
        decompress_file(skithy_file);
    } else {
        panic!(
            "Too few or too many arguments passed. Choose to either compress or decompress a file"
        )
    }
}

fn decompress_file(file_path: String) {
    let contents = fs::read(file_path.clone())
        .map_or_else(|err| panic!("Could not read file: {:?}", err), |f| f);
    for byte in contents {
        println!("Binary before shift: {:b}", byte);
        let newbit = byte >> 1;
        println!("Binary after shift: {:b}", newbit);
    }
}
