use std::fmt::Display;
use super::hash_utils::hash_leaf;


#[derive(Debug)]
pub enum Tree<T: ToString + Display> {
    Empty {
        hash: String,
    },
    Leaf {
        hash: String,
        value: T,
    },
    Node {
        hash: String,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: ToString + Display> Tree<T> {
    pub fn empty(hash: String) -> Self {
        Tree::Empty {
            hash: hash,
        }
    }

    pub fn new_leaf(value: T) -> Tree<T>        
    {
        let hash = hash_leaf(&value);
        Tree::Leaf {
            hash: hash,
            value,
        }
    }

    pub fn hash(&self) -> Option<&String> {
        match *self {
            Tree::Empty { ref hash } |
            Tree::Leaf { ref hash, ..} |
            Tree::Node { ref hash, ..} => Some(hash),
        }
    }
}


// #[derive(Debug)]
// pub struct Node<T> {
//     hash: Vec<u8>,
//     left: Option<Box<Node<T>>>,
//     right: Option<Box<Node<T>>>,
//     phantom: PhantomData<T>,
// }

// impl<T> Node<T> {
//     pub fn new(value: T) -> Node<T> {
//         Node {
//             hash: Sha256::digest(value).as_ref().into(),
//             left: None,
//             right: None,
//         }
//     }
// }

