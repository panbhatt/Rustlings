// VEctors - Allocated on HEAP. 


fn main() {
    let mut my_vec = Vec::new();   // This is the first way in which we can create a Vector of NO TYPE. we need to do push to let compiler know this. 
    my_vec.push(String::from("Florida")); 
    my_vec.push(String::from("Orlando")); 

    println!("{:?}", my_vec); 

    let mut names_vec : Vec<String> = Vec::new();  // This is another way in which we are specifying the type of the vector we are creating. 
    names_vec.push(String::from("Pankaj Bhatt")); 
    names_vec.push(String::from("Rahul Gupta")); 

    println!("{:?}", names_vec); 

    let mut city_vec = vec!["Toronto", "Calgary", "winnipeg", "Churchill", "NewYork"];
    println!("{:?}", city_vec); 

    // Slicing is same as that of Array. 
    let canada_cities = &city_vec[0..4]; 
    println!("Canada Cities => {:?}", canada_cities); 

    // Print Vector Capacity and its length. 
    println!("Length -> {}  Capacity -> {} ", city_vec.len() , city_vec.capacity());

    // Brand New Vector has capacity of ZERO 
    // Create a vec with a capacity. 
    let mut states_canada = Vec::with_capacity(5); 
    states_canada.push("ALBERTA");
    states_canada.push("ONTARIO");
    states_canada.push("QUEBEC");
    states_canada.push("BC");
    states_canada.push("SASKETCHWAN");
    states_canada.push("NUNAVAT");  //Here the Capacity would be doubled. 
    println!("States -> {}", states_canada.capacity());

    // Convert ARRAY into VEC
    let phones : Vec<&str>= [ "NOKIA", "MOTOROLA", "GOOGLE"].into();   // Convert Array to vectory. 
    let ages_of_phones : Vec<_> = [ 10,20,38,15,23].into() ; // in this case compiler is going to determine this on its own. 
    println!("Phones -> {:?}", phones); 
    println!("Phone Ages -> {:?}, Capacity -> {}", ages_of_phones, ages_of_phones.capacity());





}