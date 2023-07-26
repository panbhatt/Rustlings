use std::ops::{Deref, DerefMut};

// we are implementing a DEREF for an EMployee that will automatically deref in its AGE.

// https://fongyoong.github.io/easy_rust/Chapter_59.html

#[derive(Debug)]
struct Employee {
    age: u32,
    name: String,
}

impl Deref for Employee {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.age
    }
}

// This means
impl DerefMut for Employee {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Target is being used from above type Target = u8;
        &mut self.age
    }
}

fn main() {
    let mut emp = Employee {
        age: 38,
        name: "Pankaj Bhatt".into(),
    };

    let ref_emp = &emp;
    let refref = &&&&&&&&&&&&ref_emp;

    println!("DEREF object {} , after 20 years {}", *emp, *emp + 20); // This is DEREF

    println!("Age Square ->  {:?} ", ref_emp.pow(2)); // This means it will by default deref to iits age.

    println!("NAME is Name -> {} Age -> {:?}", refref.name, ref_emp.age); // This will give the name

    *emp = 100; // Because of DEREF it would assign 100 to the age .
    println!(" Added change to -> {}", *emp);
}
