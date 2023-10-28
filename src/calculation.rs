use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

/// calculates only the nth fibonacci number
pub fn calculate_fib_sing(n: usize) -> BigUint {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    for _ in 0..n {
        let next = a + &b;
        a = replace(&mut b, next);
    }

    a
}

/// calculates the fibonacci sequence 0..n into a vector
pub fn calculate_fib_mult(end: usize) -> Vec<BigUint> {
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    for _ in 0..=end {
        calc_fib_vector.push(a.clone());
        let next = a + &b;
        a = replace(&mut b, next);
    }

    calc_fib_vector
}
