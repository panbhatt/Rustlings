// RWLOCk, almost same as that of MUTEX.

// MUTEX in STRUCT is same as that of REFCELL

use std::sync::RwLock;

// Functions there are
// lock() - same as that of MUTEX
// read() - as like reference, you can have multiple READ
// write() - only one reference

#[derive(Debug)]
struct Book<'a> {
    name: RwLock<&'a str>,
    author: RwLock<&'a str>,
    copies_sold: RwLock<u32>,
}

fn main() {
    let my_book = Book {
        name: RwLock::new("Pankaj Bhatt"),
        author: RwLock::new("Andre Agassi"),
        copies_sold: RwLock::new(300000),
    };

    // Getting Read Lock
    let name = my_book.name.read().unwrap();
    let author = my_book.author.read().unwrap();
    let copies_sold = my_book.copies_sold.read().unwrap();

    std::mem::drop(name);
    std::mem::drop(author);
    std::mem::drop(copies_sold);

    println!("Book -> {:?} ", my_book);

    // Getting WRITE Lock, once done, after no one can take the lock
    let mut name_w = my_book.name.write().unwrap();
    let mut author_w = my_book.author.write().unwrap();
    let mut copies_sold_w = my_book.copies_sold.write().unwrap();

    *name_w = " Pankaj Bhatt ";
    *author_w = "Jim Bhatt";

    std::mem::drop(name_w);
    std::mem::drop(author_w);
    std::mem::drop(copies_sold_w);

    println!("Book -> {:?} ", my_book);
}

// used Same as that of MUTEX, only when we want to take READ lock too.
