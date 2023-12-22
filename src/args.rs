use clap::Parser;

/// fib: an overly complicated fibonacci calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// calculate the <n>th fibonacci number
    #[arg(short, long)]
    pub single: Option<usize>,

    /// calculate <n..n> fibonacci numbers
    #[arg(short, long)]
    pub multiple: Option<String>,

    /// whether to print the "analytics" (calc time etc)
    #[arg(short, long)]
    pub analytics: bool,

    /// whether to separate thousands with commas
    #[arg(short, long)]
    pub commas: bool,

    /// whether to save the number(s) to a file instead of printing
    #[arg(short, long)]
    pub file: bool,
}
