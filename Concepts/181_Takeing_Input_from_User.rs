use std::io;

// RUn this program from below command 
// rustc 181_Takeing_Input_from_User.rs  && ./181_Takeing_Input_from_User 

fn main() {
    let mut input = String::new();

    while input.trim() != "quit" {
        input.clear();
        println!("Please Enter the string -> ");
        io::stdin().read_line(&mut input).unwrap();
        println!("You wrote -> {:?}", input.trim());
        //input.trim(); // to wipe of \r \n
    }
}
