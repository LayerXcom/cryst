use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::string::ToString;


pub fn empty_hash() -> String {
    hash_leaf(&0)
}

pub fn hash_leaf<T: ToString>(input: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&input.to_string().as_ref());
    let result = hasher.result_str();
    result
}

pub fn hash_node<T: ToString>(left: &T, right: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(left.to_string().as_ref());
    hasher.input_str(right.to_string().as_ref());
    let result = hasher.result_str();
    result
}
