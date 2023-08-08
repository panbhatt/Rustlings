// PRELUDE. https://doc.rust-lang.org/std/prelude/index.html

// This means what is automatically bring ins cope of the program so that the full reference need not to be used.

#![no_implicit_prelude]
use std::convert::From;
/// Since we said, we are not going to use prelude, we have to specify the String manually.
use std::string::String;
use std::vec; 
use std::println; 

fn main() {
    let my_string = String::from("Pankaj Bhatt");
    println!("{}", my_string);

    let my_vec = vec![12,3,4]; 
    println!("{:?}", my_vec); 
}
