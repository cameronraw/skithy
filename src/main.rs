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
    let freq_table = create_frequency_table(contents);
    // let mut freq_vec: Vec<(u8, usize)> = freq_table.into_iter().collect();
    // freq_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    let mut freq_vec = frequency_table_to_ordered_tuple_vec(freq_table);
    let node_vec: Vec<HuffmanNode> = vec![];
    let huffman_vec = create_huffman_vec(node_vec, freq_vec);
    let huffman_tree = create_huffman_tree(huffman_vec);
}

fn frequency_table_to_ordered_tuple_vec(freq_table: HashMap<u8, usize>) -> Vec<(u8, usize)> {
    let mut freq_vec: Vec<(u8, usize)> = freq_table.into_iter().collect();
    freq_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    freq_vec
}

fn create_frequency_table(contents: Vec<u8>) -> HashMap<u8, usize> {
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
    freq_table
}

fn create_huffman_tree(mut huffman_vec: Vec<HuffmanNode>) -> HuffmanNode {
    if huffman_vec.is_empty() {
        return HuffmanNode::new(None, 0, None, None);
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

    huffman_vec.push(HuffmanNode::new(None, 0, left_child, right_child));

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

    node_vec.push(HuffmanNode::new(None, 0, left_child, right_child));

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
        frequency: usize,
        left: Option<Box<HuffmanNode>>,
        right: Option<Box<HuffmanNode>>,
    ) -> Self {
        HuffmanNode {
            value,
            frequency,
            left,
            right,
        }
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

fn create_huffman_node_vec(ordered_vec: Vec<(u8, usize)>) -> Vec<HuffmanNode> {
    let mut huffman_node_vec: Vec<HuffmanNode> = vec![];
    ordered_vec.into_iter().for_each(|tuple| {
        huffman_node_vec.push(HuffmanNode::new(Some(tuple.0), tuple.1, None, None))
    });
    huffman_node_vec
}

#[cfg(test)]
pub mod skithy_should {
    use std::collections::HashMap;

    use rstest::rstest;

    use super::{
        create_frequency_table, create_huffman_node_vec, frequency_table_to_ordered_tuple_vec,
        HuffmanNode,
    };

    #[rstest]
    #[case(vec![1, 2, 2, 2, 2, 2, 4, 4, 5, 5, 5, 5, 5, 5], vec![(1, 1), (2, 5), (4, 2), (5, 6)])]
    #[case(vec![1, 1, 1, 12, 6, 6, 4, 4, 4, 4, 4, 4, 4, 4], vec![(1, 3), (12, 1), (6, 2), (4, 8)])]
    #[case(vec![22, 22, 22, 76, 76, 7, 7, 7, 5], vec![(22, 3), (76, 2), (7, 3), (5, 1)])]
    fn create_frequency_tables_from_vec_u8(
        #[case] input: Vec<u8>,
        #[case] expected: Vec<(u8, usize)>,
    ) {
        let freq_table = create_frequency_table(input);
        expected.into_iter().for_each(|expected_results| {
            assert_eq!(
                freq_table
                    .get(&expected_results.0)
                    .cloned()
                    .expect("No frequency value found."),
                expected_results.1
            );
        });
    }

    #[rstest]
    #[case(HashMap::from([(6, 1), (2, 3), (4, 2)]), vec![(6, 1), (4, 2), (2, 3)])]
    #[case(HashMap::from([(30, 300), (10, 100), (20, 200)]), vec![(10, 100), (20, 200), (30, 300)])]
    #[case(HashMap::from([(15, 15), (5, 5), (10, 10)]), vec![(5, 5), (10, 10), (15, 15)])]
    #[case(HashMap::from([(255, 1000), (0, 500), (127, 750)]), vec![(0, 500), (127, 750), (255, 1000)])]
    fn create_ordered_vec_from_frequency_table(
        #[case] freq_table: HashMap<u8, usize>,
        #[case] expected: Vec<(u8, usize)>,
    ) {
        let ordered_tuple_vec = frequency_table_to_ordered_tuple_vec(freq_table);

        ordered_tuple_vec
            .into_iter()
            .enumerate()
            .for_each(|(i, tuple)| {
                assert_eq!(tuple, expected.get(i).cloned().expect("Out of bounds"))
            })
    }

    #[rstest]
    #[case(vec![(10, 10), (20, 20), (30, 30)])]
    #[case(vec![(5, 100), (6, 200), (7, 300)])]
    #[case(vec![(255, 1), (254, 2), (253, 3)])]
    #[case(vec![(1, 500), (2, 500), (3, 500)])]
    #[case(vec![(12, 50), (24, 100), (36, 150)])]
    #[case(vec![(100, 1000), (101, 2000), (102, 3000)])]
    #[case(vec![(0, 0), (1, 1), (2, 2)])]
    fn convert_ordered_vec_u8_usize_to_ordered_vec_huffman_node(#[case] input: Vec<(u8, usize)>) {
        let huffman_node_vec: Vec<HuffmanNode> = create_huffman_node_vec(input.clone());

        huffman_node_vec
            .into_iter()
            .enumerate()
            .for_each(|(index, node)| {
                let expected_node_details = input.get(index).cloned().expect("Out of bounds");
                assert_eq!(
                    node.value.expect("Actual value was set to None"),
                    expected_node_details.0
                );
                assert_eq!(expected_node_details.1, node.frequency);
            });
    }
}
