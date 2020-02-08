use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::num::ParseIntError;

const MIN_NUMBER: i32 = 1;
const MAX_NUMBER: i32 = 1000;

pub fn start() {
  let random_number: i32 = generate_random_number();

  println!("Playing guess the number!");
  println!("Enter a number between {} and {}", MIN_NUMBER, MAX_NUMBER);

  loop {
    let guessed_number: i32 = match get_input_parsed() {
      Ok(num) => num,
      Err(_) => {
        println!("Enter a valid number!");
        continue;
      }
    };

    match guessed_number.cmp(&random_number) {
      Ordering::Less => println!("Try a bigger number!"),
      Ordering::Greater => println!("Try a smaller one!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}

fn get_input() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).expect("There was an error while reading.");
  buffer
}

fn get_input_parsed() -> Result<i32, ParseIntError> {
  get_input().trim().parse()
}

fn generate_random_number() -> i32 {
  rand::thread_rng().gen_range(MIN_NUMBER, MAX_NUMBER)
}
