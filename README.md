# Rust Wallet Key Manager for Eclipse 

This Rust project provides a simple tool for managing private and public keys in Base58 format, commonly used for wallets like Backpack wallet. It decodes the keys, validates their size, and serializes them into a JSON file for storage.

## Features

- **Base58 Decoding**: Converts Base58-encoded private and public keys into byte arrays.
- **Key Validation**: Ensures that both private and public keys are 32 bytes long, trimming the private key if necessary.
- **Keypair Serialization**: Concatenates the private and public keys into a single keypair and serializes it into a JSON file (`wallet.json`).
  
## Prerequisites

To run this code, you need to have the following installed:

- [Rust](https://www.rust-lang.org/) (Programming Language)
- [bs58](https://docs.rs/bs58/) (Base58 encoding/decoding library for Rust)
- [serde_json](https://docs.rs/serde_json/) (Serialization library for JSON in Rust)
  
## Project Structure

The project is organized as follows:
rust-wallet-key-manager/ │ └── wallet_key_json/ ├── Cargo.toml # Rust manifest file └── src/ └── main.rs # Main Rust code


## Installation
    
1. Install dependencies by adding the necessary crates to your `Cargo.toml` file:

    ```toml
    [dependencies]
    bs58 = "0.4"
    serde_json = "1.0"
    ```

2. Compile the project:

    ```bash
    cargo build
    ```

## Usage

1. Open the `src/main.rs` file and replace the placeholder values `your_base58_private_key_here` and `your_base58_public_key_here` with your actual Base58-encoded private and public keys.

2. Run the project:

    ```bash
    cargo run
    ```

3. Once the program runs successfully, it will generate a file called `wallet.json` in the project directory containing the serialized keypair.

## Feedback & Issues

If you encounter any bugs, have suggestions, or want to discuss this project, feel free to comment here https://github.com/Skillz23/rust-wallet-key-manager-for-Eclipse/discussions/1

## Example

For example, if your private and public keys are as follows:

- Private Key: `8p3fsFU7GnRTU5L26WQiqM7RAEs8nXyZ2L9Qqc8vCRT8XctwUqe`
- Public Key: `B2QmI8eA5WNSnH5F7MZRVNYvExg5AQkrtk`

The resulting `wallet.json` file will look something like this:

```json
[67, 98, 65, 213, 135, 20, ... ]

