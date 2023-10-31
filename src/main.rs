mod args;
mod calculation;
mod print_fib;
mod print_other;

use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    let args = crate::args::Args::parse();

    if let Some(n) = args.single {
        print_fib::single(n)
    }
    if let Some(range) = args.multiple {
        let range: Vec<&str> = range.split("..").collect();
        let start = range[0].parse::<usize>()?;
        let end = range[1].parse::<usize>()?;
        print_fib::multiple(start..end);
    }

    Ok(())
}
