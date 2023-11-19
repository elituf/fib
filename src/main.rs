mod args;
mod calculation;
mod file;
mod format;

use std::ops::Range;

use clap::Parser;
use eyre::Result;

fn parse_range(range: &String) -> Result<Range<usize>> {
    let range_vec: Vec<&str> = range.split("..").collect();
    let start = range_vec[0].parse::<usize>()?;
    let end = range_vec[1].parse::<usize>()?;
    Ok(start..end)
}

fn main() -> Result<()> {
    let args = crate::args::Args::parse();

    if let Some(n) = args.single {
        let output = format::single(n);

        if args.file {
            file::save_to_file(output, n.to_string())?;
        } else {
            println!("{}", output);
        }
    }
    if let Some(range) = args.multiple {
        let output = format::multiple(parse_range(&range)?)?;

        if args.file {
            file::save_to_file(output, range)?;
        } else {
            println!("{}", output);
        }
    }

    Ok(())
}
