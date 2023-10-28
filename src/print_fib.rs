use clap::Parser;
use colored::Colorize;
use num_bigint::BigUint;
use std::{ops::Range, time::Instant};
use thousands::Separable;

use crate::{args::Args, calculation, print_other};

pub fn single(amount: usize) {
    let args = Args::parse();

    let was = Instant::now();
    let fib = calculation::calculate_fib_sing(amount);
    let calc_duration = was.elapsed();
    if args.commas {
        println!("{}", fib.separate_with_commas());
    } else {
        println!("{}", fib);
    }
    let print_duration = was.elapsed() - calc_duration;

    if args.analytics {
        print_other::analytics(amount, calc_duration, print_duration);
    }
}

pub fn multiple(amount: Range<usize>) {
    let args = Args::parse();

    let was = Instant::now();
    let fib_vector: Vec<BigUint> = calculation::calculate_fib_mult(amount.end);
    let calc_duration = was.elapsed();
    for (index, fib) in fib_vector.iter().enumerate() {
        if index >= amount.start {
            if args.commas {
                println!("{}. {}", index.to_string().bold(), fib.separate_with_commas());
            } else {
                println!("{}. {}", index.to_string().bold(), fib);
            }
        }
    }
    let print_duration = was.elapsed() - calc_duration;

    if args.analytics {
        print_other::analytics(amount.end, calc_duration, print_duration);
    }
}
