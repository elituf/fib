use argh::FromArgs;
use num_bigint::BigUint;
use num_traits::{One, Zero};
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

/// Calculates the fibonacci sequence starting at 0
fn calculate_fib(limit_nth: usize) -> Vec<BigUint> {
    let mut calc_fib_vector: Vec<BigUint> = Vec::new();

    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();

    for _ in 0..=limit_nth {
        calc_fib_vector.push(a.clone());
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
        let fib_vector: Vec<BigUint> = calculate_fib(args.single.unwrap());

        println!(
            "{}",
            fib_vector
                .get(args.single.unwrap())
                .unwrap()
                .separate_with_commas()
        );
    } else if args.multiple.is_some() {
        let fib_vector: Vec<BigUint> = calculate_fib(args.multiple.unwrap());

        for (index, num) in fib_vector.iter().enumerate() {
            println!("{}. {}", index, num.separate_with_commas());
        }
    } else {
        println!("please run fib --help for more information.");
    }
}
