use eyre::Result;
use std::{env::current_dir, fs::File, io::Write};

/// saves contents to current dir and prints a small info message
pub fn save(output: &str, n: &str) -> Result<()> {
    let file_name = format!("{n}th_fibonacci.txt");
    let current_dir = current_dir()?.canonicalize()?;

    let mut file = File::create(&file_name)?;
    file.write_all(output.as_bytes())?;
    println!(
        "saved file {} to {}",
        file_name,
        current_dir.to_string_lossy().trim_start_matches(r"\\?\")
    );

    Ok(())
}
