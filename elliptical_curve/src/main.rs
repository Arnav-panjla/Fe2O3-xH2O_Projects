//Date: 2021-09-26

//This code demonstrates how to generate a public key from a secret key, sign a message using the secret key, and verify the signature using the public key.
//The code uses the secp256k1 crate for elliptic curve cryptography and the sha2 crate for computing SHA-256 hashes.

/* 
To run this code, add the following dependencies to your Cargo.toml file:
[dependencies]
secp256k1 = "0.30.0"
sha2 = "0.10.6"
hex = "0.4"
*/


use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use sha2::{Sha256, Digest};

// Computes a SHA-256 hash of the input data.
fn compute_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

fn main() {
    let secp = Secp256k1::new();

    let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    println!("Public key is : {:?}", public_key);

    let hash_msg = compute_hash(b"I am batman");
    println!("The message SHA256 hash is : {:?}", hash_msg.iter().map(|b| format!("{:02x}", b)).collect::<String>());
    let message = Message::from_digest(hash_msg);
    
    
    let sig = secp.sign_ecdsa(&message, &secret_key);
    println!("Signature is : {:?}", sig);

    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
    println!("Signature is valid!");
}