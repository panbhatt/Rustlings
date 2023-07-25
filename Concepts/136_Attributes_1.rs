// Attributes are nothing but what we put after #[] ,, e.g. dervice is an attribute, that autogenerates the code for us. 

#[derive(Debug, Clone,  PartialEq, Eq, PartialOrd, Ord)]
struct Employee {
    name : String, 
}

fn main() {
    let emp = Employee {
        name : "Pankaj Bhatt.".into(),
    }; 

    println!("{}", emp.name);
}

// we need to check website godbolt.org  to see the compiled assembly file for the soure code. 