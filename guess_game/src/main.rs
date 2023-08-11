use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut guess = String::new();

    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    print!("Guessing game initialized. \nEnter a guess: ");
    io::stdout()
        .flush()
        .expect("Toilet's clogged! Failed to flush");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess.trim());

    println!("The secret number was: {secret}");

    let x: i32 = guess
        .trim()
        .parse()
        .expect("Failed to parse i8 int from guess");

    if secret == x {
        println!("You got it!");
    } else {
        println!("Better luck next time.");
    }
}
