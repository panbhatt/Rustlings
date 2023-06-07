// Returning Value from the LOOP.

fn main() {
    let mut counter = 1;

    let value = loop {
        if counter >= 100 {
            break counter; // This way we can return the value of the counter variable to the variable.
        }
        counter += 1;
    };

    println!("{}", value);

    let names: Vec<String> = vec!["ABC".to_string(), "XYZ".to_string()];

    // ITERATING through the VECTOR via a loop.
    for name in names {
        println!(" -> {}", name);
    }
}
