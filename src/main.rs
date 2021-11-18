use rand::Rng;
use core::num;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Cast a string to an int type
        // Return an error if a non-numeric value is input.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            },
        };

        // Compares a secret number with a input number and displays a message for each condition.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        
        println!("Your guessed: {}", guess);
    }

    println!("The secret number is: {}", secret_number);
}
