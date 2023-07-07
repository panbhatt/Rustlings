// 'static
// 'a Named lifetime. 

#[derive(Debug)]
struct City<'a>{ // Struct with two named lifetime
    name : &'a str,  // If thi sis &str, but it is going to give error that expected names lifetime parameter. , 
    date_founded : u32, 
}

// By default RUST figures it out and assign the argument to a function a default lifetime of tha tfunction. 

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

    
}

// TUTORIAL on Rust lifetimes: https://www.youtube.com/watch?v=1QoT9fmPYr8 