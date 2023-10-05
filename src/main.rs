use colored::Colorize;
use ibig::{ubig, UBig};
use std::time::Instant;

mod args;

/// calculates the fibonacci sequence starting at 0
fn calculate_fib(limit_nth: usize) -> Vec<UBig> {
    let mut calc_fib_vector: Vec<UBig> = Vec::new();
    let mut a: UBig = ubig!(0);
    let mut b: UBig = ubig!(1);

    for _ in 0..=limit_nth {
        calc_fib_vector.push(a.clone()); // first we push a copy of a to the vec
        let next: UBig = a + b.clone(); // then we set the next value to be the sum of a + copy of b
        a = b; // then we set a to be equal to b
        b = next; // finally we set b to be next
    }

    calc_fib_vector
}

fn main() {
    let args: args::Args = argh::from_env();

    match (args.single, args.multiple) {
        (Some(single), None) => {
            let was = Instant::now();
            let fib_vector: Vec<UBig> = calculate_fib(single);
            let is = was.elapsed();

            println!("{}", fib_vector[single]);

            println!("\n{} {:?}", "Time taken:".green(), is);
        }
        (None, Some(multiple)) => {
            let was = Instant::now();
            let fib_vector: Vec<UBig> = calculate_fib(multiple);
            let is = was.elapsed();

            for (index, num) in fib_vector.iter().enumerate() {
                println!("{}{} {}", index.to_string().bold(), ".".bold(), num);
            }
            println!("\n{} {:?}", "Time taken:".green(), is);
        }
        (None, None) => {
            println!("please run fib --help for more information.");
        }
        (Some(_), Some(_)) => {
            println!("please pick either --single or --multiple!");
        }
    }
}
