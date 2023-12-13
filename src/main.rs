use args::Args;
use clap::Parser;
use eyre::Result;
use std::num::ParseIntError;
use std::ops::Range;

mod args;
mod calculation;
mod file;
mod format;

fn parse_range(range: &String) -> Result<Range<usize>> {
    let range: Result<Vec<usize>, ParseIntError> = range
        .split("..")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.parse::<usize>())
        .collect();

    match range {
        Ok(range) => Ok(range[0]..range[1]),
        Err(why) => Err(why.into()),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

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
