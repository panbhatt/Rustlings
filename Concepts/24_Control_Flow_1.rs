// Control Flow. 

fn main() {
    // IF/ELSE

    let is_single = false; 

    if is_single {
        println!("Read to mingle -> {is_single}"); 
    } else {
        println!("Seems you are hooked up. "); 
    }

    let age = 38; 

    if age < 35 && is_single {
        println!("You are getting old bro"); 
    } else {
        println!("You are still  Young "); 
    }

    println!("================================================================================="); 
    // MATCH ,, Kind of switch
    let my_age : u8 = 37; 
    match my_age {  // U have to think of everything. , every possible combinattion. 
        37 => println!("You are 37 "),
        38 => println!("You are 38 "),
        _ => println!("I dont' know your age. "), 
    }; 

    // Returing values 
    let is_employed = match my_age {
        37 => true,
        38 => true,
        _ => false, 
    };

    println!(" Are you employed = {is_employed} ");

    let my_name = "Pankaj";

    match ( my_age, my_name)  {
        (37, "Pankaj") => println!("Yup, its me"),
        (38, "Pankaj") => println!("Yup, its me, but age is not right"),
        (37, "Rahul") => println!("Nope, its not me, so sorrry "),
        _ => println!("I don't know who you are "), 

    }

    println!("================================================================================="); 

    // If in MATCH
    let married = true; 
    let children = 3; 
    match ( married, children)  {
        ( married, children) if married && children > 0 => println!("You are nice guy. Having kids after marriage is good "), 
        ( married, children) if married == false && children > 0 => println!("You think, marriage is a burden. Dont have lots of fun..."), 
        _ => println!("Rest, I dont care, you can live your life "), 

    }


}