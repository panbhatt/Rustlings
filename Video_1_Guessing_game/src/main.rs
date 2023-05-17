use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*; 

fn main1() {
    println!("Hello, world!, this is first progrlam");
    println!("First THING");
    let ram = 12i32;
    let ss = format!("Hello  {} ", ram);
    println!("{}", ss);
}

fn main() {
    let rang_numbers = rand::thread_rng().gen_range(1..101);
    

    println!("Generating random numbers = {rang_numbers}");

    loop {
        println!("Please input a number ");
        let mut inputString = String::new();

        io::stdin()
            .read_line(&mut inputString)
            .expect("failed to read input string");
        println!("YOur Guessed number is {inputString}");

        let gnum: u32 = match inputString.trim().parse() {
            Ok(num ) => num,
            Err(err) => {
                println!("An Error occurred while parsing, {err}");
                continue;
            }
        };

        match rang_numbers.cmp(&gnum) {
            Ordering::Less => println!("{}", "LESS".red()),
            Ordering::Equal => {
                println!("{}", "Equal".green());
                break;
            }
            Ordering::Greater => println!("{}", "Greater".red()),
        }
    }
}

fn Greet(name: String) {
    println!("Hello {}", name);
}
