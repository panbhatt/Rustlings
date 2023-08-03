// Tour of standard library contains.
// Arrays, bool, char, Vec

use std::convert::TryFrom; 

fn main() {
    let cities = ["Delhi", "MIAMI", "SFO"]; // or use & in front of [];
                                            // you can't iterate over an Error
    for i in cities.iter() {
        println!("{}", i);
    }

    // Array destructors. 
    let [city1, city2, city3] = cities; 
    println!("City 1 = {}, City 2 = {}, City 3 ={}", city1, city2, city3); 
    

    // CHAR. 
    let hindi_word = "हिमालय"; 
    println!("=========== HINDI WORD in UNICODE====================="); 
    for character in hindi_word.chars() {
        println!("character {} -> {}", character, character.escape_unicode()); 
    }

    println!("Char from ->  {}", char::from(97)); // value has to u8
    println!("U8 Max : {}", u8::MAX); 

    println!("{:?}", char::try_from(345u32).unwrap_or('?')) ; // 345 treat me as u32. 

    println!("======================== INTEGERS =========================="); 
    // checked_add /mul/ div/ sub () -> 

    let some_number = 200_u8; 
    let other_number = 200_u8; 

    println!("ADD  with checked {:?}", some_number.checked_add(other_number)); // Results NONE as it is an ERROR. since A +B -> wont work. 


}
