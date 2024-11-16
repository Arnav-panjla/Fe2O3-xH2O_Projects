use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use base64::{engine::general_purpose, Engine};
use std::io::{self, Write};

fn encrypt_message(public_key: &RsaPublicKey, message: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = OsRng;
    let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())?;
    Ok(general_purpose::STANDARD.encode(&encrypted_data))
}

fn decrypt_message(private_key: &RsaPrivateKey, encrypted_base64: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_base64)?;
    let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, &encrypted_bytes)?;
    Ok(String::from_utf8(decrypted_data)?)
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RSA Encryption/Decryption Program");
    println!("=================================\n");

    // Generate RSA keys
    println!("Generating RSA keys...");
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let public_key = RsaPublicKey::from(&private_key);
    println!("✓ RSA keys generated successfully\n");

    // Get and encrypt the message
    let message = get_user_input("Enter the message to encrypt: ")?;
    println!("\nOriginal Message: {}", message);
    
    let encrypted_base64 = encrypt_message(&public_key, &message)?;
    println!("\nEncrypted Message (Base64):\n{}", encrypted_base64);

    // Decrypt the message
    let decrypted_message = decrypt_message(&private_key, &encrypted_base64)?;
    println!("\nDecrypted Message: {}", decrypted_message);

    Ok(())
}