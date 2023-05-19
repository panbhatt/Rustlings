

fn main() {

    let start_counter = {   // A Variable value can be calculated while it is being declared. 
        let initial_counter = 1; 
        let final_counter  = 5; 

        final_counter - initial_counter
    };

    println!("Counter  = {start_counter}");

    let my_age = {
        let father_age = 70; 
        father_age / 2 ;   // This code wont' work, you have to remove semicolon to understand the empty values. 
    }

    println!("My Age based on my father's age is = {} " , my_age);
}