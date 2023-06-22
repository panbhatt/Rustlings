// Traits - AsRef Implementation . It is. Used to do a cheap reference-to-reference conversion.
// https://doc.rust-lang.org/std/convert/trait.AsRef.html

// it can be implemented for Any Type.

use std::convert::AsRef;
use std::fmt::Display;

//This function will only accept only STring as it implements both Display and AsREf
fn print_str<T: AsRef<str> + Display>(input: T) {
    println!("{}", input);
}

fn main() {
    print_str("HELLO WORLD");
}
