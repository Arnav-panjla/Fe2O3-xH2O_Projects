use rand::Rng;

fn polynomial(inputs: &[i64]) -> i64 {
    let mut result = 0;
    let coefficient = [2,4,7,1,5,8];
    // For each input variable, assign a random coefficient and add the term to the result
    for (i, &x) in inputs.iter().enumerate() {
        // Generate a random coefficient between -10.0 and 10.0
        result += coefficient[i] * x; // For example, using x^2, but you could adjust for other terms.
    }

    result
}

fn sum_over_binary_inputs(num_vars: u16, polynomial: fn(&[i64]) -> i64) -> i64 {
    let mut sum = 0;
    // Iterate over all binary combinations of inputs
    for combination in 0..(1 << num_vars) {
        let mut inputs = Vec::new();
        for i in 0..num_vars {
            let bit = (combination >> i) & 1;
            inputs.push(bit as i64);
        }
        sum += polynomial(&inputs);
    }
    sum
}

fn prover(num_vars: u16, polynomial: fn(&[i64]) -> i64) -> i64 {
    // Prover calculates the sum over all binary inputs
    sum_over_binary_inputs(num_vars, polynomial)
}

fn verifier(num_vars: u16, expected_sum: i64, polynomial: fn(&[i64]) -> i64) -> bool {
    // Verifier checks if the sum calculated by the prover matches the expected sum
    
    let calculated_sum = sum_over_binary_inputs(num_vars, polynomial);
    calculated_sum == expected_sum // Allow small floating-point differences
}

fn main() {
    let num_vars = 6; // Number of variables in the polynomial
    
    // Prover computes the sum
    let sum = prover(num_vars, polynomial);
    println!("Prover computed sum: {}", sum);
    
    // Verifier checks the sum
    let is_valid = verifier(num_vars, sum, polynomial);
    
    if is_valid {
        println!("Verification successful!");
    } else {
        println!("Verification failed!");
    }
}
