use std::io;

fn main() {

    // println! is macro - a feature that allows to write code to generate other code
    println!("Guess the number");
    println!("Please input your guess.");

    // a mutable variable of type String
    // :: means it's an associated function, new() is an associated function of teh String type
    let mut guess = String::new();
    // calls the stdin() function from the io module
    io::stdin()
        // the & indicates that the argument is a reference &mut means a mutable reference
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    
}
