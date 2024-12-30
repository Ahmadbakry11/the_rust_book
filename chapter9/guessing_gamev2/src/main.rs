use rand::Rng;
use std::{cmp::Ordering, io};

pub struct Guess {
    value: i32,
} 

impl Guess {
    pub fn new(value: i32) -> Result<Guess, io::Error> {
        if value < 1 || value > 100 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Value must be between 1 and 100"));
        }

        Ok(Guess {value})
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    let stars = "*****************************************************************";
    println!("{stars}");
    println!("Welcome to the guessing game!");
    println!("{stars}");
    let secret_number:i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Enter your guess!");
        println!("The secret number is {}", secret_number);
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Could not read the input");
        
        
        let guess:Result<Guess, io::Error> = match guess.trim().to_string().parse::<i32>() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Only numbers allowed!");
                continue;
            }
        };

        let guess = match guess {
            Ok(n) => n,
            Err(_) => {
                println!("Only numbers between 1 and 100 allowed");
                continue;
            },
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
            

        println!("You guessed: {}", guess.value());
    }
}
