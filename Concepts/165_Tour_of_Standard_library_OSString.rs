// String is of different types 
// String, &str, CString, CStr, OsString, OsStr

use std::ffi::{OsString, CString} ; 

fn main() {
    let os_string = OsString::from("MY Name is  Pankaj Bhatt"); 
    match os_string.into_string() {
        Ok(val) => println!("String -> {:?}", val), 
        Err(invalid_valu) => println!("Invalid Value string -> {:?}", invalid_valu)
    }; 

    // CStr -> reference any unsafe string
    // unsafe -> keep people cautious
    unsafe {
        let cString = CString::new("Pankaj bhatt"); 
        println!("{:?}", cString);
    }
}