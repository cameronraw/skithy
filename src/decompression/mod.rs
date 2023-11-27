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

    println!("Huffman Nodes after parsing: {:?}", huffman_nodes);

    // loop {
    //     let mut main_node = huffman_nodes.first().unwrap().clone();
    //     if huffman_nodes.len() <= 1 {
    //         return main_node;
    //     }
    //     if let Some(left_node) = huffman_nodes.get(1).cloned() {
    //         main_node.left = Some(Box::new(left_node));
    //         huffman_nodes.remove(1);
    //     }
    //     if let Some(right_node) = huffman_nodes.get(1).cloned() {
    //         main_node.right = Some(Box::new(right_node));
    //         huffman_nodes.remove(1);
    //     }
    //     println!(
    //         "Main node - vec len {:?}: {:?}",
    //         huffman_nodes.len(),
    //         main_node
    //     );
    //     if huffman_nodes.len() == 1 {
    //         return huffman_nodes.first().cloned().unwrap();
    //     }
    // }

    while huffman_nodes.len() > 1 {
        let mut main_node = huffman_nodes.first().cloned().unwrap();
        if let Some(mut left_node) = huffman_nodes.get(1).cloned() {
            left_node.binary = vec![false];
            main_node.left = Some(Box::new(left_node));
            huffman_nodes.remove(1);
        }
        if let Some(mut right_node) = huffman_nodes.get(1).cloned() {
            right_node.binary = vec![true];
            main_node.right = Some(Box::new(right_node));
            huffman_nodes.remove(1);
        }
        if huffman_nodes.len() == 1 {
            return main_node;
        }
    }

    huffman_nodes.first().cloned().unwrap()
}

#[cfg(test)]
pub mod decompress_module {
    use super::parse_tree_data;

    #[test]
    fn should_parse_tree_data() {
        let tree_data = [0x00, 0, 1, 0xFF, b'A', 1, 0xFF, b'B', 1];
        let resulting_tree = parse_tree_data(&tree_data);

        print!("Resulting tree: {:?}", resulting_tree);

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
}
