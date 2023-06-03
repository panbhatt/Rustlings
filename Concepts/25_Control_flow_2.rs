// MOre Variations of Match

fn main() {
    let red = (200, 12, 0);
    let green = (12, 200, 0);
    let blue = (0, 12, 200);

    match_colors(red);
    match_colors(green);
    match_colors(blue);

    let my_age = 10;
    let is_old = if my_age > 35 { "old" } else { "young" }; // we can't return mutliple values like int/string from this.

    println!("Old or Young ->  {is_old} ");

    println!("==============================================================");
    // Use of a variable in one of the IF CLAUSE of the MATCH
    match my_age {
        age @ 4 if age % 2 == 0 => println!("Age ->  {} is an EVEN Number", age),
        age @ 9 if age % 2 != 0 => println!("Age ->  {} is an ODD Number", age),
        age => {
            println!("Unknown number {age} "); // This will get printed since the number is neither 4 or 9.
            println!("Second println statement in the block. ");
        }
    };

    println!("==============================================================");
}

// Just demonstrating that we can use tuples and we can break it down in multiple variables i..e destructing.
fn match_colors(rgb: (u8, u8, u8)) {
    match rgb {
        (r, _, _) if r > 100 => println!("Very large RED"),
        (_, b, _) if b > 100 => println!("Very large BLUE"),
        (_, _, g) if g > 100 => println!("Very large GREEN"),
        _ => println!("Default Case "),
    };
}
