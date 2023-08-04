// Functions of VEC

// sort, sort_unstable() -> may not keep the order of same elements same, bit faster. , dedup() -> remove adjacents same elements.

fn main() {
    let mut vec_numbers = vec![10, 30, 5, 10, 3300, 45, -34, 00, 00, 00];
    vec_numbers.sort();
    println!("{:?}", vec_numbers);

    vec_numbers.sort_unstable();
    println!("{:?}", vec_numbers);

    let mut vec_stars = vec!["sun", "moon", "moon", "sun", "sun", "venus"];
    vec_stars.dedup();
    println!("{:?}", vec_stars);
}
