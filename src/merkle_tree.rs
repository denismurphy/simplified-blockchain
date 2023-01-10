use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub struct MerkleTree {
    pub root: Vec<Vec<u8>>,
    pub nodes: HashMap<String, Vec<u8>>,
}

impl MerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> MerkleTree {
        let mut nodes = HashMap::new();
        let mut new_data = Vec::new();
        for item in data {
            let mut hasher = Sha256::new();
            hasher.input(&item);
            let hash = hasher.result();
            let hash_str = hex::encode(hash);
            nodes.insert(hash_str.clone(), item);
            new_data.push(hash_str);
        }
        let mut root = "".to_string();
        if new_data.len() % 2 == 1 {
            new_data.push(new_data[new_data.len()-1].clone());
        }
        while new_data.len() > 1 {
            let mut level = Vec::new();
            for mut i in 0..new_data.len() {
                let mut hasher = Sha256::new();
                let left_node = new_data[i].as_bytes();
                let right_node = new_data[i+1].as_bytes();
                hasher.input(left_node);
                hasher.input(right_node);
                let hash = hasher.result();
                let hash_str = hex::encode(hash);
                level.push(hash_str);
                nodes.insert(hash_str.clone(), vec![left_node.to_vec(), right_node.to_vec()].concat());
                i+=2;
            }
            if level.len() % 2 == 1 {
                level.push(level[level.len()-1].clone());
            }
            new_data = level;
        }
        if !new_data.is_empty() {
            root = new_data[0].clone();
        }
        MerkleTree {
            root: root,
            nodes: nodes,
        }
    }
}
