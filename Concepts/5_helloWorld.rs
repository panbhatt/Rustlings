

fn main() {

    println!("HELLO WORLD "); 
    let name  = "Pankaj Bhatt"; 

    println!("My Name is -> {}", name) ;   // Print Argument. 
    println!("My Name is -> {name}"); // Another Way 

    println!("Age = {} Second Age = {}",giveMyAge(), giveMyAge1());

    let my_age = 37; 
    println!("My Age {} when multiplied by {} = {}",my_age, 3,multiplyMyAge(my_age, 3));  
}

fn giveMyAge() -> i8 {
    return 9; 
}
fn giveMyAge1() -> i8 {
    9  // putting a semicolon at the end is going to give errors. 
}

fn multiplyMyAge(age: i8, by_num :i8) -> u16 {
    let finalAge = age * by_num; 
    finalAge as u16   // TypeCasting in another number. 
}