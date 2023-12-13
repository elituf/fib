use crate::{
    calculation::{self, Analytics},
    Args,
};
use clap::Parser;
use std::ops::Range;
use thousands::Separable;

/// formats a single nth fibonacci
pub fn single(amount: usize) -> String {
    let args = Args::parse();
    let (fib, analytics) = calculation::calculate_fib_sing(amount);

    let mut output = String::new();

    if args.commas {
        output += &format!("{}\n", fib.separate_with_commas());
    } else {
        output += &format!("{}\n", fib);
    }

    if args.analytics {
        output += &fmt_analytics(analytics);
    }

    output
}

/// formats 0..n in fibonacci sequence
pub fn multiple(range: Range<usize>) -> String {
    let args = Args::parse();
    let (fib, analytics) = calculation::calculate_fib_mult(range.end);

    let mut output = String::new();

    for (index, fib) in fib.iter().enumerate() {
        if index >= range.start {
            if args.commas {
                output += &format!("{}. {}\n", index.to_string(), fib.separate_with_commas());
            } else {
                output += &format!("{}. {}\n", index.to_string(), fib);
            }
        }
    }

    if args.analytics {
        output += &fmt_analytics(analytics);
    }

    output
}

/// formats the various information about the current run
pub fn fmt_analytics(analytics: Analytics) -> String {
    format!(
        "time taken to calculate {}{}{:?}",
        analytics.num_digits.separate_with_commas(),
        " digits: ",
        analytics.calc_time
    )
}
