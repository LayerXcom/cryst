use node::Node;
use sha2::Sha256;


#[derive(Debug, Default)]
pub struct MerkleTree<T> {
    root: Node<T>,
    height: usize,
    nodes_amount: usize,
    // pub algorithm: &'static Algorithm,
}


impl<T> MerkleTree<T> {
    
    pub fn create_tree(data: Vec<T>) -> Self        
    {
        if data.is_empty() {
            MerkleTree::default();
        }

        let count = data.len();
        let mut height = 0;
        let mut cur = Vec::with_capacity(count);

        for d in data {    // can be closure?
            let leaf = Node::new(d);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.removve(0);
                    let right = cur.remove(0);

                    let combined_hash = Sha256::new()
                        .chain(left)
                        .chain(right)
                        .result();

                    let node = Node {
                        hash: combined_hash.as_ref().into(),
                        left: Some(Box::new(left)),
                        right: Some(Box::new(right)),
                    };

                    next.push(node);
                }
            }

            heigth += 1;
            cur = next;
        }

        debug_assert!(cur.len() == 1);
        let root = cur.remove(0);

        MerkleTree {
            root,
            heigth,
            count,
        }
    }

    // pub fn get_proof() {

    // }

    // pub fn get_root(&self) ->  {

    // }    
}