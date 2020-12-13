#![allow(unused)]
use std::io;


fn main() {
    use std::io::{self, Write};
    println!("\nIt's the Number Guessing Game!\n");
    let mut guess = String::new();
    print!("Please enter your guess: ");
    io::stdout().flush();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    println!("\nYou guessed: {}",guess);
}