fn main() {
    let mut x = 6;
    x = 10;
    println!("X = {x} ");
    let x = "Pankaj";
    println!("X = {x} ");

    const SUBSCRIBER_COUNT: u32 = 100_000;

    println!("Subscriber count = {SUBSCRIBER_COUNT}");

    // Tuple  // decstructured , . dot
    let person = ("Pankaj Bhatt", 28, "ONTARIO, FLORDIA");
    println!(
        "Name = {} , age = {} Address = {}",
        person.0, person.1, person.2
    );
    let (name, age, address) = person;
    println!("Name = {} , age = {} Address = {}", name, age, address);

    // Array  , repeating values
    let names = ["Pankaj", "Team", "FX", "Furious"];
    println!(
        "First name = {} last Name = {}",
        names[0],
        names[names.len() - 1]
    );
    let repeat_ar = ["Pankaj"; 15];
    println!(
        "First name = {} last Name = {}",
        repeat_ar[0],
        repeat_ar[repeat_ar.len() - 1]
    );

    // function
    let x1 = 43;
    let y1 = 10;
    let sum = add_two_numbers(x1, y1);
    let msg = format!("sum of x and y is {}", sum);
    println!("Message  = {}", msg);

    // --------------------------------------------------
    // Control Flow
    let senior: bool;
    if age > 30 {
        senior = true;
    } else {
        senior = false;
    }

    println!(" is this a Senior Guy = {}", senior);

    let long_address = if address.chars().count() > 5 {
        "Long Address"
    } else {
        " Short Address"
    };
    println!(" Pankaj's Address is {}", long_address);

    // --------------------------------------
    // Loop

    // 1. Loop
    let mut ctr = 1;
    println!(" LOOP example  ->");
    loop {
        print!(" {} ", ctr);
        ctr += 1;
        if ctr >= 9 {
            break;
        }
    }

    println!("\n While loop  -> \n");
    while ctr > 0 {
        print!(" {} ", ctr);
        ctr -= 1;
    }

    println!(" \n FOR LOOP -> ");
    for name in names.iter() {
        print!("{name} ");
    }
    println!("\n");

    for elem in 1..5 {
        print!(" {} ",elem) ; 
    }
}

fn add_two_numbers(x: i32, y: i32) -> i32 {
    println!(" x = {y} y = {y} ");
    return x + y; // or just x+y
}
