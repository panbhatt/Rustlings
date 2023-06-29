// Any() will return true, if any of the item matches

fn main() {
    let vec_Of_chars = "Pankaj BHatt".chars().collect::<Vec<_>>();

    println!("{}", vec_Of_chars[0] > vec_Of_chars[1]);  // print false

    // If any of the element satisfies the ANY it would return and stop. In this case, it will print only 'a'
    vec_Of_chars.iter().any(|ch| {
        if ch >= &'a' {
            print!("{}", ch);
            return true;
        }
        return false;
    });
}
