// chars() usage with Iterators. Chars returns an iterator
// chars_indices() will give a tuple.

fn main() {
    let my_name = "Pankaj Bhatt".chars().collect::<String>();

    println!("{}", my_name);

    let my_name_vector = "Pankaj Bhatt".chars().collect::<Vec<_>>();
    println!("{:?}", my_name_vector);

    let my_name_added_by_one = "Pankaj Bhatt"
        .chars()
        .map(|x| (x as i32) + 1)
        .collect::<Vec<_>>();
    println!("{:?}", my_name_added_by_one);

    "Pankaj Bhatt"
        .char_indices()
        .for_each(|(i, chr)| println!("{} :: {} ", i, chr));
}
