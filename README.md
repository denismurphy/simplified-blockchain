![Simplified Blockchain](https://github.com/denismurphy/simplified-blockchain/blob/main/images/blockchain_vector_images.svg?raw=true)

# 🔗 Simplified Blockchain: A Playful Approach 🎮

This repository contains simplified implementations of a Blockchain, Merkle Tree, Zk-SNARK, and Zk-Rollup.

## 🚀 Getting Started

Simplified implementations use `Vec<u8>` for all data and variables to keep things straightforward. Here's what we've got:

### 🔐 Simplified zk-SNARK

Our `zksnark` module includes:
- `ProvingKey` struct
- `VerificationKey` struct
- `create_proof()` function
- `verify_proof()` function

### 🌳 Simplified Merkle Tree

The `merkle_tree` module provides:
- `MerkleTree` struct
- Root calculation
- Leaf inclusion verification

### 🔄 Simplified Zk-RollupTransaction

The `ZkRollupTransaction` struct includes:
- `inputs`, `outputs`, `public_inputs`, `snark_proof`, and `snark_input` fields
- `new()` function for generation
- `verify()` function for verification

### ⛓️ Simplified Blockchain

Our `Blockchain` struct offers:
- Storage of `ZkRollupTransaction`s in a HashMap
- `new()` function for creating a new blockchain instance
- `add_transaction` method for adding new transactions

## 👨‍💻 Authors

- **Denis Murphy**

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🙏 Acknowledgments

- 🎩 This repository is inspired by the work of many great researchers and developers in the blockchain and cryptography communities.
- 🚨 This is a simplified version of the complex technologies and must be used for educational purposes only.
