// Tour of standard library contains.
// Arrays, bool, char, Vec

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

}
