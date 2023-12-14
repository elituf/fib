use crate::{
    args::Args,
    calculate::{self, Analytics},
};
use clap::Parser;
use std::ops::Range;
use thousands::Separable;

/// formats a single nth fibonacci
pub fn single(amount: usize) -> String {
    let args = Args::parse();
    let (fib, analytics) = calculate::single(amount);

    let mut output = String::new();

    if args.commas {
        let fib = fib.separate_with_commas();
        output += &format!("{fib}\n");
    } else {
        output += &format!("{fib}\n");
    }

    if args.analytics {
        output += &fmt_analytics(&analytics);
    }

    output
}

/// formats 0..n in fibonacci sequence
pub fn multiple(range: Range<usize>) -> String {
    let args = Args::parse();
    let (fib, analytics) = calculate::multiple(range.end);

    let mut output = String::new();

    for (index, fib) in fib.iter().enumerate() {
        if index >= range.start {
            if args.commas {
                let fib = fib.separate_with_commas();
                output += &format!("{index}. {fib}\n");
            } else {
                output += &format!("{index}. {fib}\n");
            }
        }
    }

    if args.analytics {
        output += &fmt_analytics(&analytics);
    }

    output
}

/// formats the various information about the current run
pub fn fmt_analytics(analytics: &Analytics) -> String {
    format!(
        "time taken to calculate {}{}{:?}",
        analytics.num_digits.separate_with_commas(),
        " digits: ",
        analytics.calc_time
    )
}
