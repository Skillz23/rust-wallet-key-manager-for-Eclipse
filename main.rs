use std::fs::File;
use std::io::Write;
use bs58;

fn main() {
    // Replace these with your actual private key and public key in Base58 format
    let private_key_str = "your_base58_private_key_here";
    let public_key_str = "your_base58_public_key_here";

    // Decode the keys from Base58 format to byte arrays
    let mut private_key_bytes = bs58::decode(private_key_str).into_vec().expect("Invalid private key format");
    let public_key_bytes = bs58::decode(public_key_str).into_vec().expect("Invalid public key format");

    // If private key is 64 bytes, take only the first 32 bytes
    if private_key_bytes.len() == 64 {
        private_key_bytes = private_key_bytes[0..32].to_vec();
    }

    // Ensure both keys are 32 bytes
    assert_eq!(private_key_bytes.len(), 32, "Private key must be 32 bytes");
    assert_eq!(public_key_bytes.len(), 32, "Public key must be 32 bytes");

    // Concatenate private and public keys into a single array
    let mut keypair = Vec::new();
    keypair.extend_from_slice(&private_key_bytes);
    keypair.extend_from_slice(&public_key_bytes);

    // Serialize the keypair as a JSON array
    let json_content = serde_json::to_string(&keypair).unwrap();

    // Write the JSON content to a file
    let mut file = File::create("wallet.json").unwrap();
    file.write_all(json_content.as_bytes()).unwrap();

    println!("wallet.json file created with the keys in the correct format.");
}
