fn main() {
    let number = 998;

    println!("\t Printing escape characters \n ");

    println!("\\t Printing Escape characters \\n");

    // THis will preserver the New line characters
    println!(
        "Print Raw String
    asdf
    This is My Name
    Pankaj Bhatt
    
    "
    );

    // Raw String , Any number of ### Can be used. Has to match to th one we start with.
    println!(
        r###"This is th first thing 
    Name -> Pankaj Bhatt
    Class -> MCA
    dir: -> c:\users\bhatp48
    "###
    );

    // Print in bytes
    println!("{:?}", b"Pankaj Bhatt"); // This will print the bytes.

    // {:X} -> ths will print in hexadecimals.
    println!("{:X}", '₹' as u32);

    println!(
        "{:?}, HEx -> {:X}, Oct -> {:o}, binary -> {:b} ",
        number, number, number, number
    );
}
