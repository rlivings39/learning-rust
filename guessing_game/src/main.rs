use std::cmp::Ordering;
use std::io;

use rand::Rng;

mod guess;
use crate::guess::Guess;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::rng().random_range(1..=100);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    guess = guess.trim().to_string();

    match guess.as_str() {
      "quit" => {
        println!("Quitting...");
        break;
      }
      _ => {}
    }
    let guess: i32 = match guess.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid number. Try again.");
        continue;
      }
    };

    let guess: Guess = match Guess::new(guess) {
      Ok(g) => g,
      Err(e) => {
        println!("{}", e);
        continue;
      }
    };
    let value = guess.value();
    println!("You guessed: {value}");

    match value.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
