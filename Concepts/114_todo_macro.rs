// TO DO macro 
// todo! is short form for unimplemented!
// Means we can write todo!() and leave that fn empty (but make sure it is not being called, otherwise todo macro will PANI. 

// Used, just for fill in purposes, so that we can work on it later on. iit is used fo PROTOTYPING. s

enum BookType {
    HardCover,
    SoftCover, 
}

#[derive(Debug)]
struct Book {
    name : String, 
}

impl Book {
    fn new(name : String) -> Book {
        Book {
            name
        }
    }
}

fn get_book(bookId : u32) -> Option<Book> {
    todo!()
}

fn delete_book(bookId : u32) -> Result<bool, u32> {
    unimplemented!()
}

fn main() {
    let shakespeare_book = Book {
        name : "Hamlet".to_string(), 
    };

    println!("{:?}", shakespeare_book);
}