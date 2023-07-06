// LIFETIMES -> how long a variable is going to live.

fn main() {
    let my_name = "Pankaj Bhatt";

    let my_stati_name: &'static str = "Pankaj Bhatt"; // Remains during the lifetime of the program. ie. why static.

    prints(my_name);
    prints(&String::from("RAHUL CHUTIYA")); // This works because of DEREF is implemented in String i.e. why PRINTS takes it.
}

fn prints(s: &str) {
    // takes both &str and String.
    println!("{}", s);
}
