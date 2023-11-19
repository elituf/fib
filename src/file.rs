use eyre::Result;
use std::{env::current_dir, fs::File, io::Write};

pub fn save_to_file(output: String, amount: String) -> Result<()> {
    let file_name = format!("{}th_fibonacci.txt", amount);
    let current_dir = current_dir()?.canonicalize()?;

    let mut file = File::create(&file_name)?;
    file.write(output.as_bytes())?;
    println!("saved file {} to {}", file_name, current_dir.to_string_lossy().trim_start_matches(r"\\?\"));

    Ok(())
}
