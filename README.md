![Simplified Blockchain](https://github.com/denismurphy/simplified-blockchain/blob/main/images/blockchain_vector_images.svg?raw=true)

# Simplified Blockchain: A Playful Approach

This repository contains a simplified implementation of a Blockchain, Merkle Tree, Zk-SNARK and Zk-Rollup. The main focus of this repository is to provide a simplified and easy to understand codebase for understanding the underlying concepts and principles of these technologies.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

This repository uses Rust, so you'll need to have a Rust development environment set up on your machine in order to run the code. You can download and install Rust [here](https://www.rust-lang.org/learn/get-started).

## Usage

This repository contains the implementation of Simplified zk-SNARK, Simplified Merkle Tree, Simplified Zk-Rollup and Simplified Blockchain. All data passed and all variables used are of type `Vec<u8>` instead of other types.

### Simplified zk-SNARK

The `zksnark` module provides a simplified implementation of zk-SNARKs. It includes a `ProvingKey` struct, a `VerificationKey` struct, and a `create_proof()` and `verify_proof()` function to generate and verify zk-SNARK proofs.

### Simplified Merkle Tree

The `merkle_tree` module provides a simple implementation of a Merkle tree. It includes a `MerkleTree` struct that takes in a vector of data and provides the root of the Merkle tree as well as the ability to verify inclusion of a leaf in the tree.

### Simplified Zk-RollupTransaction

The `ZkRollupTransaction` struct is used to construct a zero knowledge proof for a Rollup transaction on the blockchain. It includes `inputs`, `outputs`, `public_inputs`, `snark_proof`, and `snark_input` fields, and a `new()` and `verify()` functions to generate and verify Zk-RollupTransaction.

### Simplified Blockchain

The `Blockchain` struct is a simple implementation of a blockchain. it stores a collection of `ZkRollupTransaction`s in a HashMap with the keys as the SHA-256 hash of the snark_input of each transaction. It has a new() function that creates a new instance of the blockchain struct, it takes proving_key and the verification_key as input. and it has add_transaction method which takes inputs and outputs as Vec<u8> and creates a new transaction using those inputs and outputs, then it calculates the SHA-256 hash of the snark_input and stores the transaction in the chain with the hash as the key.

## Contributing

This repository is intended as a learning tool and is not meant for production use. However, if you find any bugs or would like to improve the code, please feel free to open an issue or a pull request.

## Authors

-   **Denis Murphy**

## License

This project is licensed under the MIT License

## Acknowledgments

-   This repository is inspired by the work of many great researchers and developers in the blockchain and cryptography communities.
-   This is a simplified version of the complex technologies and must be used for educational purposes only.
