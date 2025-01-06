/*

# BLS Signature Scheme Alogrithm using elliptic curve cryptography


# Step 1: Initialization
# Let M be the message to be signed
# Let participants' public keys be (PK_1, PK_2, ..., PK_n)
# Let participants' private keys be (sk_1, sk_2, ..., sk_n)

# Initialize empty list for individual signatures
signatures = []

# Step 2: Signing Phase
for i = 1 to n:
    # Each participant computes their own signature:
    # H(M) is the elliptic curve point corresponding to the hash of the message M
    H_M = HashToEllipticCurve(M)
    
    # Participant i computes their signature as:
    signature_i = sk_i * H_M  # Scalar multiplication of private key with H(M)
    
    # Store the individual signature
    signatures.append(signature_i)

# Step 3: Aggregating Signatures
# Aggregate the signatures by adding them together
aggregated_signature = Infinity  # This represents the identity element on the curve

for i = 1 to n:
    aggregated_signature = aggregated_signature + signatures[i]  # Elliptic curve point addition

# Step 4: Verification Phase
# Now, to verify the aggregated signature, the verifier needs the public keys and the aggregated signature
# Compute the hash of the message
H_M = HashToEllipticCurve(M)

# Verify that the aggregated signature is valid
# This is done by checking the equation: e(aggregated_signature, G) == e(H(M), PK_aggregated)
# Where e() is the pairing function, and PK_aggregated is the aggregated public key.

# Compute the aggregated public key from the participants' public keys
PK_aggregated = Infinity  # Identity element on the curve
for i = 1 to n:
    PK_aggregated = PK_aggregated + PK_i  # Public key aggregation (point addition)

# Perform the pairing check for verification
if e(aggregated_signature, G) == e(H_M, PK_aggregated):
    print("Signature is valid.")
else:
    print("Signature is invalid.")
 */


 use ark_ec::{AdditiveGroup, PrimeGroup, CurveGroup};
 use ark_ec::{pairing::Pairing, AffineRepr};
 use ark_ff::{PrimeField, Field};
 use ark_bls12_381::{G1Affine, G1Projective as G1, Fr as ScalarField, G2Projective as G2, Fq12 as Fq12};
 use ark_std::{Zero, UniformRand, ops::Mul, rand::Rng};
 use sha2::{Sha256, Digest};
 
 // Hash message to curve point ( G1)
 fn hash_to_curve(message: &[u8]) -> G1 {
     let mut hasher = Sha256::new();
     hasher.update(message);
     let result = hasher.finalize();
     let scalar = ScalarField::from_le_bytes_mod_order(&result);
     let base_point = G1::generator();
     base_point * scalar
 }
 
 // Generate a keypair ( G2 )
 fn keygen<R: Rng>(rng: &mut R) -> (ScalarField, G2) {
     // Generate private key (random scalar)
     let private_key = ScalarField::rand(rng);
     // Generate public key (generator point * private key)
     let public_key = G2::generator() * private_key;
     
     (private_key, public_key)
 }
 
 // Sign a message
 fn sign(private_key: &ScalarField, message: &[u8]) -> G1 {
     // Hash message to curve point and multiply by private key
     let h = hash_to_curve(message);
     h * private_key
 }
 
// Verify a signature
fn verify(public_key: &G2, message: &[u8], signature: &G1) -> bool {
    let h = hash_to_curve(message);
    
    // Convert points to affine for pairing
    let sig_affine = signature.into_affine();
    let pk_affine = public_key.into_affine();
    let g1_affine = G1::generator().into_affine();
    
    // Compute pairings
    let pairing1 = ark_bls12_381::Bls12_381::pairing(sig_affine, G2::generator().into_affine());
    let pairing2 = ark_bls12_381::Bls12_381::pairing(h.into_affine(), pk_affine);
    
    pairing1 == pairing2
}
 
 fn main() {
     let mut rng = ark_std::test_rng();
     
     // Generate keypair
     let (private_key, public_key) = keygen(&mut rng);
     println!("Private key: {:?}", private_key);
     println!("Public key: {:?}", public_key);
     
     // Message to sign
     let message = b"Hello, BLS Signatures!";
     
     // Sign the message
     let signature = sign(&private_key, message);
     println!("Signature: {:?}", signature);
     
     // Verify the signature
     let is_valid = verify(&public_key, message, &signature);
     println!("Signature valid? {}", is_valid);
     
     // Try verifying with wrong message
     let wrong_message = b"Wrong message!";
     let is_invalid = verify(&public_key, wrong_message, &signature);
     println!("Wrong message signature valid? {}", is_invalid);
 }