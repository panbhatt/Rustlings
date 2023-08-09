// Macro looks like an Match statement . this is something that write codes automatically. on what you give it.

// Very first Marc, simple macro without any input
macro_rules! give_six {
    () => {
        6
    };
}

// Macro with Hardcoded input.
macro_rules! six_or_print {
    (6) => {
        6 // REturning i32
    };
    () => {
        println!("------------------->>>>>> Nothing passed"); // returning empty ()
    };
}

macro_rules! might_print {
    (Panka Bhatt My token) => { println!("Guessed right") ; 1 } ;
    () => { 0 } ;
}

fn main() {
    let six = give_six!();
    println!(" Returned value is -> {}", six);
    println!(" SIX or EMPTY -> {:?} ", six_or_print!(6));
    println!(" SIX or EMPTY -> {:?} ", six_or_print!());

    println!("==================================");
    might_print!(Panka Bhatt My token);
    might_print!();
}
