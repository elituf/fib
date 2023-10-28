mod args;
mod calculation;
mod print_fib;
mod print_other;

use clap::Parser;

fn main() {
    let args = crate::args::Args::parse();

    match (args.single, args.multiple) {
        (Some(amount), None) => print_fib::single(amount),
        (None, Some(amount)) => print_fib::multiple(amount),
        (_, _) => println!("please run fib --help for more information."),
    }
}
