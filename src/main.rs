mod args;
mod calculation;
mod format;

use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    let args = crate::args::Args::parse();

    if let Some(n) = args.single {
        println!("{}", format::single(n));
    }
    if let Some(range) = args.multiple {
        println!("{}", format::multiple(range)?);
    }

    Ok(())
}
