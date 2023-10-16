use colored::Colorize;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;
use std::time::{Duration, Instant};
use thousands::Separable;

mod args;

/// calculates only the nth fibonacci number
fn calculate_fib_sing(nth: usize) -> BigUint {
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    for _ in 0..nth {
        let next = a + &b;
        a = replace(&mut b, next);
    }

    a
}

/// calculates the fibonacci sequence 0..n into a vector
fn calculate_fib_mult(limit_nth: usize) -> Vec<BigUint> {
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();
    let mut a = BigUint::zero();
    let mut b = BigUint::one();
    for _ in 0..=limit_nth {
        calc_fib_vector.push(a.clone());
        let next = a + &b;
        a = replace(&mut b, next);
    }

    calc_fib_vector
}

/// prints the various information about the current run
fn print_analytics(amount: usize, calc_duration: Duration, print_duration: Duration) {
    println!(
        "\n{}{:?}\n{}{:?}",
        format!(
            "{} {} {}",
            "time taken to calculate",
            amount.separate_with_commas(),
            "digits: "
        )
        .green(),
        calc_duration,
        "additional time taken to print: ".green(),
        print_duration
    );
}

fn main() {
    let args: args::Args = argh::from_env();

    match (args.single, args.multiple) {
        (Some(single), None) => {
            let was = Instant::now();
            let fib = calculate_fib_sing(single);
            let calc_duration = was.elapsed();
            println!("{fib}");
            let print_duration = was.elapsed() - calc_duration;

            if args.analytics {
                print_analytics(single, calc_duration, print_duration)
            }
        }
        (None, Some(multiple)) => {
            let was = Instant::now();
            let fib_vector: Vec<BigUint> = calculate_fib_mult(multiple);
            let calc_duration = was.elapsed();
            for (index, num) in fib_vector.iter().enumerate() {
                println!("{}{} {}", index.to_string().bold(), ".".bold(), num);
            }
            let print_duration = was.elapsed() - calc_duration;

            if args.analytics {
                print_analytics(multiple, calc_duration, print_duration)
            }
        }
        (_, _) => {
            println!("please run fib --help for more information.");
        }
    }
}
