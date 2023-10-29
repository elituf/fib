mod args;
mod calculation;
mod print_fib;
mod print_other;

use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    let args = crate::args::Args::parse();

    match (args.single, args.multiple) {
        (Some(amount), None) => print_fib::single(amount),
        (None, Some(amount)) => {
            let range: Vec<&str> = amount.split("..").collect();
            let start: usize = range[0].parse()?;
            let end: usize = range[1].parse()?;

            print_fib::multiple(start..end);
        }
        (_, _) => return Ok(()),
    }

    Ok(())
}
