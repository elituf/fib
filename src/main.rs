mod args;
mod calculation;
mod format;

use clap::Parser;
use colored::Colorize;
use eyre::Result;
use std::{env::current_dir, fs::File, io::Write};

fn file(output: String, amount: String) -> Result<()> {
    let file_name = format!("{}th_fibonacci.txt", amount);
    let current_dir = current_dir()?.canonicalize()?;

    let mut file = File::create(&file_name)?;
    file.write(output.as_bytes())?;
    println!("saved file {} to {}", file_name.bold(), current_dir.to_string_lossy().trim_start_matches(r"\\?\"));

    Ok(())
}

fn main() -> Result<()> {
    let args = crate::args::Args::parse();

    if let Some(n) = args.single {
        let output = format::single(n);

        if args.file {
            file(output, n.to_string())?;
        } else {
            println!("{}", output);
        }
    }
    if let Some(range) = args.multiple {
        let range_vec: Vec<&str> = range.split("..").collect();
        let range_start = range_vec[0].parse::<usize>()?;
        let range_end = range_vec[1].parse::<usize>()?;

        let output = format::multiple(range_start..range_end)?;

        if args.file {
            file(output, range)?;
        } else {
            println!("{}", output);
        }
    }

    Ok(())
}
