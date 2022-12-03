#![deny(clippy::all)]

// We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

// Crates.io, seach rand and copy the info into the Cargo.toml file

// Bring the input/output library into scope from the standard library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    // loop is like the while in other languages
    loop {
        println!("Please input your guess:");

        // new is the associated function of the String type
        // It creates a new empty string (it appears in many types)
        let mut guess = String::new();

        // This will cause the prompt to wait until input
        io::stdin()
            // &mut guess rather than &guess because references are immutable by default
            // read_line reads whatever you input into a string and place it in guess
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the String the program reads as input into a real number type so we can compare it numerically to the secret number

        // Shadowing the guess variable to reuse it
        // trim is a method of String that removes whitespaces
        // parse method converts the String into another type, in this case we annotated the type u32 to the shadowed variable: it returns a Result type to handle
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // if is a number, fine
            Err(_) => continue, // if not, skip this iteration and ask for another number
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
