fn main() {

    // All the Primitive types like Bool, Char, int, float had a copy trait, so they can be copied. 
    
    // In Rust ownership model, we have only ne owner of a object being created. 
    let name = String::from("Pankaj Bhatt");
    //let name_copy = name ;   // THis is not a valid statement, only one owner is allowed. 

    let name_copy = name.clone() ;  // Creates a clone i..e brand new copy. 

    println!("{name} - {name_copy}"); 

    let my_name = String::from("Pankaj Bhatt from FLORIDA");
    takesOwnership(my_name.clone()); 

    // println!("My State name is {my_name}");  THis wont work as ownreship is passed. 

    let my_new_name = takesOwnership(my_name); 

    println!(" Length of my new name = {}", calculate_length(&my_new_name) ); // This is not taking ownership via using reference. 

    let mut my_state_name = String::from("Florida ");  // Here we are passing an mutable reference to a variable. Only 1 mutable reference is allowed. 
    add_love_to_me(&mut my_state_name); 
    println!("{my_state_name}"); 

    // Dangling pointer.. below code wont' be able to compile, because in dangle country method S will go out of scope as soon as the function ends. 
    /*let country = dangle_country() 
    println!("{country}");*/

    // STRING SLICES
    let country_name = String::from("United States of America"); 
    let first_word = &country_name[0..7]; 
    let last_word = &country_name[17..]; 

    println!("{first_word} {last_word}");

    // SImilarly you can have array slices. 

    let us_counter_name = "United State Of AMerical";
    println!("{} " , str_slice(us_counter_name))

}

fn takesOwnership(state : String) -> String {
    println!("takeOwnership -> state : {state}");  

    state
}


fn calculate_length(state : &String) -> usize {  // References are immutable, here we are borrowing the thing.  only one owner
    state.len()

}

fn add_love_to_me(state : &mut String) {
    state.push_str(" love")
}

/*fn dangle_country() -> &String{
    let s = String::from("USA")
    return &s
}*/

fn str_slice(short_name : &str) -> &str {

    &short_name[0..10]
}