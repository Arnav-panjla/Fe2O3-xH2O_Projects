// Import necessary modules from arkworks
use ark_ff::Field; // For working with finite fields
use ark_relations::{
    lc, // For linear combinations
    r1cs::{ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError}, // For R1CS-based constraint systems
};
use ark_snark::CircuitSpecificSetupSNARK; // For setting up SNARKs
use ark_std::rand::{Rng, RngCore, SeedableRng}; // Random number generation
use ark_std::test_rng; // A standard test RNG

// Struct representing the cubic equation circuit
// The circuit will prove knowledge of an 'x' such that x^3 + x + 5 = 35 (or a public value)
struct CubicDemoCircuit<F: Field> {
    pub x: Option<F>, // The secret value 'x' (Option because it's not assigned during setup)
}

// Implement the `ConstraintSynthesizer` trait for the CubicDemoCircuit to define constraints
impl<F: Field> ConstraintSynthesizer<F> for CubicDemoCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate witness variable 'x' for the secret value
        let x_val = self.x;
        let x = cs.new_witness_variable(|| x_val.ok_or(SynthesisError::AssignmentMissing))?;

        // Step 1: Create 'tmp_1' to represent x^2 (x * x)
        let tmp_1_val = x_val.map(|e| e.square()); // tmp_1 = x^2
        let tmp_1 = cs.new_witness_variable(|| tmp_1_val.ok_or(SynthesisError::AssignmentMissing))?;
        // Enforce the constraint: x * x = tmp_1
        cs.enforce_constraint(lc!() + x, lc!() + x, lc!() + tmp_1)?;

        // Step 2: Create 'x_cubed' to represent x^3 (tmp_1 * x)
        let x_cubed_val = tmp_1_val.map(|mut e| {
            e.mul_assign(&x_val.unwrap()); // x_cubed = tmp_1 * x
            e
        });
        let x_cubed = cs.new_witness_variable(|| x_cubed_val.ok_or(SynthesisError::AssignmentMissing))?;
        // Enforce the constraint: tmp_1 * x = x_cubed
        cs.enforce_constraint(lc!() + tmp_1, lc!() + x, lc!() + x_cubed)?;

        // Step 3: Allocate 'out' as the public output variable for x^3 + x + 5
        let out = cs.new_input_variable(|| {
            let mut tmp = x_cubed_val.unwrap();
            tmp.add_assign(&x_val.unwrap()); // tmp = x_cubed + x
            tmp.add_assign(F::from(5u32)); // tmp = x_cubed + x + 5
            Ok(tmp) // This will be the public output
        })?;
        
        // Enforce the final constraint: x^3 + x + 5 = out
        cs.enforce_constraint(
            lc!() + x_cubed + x + (F::from(5u32), ConstraintSystem::<F>::one()), // x^3 + x + 5
            lc!() + ConstraintSystem::<F>::one(), // Identity term for the left-hand side
            lc!() + out, // Right-hand side is the output variable
        )?;

        Ok(())
    }
}

fn main() {
    use ark_bls12_381::{Bls12_381, Fr as BlsFr}; // Using the BLS12-381 curve and field for cryptography
    use ark_groth16::Groth16; // The Groth16 zk-SNARK construction
    use ark_snark::SNARK; // Generic SNARK traits

    // Set up random number generator
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

    // Generate the setup parameters (proving and verification keys) for the circuit
    let (pk, vk) = Groth16::<Bls12_381>::setup(
        CubicDemoCircuit::<BlsFr> { x: None }, // No secret input during setup
        &mut rng, // Using the RNG
    )
    .unwrap();

    // Step 1: Prove that we know an 'x' such that x^3 + x + 5 = 35 for x = 3
    let proof1 = Groth16::<Bls12_381>::prove(
        &pk, // Proving key
        CubicDemoCircuit::<BlsFr> {
            x: Some(BlsFr::from(3)), // Secret input x = 3
        },
        &mut rng, // RNG for randomness
    )
    .unwrap();

    // Verify proof1: Check that the public output is 35
    assert!(Groth16::<Bls12_381>::verify(&vk, &[BlsFr::from(35)], &proof1).unwrap());

    // Step 2: Prove that we know an 'x' such that x^3 + x + 5 = 73 for x = 4
    let proof2 = Groth16::<Bls12_381>::prove(
        &pk, // Proving key
        CubicDemoCircuit::<BlsFr> {
            x: Some(BlsFr::from(4)), // Secret input x = 4
        },
        &mut rng, // RNG for randomness
    )
    .unwrap();

    // Verify proof2: Check that the public output is 73
    assert!(Groth16::<Bls12_381>::verify(&vk, &[BlsFr::from(73)], &proof2).unwrap());

    // Invalid verification: The proof for x = 4 should not work for the public output 35
    assert!(!Groth16::<Bls12_381>::verify(&vk, &[BlsFr::from(35)], &proof2).unwrap());

    // Invalid verification: The proof for x = 3 should not work for the public output 73
    assert!(!Groth16::<Bls12_381>::verify(&vk, &[BlsFr::from(73)], &proof1).unwrap());
}
