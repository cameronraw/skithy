use clap::Parser;
use std::{env, fs, io::Error, path::PathBuf, collections::HashMap};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    create_file_path(args.file_path).map_or_else(
        |err| panic!("Failed to compress file: {}", err),
        |file_path| match fs::read(file_path) {
            Ok(contents) => {
                on_read_successful(contents);
            }
            Err(err) => {
                println!("Failed to read file: {}", err);
            }
        },
    );
}

fn on_read_successful(contents: Vec<u8>) {
    let mut freq_table: HashMap<u8, usize> = HashMap::new();
    for byte in contents.into_iter() {
        match freq_table.get(&byte) {
            Some(count) => {
                freq_table.insert(byte, count + 1);
            },
            None => {
                freq_table.insert(byte, 1);
            },
        }
    }
    println!("{:?}", freq_table);
}

fn create_file_path(file_path: String) -> Result<PathBuf, Error> {
    let path_buf = env::current_dir()?.join(file_path);
    Ok(path_buf)
}
