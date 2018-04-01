extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

const MIN_NUMBER: i32 = 1;
const MAX_NUMBER: i32 = 100;

fn main() {
    println!(
        "Guess the number between {} and {}!",
        MIN_NUMBER, MAX_NUMBER
    );
    let secret_number =
        rand::thread_rng().gen_range(MIN_NUMBER, MAX_NUMBER + 1);
    loop {
        print!("Guess: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\"{}\" is not a valid number.", guess.trim());
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
