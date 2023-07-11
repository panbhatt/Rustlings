// MUTEX in STRUCT is same as that of REFCELL

use std::sync::Mutex;

#[derive(Debug)]
struct Book<'a> {
    name: Mutex<&'a str>,
    author: Mutex<&'a str>,
    copies_sold: Mutex<u32>,
}

fn main() {
    let my_book = Book {
        name: Mutex::new("Pankaj Bhatt"),
        author: Mutex::new("Andre Agassi"),
        copies_sold: Mutex::new(300000),
    };

    /* If you take two lock on the book object, it would be waiting .
    let my_mutator1 = my_book.name.lock();
    let my_mutator2 = my_book.name.lock();
    */

    // Rest is same as that of
    *my_book.name.lock().unwrap() = "My Autobiography";
    println!("{:?}", my_book);

    let my_mutator1 = my_book.name.lock();
    std::mem::drop(my_mutator1); // This will drop the LOCK.  and another (the line below will take the lock)
    if let Ok(mut my_mutator2) = my_book.name.try_lock() {
        *my_mutator2 = "The Hollow Hills";
    }

    for _ in 0..100 {
        *my_book.copies_sold.lock().unwrap() += 1;
    }

    println!("My Book -> {:?}", my_book);
}
