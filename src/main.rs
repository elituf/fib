use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use text_io::read;
use thousands::Separable;

fn main() {
    print!("Would you like to do a single (nth) fibonacci number or multiple (0..n)? [s/m]: ");
    let choice: char = read!();

    print!("How many/up to what digit would you like? [literally any number]: ");
    let amount: BigUint = read!();

    let mut fibonacci_sequence: Vec<BigUint> = Vec::new();

    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();

    let amount_usize = amount.to_usize().expect("okay not that big of a number");

    for _ in 0..=amount_usize {
        fibonacci_sequence.push(a.clone());
        let next = a + b.clone();
        a = b;
        b = next;
    }

    match choice {
        's' => {
            if let Some(chosen_number) = fibonacci_sequence.get(amount_usize) {
                println!("{}", chosen_number.separate_with_commas());
            } else {
                println!("Invalid selection");
            }
        }
        'm' => {
            for (index, num) in fibonacci_sequence.iter().enumerate() {
                println!("{}. {}", index, num.separate_with_commas());
            }
        }
        _ => println!("don't do that"),
    }
}
