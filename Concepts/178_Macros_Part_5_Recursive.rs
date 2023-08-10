//  Macro that can call itself. like dbg, println format (calls format_args!)

macro_rules! my_macro {
    () => {
        println!("I am not printing anything ");
    };
    ($input : expr) => {
        my_macro!();
    };
    ($($input : expr), *) => {
        my_macro!();
    };
}

fn main() {
    my_macro!();
    my_macro!("Pankaj");
    my_macro!("Pankaj", "Bhatt");
    my_macro!(8,9);
}

// These all are going to print the value same line as they are calling itself locally.
