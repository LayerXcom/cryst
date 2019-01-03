use std::fmt::Display;
use super::tree::Tree;
use super::hash_utils::*;


#[derive(Debug)]
pub struct MerkleTree<T: ToString + Display> {
    root: Tree<T>,
    height: usize,
    leaves_num: usize,    
    // pub algorithm: &'static Algorithm,
}


impl<T: ToString + Display> MerkleTree<T> {
    
    pub fn create_tree(data: Vec<T>) -> Self         
    {
        if data.is_empty() {
            return MerkleTree {
                    root: Tree::empty(empty_hash()),
                    height: 0,
                    leaves_num: 0,
                };
        }

        let leaves_num = data.len();
        let mut height = 0;
        let mut cur = Vec::with_capacity(leaves_num);

        for d in data {    // can be closure?
            let leaf = Tree::new_leaf(d);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = hash_node(left.hash().unwrap(), right.hash().unwrap());

                    let node = Tree::Node {
                        hash: combined_hash,
                        left: Box::new(left),
                        right: Box::new(right),
                    };

                    next.push(node);
                }
            }

            height += 1;
            cur = next;
        }

        debug_assert!(cur.len() == 1);
        let root = cur.remove(0);

        MerkleTree {
            root,
            height,
            leaves_num,
        }
    }

    // pub fn get_proof() {

    // }

    pub fn get_root(&self) -> String {
        self.root.hash().unwrap().to_string()
        // Sha256::digest(self.root.to_string().as_ref()).as_ref().into()
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_leaves_num(&self) -> usize {
        self.leaves_num
    }


}