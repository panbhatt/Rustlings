// There are two types of attributes. Inner and Outer

// Outer Attributes like #[dervice(Debug)] apply to the element that is present on means that follows the attribute
// Inner Attributes are like global attributes that applies to all the source code that follows it.

// #![warn(dead_code)]
// #![warn(unused_variable)]

// Opp of above is allow.
// #[allow(unused_variable)]
// #[allow(dead_code)]

#[derive(Debug)] // This is outer attributes
struct Employee {}

fn main() {
    #![allow(dead_code)] // This will make sure no warnings come at run time.
    #![allow(unused_variables)] // This would allow unused variable only in the MAIN BLock to do it everything, we have to put it at the top.
    let my_number = 10;
    println!("Running Code");
}

// Another Attributes are 
// #[test] 

// Check attributes section at the standard docs (at the bottom) for all the attributes
