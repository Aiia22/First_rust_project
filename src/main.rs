/// This program is a simple number guessing game written in Rust.
/// The program generates a random number between 1 and 100, and the user has to guess it.
/// The user is prompted to enter their guess, and the program provides feedback on whether the guess is too low, too high, or correct.
/// The game continues until the user guesses the correct number.

/// Import the `rand` crate for random number generation.
use rand::Rng;

/// Import the `io` module from the standard library for handling input and output.
use std::io;

fn main() {
    // Print the initial message to the user.
    println!("Devine un nombre entre 1 et 100 !");

    // Generate a random number between 1 and 100.
    let secret = rand::thread_rng().gen_range(1..=100);

    // Start an infinite loop to repeatedly prompt the user for their guess.
    loop {
        // Prompt the user to enter their guess.
        println!("Entre ta proposition :");

        // Create a mutable String to store the user's input.
        let mut guess = String::new();

        // Read the user's input from the standard input and store it in the `guess` variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Échec de la lecture");

        // Attempt to parse the user's input as an unsigned 32-bit integer.
        // If parsing fails, print an error message and continue the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ce n'est pas un nombre, réessaie !");
                continue;
            }
        };

        // Compare the user's guess to the secret number and provide feedback.
        if guess < secret {
            println!("Trop petit !");
        } else if guess > secret {
            println!("Trop grand !");
        } else {
            // If the guess is correct, print a congratulatory message and break out of the loop.
            println!("Bravo, tu as trouvé !");
            break;
        }
    }
}
