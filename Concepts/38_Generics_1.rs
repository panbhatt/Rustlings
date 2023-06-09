// First Part of GENERICS of Easy Rust.
// Using <> , T/U/V can be used.

// At Run time, it would create that many variants of the functions. POLYMORPHISM.

fn return_thing<T>(thing: T) -> T {
    //    println!("Your number is {:?}", thing);  This would not work as we dint restrict the T with std:;fmt::Debug i.e TRAIT applies for T
    println!("Something has been given");
    thing
}

fn main() {
    let number = return_thing(32);
    let my_name = return_thing("HELLO".to_string());
}
