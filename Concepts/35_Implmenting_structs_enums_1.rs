// Giving STRUCTS n ENUMS methods

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
}
// Here we are implementing one NEW() that would be like Static method and two get_* functions.
impl Employee {
    fn new(name: String, age: u8) -> Employee {  // Static/Associated function. 
        Employee { name, age }
    }

    fn new1() -> Self {  // Static/Associated function. 
        Self { name : "XXXX".to_string(), age : 34 }
    }

    fn get_name(&self) -> &str {  // Found bound to the object. 
        return self.name.as_str();
    }
    fn get_age(&self) -> u8 {
        self.age
    }

    fn set_age(&mut self, age: u8) {  // A Method that changes the thing. 
        self.age = age

    }
}

fn main() {
    let mut me = Employee::new("Pankaj Bhat".to_string(), 38);
    println!("Emp Name : {} Age : {}", me.get_name(), me.get_age());
    println!("======================================"); 
    me.age = 40 ; 
    println!("Emp Name : {} Age :  {}", me.get_name(), me.get_age());

    println!("======================================"); 
    let friend = Employee::new1(); 
    println!("Emp Name : {} Age : {}", friend.get_name(), friend.get_age());
}
