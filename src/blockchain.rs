use std::collections::HashMap;
use sha2::{Sha256, Digest};
use zksnark::{ProvingKey, VerificationKey};
use merkle_tree::MerkleTree;
use crate::zk_rollup_transaction::ZkRollupTransaction;

pub struct Blockchain {
    pub chain: HashMap<String, ZkRollupTransaction>,
    pub proving_key: ProvingKey,
    pub verification_key: VerificationKey,
}

impl Blockchain {
    pub fn new(proving_key: ProvingKey, verification_key: VerificationKey) -> Blockchain {
        Blockchain {
            chain: HashMap::new(),
            proving_key: proving_key,
            verification_key: verification_key,
        }
    }

    pub fn add_transaction(&mut self, inputs: Vec<Vec<u8>>, outputs: Vec<Vec<u8>>) {
        let transaction = ZkRollupTransaction::new(inputs, outputs, &self.proving_key, &self.verification_key);
        let mut hasher = Sha256::new();
        hasher.input(&transaction.snark_input);
        let hash = hasher.result();
        let hash_str = hex::encode(hash);
        self.chain.insert(hash_str, transaction);
    }
}
