// Most of the common traits.

use std::fmt::Display; 

fn print_vec<T: Display>(input: &Vec<T>) {
    for iterm in input {
        print!("{} -", iterm);
    }
    println!("");
}

fn main() {
    // FROM TRAIT, it requires from(T) function to be implemented. 
    let nm_vec = Vec::from([8, 9, 10]);
    print_vec(&nm_vec);

    let str_vec = Vec::from("PANKAJ BHATT");
    print_vec(&str_vec);

    let str1_vec = Vec::from("PANKAJ BHATT".to_string());
    print_vec(&str1_vec);
}
