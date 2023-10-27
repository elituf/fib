use colored::Colorize;
use num_bigint::BigUint;
use std::time::Instant;

use crate::{args, calculation, print_other};

pub fn single(amount: usize) {
    let args: args::Args = argh::from_env();

    let was = Instant::now();
    let fib = calculation::calculate_fib_sing(amount);
    let calc_duration = was.elapsed();
    println!("{fib}");
    let print_duration = was.elapsed() - calc_duration;

    if args.analytics {
        print_other::analytics(amount, calc_duration, print_duration);
    }
}

pub fn multiple(amount: usize) {
    let args: args::Args = argh::from_env();

    let was = Instant::now();
    let fib_vector: Vec<BigUint> = calculation::calculate_fib_mult(amount);
    let calc_duration = was.elapsed();
    for (index, num) in fib_vector.iter().enumerate() {
        println!("{}{} {}", index.to_string().bold(), ".".bold(), num);
    }
    let print_duration = was.elapsed() - calc_duration;

    if args.analytics {
        print_other::analytics(amount, calc_duration, print_duration);
    }
}
