use std::io::{self, Write};
use rand::Rng;

fn main() {
    let mut guess = String::new();

    let secret : i8 = rand::thread_rng().gen_range(1..=100);

    print!("Guessing game initialized. \nEnter a guess: ");
    io::stdout().flush().expect("Toilet's clogged! Failed to flush");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    println!("The secret number was: {secret}");
}
