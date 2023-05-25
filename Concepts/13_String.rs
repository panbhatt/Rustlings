use std::mem::*; 

fn main(){

    let name = "Pankaj Bhatt"; 
    let mut name_str = String::from(name); // There is another version from_utf8 for which Display trait is not being implemented. 
    name_str.push_str(". I live in FLORIDA and I own a decent home there. "); 

    println!("{} - {}",name,  name_str);
    println!("Name is (converted from &str -> String): {}",name.to_string()); 

    // use SIZE OF
    println!("size of name is : {}", size_of_val(name));
    println!("size of name_str object  is : {}", size_of_val(&name_str));
    println!("size of i32 is : {:?}", size_of::<i32>());
    println!("size of String Object is : {:?}", size_of::<String>());

    // Format Marco
    let full_name = format!("Name is - {name}");
    println!("{full_name}");
}