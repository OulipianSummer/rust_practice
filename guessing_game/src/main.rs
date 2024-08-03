use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    // Generate a random number in this thread between 1 and 100 (inclusive).
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // Keep asking for input until guess is correct.
    loop {   
        println!("Please input your guess.");

        let mut guess = String::new();

        // Get user input.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Continue guessing until a valid u32 is provided.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); 
        
        // Check if the guessed matches the secret number.
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
