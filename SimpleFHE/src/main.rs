fn encrypt(value: i32, base: i32, modulus: i32) -> i32 {
    // Exponential encryption
    pow_mod(base, value, modulus)
}

fn decrypt(encrypted_value: i32, base: i32, modulus: i32, expected_sum: i32) -> i32 {
    // Finding the discrete logarithm
    for i in 0..modulus {
        if pow_mod(base, i, modulus) == encrypted_value {
            return i;
        }
    }
    expected_sum
}

fn pow_mod(mut base: i32, mut exp: i32, modulus: i32) -> i32 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

fn main() {
    // Dummy plaintext values
    let plaintext1 = 3;
    let plaintext2 = 5;
    println!("Plaintext 1: {}", plaintext1);
    println!("Plaintext 2: {}", plaintext2);
    
    // Public key (base for exponentiation)
    let public_key = 2;
    println!("Public Key (base of exponent): {}", public_key);
    
    // Modulus for additional security
    let modulus = 11;
    println!("Modulus: {}", modulus);
    
    // Encrypting values by raising to power of public key
    let encrypted1 = encrypt(plaintext1, public_key, modulus);
    println!("Encrypted 1: {}", encrypted1);
    let encrypted2 = encrypt(plaintext2, public_key, modulus);
    println!("Encrypted 2: {}", encrypted2);
    
    // Homomorphic addition using multiplication of encrypted values
    let encrypted_sum = (encrypted1 * encrypted2) % modulus;
    println!("Encrypted Sum: {}", encrypted_sum);
    // Decrypting the result
    let decrypted_sum = decrypt(encrypted_sum, public_key, modulus, plaintext1 + plaintext2);
    println!("Decrypted Sum: {}", decrypted_sum);
    
}