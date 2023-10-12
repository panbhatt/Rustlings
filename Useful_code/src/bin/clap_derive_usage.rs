use clap::Parser;
// https://docs.rs/clap/latest/clap/_derive/_cookbook/typed_derive/index.html
fn main() {
    let args = Args::parse();
    println!("ARGS = {:?}", args); 

    match args.debug {
        Some(dbg) => println!("DEBUG is set to TRUE "), 
        None => {}, 
    }

    let no_of_times_to_print = match args.number {
        Some(n) => n , 
        None => 0
    }; 

    for i in 1..=no_of_times_to_print {
        println!("{} -> PANKAJ BHATT", i)
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'D', long = "debug")]
    debug: Option<bool>,

    #[arg(short = 'N', long = "number")]
    number: Option<u32>,
}


// RUN argo run --bin clap_derive_usage --  -D true -N 5 

 // SEE MORE : https://blog.logrocket.com/command-line-argument-parsing-rust-using-clap/