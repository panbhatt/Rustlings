// CLOSURES

// ANYNYMOUS function

fn main() {
    let mut age = 38;
    // More then one line closure. 
    let myprint = || {
        println!("HELLOW WORLD");
        println!(" OR to universe. I am {} years old", age);
    };

    myprint(); // we are calling my print as a closure.

    let my_print_argument = |input: &str| println!("INPUT IS -> {}", input);

    my_print_argument("PANKAJ BHATT"); // Here input is being passed.
}
