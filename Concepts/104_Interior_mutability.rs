// INTERIO MUTABILITY

// Imagine we have a struct PHONE that has some attributes, and one is on_sale that can be flipped to TRUE/FALSE depending on the condition,
// Now, if we want to implement a TRAIT that modifies the interior state of the PHONE object, it's not good, so we need CELL.

#[derive(Debug)]
struct Phone {
    name: String,
    on_sale: bool,
}

impl Phone {
    fn put_on_sale(&mut self) {
        self.on_sale = true;
    }

    fn remove_from_sale(&mut self) {
        // it we are modifying the value and the variable needs to be mut
        self.on_sale = false;
    }
}

fn main() {
    let mut ph = Phone {
        name: "Nokia".to_string(),
        on_sale: false,
    };

    ph.put_on_sale();
    println!("{:#?}", ph);
}
