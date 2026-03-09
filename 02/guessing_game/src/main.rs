use std::io;

fn main() {

    // println! is macro - a feature that allows to write code to generate other code
    println!("Guess the number");
    println!("Please input your guess.");

    // a mutable variable of type String
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    
}
