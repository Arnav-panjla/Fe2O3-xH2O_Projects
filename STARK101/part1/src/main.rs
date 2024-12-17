use num_bigint::{BigInt, ToBigUint, BigUint};
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq, Eq)]
struct FiniteField {
    value: BigUint,
    modulo: BigUint,
}

impl fmt::Display for FiniteField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FiniteField {
    fn new(value: BigUint, modulo: BigUint) -> Self {
        let value = value % &modulo;
        Self { value, modulo }
    }

    fn add(&self, other: &FiniteField) -> Self {
        assert_eq!(self.modulo, other.modulo, "Moduli must be the same for addition");
        Self::new((&self.value + &other.value) % &self.modulo, self.modulo.clone())
    }

    fn mul(&self, other: &FiniteField) -> Self {
        assert_eq!(self.modulo, other.modulo, "Moduli must be the same for multiplication");
        Self::new((&self.value * &other.value) % &self.modulo, self.modulo.clone())
    }

    fn sub(&self, other: &FiniteField) -> Self {
        assert_eq!(self.modulo, other.modulo, "Moduli must be the same for subtraction");
        let value = if self.value >= other.value {
            (&self.value - &other.value) % &self.modulo
        } else {
            (self.modulo.clone() + &self.value - &other.value) % &self.modulo
        };
        Self::new(value, self.modulo.clone())
    }

    fn div(&self, other: &FiniteField) -> Self {
        assert_eq!(self.modulo, other.modulo, "Moduli must be the same for division");
        self.mul(&other.mod_inv())
    }

    fn mod_inv(&self) -> Self {
        // Extended Euclidean Algorithm for modular multiplicative inverse
        let mut a = self.value.clone();
        let mut b = self.modulo.clone();
        let mut x = BigUint::zero();
        let mut y = BigUint::one();
        let mut u = BigUint::one();
        let mut v = BigUint::zero();

        while a != BigUint::zero() {
            let q = &b / &a;
            let r = &b % &a;
            let m = x.clone() - &u * &q;
            let n = y.clone() - &v * &q;
            
            b = a;
            a = r;
            x = u;
            y = v;
            u = m;
            v = n;
        }

        // Ensure the inverse is non-negative
        let inv = if x >= self.modulo {
            x % &self.modulo
        } else if x < BigUint::zero() {
            (x + &self.modulo) % &self.modulo
        } else {
            x
        };

        Self::new(inv, self.modulo.clone())
    }

    fn pow(&self, exp: BigUint) -> Self {
        if exp == BigUint::zero() {
            return Self::new(BigUint::one(), self.modulo.clone());
        }

        let mut res = FiniteField::new(BigUint::one(), self.modulo.clone());
        let mut base = self.clone();
        let mut exp = exp;

        while exp > BigUint::zero() {
            if &exp % 2u32 == BigUint::one() {
                res = res.mul(&base);
            }
            base = base.mul(&base);
            exp = exp / 2u32;
        }
        res
    }
}

// Generate a Fibonacci sequence in a finite field
fn generate_fibonacci(n: usize, modulo: BigUint) -> Vec<FiniteField> {
    let mut fib = vec![
        FiniteField::new(BigUint::zero(), modulo.clone()),
        FiniteField::new(BigUint::one(), modulo.clone()),
    ];
    
    for i in 2..n {
        let next = fib[i - 1].add(&fib[i - 2]);
        fib.push(next);
    }
    fib
}

fn main() {
    let modulo = BigUint::from(17u32);
    let fib = generate_fibonacci(10, modulo.clone());
    
    println!("Generated Fibonacci sequence in modulo {}", modulo);
    for (i, num) in fib.iter().enumerate() {
        println!("{}: {}", i, num);
    }
}