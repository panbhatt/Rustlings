// All() will return true, if any of the item matches

fn main() {
    let vec_Of_chars = "Pankaj BHatt".chars().collect::<Vec<_>>();

    // If any of the element satisfies the ANY it would return and stop. In this case, it will print only 'a'
    let value = vec_Of_chars.iter().all(|ch| {
        if ch >= &'A' && ch <= &'Z' {
            print!("{}", ch);
            return true;
        }
        return false;
    });

    println!("is All Capital Letters-> {}", value);

    println!("{:?}", ('a'..'z').collect::<Vec<_>>());
    println!("{:?}", (1..25).collect::<Vec<_>>());
}
