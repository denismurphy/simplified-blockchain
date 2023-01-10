use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Sha512, Digest};

struct Snark {
    statement: String,
    public_params: PublicParams,
    private_witness: PrivateWitness,
}

impl Snark {
    fn new(statement: String, public_params: PublicParams, private_witness: PrivateWitness) -> Self {
        Self {
            statement,
            public_params,
            private_witness,
        }
    }

    fn generate_proof(&self) -> Proof {
        let commitment = self.private_witness.calculate_commitment(self.public_params.commitment_randomness);
        let response = self.private_witness.calculate_response(self.public_params.challenge);
        Proof { commitment, response }
    }

    fn verify_proof(&self, proof: &Proof) -> bool {
        let commitment_correct = proof.commitment == self.private_witness.calculate_commitment(self.public_params.commitment_randomness);
        let response_correct = proof.response == self.private_witness.calculate_response(self.public_params.challenge);
        commitment_correct && response_correct
    }
}

struct PublicParams {
    commitment_randomness: Vec<u8>,
    challenge: Vec<u8>,
}

struct PrivateWitness {
    secret: Vec<u8>,
}

impl PrivateWitness {
    fn calculate_commitment(&self, commitment_randomness: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.secret);
        hasher.update(&commitment_randomness);
        hasher.finalize().to_vec()
    }

    fn calculate_response(&self, challenge: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.secret);
        hasher.update(&challenge);
        hasher.finalize().to_vec()
    }
}

struct Proof {
    commitment: Vec<u8>,
    response: Vec<u8>,
}