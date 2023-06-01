// Array must be of same type like Golang 

fn main() {

    let my_array = [ "One", "Two", "Three"]; // This array is of size [&str; 3] 

    println!("{:?}", my_array); 

    let names_array : [ &str; 3] = [ "Pankaj Bhatt", "Rahul Gupa", "Kunal Gupta"];
    println!("{:?}", names_array);  

    let mut repeated_array =  [ "Pankaj" ; 10 ] ; // An Array with repeated 10 occurrences of string "Pankaj"

    println!("{:?}", repeated_array); 
    repeated_array[0] = "Bhatt"; 
    println!("{:?}", repeated_array); 
    println!("{}", repeated_array[0]); 

    let  names_half_array = &mut repeated_array[0..5]; // only starting 5 elements by this method [i..j]  
    println!("{:?}", names_half_array);
    names_half_array[1] = "Gupta"; 
    println!("{:?}", names_half_array);
    println!("->  {:?}", repeated_array);

    // They both point to the same array, underlying. 

}