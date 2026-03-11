use std::io;
use rand::Rng;
// the ordering type is an enum with values Less, Greater and Equal
use std::cmp::Ordering;

fn main() {

    // println! is macro - a feature that allows to write code to generate other code
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    println!("The random number is {secret_number}");
    println!("Please input your guess.");

    // a mutable variable of type String
    // :: means it's an associated function, new() is an associated function of teh String type
    let mut guess = String::new();
    // calls the stdin() function from the io module
    io::stdin()
        // the & indicates that the argument is a reference &mut means a mutable reference
        // like variables, references are immutable by default
        // Result is an enum that can be either Ok or Err, and expect() is a method that returns the value inside the Ok variant or panics if it's an Err
        .read_line(&mut guess)
        .expect("Failed to read line");

    // This shadows the previous variable guess
    // We have to use the trim(), otherwise the "Enter" i.e. \r\n" will not be trimmed from the String
    // The pase() method knows the type in which to convert to since we gave the guess variable a specific type when declaring
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
    }
    
}
