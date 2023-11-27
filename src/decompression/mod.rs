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
        println!("{:?}", relevant_bytes);
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

        assert!(resulting_tree.value.is_none());
        match (resulting_tree.left, resulting_tree.right) {
            (Some(left_node), Some(right_node)) => {
                assert!(left_node.value.is_some_and(|x| x == b'A'));
                assert!(right_node.value.is_some_and(|x| x == b'B'));
                assert!(left_node.binary == vec![false]);
                assert!(right_node.binary == vec![true]);
            }
            (None, None) => panic!("Left and Right nodes were None"),
            (None, Some(_)) => panic!("The Left node was None"),
            (Some(_), None) => panic!("The Right node was None"),
        }
    }

    fn assert_tree_against_bytes(tree: HuffmanNode, bytes: &[u8]) {
        if bytes.first().cloned().unwrap() == 0xFF_u8 {
            assert!(tree.value.is_some_and(|x| x == bytes.get(1).cloned().unwrap()));
        } else if bytes.first().cloned().unwrap() == 0x00_u8 {
            assert!(tree.value.is_none());
        }

    }
}
