// REFCELLS are a combination of
// cells -> Through which we can modify things in an immutable struct object
// Rc -> Reference count.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // You can use drop instead of std::mem::drop as it is preluded already. like println! and other macro.

    // RefCell
    let rfc = Rc::new(RefCell::new(String::from("Pankaj Bhatt")));
    // it means, we are creating a string that can be modified and have multiple pointers point ot it. s
    modify_refcell_string(Rc::clone(&rfc)); // This will increase the count to 2.
    println!(
        "Total number of pointers/owners point of String -> {}",
        Rc::strong_count(&rfc)
    );
    println!("String is -> {:?}", rfc);
}

fn modify_refcell_string(input: Rc<RefCell<String>>) {
    let mut rc_string = input.borrow_mut();
    println!(
        "Total number of pointers/owners point of String -> {}",
        Rc::strong_count(&input)
    );
    rc_string.push('@');
}
