

// This is an MODULE although private unless PUB is being used. 
mod print {
    use std::fmt::Display; 

    pub fn print_me<T: Display> (input : T) {
        println!("MOD PRINT ->  {}", input);
    }
}

// This is an MODULE although private unless PUB is being used. 
mod play {
    pub fn kick(input : i8) -> i16{
        let kick_value = input as i16 * 2;
        return kick_value; 
    }
}

fn main() {

    crate::print::print_me("Pankaj Bhatt"); 

    let kick_value = crate::play::kick(100); 
    println!("Getting value from KICK = {}", kick_value); 

}