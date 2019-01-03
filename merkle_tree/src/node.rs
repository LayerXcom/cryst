use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct Node<T> {
    hash: Vec<u8>,
    left: Option<Box<Node<T>>>,
    right Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            hash: Sha256::digest(value).as_ref().into(),
            left: None,
            right: None,
        }
    }
}
