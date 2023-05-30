
fn main() {

    let name = get_name(); 
    println!("{}", name);


}

// This is not going to be allowed, unless we change the function signature from &str to &'static str
//fn get_name() -> &str {
    //fn get_name() -> &'static str {    // Even this is not going to work. 
fn get_name() -> String {  // This would work as we are transferring the ownership from one function to another. 
    let name = String::from("Pankaj Bhatt"); 

    //return &name; 
    return name;   


}