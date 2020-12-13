use std::io;
use rand::Rng;


fn main() {
    use std::io::{self, Write};
    println!("\nIt's the Number Guessing Game!\n");
    let secret_num = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}\n",secret_num);
    let mut guess = String::new();
    print!("Please enter your guess: ");
    io::stdout().flush();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    println!("\nYou guessed: {}",guess);
}