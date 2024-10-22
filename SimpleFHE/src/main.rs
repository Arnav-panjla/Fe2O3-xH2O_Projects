fn main() {
    // Simple RSA parameters
    let p: i64 = 61; // A prime number
    let q: i64 = 53; // Another prime number
    let n: i64 = p * q; // Modulus for public key
    let e: i64 = 17; // Public exponent
    let phi: i64 = (p - 1) * (q - 1);
    let d: i64 = mod_inverse(e, phi).unwrap(); // Private exponent

    // Example plaintext values
    let plaintext1: i64 = 10;
    let plaintext2: i64 = 15;

    // Encrypt the values
    let encrypted1 = encrypt(plaintext1, e, n);
    let encrypted2 = encrypt(plaintext2, e, n);

    // Perform homomorphic addition
    let encrypted_sum = (encrypted1 * encrypted2) % n;

    // Decrypt the result
    let decrypted_sum = decrypt(encrypted_sum, d, n);

    // Print the results
    println!("Decrypted Sum: {}", decrypted_sum);
}

// Encrypt a plaintext using public key
fn encrypt(plaintext: i64, e: i64, n: i64) -> i64 {
    mod_exp(plaintext, e, n)
}

// Decrypt a ciphertext using private key
fn decrypt(ciphertext: i64, d: i64, n: i64) -> i64 {
    mod_exp(ciphertext, d, n)
}

// Modular exponentiation
fn mod_exp(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut b = base % modulus;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 { // If e is odd
            result = (result * b) % modulus;
        }
        b = (b * b) % modulus; // Square the base
        e /= 2; // Divide the exponent by 2
    }
    result
}

// Compute the modular inverse using the Extended Euclidean Algorithm
fn mod_inverse(a: i64, m: i64) -> Option<i64> {
    let mut m0 = m;
    let mut y: i64 = 0;
    let mut x: i64 = 1;

    if m == 1 {
        return Some(0);
    }

    let mut a = a % m;
    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;

        let t = y;
        y = x - q * y;
        x = t;
    }

    if x < 0 {
        x += m;
    }

    Some(x)
}