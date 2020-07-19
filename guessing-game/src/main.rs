use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1, 101);

    loop {
        println!("\nPlease input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("That's not a number!");
                continue;
            },
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}