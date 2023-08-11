// WOrking with arguments passed by user.
use std::env::args;
use std::io;

// RUn this program from below command
// rustc 182_Working_with_args.rs  && ./182_Working_with_args capital hello there  -> Turn in uppercase. 
// rustc 182_Working_with_args.rs  && ./182_Working_with_args lowercase HELLO THERE -> turn into lowercase 

#[derive(Debug)]
enum Actions {
    Capital,
    LowerCase,
    Nothing,
}

fn main() {
    let mut action = Actions::Nothing;
    let input = args().skip(1).collect::<Vec<String>>();

    if input.len() >= 2 {
        match input[0].as_str() {
            "capital" => action = Actions::Capital,
            "lowercase" => action = Actions::LowerCase,
            _ => {}
        };
    }
    println!(" Action = {:?} ", action);

    for word in input.iter().skip(1) {
        match action {
            Actions::Capital => println!("{} -> {}", word, word.to_uppercase()),
            Actions::LowerCase => println!("{} -> {}", word, word.to_lowercase()),
            _ => println!("SAME -> {}", word),
        }
    }
}
