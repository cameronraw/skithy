use clap::Parser;
use std::{collections::HashMap, env, fs, io::Error, path::PathBuf};

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
            }
            None => {
                freq_table.insert(byte, 1);
            }
        }
    }
    let mut freq_vec: Vec<(u8, usize)> = freq_table.into_iter().collect();
    freq_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    let node_vec: Vec<HuffmanNode> = vec![];
    let huffman_vec = create_huffman_vec(node_vec, freq_vec);
    let huffman_tree = create_huffman_tree(huffman_vec);
}

fn create_huffman_tree(mut huffman_vec: Vec<HuffmanNode>) -> HuffmanNode {
    if huffman_vec.is_empty() {
        return HuffmanNode::new(None, None, None);
    }

    if huffman_vec.len() == 1 {
        // TODO: Return the node
        return huffman_vec.first().unwrap().clone();
    }

    let left_child = huffman_vec.first().map(|item| Box::new(item.clone()));
    let right_child = huffman_vec.get(1).map(|item| Box::new(item.clone()));

    if left_child.is_some() {
        huffman_vec.remove(0);
    }
    if right_child.is_some() {
        huffman_vec.remove(0);
    }

    huffman_vec.push(HuffmanNode::new(None, left_child, right_child));

    create_huffman_tree(huffman_vec)
}

fn create_huffman_vec(
    mut node_vec: Vec<HuffmanNode>,
    mut freq_vec: Vec<(u8, usize)>,
) -> Vec<HuffmanNode> {
    let left_child = freq_vec.first().map(|item| {
        Box::new(HuffmanNode {
            value: Some(item.0),
            frequency: item.1,
            left: None,
            right: None,
        })
    });

    let right_child = freq_vec.get(1).map(|item| {
        Box::new(HuffmanNode {
            value: Some(item.0),
            frequency: item.1,
            left: None,
            right: None,
        })
    });

    if left_child.is_none() && right_child.is_none() {
        return node_vec;
    }

    if left_child.is_some() {
        freq_vec.remove(0);
    }

    if right_child.is_some() {
        freq_vec.remove(0);
    }

    node_vec.push(HuffmanNode::new(None, left_child, right_child));

    if !freq_vec.is_empty() {
        return create_huffman_vec(node_vec, freq_vec);
    }

    node_vec
}

#[derive(Clone)]
struct HuffmanNode {
    value: Option<u8>,
    frequency: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    pub fn new(
        value: Option<u8>,
        left: Option<Box<HuffmanNode>>,
        right: Option<Box<HuffmanNode>>,
    ) -> Self {
        let new_node = HuffmanNode {
            value,
            frequency: 0,
            left,
            right,
        };
        new_node.calculate_freq();
        new_node
    }

    fn calculate_freq(&self) -> usize {
        let left_freq = self
            .left
            .as_ref()
            .map_or_else(|| 0, |child| child.frequency);
        let right_freq = self
            .right
            .as_ref()
            .map_or_else(|| 0, |child| child.frequency);
        left_freq + right_freq
    }
}

fn create_file_path(file_path: String) -> Result<PathBuf, Error> {
    let path_buf = env::current_dir()?.join(file_path);
    Ok(path_buf)
}
