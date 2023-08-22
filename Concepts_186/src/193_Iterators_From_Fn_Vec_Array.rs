// Here we are going to create custom Iterators from the Fron_fn method present in the both the VEC and ARRAY.
//use std::iter::from_fn;

fn main() {
    let mut letters_1 = 'a'..='z';
    let mut letters_rev = ('a'..='z').rev();

    let mut new_iterator = std::iter::from_fn(|| {
        // we will try to concate both letters and rev letters and will print it afterwrads.
        letters_1.next().and_then(|first_letter| {
            if let Some(second_char) = letters_rev.next() {
                Some(format!("{}{}", first_letter, second_char))
            } else {
                None
            }
        })
    });

    //new_iterator.for_each(|letter| print!("\t{}", letter));

    println!("\n========================= USING ARRAY FN ============================");

    let arr: [usize; 10] = std::array::from_fn(|x| x * 10); // since it is an Array, we need to hardcode its size.
    println!("SImple Case -> {arr:?}");

    // Please enable line no 19, to check VEC ITERATORS combining.
    let my_array: [(usize, String); 30] = std::array::from_fn(|i| {
        let next_element = new_iterator.next();

        if let Some(ele) = next_element {
            (i, ele)
        } else {
            (i, "".to_string())
        }
    });

    println!("From ARRAY : -> {my_array:?}");
}
