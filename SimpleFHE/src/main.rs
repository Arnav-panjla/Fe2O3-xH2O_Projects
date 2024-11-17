use rand::{Rng, thread_rng};

#[derive(Clone)] // Add Clone derive
struct LWEParams {
    n: usize,        // dimension of the secret key
    q: i64,          // modulus
    noise_bound: i64 // bound for noise
}

struct KeyPair {
    secret_key: Vec<i64>,  // s in {0,1}^n
    public_params: LWEParams
}

struct Ciphertext {
    a: Vec<i64>,  // random vector
    b: i64        // dot product + message + noise
}

// Rest of the implementations remain the same...

fn main() {
    // Set parameters
    let params = LWEParams {
        n: 100,           // dimension
        q: 1 << 16,       // modulus (2^16)
        noise_bound: 4    // small noise bound
    };
    
    let key_pair = KeyPair::new(params.clone()); // Clone params here
    
    // Test encryption and decryption
    println!("Testing single encryption/decryption:");
    for m in 0..5 {
        let ct = key_pair.encrypt(m);
        let decrypted = key_pair.decrypt(&ct);
        println!("Original: {}, Decrypted: {}", m, decrypted);
    }
    
    // Test homomorphic addition
    println!("\nTesting homomorphic addition:");
    let m1 = 3;
    let m2 = 2;
    
    let ct1 = key_pair.encrypt(m1);
    let ct2 = key_pair.encrypt(m2);
    
    let ct_sum = add_ciphertexts(&ct1, &ct2, params.q); // Now params is still available
    let sum_decrypted = key_pair.decrypt(&ct_sum);
    
    println!("{} + {} = {}", m1, m2, sum_decrypted);
    
    // Test range of values
    println!("\nTesting value range:");
    for i in 0..3 {
        for j in 0..3 {
            let ct_i = key_pair.encrypt(i);
            let ct_j = key_pair.encrypt(j);
            let ct_sum = add_ciphertexts(&ct_i, &ct_j, params.q);
            let sum = key_pair.decrypt(&ct_sum);
            println!("{} + {} = {}", i, j, sum);
        }
    }
}