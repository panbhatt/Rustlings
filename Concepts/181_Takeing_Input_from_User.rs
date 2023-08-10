use std::env::args;
use std::io;

// RUn this program from below command
// rustc 181_Takeing_Input_from_User.rs  && ./181_Takeing_Input_from_User

fn main() {
    let mut input = String::new();

    print_args(); 

    println!("======= Start Entering Data ====="); 

    while input.trim() != "quit" {
        input.clear();
        println!("Please Enter the string -> ");
        io::stdin().read_line(&mut input).unwrap();
        println!("You wrote -> {:?}", input.trim());
        //input.trim(); // to wipe of \r \n
    }
}

fn print_args() {
    let cmd_args = args().skip(1); ;  // To remove the name of the program . 
    println!("======= Command Line Arguments ====="); 
    for entry in cmd_args {
        println!("{:?}", entry);
    }
}
