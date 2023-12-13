use clap::Parser;
use eyre::Result;
use std::ops::Range;

mod calculation;
mod file;
mod format;

/// fib: an overly complicated fibonacci calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// calculate the nth fibonacci number
    #[arg(short, long)]
    pub single: Option<usize>,

    /// calculate [n..n] fibonacci numbers
    #[arg(short, long)]
    pub multiple: Option<String>,

    /// whether to print the "analytics" (calc time, print time etc)
    #[arg(short, long)]
    pub analytics: bool,

    /// whether to separate thousands with commas
    #[arg(short, long)]
    pub commas: bool,

    /// whether to save the number(s) to a file instead of printing
    #[arg(short, long)]
    pub file: bool,
}

fn parse_range(range: &String) -> Result<Range<usize>> {
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
            file::save_to_file(output, n.to_string())?;
        } else {
            println!("{}", output);
        }
    }
    if let Some(range) = args.multiple {
        let output = format::multiple(parse_range(&range)?);

        if args.file {
            file::save_to_file(output, range)?;
        } else {
            println!("{}", output);
        }
    }

    Ok(())
}
