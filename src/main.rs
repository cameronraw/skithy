mod compression;
mod huffman;
mod decompression;

use clap::Parser;
use compression::compress_file;
use decompression::decompress_file;

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

