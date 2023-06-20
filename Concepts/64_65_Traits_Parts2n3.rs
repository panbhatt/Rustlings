use std::fmt;

// Implementation of custom DISPLAY trait for our own data type.
#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let emp_type = match self.age {
            0..=10 => "kid",
            11..=30 => "Young",
            _ => "Experienced. ",
        };
        write!(
            f,
            "Name is -> {} & Age is -> {}, As per Age he is -> {}",
            self.name, self.age, emp_type
        )
    }
}

fn main() {
    let pankaj = Employee {
        name: "Pankaj Bhatt".to_string(),
        age: 37,
    };

    println!("{:?}", pankaj); // This will print the debug info as it is via DEBUG.
    println!("{}", pankaj); // THis will invoke the fmt Display function by default

    println!("Total Characters - {}", pankaj.to_string().chars().count()); // WIth Display Trait you get a to_String() function
}
