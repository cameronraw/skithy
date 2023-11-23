use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HuffmanNode {
    pub value: Option<u8>,
    pub binary: Vec<bool>,
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
            binary: vec![],
            frequency,
            left,
            right,
        }
    }

    pub fn from(
        value: Option<u8>,
        left: Option<Box<HuffmanNode>>,
        right: Option<Box<HuffmanNode>>,
    ) -> Self {
        let mut node = HuffmanNode {
            value,
            binary: vec![],
            frequency: 0,
            left,
            right,
        };

        node.calc_freq();

        node
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        let freq_table = HuffmanNode::create_frequency_table(buffer);
        let freq_vec = HuffmanNode::frequency_table_to_ordered_tuple_vec(freq_table);
        let huffman_vec = HuffmanNode::create_huffman_node_vec(freq_vec);
        let mut huffman_tree = HuffmanNode::create_huffman_tree(huffman_vec);
        huffman_tree.assign_binary(vec![]);
        huffman_tree
    }

    pub fn find_encoding(&self, byte: u8) -> Option<Vec<bool>> {
        let mut encoding: Option<Vec<bool>> = None;
        if self.value.is_some_and(|x| x == byte) {
            encoding = Some(self.binary.clone());
        }
        if let Some(ref left_node) = self.left {
            if encoding.is_none() {
                encoding = left_node.find_encoding(byte)
            }
        }
        if let Some(ref right_node) = self.right {
            if encoding.is_none() {
                right_node.find_encoding(byte);
                encoding = right_node.find_encoding(byte)
            }
        }
        encoding
    }
    pub fn assign_binary(&mut self, mut prefix: Vec<bool>) {
        if self.value.is_some() {
            self.binary.extend(prefix);
        } else {
            if let Some(ref mut left_node) = self.left {
                prefix.push(false);
                left_node.assign_binary(prefix.clone());
            }
            if let Some(ref mut right_node) = self.right {
                prefix.push(true);
                right_node.assign_binary(prefix.clone());
            }
        }
    }

    fn calc_freq(&mut self) {
        self.frequency += self.left.as_ref().map_or(0, |v| v.frequency)
            + self.right.as_ref().map_or(0, |v| v.frequency);
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
    fn frequency_table_to_ordered_tuple_vec(freq_table: HashMap<u8, usize>) -> Vec<(u8, usize)> {
        let mut freq_vec: Vec<(u8, usize)> = freq_table.into_iter().collect();
        freq_vec.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        freq_vec
    }
    fn create_huffman_node_vec(ordered_vec: Vec<(u8, usize)>) -> Vec<HuffmanNode> {
        let mut huffman_node_vec: Vec<HuffmanNode> = vec![];
        ordered_vec.into_iter().for_each(|tuple| {
            huffman_node_vec.push(HuffmanNode::new(Some(tuple.0), tuple.1, None, None))
        });
        huffman_node_vec
    }
    fn create_huffman_tree(mut huffman_vec: Vec<HuffmanNode>) -> HuffmanNode {
        if huffman_vec.is_empty() {
            return HuffmanNode::new(None, 0, None, None);
        }

        if huffman_vec.len() == 1 {
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

        huffman_vec.push(HuffmanNode::from(None, left_child, right_child));

        HuffmanNode::create_huffman_tree(huffman_vec)
    }
}

#[cfg(test)]
pub mod skithy_should {
    use std::collections::HashMap;
    use super::HuffmanNode;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1, 2, 2, 2, 2, 2, 4, 4, 5, 5, 5, 5, 5, 5], vec![(1, 1), (2, 5), (4, 2), (5, 6)])]
    #[case(vec![1, 1, 1, 12, 6, 6, 4, 4, 4, 4, 4, 4, 4, 4], vec![(1, 3), (12, 1), (6, 2), (4, 8)])]
    #[case(vec![22, 22, 22, 76, 76, 7, 7, 7, 5], vec![(22, 3), (76, 2), (7, 3), (5, 1)])]
    fn create_frequency_tables_from_vec_u8(
        #[case] input: Vec<u8>,
        #[case] expected: Vec<(u8, usize)>,
    ) {
        let freq_table = HuffmanNode::create_frequency_table(input);
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
        let ordered_tuple_vec = HuffmanNode::frequency_table_to_ordered_tuple_vec(freq_table);

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
        let huffman_node_vec: Vec<HuffmanNode> = HuffmanNode::create_huffman_node_vec(input.clone());

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

    #[rstest]
    #[case(
        vec![
            HuffmanNode::new(Some(3), 5, None, None),
            HuffmanNode::new(Some(5), 10, None, None),
            HuffmanNode::new(Some(1), 15, None, None),
            HuffmanNode::new(Some(2), 20, None, None),
            HuffmanNode::new(Some(6), 25, None, None),
            HuffmanNode::new(Some(4), 30, None, None),
        ]
    )]
    #[case(
        vec![
            HuffmanNode::new(Some(10), 5, None, None),
            HuffmanNode::new(Some(11), 9, None, None),
            HuffmanNode::new(Some(7), 12, None, None),
            HuffmanNode::new(Some(12), 13, None, None),
            HuffmanNode::new(Some(8), 17, None, None),
            HuffmanNode::new(Some(9), 22, None, None),
        ]
    )]
    #[case(
        vec![
            HuffmanNode::new(Some(13), 3, None, None),
            HuffmanNode::new(Some(14), 6, None, None),
            HuffmanNode::new(Some(15), 9, None, None),
            HuffmanNode::new(Some(16), 12, None, None),
            HuffmanNode::new(Some(17), 15, None, None),
        ]
    )]
    #[case(
        vec![
            HuffmanNode::new(Some(19), 1, None, None),
            HuffmanNode::new(Some(20), 4, None, None),
            HuffmanNode::new(Some(21), 7, None, None),
            HuffmanNode::new(Some(22), 10, None, None),
            HuffmanNode::new(Some(18), 20, None, None),
        ]
    )]
    fn convert_huffman_node_vec_to_tree(#[case] input: Vec<HuffmanNode>) {
        let huffman_node_tree = HuffmanNode::create_huffman_tree(input.clone());

        // The head node's frequency is the sum of all frequencies
        assert_eq!(
            huffman_node_tree.frequency,
            input
                .iter()
                .map(|node| node.frequency)
                .reduce(|acc, x| acc + x)
                .unwrap()
        );

        assert!(tree_contains_all_leaf_nodes(huffman_node_tree, &input));
    }

    fn tree_contains_all_leaf_nodes(
        huffman_tree: HuffmanNode,
        original_vec: &Vec<HuffmanNode>,
    ) -> bool {
        let mut vec_to_check = original_vec.clone();

        if let Some(value) = huffman_tree.value {
            let matching_item_vec: Vec<HuffmanNode> = vec_to_check
                .clone()
                .into_iter()
                .filter(|node| node.value.is_some_and(|x| x == value))
                .collect();
            assert_eq!(matching_item_vec.len(), 1);

            let matching_item = matching_item_vec.first().unwrap();
            assert_eq!(huffman_tree.frequency, matching_item.frequency);
            assert!(huffman_tree.left.is_none());
            assert!(huffman_tree.right.is_none());
            vec_to_check.retain(|x| x.value.is_none() || x.value.is_some_and(|y| y != value));

            true
        } else {
            let left_result = if let Some(next_left) = huffman_tree.left {
                tree_contains_all_leaf_nodes(*next_left, original_vec)
            } else {
                false
            };
            let right_result = if let Some(next_right) = huffman_tree.right {
                tree_contains_all_leaf_nodes(*next_right, original_vec)
            } else {
                false
            };

            left_result || right_result
        }
    }
}
