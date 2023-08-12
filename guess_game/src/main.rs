use rand::Rng;
use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    println!("Guessing game initialized.");

    loop {
        let mut guess = String::new();

        let secret: u32 = rand::thread_rng().gen_range(1..=100);

        print!("Enter a guess: ");
        io::stdout()
            .flush()
            .expect("Toilet's clogged! Failed to flush");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess.trim());

        println!("The secret number was: {}", secret);

        let x: u32 = guess
            .trim()
            .parse()
            .expect("Failed to parse u32 int from input, be sure to type a number.");

        match x.cmp(&secret) {
            Ordering::Less => println!("Aim higher."),
            Ordering::Greater => println!("Way off."),
            Ordering::Equal => println!("Lucky guess."),
        }
    }
}
