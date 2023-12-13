use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::{
    mem::replace,
    time::{Duration, Instant},
};

pub struct Analytics {
    pub calc_time: Duration,
    pub num_digits: usize,
}

/// calculates only the nth fibonacci number
pub fn calculate_fib_sing(n: usize) -> (BigUint, Analytics) {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    let start_time = Instant::now();
    for _ in 0..n {
        let next = a + &b;
        a = replace(&mut b, next);
    }
    let end_time = Instant::now();

    let analytics = Analytics {
        calc_time: end_time - start_time,
        num_digits: n,
    };

    (a, analytics)
}

/// calculates the fibonacci sequence 0..n into a vector
pub fn calculate_fib_mult(end: usize) -> (Vec<BigUint>, Analytics) {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();

    let start_time = Instant::now();
    for _ in 0..=end {
        calc_fib_vector.push(a.clone());
        let next = a + &b;
        a = replace(&mut b, next);
    }
    let end_time = Instant::now();

    let analytics = Analytics {
        calc_time: end_time - start_time,
        num_digits: end,
    };

    (calc_fib_vector, analytics)
}
