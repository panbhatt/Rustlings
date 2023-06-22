// How Iterators Work.
// .next() : Collection normally gives us one value at a time.

use std::iter;

fn main() {
    let my_vec = vec![1, 2, 3];
    let mut my_iter = my_vec.iter();

    assert_eq!(my_iter.next(), Some(&1));
    assert_eq!(my_iter.next(), Some(&2));
    assert_eq!(my_iter.next(), Some(&3));
    assert_eq!(my_iter.next(), None);
    println!("Successfully asserted");

    // Custom Creation of an iterator.
    let four_vec = iter::repeat(4).take(4).collect::<Vec<i32>>();
    four_vec.iter().for_each(|x| println!(" {}", x));

    // ITERATOR have skip method too.
    println!("===========================================");
    let two_vec = iter::repeat(40).take(4).skip(2).collect::<Vec<i32>>();
    two_vec.iter().for_each(|x| println!(" {}", x));
}
