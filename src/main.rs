use std::io;
use rand::Rng;


fn main() {
    use std::io::{self, Write};
    println!("\nIt's the Number Guessing Game!\n");
    let secret_num = rand::Rng().gen_range(0,101);
    println!("Here's the Secret# {} ",secret_num);
    let mut guess = String::new();
    print!("Please enter your guess: ");
    io::stdout().flush();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    println!("\nYou guessed: {}",guess);
}