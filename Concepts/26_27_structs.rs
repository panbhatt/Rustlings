// Three kind of structs in RS. 

struct FileDirectory; // unit struct. doesn't have anything 

#[derive(Debug)]
struct Color(u8,u8,u8) ; // TUPLE struct, i.e NAME TUPLE.

// Named struct
#[derive(Debug)]
struct Employee {
    name : String,
    age : u8,
    eye_color : Color
}

#[derive(Debug)]
struct Country {
    population : u64, 
    capital : String, 
    leader_name : String, 
}

fn main() {

    let my_dir = FileDirectory;
    let some_colors = Color(10,230,190); 
    println!("{:?}", some_colors); 
    println!("First Color - Red Value is {}", some_colors.0);

    println!("----------------------------------------------------------------");
    let sw_engg = Employee {
        name : String::from("Pankaj Bhatt"),
        age :37, 
        eye_color : Color(255,255,255)
    }; 

    println!("Employee Struct is -> {:#?}", sw_engg);

    println!("----------------------------------------------------------------");
    let india = Country {
        population : 3434_3434_34343_909,
        capital : String::from("DELHI"), 
        leader_name : String::from("MODI"), 
    };

    println!("India Details are -> {:?}", india);

}

