

fn main() {

     
        let mut kandaa_name = String::from("United State of Kanada, "); 
        print_len_and_add(&mut kandaa_name); 
        print_len_and_add(&mut kandaa_name); 

       let   country_name = String::from(" Canada ");
       add_last_name(country_name);  // Surprising this will work, because you are not using country_name anymore in this code blocks, if you enable the below line, it wont work 
       //println!("{}", country_name);
}


fn print_len_and_add(s : &mut String) {
    println!("{s} length -> {}", s.len());
    s.push_str(" Ontario "); 
}

fn add_last_name(mut name : String) {
    name.push_str("Kanada"); 
}

