// 'static
// 'a 

#[derive(Debug)]
struct City {
    name : &'static str,  // If thi sis &str, but it is going to give error that expected names lifetime parameter. , but static will work. 
    date_founded : u32, 
}

fn main() {
    let dehradun = City {
        name : "Dehradun",
        date_founded : 1850,
    };

    let jaipur = City {
        name : "Jaipur",
        date_founded : 1850,
    };

    println!("{:#?}, {:#?}", dehradun, jaipur); // TILL THIS WORKS. 

    let cities = vec!["Miami".to_string()]; 

    let delhi = City {
        name : &cities[0],    // This wont work as we are expecting name to be static i.e. lived throughout the program, but it is going away. 
        date_founded : 1660, 
    };
}

// TUTORIAL on Rust lifetimes: https://www.youtube.com/watch?v=1QoT9fmPYr8 