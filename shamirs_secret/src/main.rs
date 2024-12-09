use rand::Rng;

// Generate shares using Shamir's Secret Sharing
fn generate_shares(secret: u32, n: usize, k: usize) -> Vec<(usize, u32)> {
    let mut rng = rand::thread_rng();
    let mut coefficients = vec![secret];
    for _ in 1..k {
        coefficients.push(rng.gen_range(0..10000));
    }

    (1..=n)
        .map(|i| {
            let mut share = 0;
            for (j, coeff) in coefficients.iter().enumerate() {
                share += coeff * (i as u32).pow(j as u32);
            }
            (i, share)
        })
        .collect()
}

// Reconstruct the secret using Lagrange interpolation
fn reconstruct_secret(shares: &[(usize, u32)], k: usize) -> u32 {
    let mut secret = 0;

    for i in 0..k {
        let (xi, yi) = shares[i];
        let mut numerator = 1;
        let mut denominator = 1;

        for j in 0..k {
            if i != j {
                let (xj, _) = shares[j];
                numerator *= xj as u32;
                denominator *= xj as u32 - xi as u32;
            }
        }

        secret += yi * numerator / denominator;
    }

    secret
}

fn main() {
    let secret = 1234; // The secret to be shared
    let n = 5;         // Number of shares
    let k = 3;         // Threshold for reconstruction

    // Generate shares
    let shares = generate_shares(secret, n, k);
    println!("Generated Shares: {:?}", shares);

    // Select k shares to reconstruct the secret
    let selected_shares = &shares[0..k];
    let reconstructed_secret = reconstruct_secret(selected_shares, k);
    println!("Reconstructed Secret: {}", reconstructed_secret);

    assert_eq!(secret, reconstructed_secret);
}