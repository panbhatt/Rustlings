// DEstructing STRUCT

#[derive(Debug)]
struct Employee {
    name : String, 
    age : u8, 
    happy : bool 
}

fn main() {
    let rgb = (32,255,33); 

    let (r,g, b) = rgb ; // THis is destructing. 

    println!("RED -> {}",r); 
    println!("=================STRUCTURE DESTRUCTURING===================="); 
    let emp = Employee { name : "Pankaj Bhatt".to_string(), age : 37, happy : false }; 

    let Employee { name : nm, age, happy : is_happy, .. } = emp;   // .. i dont care about every structure. 
    println!("NAME -> {} , Is Happy = {}",nm, is_happy); 
}