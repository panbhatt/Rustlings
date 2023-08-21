// Here we are going to create custom Iterators from the Fron_fn method present in the both the VEC and ARRAY. 
//use std::iter::from_fn; 

fn main() {

    let mut letters_1 = 'a'..='z';
    let mut letters_rev = ('a'..='z').rev();

    let new_iterator = std::iter::from_fn(|| {
        // we will try to concate both letters and rev letters and will print it afterwrads. 
        letters_1.next().and_then(|first_letter| {
               if let Some(second_char) = letters_rev.next() {
                Some(format!("{}{}", first_letter, second_char))
               } else {
                None
               }
        })
    });

    new_iterator.for_each(|letter| print!("\t{}",letter));

}