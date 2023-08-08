// Function pointer with Struct.

// An Employee struct whose Salary can be generate from either private or public.


use std::fmt::Display;

#[derive(Debug)]
struct Employee<T> {
    name: String,
    get_salary: fn(f64) -> f64,
    age: u8,
    msg_generator : fn(T)
}

fn generate_salary_private(age: f64) -> f64 {
    ((age * 1.5) as f64 + (age / 5.0 * 2.0))
}

fn generate_salary_public(age: f64) -> f64 {
    (age * 2.0) + (age / 5.0 * 1.0)
}

fn print_msg<T : Display>(msg : T)  {
    println!("Generated Message  {}", msg); 
}

fn main() {
    let priv_employee : Employee<String> = Employee {
        name: "Pankaj Bhatt".to_string(),
        get_salary: generate_salary_private,
        age: 40,
        msg_generator : print_msg
    };

    let public_employee : Employee<f64> = Employee{
        name: "Kuputra Gupta".to_string(),
        get_salary: generate_salary_public,
        age: 30,
        msg_generator : print_msg
    };

    println!(
        "Private Emp Salary - {} ",
        (priv_employee.get_salary)(f64::from(priv_employee.age))
    );

    println!(
        "Public Emp Salary - {} ",
        (public_employee.get_salary)(f64::from(public_employee.age))
    );

    // Using Generics to print the message. 
    (priv_employee.msg_generator)("I live in Florida".to_string()); 

    (public_employee.msg_generator)(56787.89); 
}
