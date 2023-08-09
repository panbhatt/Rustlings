// Create struct via MACRO

// tt -> token tree
// ident -> identifier
// ty -> A Type e.g. String, usize, Vec<u8>

/*struct Employee {
    name: String,
}*/

// This Macro, create a struct for you at run time.

macro_rules! create_struct {
    ($name : tt, $field : ident, $type_of : ty) => {
        #[derive(Debug)]
        struct $name {
            $field: $type_of,
        }
    };
    ($name : tt, $($field : ident, $type_of : ty),*) => {  // This will receive multiple types of inputs.
        #[derive(Debug)]
        struct $name {
            $($field: $type_of), *
        }
    };
}

create_struct!(Employee, name, String); // So we are creating a STRUCT at runtime.
create_struct!(Book, name, String, pages, u16); // So we are creating a STRUCT at runtime.

fn main() {
    let my_employee = Employee {
        name: "Pankaj Bhatt".into(),
    };

    println!("Employee -> {:?}", my_employee);

    let my_book = Book {
        name: "Andre Agassi".into(),
        pages: 342,
    };

    println!("Book -> {:?}", my_book);
}
