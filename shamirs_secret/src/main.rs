/*
secret = 1234
n = 6
k = 3
a1 = 166 // Polynomial coefficient
a2 = 94 // Polynomial coefficient
f(x) = 1234 + 166X + 94X**2 // Polynomial
modulo = 1613

*/

use lambdaworks_math::{
    field::element::FieldElement,
    polynomial::Polynomial,
};

fn main () {

    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField; 

    const ORDER: u64 = 1613; // Defining modulo
    type F = U64PrimeField<ORDER>;
    type FE = FieldElement<F>;

    let secret = FE::new(1234);
    let threshold = 3;
    let secret_p = 1234;

    // Polynomial:

    fn polynomial_f() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(1234), FE::new(166), FE::new(94)])
    }

    // Defining shares:

    fn defining_shares (a: FE) -> FE {
        let result = polynomial_f().evaluate(&a);
        return result;
    }

    // Reconstructing the secret using a subset of shares:

    let subset_shares: Vec<(FE, FE)> = vec! [
        (FE::new(1), defining_shares(FE::new(1))),
        (FE::new(2), defining_shares(FE::new(2))),
        (FE::new(3), defining_shares(FE::new(3)))];    

    let mut reconstructed_secret = FE::new(0);

    for i in 0..threshold {
        
        let (x_i, y_i) = subset_shares[i];

        let mut numerator: FE = FE::new(1);
        let mut denominator: FE = FE::new(1);
        for j in 0..threshold {
            if i != j {
                let (x_j, _) = subset_shares[j];
                numerator = numerator * (- x_j);
                denominator = denominator * (x_i - x_j);
            }
        }
        
        let pow_result = denominator.pow(1611_u64);
        reconstructed_secret = reconstructed_secret + (y_i * numerator * pow_result);        
        
    }

    if reconstructed_secret == secret {
        println!("Verified!");
        println!("The secret is: {}", secret_p);
    }

}