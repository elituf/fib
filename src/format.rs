use clap::Parser;
use eyre::Result;
use num_bigint::BigUint;
use std::{ops::Range, time::Duration, time::Instant};
use thousands::Separable;

use crate::{args::Args, calculation};

/// formats a single nth fibonacci
pub fn single(amount: usize) -> String {
    let args = Args::parse();
    let was = Instant::now();
    let fib = calculation::calculate_fib_sing(amount);

    let mut output = String::new();

    if args.commas {
        output += &format!("{}\n", fib.separate_with_commas());
    } else {
        output += &format!("{}\n", fib);
    }

    if args.analytics {
        output += &analytics(amount, was.elapsed());
    }

    output
}

/// formats 0..n in fibonacci sequence
pub fn multiple(range: Range<usize>) -> Result<String> {
    let args = Args::parse();
    let was = Instant::now();
    let fib_vector: Vec<BigUint> = calculation::calculate_fib_mult(range.end);

    let mut output = String::new();

    for (index, fib) in fib_vector.iter().enumerate() {
        if index >= range.start {
            if args.commas {
                output += &format!("{}. {}\n", index.to_string(), fib.separate_with_commas());
            } else {
                output += &format!("{}. {}\n", index.to_string(), fib);
            }
        }
    }

    if args.analytics {
        output += &analytics(range.end, was.elapsed());
    }

    Ok(output)
}

/// formats the various information about the current run
pub fn analytics(amount: usize, calc_duration: Duration) -> String {
    format!(
        "{}{:?}",
        format!("{}{}{}", "time taken to calculate ", amount.separate_with_commas(), " digits: "),
        calc_duration // TODO: maybe re-add print duration
    )
}
