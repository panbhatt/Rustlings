// Take(0 function of Cell and RefCell.

use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Employee<'a> {
    name: String,
    age: Cell<&'a str>,
    address: RefCell<&'a str>,
}

fn main() {
    println!("{:?} - {:?} ", u32::default(), bool::default());

    let emp = Employee {
        name: "Pankaj Bhatt".to_string(),
        age: Cell::new("Thrity Seven"),
        address: RefCell::new("Delray Beach, Florida, Usa"),
    };

    println!("{:?}", emp.age.take());

    // Two BORROW simulteanously.

    let ref1 = emp.address.borrow_mut();

    // we can try out BORROWING if it works or not.
    if let Ok(mut address) = emp.address.try_borrow_mut() {
        *address = "Boca Raton, Florida, Usa";
    } else {
        println!("ADDRESS ALREADY BORROWED");
    }
    std::mem::drop(ref1);

    println!("{:?}", emp);
}
