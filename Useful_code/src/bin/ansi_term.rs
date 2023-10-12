use ansi_term::{Colour, Style};

fn main() {

    // This will print COLORFUL TEXT 
    println!("RED -> {} BLUE -> {} GREEN -> {}", Colour::Red.paint("RED"), Colour::Blue.paint("BLUE"), Colour::Green.paint("GREEN"));

    // This wll print BOLD. 
    println!("{} and this is not", Style::new().bold().paint("PANKAJ BHATT")); 

    // BOLD and COLORED IN TERMINAL
    println!("{}", Colour::Red.bold().paint("RED AND BOLD"));

}