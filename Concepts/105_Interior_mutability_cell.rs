// INTERIO MUTABILITY
use std::cell::Cell;

// This is just another version of the CELL where we put it in an IMMUTABLE struct and we can modify it.

#[derive(Debug, Clone)]
struct Phone {
    name: String,
    on_sale: Cell<bool>,
    discount: Cell<u8>,
}

fn main() {
    let nokia_phone = Phone {
        name: "Nokia".to_string(),
        on_sale: Cell::new(false),
        discount: Cell::new(0),
    };

    println!("Phone {:#?}", nokia_phone);
    nokia_phone.on_sale.set(true);
    nokia_phone.discount.set(10);
    println!("After Discount ===== > ");
    dbg!(nokia_phone.clone());

    println!("On Sale -> {}", nokia_phone.on_sale.get());
    println!("Discount ->  {}", nokia_phone.discount.get());
}
