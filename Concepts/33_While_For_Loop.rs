fn main() {
    let mut counter = 1;

    println!("++++++++++++ WHILE LOOP ++++++++++++++++");
    while counter < 10 {
        print!("{}", counter);
        counter += 1;
    }

    println!("");
    println!("++++++++++++ FOR LOOP EXCLUSIVE ++++++++++++++++");

    // For Loop with Range
    for number in 1..5 {
        // This is EXCLUSIVE RANGE.
        print!(" {}", number);
    }

    println!("");
    println!("++++++++++++ FOR LOOP INCLUSIVE ++++++++++++++++");

    for number in 1..=5 {
        // This is INCLUSIVE RANGE.
        print!(" {}", number);
    }

    println!("");
    for _ in 0..3 {
        // _ it is like I am not going to use it. , you can laos put _ in front of the identifier.
        println!(" Just Printing");
    }
}
