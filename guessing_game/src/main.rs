use std::io;
fn main() {
    println!("Welcome to guessing game.");
    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read line");

    println!("Your guess was {}", guess);
}
