mod args;
mod calculation;
mod print_fib;
mod print_other;

fn main() {
    let args: args::Args = argh::from_env();

    match (args.single, args.multiple) {
        (Some(amount), None) => print_fib::single(amount),
        (None, Some(amount)) => print_fib::multiple(amount),
        (_, _) => println!("please run fib --help for more information."),
    }
}
