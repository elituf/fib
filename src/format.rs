use clap::Parser;
use colored::Colorize;
use eyre::Result;
use num_bigint::BigUint;
use std::{time::Duration, time::Instant};
use thousands::Separable;

use crate::{args::Args, calculation};

/// formats a single nth fibonacci
pub fn single(amount: usize) -> String {
    let args = Args::parse();
    let was = Instant::now();
    let fib = calculation::calculate_fib_sing(amount);
    let calc_duration = was.elapsed();

    let mut output = String::new();

    if args.commas {
        output += &format!("{}\n", fib.separate_with_commas());
    } else {
        output += &format!("{}\n", fib);
    }

    if args.analytics {
        output += &analytics(amount, calc_duration);
    }

    output
}

/// formats 0..n in fibonacci sequence
pub fn multiple(range: String) -> Result<String> {
    let range: Vec<&str> = range.split("..").collect();
    let range_start = range[0].parse::<usize>()?;
    let range_end = range[1].parse::<usize>()?;

    let args = Args::parse();
    let was = Instant::now();
    let fib_vector: Vec<BigUint> = calculation::calculate_fib_mult(range_end);
    let calc_duration = was.elapsed();

    let mut output = String::new();

    for (index, fib) in fib_vector.iter().enumerate() {
        if index >= range_start {
            if args.commas {
                output += &format!("{}. {}\n", index.to_string().bold(), fib.separate_with_commas());
            } else {
                output += &format!("{}. {}\n", index.to_string().bold(), fib);
            }
        }
    }

    if args.analytics {
        output += &analytics(range_end, calc_duration);
    }

    Ok(output)
}

/// formats the various information about the current run
pub fn analytics(amount: usize, calc_duration: Duration) -> String {
    format!(
        "{}{:?}",
        format!("{} {} {}", "time taken to calculate", amount.separate_with_commas(), "digits: ").green(),
        calc_duration // TODO: maybe re-add print duration
    )
}
