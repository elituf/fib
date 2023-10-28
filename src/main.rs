mod args;
mod calculation;
mod print_fib;
mod print_other;

use clap::Parser;

fn main() {
    let args = crate::args::Args::parse();

    match (args.single, args.multiple) {
        (Some(amount), None) => print_fib::single(amount),
        (None, Some(amount)) => {
            // FIXME
            let range: Vec<&str> = amount.split("..").collect();
            let start: usize = range[0].parse().expect("i need an input with format n..n");
            let end: usize = range[1].parse().expect("i need an input with format n..n");

            print_fib::multiple(start..end)
        }
        (_, _) => println!("please run fib --help for more information."),
    }
}
