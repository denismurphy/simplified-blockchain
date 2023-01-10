use sha2::{Sha256, Digest};
use zksnark::{ProvingKey, VerificationKey};
use merkle_tree::MerkleTree;
use crate::merkle_tree::MerkleTree;

use sha2::{Sha256, Digest};
use zksnark::{ProvingKey, VerificationKey};
use merkle_tree::MerkleTree;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkRollupTransaction {
    pub inputs: Vec<Vec<u8>>,
    pub outputs: Vec<Vec<u8>>,
    pub public_inputs: Vec<u8>,
    pub snark_proof: Vec<u8>,
    pub snark_input: Vec<u8>,
}

impl ZkRollupTransaction {
    pub fn new(inputs: Vec<Vec<u8>>, outputs: Vec<Vec<u8>>, proving_key: &ProvingKey, verification_key: &VerificationKey) -> ZkRollupTransaction {
        let inputs_mt = MerkleTree::new(inputs);
        let inputs_root = inputs_mt.root;
        let outputs_mt = MerkleTree::new(outputs);
        let outputs_root = outputs_mt.root;
        let public_inputs = vec![inputs_root, outputs_root].concat();
        let snark_input = public_inputs.as_bytes();
        let snark_proof = create_proof(snark_input, proving_key);
        let public_inputs = vec![inputs_root, outputs_root, snark_proof].concat();

        ZkRollupTransaction {
            inputs: inputs,
            outputs: outputs,
            public_inputs: public_inputs,
            snark_proof: snark_proof,
            snark_input: snark_input,
        }
    }

    pub fn verify(&self, verification_key: &VerificationKey) -> bool {
        verify_proof(&self.snark_proof, &self.snark_input, verification_key)
    }
}
