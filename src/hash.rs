use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Enum to declare and define datanodes
#[derive(Debug)]
pub enum DataNode {
    A,
    B,
    C,
}

//Function to hash the key and return the data node
//TODO: for now we are using DefaultHasher, but soon we will need dynamic management and therefore will use MomentoHash?
pub fn hash_key_to_node(key: &str) -> DataNode {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_val = hasher.finish();

    //Simple consistent Hashing : modulo the number of nodes
    //TODO: find how to find the length of enum variants
    match hash_val % 3 {
        0 => DataNode::A,
        1 => DataNode::B,
        _ => DataNode::C,
    }
}
