//  TRAITS == POWERS. Add, debug, copy , clone

#[derive(Debug)]
struct Employee {
    name: String,
    age: i32,
}

// has to be VERB or Adjectives
// IN this case, this trait tells that it is going to be a senior employee
trait Senior {
    fn print(&self) {
        println!(" Seniors can print as much as paper available. ");
    }

    fn fly(&self) {
        println!(" Can Fly too ");
    }

    //fn run(&self); // if u leave it empty, u have to override it for sure.
}

impl Senior for Employee {
    fn fly(&self) {
        println!("{} -  Can Fly too ", self.name);
    }
} // THis is how we are implementing TRAIT SENIOR for an EMPLOYEE, we can also override the functions.

fn main() {
    let me = Employee {
        name: String::from("Pnakaj Bhatt, "),
        age: 37,
    };

    me.print(); // This is a trait that comes
    me.fly(); // This wil print the name as the TRAIT function has been overridden.
}
