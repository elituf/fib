use argh::FromArgs;
use thousands::Separable;

/// fib: an overly complicated fibonacci calculator
#[derive(FromArgs, PartialEq)]
struct Args {
    /// calculate the nth fibonacci number
    #[argh(option, short = 's')]
    single: Option<usize>,

    /// calculate 0..n fibonacci numbers
    #[argh(option, short = 'm')]
    multiple: Option<usize>,
}

fn calculate_fib(amount: usize) -> Vec<u128> {
    let mut calc_fib_vector: Vec<u128> = Vec::new();

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 0..=amount {
        calc_fib_vector.push(a);
        let next = a + b.clone();
        a = b;
        b = next;
    }

    calc_fib_vector
}

fn main() {
    let args: Args = argh::from_env();

    if args.single.is_some() && args.multiple.is_some() {
        panic!("/ please pick either --single or --multiple! /")
    } else if args.single.is_some() {
        let fib_vector: Vec<u128> = calculate_fib(args.single.unwrap());

        println!(
            "{}",
            fib_vector
                .get(args.single.unwrap())
                .unwrap()
                .separate_with_commas()
        );
    } else if args.multiple.is_some() {
        let fib_vector: Vec<u128> = calculate_fib(args.multiple.unwrap());

        for (index, num) in fib_vector.iter().enumerate() {
            println!("{}. {}", index, num.separate_with_commas());
        }
    }
}
