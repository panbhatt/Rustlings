// INTERIO MUTABILITY
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};

// Exactly same as Cell, but just that at run time dynamically borrowing rules are checked.

#[derive(Debug, Clone)]
struct Phone {
    name: String,
    on_sale: RefCell<bool>,
    discount: RefCell<u8>,
}

fn main() {
    let nokia_phone = Phone {
        name: "Nokia".to_string(),
        on_sale: RefCell::new(false),
        discount: RefCell::new(0),
    };

    dbg!(nokia_phone.clone());
    let mut phone_on_sale_reference = nokia_phone.on_sale.borrow_mut();
    *phone_on_sale_reference = true;

    println!("After Borrow -> {:#?}", nokia_phone); // This will show that it is borrowed.
                                                    // we need to use std::mem::drop to drop the reference after we borrowed, it and we can't have multiple borrows at once.
    std::mem::drop(phone_on_sale_reference);

    println!("Return Borrow -> {:#?}", nokia_phone);

    *nokia_phone.discount.borrow_mut() = 20; // This is another shortcut that saves three line.

    println!("Return Borrow On Discount -> {:#?}", nokia_phone);
}
