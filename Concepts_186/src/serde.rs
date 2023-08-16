// Paste this file in main.js and do cargo run. 

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    name: String,
    age: u8,
    is_employed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Employee_Request {
    name: String,
    age: u8,
}

fn parse_Employee(raw_str: &str) -> Result<Employee, String> {
    match serde_json::from_str::<Employee_Request>(raw_str) {
        Ok(good_request) => Ok(Employee {
            name: good_request.name,
            age: good_request.age,
            is_employed: false,
        }),
        Err(e) => {
            println!("Error occured while parsing");
            Err(e.to_string())
        }
    }
}

fn main() {
    let good_employee = r#"
        {
            "name" : "Pankaj Bhatt", 
            "age" : 38
        }    
            "#;

    let bad_employee = r#"
        {
            "name" : "Pankaj Bhatt", 
            "age2" : 38
        }    "#;

    let emp = parse_Employee(good_employee);
    match emp {
        Ok(val) => println!("{val:#?}"),
        Err(val) => println!(" ERROR -> {val:?}"),
    }

    println!("============================");
    let emp = parse_Employee(bad_employee);
    match emp {
        Ok(val) => println!("{val:#?}"),
        Err(val) => println!(" ERROR -> {val:?}"),
    }
}
