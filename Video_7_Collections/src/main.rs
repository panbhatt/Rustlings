use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    let _ar = [1,2,3]; 
    let mut my_vec = vec![1,2,3];
    my_vec.push(10);
    my_vec.push(100); 

    println!("{:?} ", my_vec); 

    println!("Four Element is -> {}", &my_vec[4]);

    match my_vec.get(2) {
        Some(number) => println!("Third Element -> {:?}", number),
        None => println!("None Found"),
    };

    match my_vec.get(20) {
        Some(number) => println!("Third Element -> {:?}", number),
        None => println!("None Found at index 20"),
    };

    for item in &mut my_vec {
        *item += 1000; 
    }

    for item in my_vec {
        print!("{item} ")
    }

    println!("====================== STRINGS ==========================="); 
    println!("BYTES  -> ");
    let namaste = String::from("नमस्ते"); 

    for byte in namaste.bytes() {
        print!("{byte} ");
    }
    println!(""); 

    println!("CHAR's  -> ");

    for byte in namaste.chars() {
        print!("{byte} ");
    }
    println!(""); 

    println!("GRAPHENE CLUSTERS  -> ");
    

    for gra in namaste.graphemes(true){
        print!("{gra} ");
    }
    println!(""); 

    println!("=========================== HASH MAP ============================"); 

    let mut scores = HashMap::new();
    scores.insert(0,10.0);
    scores.insert(1,20.0);

    scores.entry(10).or_insert(40.0); 

    println!("{:?}",scores);
    let first_element = scores.get(&10); 
    if first_element.is_some() {
        println!("{:?}",first_element.unwrap()); 
    }

}
