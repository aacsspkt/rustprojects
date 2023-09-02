use std::io;
use rand::{Rng, thread_rng};

fn main() {
    
    println!("Welcome to Number guessing game.");
    
    let mut rng = thread_rng();
    let num : u32 = rng.gen_range(0..=100);
    
    let mut tries = 0;
    // println!("Random num: {}", num);
    loop {
        println!("Please enter your guess between (1-100):");
        
        tries+=1;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");
        
        let guess = guess.trim();
        let guess = match guess.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid guess value");
                continue;
            }
        };

        if guess > num {
            println!("Too high!");
        } 
        else if guess < num {
            println!("Too low!");
        } else {
            println!("CORRECT! after tries: {}", tries);
            break;
        }
    }
}