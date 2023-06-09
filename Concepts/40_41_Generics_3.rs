use std::fmt::{Debug, Display};

use std::cmp::PartialOrd;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(thing: T) {
    println!("{:?}", thing);
}

//fn compare_and_display<T:Display, U:Display + PartialOrd, V : Debug>(stmt : T, num_1 : U, num_2 : U, animal : V)   // This definition has been moved to another place in where.
fn compare_and_display<T: Display, U, V>(stmt: T, num_1: U, num_2: U, animal: V)
where
    U: Display + PartialOrd,
    V: Debug,
{
    let mut result = false;
    if num_1 > num_2 {
        result = true;
    }

    println!(
        "{}! {} Is greater then {} ? {} ",
        stmt, num_1, num_2, result
    );
    println!("By the way I have an animal too : {:?}", animal);
}

fn main() {
    let dog = Animal {
        name: "Dog".to_string(),
        age: 8,
    };

    let age = 30;

    print_item(dog);
    print_item(age);

    let cat = Animal {
        name: "Charlie".to_string(),
        age: 2,
    };

    compare_and_display("HELLO", 10, 5, cat); // DOG can't be passed here, as its ownership is already passed to the function previously.
}
