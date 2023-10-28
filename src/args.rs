use clap::Parser;

/// fib: an overly complicated fibonacci calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// calculate the nth fibonacci number
    #[arg(short, long)]
    pub single: Option<usize>,

    /// calculate [n..n] fibonacci numbers
    #[arg(short, long)]
    pub multiple: Option<String>, // FIXME

    /// whether to print the "analytics" (calc time, print time etc)
    #[arg(short, long)]
    pub analytics: bool,

    /// whether to separate thousands with commas
    #[arg(short, long)]
    pub commas: bool,
}
