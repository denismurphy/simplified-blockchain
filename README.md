![Simplified Blockchain](https://github.com/denismurphy/simplified-blockchain/blob/main/images/blockchain_vector_images.svg?raw=true)

# 🔗 Simplified Blockchain: A Playful Approach 🎮

Welcome to the exciting world of blockchain technology! 🚀 This repository contains simplified implementations of a Blockchain, Merkle Tree, Zk-SNARK, and Zk-Rollup. Our goal is to make these complex concepts accessible and fun to learn! 🧠💡

## 🚀 Getting Started

Let's dive into the blockchain universe! Here's how to get your own copy up and running.

### 🛠️ Prerequisites

This project is built with Rust, so you'll need to have Rust installed on your machine. Don't have it yet? No worries! 

🦀 [Download and install Rust here](https://www.rust-lang.org/learn/get-started)

## 🎯 Usage

Our simplified implementations use `Vec<u8>` for all data and variables to keep things straightforward. Here's what we've got:

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

## 🤝 Contributing

While this is primarily a learning tool, we welcome bug reports and improvement suggestions! Feel free to open an issue or pull request.

## 👨‍💻 Authors

- **Denis Murphy**

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🙏 Acknowledgments

- 🎩 This repository is inspired by the work of many great researchers and developers in the blockchain and cryptography communities.
- 🚨 This is a simplified version of the complex technologies and must be used for educational purposes only.
