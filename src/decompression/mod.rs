use std::fs;

use crate::huffman::HuffmanNode;

pub fn decompress_file(file_path: String) {
    let contents = fs::read(file_path.clone())
        .map_or_else(|err| panic!("Could not read file: {:?}", err), |f| f);
    let tree_data_len = *contents.first().unwrap();
    let tree_data: &[u8] = contents.get(1..=tree_data_len as usize).unwrap();
    let _tree = parse_tree_data(tree_data);
    // for byte in contents {
    //      let _newbit = byte >> 1;
    //      This is where we need the Huffman Tree. 0 would mean to traverse left, and 1 to traverse
    //      right.
    // }
}

fn parse_tree_data(tree_data_bytes: &[u8]) -> HuffmanNode {
    let mut huffman_nodes: Vec<HuffmanNode> = vec![];
    let mut counter = 0;
    while counter < tree_data_bytes.len() {
        let relevant_bytes = tree_data_bytes.get(counter..=counter + 2).unwrap();
        huffman_nodes.push(HuffmanNode::new(
            {
                let value = *relevant_bytes.get(1).unwrap();
                if *relevant_bytes.first().unwrap() == 0xFF_u8 {
                    Some(value)
                } else {
                    None
                }
            },
            0,
            None,
            None,
        ));
        counter += 3;
    }

    let main_node = huffman_nodes.first().cloned().unwrap();
    reconstruct_huffman_node(main_node, huffman_nodes, vec![])
}

fn reconstruct_huffman_node(mut main_node: HuffmanNode, mut huffman_nodes: Vec<HuffmanNode>, acc_binary: Vec<bool>) -> HuffmanNode {
    while huffman_nodes.len() > 1 {
        if let Some(mut left_node) = huffman_nodes.get(1).cloned() {
            let mut left_acc_binary = acc_binary.clone();
            left_acc_binary.push(false);
            left_node.binary = left_acc_binary.clone();
            huffman_nodes.remove(1);
            left_node = reconstruct_huffman_node(left_node, huffman_nodes.clone(), left_acc_binary);
            main_node.left = Some(Box::new(left_node));
        }
        if let Some(mut right_node) = huffman_nodes.get(1).cloned() {
            let mut right_acc_binary = acc_binary.clone();
            right_acc_binary.push(true);
            right_node.binary = right_acc_binary.clone();
            huffman_nodes.remove(1);
            right_node = reconstruct_huffman_node(right_node, huffman_nodes.clone(), right_acc_binary);
            main_node.right = Some(Box::new(right_node));
        }
    }
    main_node
}

#[cfg(test)]
pub mod decompress_module {
    use rstest::rstest;
    use crate::huffman::HuffmanNode;

    use super::parse_tree_data;

    #[rstest]
    #[case(&[0x00, 0, 1, 0xFF, b'A', 1, 0xFF, b'B', 1])]
    #[case(&[
        0x00, 0, 1,
        0xFF, b'C', 1,
        0x00, 0, 2, 
        0xFF, b'D', 2, 
        0xFF, b'E', 2, 
    ])]
    fn should_parse_tree_data(#[case] input: &[u8]) {
        let resulting_tree = parse_tree_data(input);
        assert_tree_against_bytes(resulting_tree, input.chunks(3).collect(), vec![]);
    }

    fn assert_tree_against_bytes(tree: HuffmanNode, mut bytes: Vec<&[u8]>, acc_binary: Vec<bool>) -> HuffmanNode {
        let this_nodes_slice = *bytes.first().clone().unwrap();
        bytes.remove(0);

        assert_eq!(tree.binary, acc_binary);
        if this_nodes_slice.first().cloned().unwrap() == 0xFF_u8 {
            let expected = this_nodes_slice.get(1).cloned().unwrap();
            let actual = tree.value.unwrap();
            assert_eq!(expected, actual);
        } else if this_nodes_slice.first().cloned().unwrap() == 0x00_u8 {
            assert!(tree.value.is_none());
            assert!(tree.left.is_some() || tree.right.is_some());
            if let Some(ref left_node) = tree.left {
                let mut left_binary = acc_binary.clone();
                left_binary.push(false);
                assert_tree_against_bytes(*left_node.clone(), bytes, left_binary);
            }
            if let Some(ref right_node) = tree.right {
                let mut right_binary = acc_binary.clone();
                right_binary.push(true);
                assert_tree_against_bytes(*right_node.clone(), bytes, right_binary);
            }
        }
        tree
    }
}
