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

/// calculates only the nth fibonacci number, and returns with calculation time
pub fn single(n: usize) -> (BigUint, Analytics) {
    let start_time = Instant::now();
    let (mut a, mut b) = (BigUint::zero(), BigUint::one());
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

/// calculates the fibonacci sequence 0..n into a vector, and returns with calculation time
pub fn multiple(end: usize) -> (Vec<BigUint>, Analytics) {
    let start_time = Instant::now();
    let (mut a, mut b) = (BigUint::zero(), BigUint::one());
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();
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
