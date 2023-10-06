use colored::Colorize;
use num_bigint::{BigUint, ToBigUint};
use std::mem::replace;
use std::time::Instant;
use thousands::Separable;

mod args;

/// calculates only the nth fibonacci number
fn calculate_fib_sing(nth: usize) -> BigUint {
    let mut a: BigUint = 0.to_biguint().expect("0");
    let mut b: BigUint = 1.to_biguint().expect("1");

    for _ in 0..nth {
        let next = a + &b;
        a = replace(&mut b, next);
    }

    a
}

/// calculates the fibonacci sequence starting at 0 into a vector
fn calculate_fib_mult(limit_nth: usize) -> Vec<BigUint> {
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();
    let mut a: BigUint = 0.to_biguint().expect("0");
    let mut b: BigUint = 1.to_biguint().expect("1");

    for _ in 0..=limit_nth {
        calc_fib_vector.push(a.clone());
        let next = a + &b;
        a = replace(&mut b, next);
    }

    calc_fib_vector
}

fn main() {
    let args: args::Args = argh::from_env();

    match (args.single, args.multiple) {
        (Some(single), None) => {
            let was = Instant::now();
            let fib = calculate_fib_sing(single);
            let calc_duration = was.elapsed();

            println!("{fib}");

            println!(
                "\n{}{:?}",
                format!(
                    "{} {}{}",
                    "Time taken to calculate",
                    single.separate_with_commas(),
                    "th digit: "
                )
                .green(),
                calc_duration,
            );
        }
        (None, Some(multiple)) => {
            let was = Instant::now();
            let fib_vector: Vec<BigUint> = calculate_fib_mult(multiple);
            let calc_duration = was.elapsed();

            for (index, num) in fib_vector.iter().enumerate() {
                println!("{}{} {}", index.to_string().bold(), ".".bold(), num);
            }

            println!(
                "\n{}{:?}",
                format!(
                    "{} {}{}",
                    "Time taken to calculate",
                    multiple.separate_with_commas(),
                    "th digit: "
                )
                .green(),
                calc_duration
            );
        }
        (None, None) => {
            println!("please run fib --help for more information.");
        }
        (Some(_), Some(_)) => {
            println!("please pick either --single or --multiple!");
        }
    }
}
