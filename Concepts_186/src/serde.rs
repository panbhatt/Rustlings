use serde;
use serde_json;

#[derive(Debug, Serialize, Deserialized)]
struct Employee {
    name: String,
    age: u8,
    is_employed: bool,
}

#[derive(Debug, Serialize, Deserialized)]
struct Employee_Request {
    name: String,
    age: u8,
}

fn parse_Employee(raw_str: &str) -> Result<Employee, serde::Error> {
    match serde_json::from_str(raw_str) {
        Ok(good_request) => Ok(Employee {
            name: good_request.name,
            age: good_request.age,
        }),
        Err(e) => {
            println!("Error occured while parsing");
            Err((), e)
        }
    }
}

fn main() {
    let good_employee = r#"
        {
            "name" : "Pankaj Bhatt", 
            "age" : 38, 
        }    
            "#;

    let bad_employee = r#"
        {
            "name" : "Pankaj Bhatt", 
            "age" : 38, 
            "is_empldoyed" : true
        }    
            "#;

    let emp = parse_Employee(good_employee);
    println!("{}", emp);
}
