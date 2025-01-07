/*
    ECDSA_SIGN(message, private_key, curve):
    # 1. Hash the message
    H = HASH(message)

    # 2. Choose a random integer k in the range [1, n-1]
    k = RANDOM_INTEGER(1, n-1)

    # 3. Compute the elliptic curve point R = k * P
    R = k * P  # P is the base point of the elliptic curve
    r = x(R)  # r is the x-coordinate of point R

    # 4. If r == 0, return to step 2 and choose a new k
    if r == 0:
        return ECDSA_SIGN(message, private_key, curve)

    # 5. Compute s = k^-1 * (H + r * private_key) mod n
    s = (k^-1 * (H + r * private_key)) % n

    # 6. If s == 0, return to step 2 and choose a new k
    if s == 0:
        return ECDSA_SIGN(message, private_key, curve)

    # 7. Return the signature (r, s)
    return (r, s)
*/

use ark_ec::{AdditiveGroup, PrimeGroup, CurveGroup};
use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::{PrimeField, Field, BigInteger};
use ark_secp256k1::{Affine, Projective as G1, Fr as ScalarField};
use sha2::{Digest, Sha256};
use ark_std::{Zero, UniformRand, ops::Mul, rand::Rng};

fn hash(message: &[u8]) -> ScalarField {
    let mut hasher = Sha256::new();
    hasher.update(message);
    let result = hasher.finalize();
    let scalar = ScalarField::from_le_bytes_mod_order(&result);
    scalar
}

fn keygen<T: ark_std::rand::Rng>(rng: &mut T) -> (ScalarField, G1) {
    let private_key = ScalarField::rand(rng);
    let public_key = G1::generator() * private_key;
    (private_key, public_key)
}

fn sign(private_key: &ScalarField, message: &[u8]) -> (ScalarField, ScalarField) {

    let mut rng = ark_std::test_rng();

    let k = ScalarField::rand(&mut rng);
    let r = G1::generator() * k;

    let r_x_bigint = r.into_affine().x.into_bigint();
    let r_x = ScalarField::from_be_bytes_mod_order(&r_x_bigint.to_bytes_be());

    println!("r_x: {:?}", r_x);
    let h = hash(message);
    println!("h: {:?}", h);
    let s = k.inverse().unwrap() * (h + r_x * private_key);
    (r_x, s)
}

fn verify(public_key: &G1, message_hash: ScalarField, signature: (ScalarField, ScalarField)) -> bool {
    let (r, s) = signature;
    let w = s.inverse().unwrap();
    println!("w: {:?}", w);
    let u = message_hash * w;
    println!("u: {:?}", u);
    let v = r * w;
    println!("v: {:?}", v);
    let p = G1::generator() * u + *public_key * v;
    println!("G1*u + pk*v: {:?}", p.into_affine());
    let r_prime_bigint = p.into_affine().x.into_bigint();
    println!("r_prime_bigint: {:?}", r_prime_bigint);
    let r_prime = ScalarField::from_be_bytes_mod_order(&r_prime_bigint.to_bytes_be());
    r == r_prime
}

fn main() {

    println!("In this example we will sign a message using the ECDSA algorithm.");
    println!("Alice will sign a message and Bob will verify the signature.");

    
    // Sign a message
    let message = b"Hello, Bob!";
    println!("Message: {:?}", message);

    println!("-------------------------------------Alice is signing the message-----------------------------------");
    // Generate keypair
    let mut rng = ark_std::test_rng();
    let (private_key, public_key) = keygen(&mut rng);
    println!("Alice's public key: {:?}", public_key);
    let signature = sign(&private_key, message);

    println!("Signature: {:?}", signature);   

    println!("-------------------------------------Bob is verifying the signature-----------------------------------");

    let h = hash(message);
    let (r, s) = signature;
    let is_valid = verify(&public_key, h, (r, s));

    if is_valid {
        println!("The signature is valid!");
    } else {
        println!("The signature is invalid!");
    }

}