use colored::Colorize;
use std::time::Duration;
use thousands::Separable;

/// prints the various information about the current run
pub fn analytics(amount: usize, calc_duration: Duration, print_duration: Duration) {
    println!(
        "\n{}{:?}",
        format!("{} {} {}", "time taken to calculate", amount.separate_with_commas(), "digits: ").green(),
        calc_duration,
    );
    println!("{}{:?}", "additional time taken to print: ".green(), print_duration);
}
