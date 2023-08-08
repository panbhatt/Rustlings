// https://doc.rust-lang.org/std/mem/index.html  Deals with Memory

use std::mem;

// take - take the value and leave default behind.
// swap  - Replace the value
//replace - take the values out and leave behind the thing you tell it to leave.

#[derive(Debug)]
struct Ring {
    owner: String,
    former_owner: String,
}

fn main() {
    let mut r = Ring {
        owner: "Pankaj Bhatt".to_string(),
        former_owner: "Rahul Gupta".to_string(),
    };

    println!("RING {:?}", r);
    mem::swap(&mut r.owner, &mut r.former_owner);
    println!("RING {:?}", r);

    let replaced_former_owner = mem::replace(&mut r.former_owner, "Ashish".to_string());
    println!("RING {:?}", r);
    println!("\tREPLACED OWNER -> {:?}", replaced_former_owner);

    let owner = mem::take(&mut r.owner); // make owner empty and take its value l
    println!("=============== AFTER TAKE ===================");
    println!("RING -> {:?}", r);
    println!("OWNER -> {:?}", owner);
}
