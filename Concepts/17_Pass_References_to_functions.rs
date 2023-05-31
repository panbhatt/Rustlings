

fn main() {

        let country_name = String::from("United State of Kanada"); 
        print_len(country_name); 
        //print_len(country_name); // You can't make this call, as now this code is not the owner of the country_name
        let kandaa_name = String::from("United State of Kanada"); 
        print_len_mul(&kandaa_name); 
        print_len_mul(&kandaa_name); 

        // This is possible, as you are sending a immutable reference to the String. 
}

fn print_len(s : String) {
    println!("{s} length -> {}", s.len());
}

fn print_len_mul(s : &String) {
    println!("{s} length -> {}", s.len());
}

