use std::{fs, env};

fn main() {
    let file_path = env::current_dir().unwrap().join("testfile.txt");
    println!("File path: {:?}", file_path);
    match fs::read(file_path) {
        Ok(contents) => {
            println!("File contents: {:?}", contents);
        }
        Err(err) => {
            println!("Failed to read file: {}", err);
        }
    }
}
