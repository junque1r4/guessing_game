use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut guess: String = String::new();

    println!("Guess the number!");
    println!("{}({})", "The secret number is between 1 and 100".purple().bold(), secret_number);

    loop {
        
        guess.clear();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".blue()),
            Ordering::Equal => {
                println!("{}", "You win!".purple());
                break;
            },
        }
        
    }
}
