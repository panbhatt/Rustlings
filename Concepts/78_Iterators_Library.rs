#[derive(Debug)]
enum LibraryType {
    City,
    Town,
    Country,
}

#[derive(Debug)]
struct Library {
    lib_type: LibraryType,
    books: Vec<String>,
}

impl Library {
    fn new() -> Library {
        Self {
            lib_type: LibraryType::City,
            books: vec![],
        }
    }

    fn add_books(&mut self, name: String) {
        self.books.push(name);
    }
}

// This is implementation of ITERATOR for the books.
impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Option<String>
        match self.books.pop() {
            Some(book_name) => Some((book_name + " is found. ")),
            None => None,
        }
    }
}

fn main() {
    let mut doon_library = Library::new();

    doon_library.books.push("Pankaj Story".to_string());
    doon_library.books.push("Rahul Story".to_string());
    doon_library.books.push("Namit  Story".to_string());

    println!("Printing all the libraries and their books");
    println!(
        "{:?} contains Books {}",
        doon_library.lib_type,
        doon_library.books.len()
    );

    println!("Books -> ");
    for book in doon_library {
        println!("\t{}", book);
    }
}
