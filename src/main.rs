use rand::Rng;
use std::cmp::Ordering;

fn main() {
    use std::io::{self, Write};
    println!("\nIt's the Number Guessing Game!\n\nGuess the right number within 5 tries.\n");
    let secret_num = rand::thread_rng().gen_range(1,101);
    let mut n =  1;

    while n < 6 {
    let mut guess = String::new();
    print!("Please enter your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    println!("\nYou guessed: {}",guess);
    
    let guess:u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) =>{println!("You must enter a number!!\n");
      n += 1;
      if n==6 {
        println!("You're out of retries. Better luck next time.\n");
        break;
        } else {
          println!("You have {} tries left\n", 6 - &n);
        }
      continue;
      }
    };
    
    match guess.cmp(&secret_num) {
      Ordering::Equal => {
        println!("You Win!\n");
        break;
      },
      Ordering::Greater => { 
        n += 1;
        if n==6 {
          println!("You're out of retries. Better luck next time.\n");
          break;} else {
          println!("Too High! You have {} tries left.\n", 6 - &n);
        }
      },
      Ordering::Less => {
        n += 1;
        if n==6 {
          println!("You're out of retries. Better luck next time.\n");
          break;} else {
          println!("Too Low! You have {} tries left.\n", 6 - &n); 
        }
      }
    }
  }   
}