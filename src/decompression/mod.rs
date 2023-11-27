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
    todo!();
}

#[cfg(test)]
pub mod decompress_module {
    use super::parse_tree_data;

    #[test]
    fn should_parse_tree_data() {
        let tree_data = [0x00, 0, 1, 0xFF, b'A', 1, 0xFF, b'B', 1];
        let resulting_tree = parse_tree_data(&tree_data);

        assert!(resulting_tree.value.is_none());
        assert!({
            match (resulting_tree.left, resulting_tree.right) {
                (Some(left_node), Some(right_node)) => {
                    assert!(left_node.value.is_some_and(|x| x == b'A'));
                    assert!(right_node.value.is_some_and(|x| x == b'B'));
                    assert!(left_node.binary == vec![false]);
                    assert!(right_node.binary == vec![true]);
                    true
                }
                _ => false,
            }
        });
    }
}
