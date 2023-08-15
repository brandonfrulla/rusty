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

        // println!("You guessed: {}", guess.trim());

        println!("The secret number was: {}", secret); 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number between 1-100, dummy.");
                continue;
            }
        };

        // if guess == "quit" { //expected `u32`, found `&str`
        //     break;
        // }

        match guess.cmp(&secret) {
            Ordering::Less => println!("Aim higher.\n"),
            Ordering::Greater => println!("Way off.\n"),
            Ordering::Equal => {
                println!("Lucky guess!\n");
                break;
            }
        }
    }
}
