fn main() {
   let x = 5;
   println!("the value of x is:{x}");
//    This won't work since variables are immutable by default:
//    x = 6;

// What you can do is this:
let x = x+ 1;
// This is called shadowing
   println!("The value of x is: {x}");

   fn test(){
      println!("This is inside the test inside main");
      // This is not working. I cannot use x inside an inner function
      // let x = x*2;
      // not possible: cannot access from inside an inner function but inner scope is OK (see below)
      // println!("Can I use the x variable from main= {x}");
   }
   {
      // But I can do this:
      let x = x*2;
      println!("The value of the inner scope x is : {x}")
   }
   test();
   println!("The value of x after running test function: {x}")
}

/*
Very important:
In Rust a function fn ... and a scope {...} are totally different things
A block is a nested lexical scope. The Block can "see" the variables from the outer scope
Functions in Rust do not capture variables from its parent scope
Functions are independent items in Rust!!
Functions in Rust are compile-time items not runtime closures
Functions have to be explicit and predictable 
If you want to use outer variables you have to use closures like this:
fn main() {
    let x = 6;

    let test = || {
        let x = x * 2;
        println!("Inside closure: {x}");
    };

    test();
}
 */
