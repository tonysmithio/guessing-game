#![allow(unused)]
use std::io;


fn main() {
    use std::io::{self, Write};
    println!("It's the Number Guessing Game!");
    
    let mut guess = String::new();
    print!("Please enter your guess: ");
    io::stdout().flush();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    println!("You guessed: {}",guess);
}