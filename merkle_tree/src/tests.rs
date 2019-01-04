#![cfg(test)]

use crate::hash_utils::{hash_leaf, hash_node};
use crate::merkle_tree::MerkleTree;

#[test]
fn test_from_str_vec() {
    let values = vec!["one", "two", "three", "four"];

    let hashes = vec! [
        hash_leaf(&values[0]),
        hash_leaf(&values[1]),
        hash_leaf(&values[2]),
        hash_leaf(&values[3]),
    ];

    let leaves_num = values.len();
    let tree = MerkleTree::create_tree(values);

    let hash01 = hash_node(&hashes[0], &hashes[1]);
    let hash23 = hash_node(&hashes[2], &hashes[3]);

    let root = hash_node(&hash01, &hash23);
    
    assert_eq!(tree.get_leaves_num(), leaves_num);
    assert_eq!(tree.get_height(), 2);
    assert_eq!(tree.get_root(), root);
}