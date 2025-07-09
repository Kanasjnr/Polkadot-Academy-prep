#! [allow(unused)]

// constants
const NUM : u32 = 1;

fn main() {
    // variables 
    // immutble by defult
    // use "mut" to make it mutable
    let mut x = 1;
    x += 1;

    // Type interface
    let y: i32= -1;
    let z = -1;

    // shadowing
    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    // Type placeholder
    let x: _ = 1234;

    let x = 1;
    println!("x is {}", x);
    // inline print
    println!("x is {x}");
    // positional arguments
    let z = x + x;
    println!("{0} + {0} = {1}", x, z);

    // Debug can be used when data type is complex
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", (x));
}