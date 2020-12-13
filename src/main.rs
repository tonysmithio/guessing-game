use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    use std::io::{self, Write};
    println!("\nIt's the Number Guessing Game!\n");
    let secret_num = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is: {}\n",secret_num);
    loop{
    let mut guess = String::new();
    print!("Please enter your guess: ");
    io::stdout().flush();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    println!("\nYou guessed: {}",guess);
    
    let guess:u32 = guess.trim().parse()
      .expect("Please enter a number!");
    
    match guess.cmp(&secret_num) {
      Ordering::Equal => {
        println!("You Win!\n");
        break;
      }
      Ordering::Greater => println!("Too High, try again!\n"),
      Ordering::Less => println!("Too Low, try again!\n"),
    
    }
  }  
}