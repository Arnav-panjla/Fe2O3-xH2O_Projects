use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use base64::{encode, decode};
use std::io::{self, Write};

fn main() {
    // Step 1: Generate RSA Keys
    println!("Generating RSA keys...");
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    println!("RSA keys generated successfully.\n");

    // Step 2: Get User Input
    println!("Enter the message to encrypt:");
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Flush output to ensure user sees the prompt
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let message = input.trim(); // Remove trailing newline or spaces
    println!("Original Message: {}", message);

    // Step 3: Encrypt the Message
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key
        .encrypt(&mut rng, padding, message.as_bytes())
        .expect("Failed to encrypt message");
    let encrypted_base64 = encode(&encrypted_data);
    println!("Encrypted Message (Base64): {}", encrypted_base64);

    // Step 4: Decrypt the Message
    let decrypted_data = private_key
        .decrypt(padding, &encrypted_data)
        .expect("Failed to decrypt message");
    let decrypted_message = String::from_utf8(decrypted_data).expect("Failed to parse decrypted data as UTF-8");
    println!("Decrypted Message: {}", decrypted_message);
}
