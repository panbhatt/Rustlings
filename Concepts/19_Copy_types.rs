//  If someone implements COPY types, that means you can copy it.. it is a Trait implementation. String has a clone function. 
// but the small types copy has been implemented. 


fn print_me(num : i32) {
    println!("{}", num);
}

fn main() {
    let age = 29; 
    let age1 = age; 

    print_me(age); 
    print_me(age1);

    // This is possible only because age1 is a copy of age. in string it can't be done and u have to use .Clone() functionality. 
}