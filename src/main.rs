use crate::args::Args;
use clap::Parser;
use eyre::Result;
use std::ops::Range;

mod args;
mod calculate;
mod file;
mod format;

/// parses a string of type "n..n" into a proper Range<usize>
fn parse_range(range: &str) -> Result<Range<usize>> {
    let range: Vec<&str> = range.split("..").collect();
    let start: usize = range[0].parse()?;
    let end: usize = range[1].parse()?;
    Ok(start..end)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(n) = args.single {
        let output = format::single(n);

        if args.file {
            file::save(&output, &n.to_string())?;
        } else {
            println!("{output}");
        }
    }
    if let Some(range) = args.multiple {
        let output = format::multiple(parse_range(&range)?);

        if args.file {
            file::save(&output, &range)?;
        } else {
            println!("{output}");
        }
    }

    Ok(())
}
