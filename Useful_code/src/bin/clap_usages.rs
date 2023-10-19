// https://docs.rs/clap/latest/clap/_tutorial/chapter_0/index.html

use clap::{arg, command, value_parser, Arg, ArgAction, Command};
fn main() {
    let matches = command!("First Command")
        .version("0.1.0")
        .author("Pankaj Bhatt")
        .about("Sample Command Checking")
        .arg(
            Arg::new("debug")
                .short('D')
                .long("debug")
                .required(true)
                .help("turns on debugging mode"),
        )
        .arg(arg!(-C --config <CONFIG> "Optionally sets a config file to use"))
        .arg(arg!(-N --number <NUMBER> "Print the message that many times").required(true))
        .get_matches();

    let config_file = matches.get_one::<String>("debug");
    match config_file {
        Some(db) => println!("Value of DEBUG that is given is = {:?}", db),
        _ => println!("NO DEBUG VALUE PASSED"),
    };

    let no_of_times_to_print = matches.get_one::<String>("number");
    if let Some(n) = no_of_times_to_print {
        println!("N = {} ", n);
        match n.parse::<u8>() {
            Ok(m) => {
                for i in 1..=m {
                    println!("{}-> Pankaj Bhatt ", i);
                }
            }
            Err(_) => println!("Number is not an integer, should be an integer and > 0 "),
        }
    } else {
        println!("Number is not provided, so nothing can be printed. ");
    }

    println!("FIRST CLAP APP");
}

// HOW TO RUN.
//cargo run --bin clap_usages -- -h                   // -> TO get HELP 
//cargo run --bin clap_usages --  -D RAM -N 5     -> To Provide option and print it number of times. 
