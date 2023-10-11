fn main() {
    let mut my_ages = vec![50, 20, 30, 40, 10];
    my_ages.sort();
    println!("Ages = {:?}", my_ages);
    my_ages.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Reverse Ages = {:?}", my_ages);

    let mut my_salaries = vec![10.5, 23.4, 34.4, 11.2];
    my_salaries.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("My Salaries = {:?}", my_salaries);

    println!("++++++++++++++  SORTING OF STRUCUT ++++++++++++++++");
    let mut my_friends = vec![
        Person {
            name: "Pankaj Bhatt".to_string(),
            age: 38,
        },
        Person {
            name: "Rahul Gupta".to_owned(),
            age: 40,
        },
        Person {
            name: "Surinder kundal".to_owned(),
            age: 43,
        },
        Person {
            name: "Amit".to_string(),
            age: 30,
        },
    ];

    my_friends.sort();
    println!("FRIENDS = {:?}", my_friends);

    my_friends.sort_by(|a, b| b.cmp(a));
    println!("DESCENDING ORDER -> FRIENDS = {:?}", my_friends);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
