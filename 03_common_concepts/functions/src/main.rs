fn main() {
    println!("Hello, world!");
    my_fn(5, 'c');
    println!("{}", with_return(4));
}

fn my_fn(number: i32, name: char){
    println!("The number is {number} and the name is {name}");
}

fn with_return(x: i32) -> i32 {
    x + 2
}