//
use std::collections::HashMap;

// ZIP Method()
// zip() combine two iterators in one single iterator.

fn main() {
    let some_numbers = vec![1, 2, 3];
    let some_words = vec!["Pankaj", "Rahul", "Suri"];

    let some_numbers_words = some_numbers
        .iter()
        .zip(some_words.iter())
        .collect::<HashMap<_, _>>(); // This can be anything what you want instead of hashmap, you can have vector and leave it as it is, it will throw error and u will get the TYPE.

    println!("{:?}", some_numbers_words); // This will print hashmap with key numbers and value as string.

    for (key, value) in some_numbers_words {
        println!("{key} - {value}");
    }

    //println!
}
