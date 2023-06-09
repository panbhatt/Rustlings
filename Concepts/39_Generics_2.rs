use std::fmt::Debug;
use std::fmt::Display;

fn print_number<T: Display + Debug>(number: T) -> T {
    // Only those type that implements Display & Debug
    println!("Number: {:?}", number); // Because this wants to make sure that this can be printable via Trait Display.
    number
}

fn main() {
    print_number(10);
    print_number(259);

    print_number(259.5656);
    // print_number(Book { year : 2023}) ; // This wont work as Display not being implemented for the Book
}

struct Book {
    year: i8,
}
