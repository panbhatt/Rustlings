// Rule 1. 
// By default the compiler is going to give EveryARgument of a function a lifetime. 

/*fn return_str(input1 : &str, input2 : &str) -> &str {
    input1 
}*/

// THE Above code wont work, because of life unpredictable. 
fn return_str<'a>(input1 : &'a str, input2 : &'a str) -> &'a str {
    input1 
}

// Rule 2 -> if there is only one input , it works, as it automatically assigned the lifetime. 
fn return_str1(input1 : & str) -> & str {
    input1 
}

// Rule 3 -> if the first parameter is self then all the other are given the same lifetime as that of self. 

fn main() {
    println!("{}", return_str("HELLO", "WORLD   ")); 
    println!("{}", return_str1("UNIVERSE")); 
}