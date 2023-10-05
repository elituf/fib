use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::time::Instant;
use thousands::Separable;

mod args;

/// Calculates the fibonacci sequence starting at 0
fn calculate_fib(limit_nth: usize) -> Vec<BigUint> {
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();

    for _ in 0..=limit_nth {
        calc_fib_vector.push(a.clone());
        let next = a + b.clone();
        a = b;
        b = next;
    }

    calc_fib_vector
}

fn main() {
    let args: args::Args = argh::from_env();

    match (args.single, args.multiple) {
        (Some(single), None) => {
            let was = Instant::now();
            let fib_vector: Vec<BigUint> = calculate_fib(single);
            let is = was.elapsed();

            // TODO: maybe figure out how to make this only use thousands
            // separators up to a certain number (accounting for BigUint)
            println!("{}", fib_vector[single].separate_with_commas());
            println!("\nTime taken: {:?}", is);
        }
        (None, Some(multiple)) => {
            let was = Instant::now();
            let fib_vector: Vec<BigUint> = calculate_fib(multiple);
            let is = was.elapsed();

            for (index, num) in fib_vector.iter().enumerate() {
                println!("{}. {}", index, num.separate_with_commas());
            }
            println!("\nTime taken: {:?}", is);
        }
        (None, None) => {
            println!("please run fib --help for more information.");
        }
        (Some(_), Some(_)) => {
            println!("please pick either --single or --multiple!");
        }
    }
}
