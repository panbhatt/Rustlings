// This will demonstrate how first we can implement ENUM as Traints, so we will go wtih Traits first.

trait GiveName {
    fn give_name(&self) -> &'static str;
}

struct One;
struct Two;

impl GiveName for One {
    fn give_name(&self) -> &'static str {
        "ONE"
    }
}

impl GiveName for Two {
    fn give_name(&self) -> &'static str {
        "TWO"
    }
}

fn get_name<T: GiveName>(input: T) -> &'static str {
    input.give_name()
}

fn main() {
    let one = One {};
    let two = Two {};

    println!("ONE GiveName Returns :> {} ", get_name(one));
    println!("TWO GiveName Returns :> {} ", get_name(two));
}

/*
This will give output

ONE GiveName Returns :> ONE 
TWO GiveName Returns :> TWO 

Now we have to implement the same via ENUM in Part 2. 


*/
