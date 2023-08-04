// Strings are same as that of VEC.

fn main() {
    let mut my_string = String::new();

    println!("Capacity = {}", my_string.capacity());

    for i in 1..=10 {
        my_string.push('$');
        my_string.push_str(" / ");
    }

    println!(
        "New Capacity = {}, Char count = {}",
        my_string.capacity(),
        my_string.chars().count()
    );

    let mut my_new_String = String::with_capacity(64); // We are creating a string with the initial capacity.

    println!("Capacity = {}", my_new_String.capacity());

    for i in 1..=10 {
        my_new_String.push('$');
        my_new_String.push_str(" / ");
    }

    println!("New Capacity = {}", my_new_String.capacity());
    my_new_String.shrink_to_fit(); // Reduce capacity to equal to char count.
    println!("New Capacity = {}", my_new_String.capacity());

    // POP the last character of the string.
    let mut my_name: String = "pankaj Bhatt".into();
    loop {
        let pop_char = my_name.pop();
        match pop_char {
            Some(character) => print!("{}", character),
            None => {
                break;
            }
        };
    }

    // RETAIN (it is just like filter for MAP)
    let mut simple_string = String::from("Age : 28, HEIGHT = 38 Weight = 35");
    simple_string.retain(|ch| ch.is_alphabetic() || ch == ' ');
    println!("\nAfter Retaining only alphabetic : {} ", simple_string);
}
