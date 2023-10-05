use colored::Colorize;
use num_bigint::{BigUint, ToBigUint};
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

            if fib_vector[single]
                <= u128::MAX
                    .to_biguint()
                    .expect("expected an integer between 0..u128::MAX")
            {
                println!("{}", fib_vector[single].separate_with_commas());
            } else {
                println!("{}", fib_vector[single]);
            }

            println!("\n{} {:?}", "Time taken:".green(), is);
        }
        (None, Some(multiple)) => {
            let was = Instant::now();
            let fib_vector: Vec<BigUint> = calculate_fib(multiple);
            let is = was.elapsed();

            for (index, num) in fib_vector.iter().enumerate() {
                if num
                    <= &u128::MAX
                        .to_biguint()
                        .expect("expected an integer between 0..u128::MAX")
                {
                    println!(
                        "{}{} {}",
                        index.to_string().bold(),
                        ".".bold(),
                        num.separate_with_commas()
                    );
                } else {
                    println!("{}{} {}", index.to_string().bold(), ".".bold(), num);
                }
            }
            println!("\n{} {:?}", "Time taken:".green(), is);
        }
        (None, None) => {
            println!("please run fib --help for more information.");
        }
        (Some(_), Some(_)) => {
            println!("please pick either --single or --multiple!");
        }
    }
}
