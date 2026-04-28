// use std::mem;

use std::io;

fn main() {
    println!("Hello, world!");

    let p = 4.0;
    let q = 7.87435;

    let quotient = p / q;

    println!(" The quotient of {p} and {q} is: {quotient}");

    println!("The memory size of the quotient is: {} bytes", size_of_val(&quotient));
    println!("The memory size of the q is: {} bytes", size_of_val(&q));

    let tup = (3.5, 8, -125.3789, 'z');
// an underscore indicates, that the variables aren't being used later on
    let (x,_y,_z, _c) = tup;

    println!("The first value of the tuple is: {x}");

    println!("The character of the tuple is: {}", tup.3);
    // modify values only works for mutable values:
    // tup.1 = 10;

    test_array()
}

fn test_array(){
    println!("Please enter an index number");
    let a =[1,2,3,4,5, 6];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("No entry was found");

    let index: usize = index.trim().parse().expect("Index was not an integer");

    let element = a[index];

    println!("The entry for the index {index} is: {element}");

}
