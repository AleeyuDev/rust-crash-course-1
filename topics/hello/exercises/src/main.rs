// Attribute that tells the Rust compiler to suppress warnings
// about unused code
// Attributes - metadata for the compiler
#![allow(unused)]

// main() is the entry point of Rust program

// println! is a macro that prints text to the console.
// Macros in Rust generate code at compile time and are
// invoked with an exclamation mark (!).


    // constants 100_000; // constant value
    const MAX_POINTS: u32 = 100_000; // constant value
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);


  fn  main() {
    // println!("Hello,Rust!");

    
//   variables are immutable by default in Rust.
//   To make a variable mutable, you need to use the mut keyword.
//  Here, we declare a mutable variable x and assign it the value 5.


    let mut x = 1; // mutable variable

//   type inference

let y: i32 = -10; // explicitly typed variable
let z: = -10 
let z = 3.14; // floating-point variable

// shadowing
let x: i32 = 1; // x is now shadowed and has the value 10
let x: i32 = 2; // x is now shadowed and has the value 
let x: i32= bool = true; // x is now shadowed and has the value true

    println!("The value of x is: {}", x);

    // _ is a special variable name that tells the compiler  
    // that we don't care about this variable.
    let _unused_variable = 5;
    let _ = 10; // ignoring the value 10  
    
    // type placeholders
    let _ =
    let x: _ = 1234;

    //  println!("The value of y is: {}", y);

let x = 1;


    println!("The value of x is: {}", x);




    
//    inline formatting
       println!("The value of z is: {x}", z);
    //  positional arguments
       println!("The value of z is: {0}, the value of x is: {1}", z, x);


       let z = x + x;
       println!("The value of z is: {x} + {x} =, the value of x is: {z}");
    // named arguments
       println!("The value of z is: {z}, the value of x is: {x}", z=z, x=x);
       println!("{0} + {0} = {1}", x, x + x );
    // special formatting       
       println!("The value of z is: {z}");
    //    Debugging 

    println!("DEBUG  X {:?}", x);
    println!("DEBUG  X {:#?}", x);
   println!("The value of z is: {z:#?}");



    let  mut count =  20;


   }




