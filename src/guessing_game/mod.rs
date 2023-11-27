pub mod guessing_game;
use rand::Rng;
use std::{cmp, io};

fn guess() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
