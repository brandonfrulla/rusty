use std::io;

fn main() {
    let mut guess = String::new();

    print!("Guessing game initialized. \nEnter a guess: ");
    io::stdout().flush().expect("Toilet's clogged! Failed to flush");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}
