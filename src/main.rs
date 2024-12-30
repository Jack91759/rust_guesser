use std::io::{self, Write};
use rand::Rng;
use colored::*;

fn main() {
    // Seed the random number generator
    let mut rng = rand::thread_rng();

    // Define the range for the "guess the number" game
    let secret_number = rng.gen_range(1..=100);

    // Introduce the game with some colorful text
    println!("{}", "Welcome to Guess the Colorful Number!".green());
    println!("{}", "Can you guess the secret number between 1 and 100?".yellow());

    loop {
        print!("{}", "Enter your guess: ".bold());
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please input a valid number.".red());
                continue;
            }
        };

        // Check if the guess is correct and provide colorful feedback
        if guess < secret_number {
            println!("{}", "Too low!".blue());
        } else if guess > secret_number {
            println!("{}", "Too high!".magenta());
        } else {
            println!("{}", "Congratulations! You guessed it!".green().bold());
            break;
        }
    }
}
