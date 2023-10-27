use argh::FromArgs;

/// fib: an overly complicated fibonacci calculator
#[derive(FromArgs, PartialEq, Eq)]
pub struct Args {
    /// calculate the nth fibonacci number
    #[argh(option, short = 's')]
    pub single: Option<usize>,

    /// calculate 0..n fibonacci numbers
    #[argh(option, short = 'm')]
    pub multiple: Option<usize>,

    /// whether to print the "analytics" (calc time, print time etc)
    #[argh(switch, short = 'a')]
    pub analytics: bool,

    /// whether to separate with commas
    #[argh(switch, short = 'c')]
    pub commas: bool,
}
