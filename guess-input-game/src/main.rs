
// use std::io;
// use std::io as input_output; (Noted: we can alias it as well!)

fn main() {
  println!("Guess the input!");
  println!("Please input your guess: ");

  let mut input = String::new();

  std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  println!("You guessed: {input}");
}
