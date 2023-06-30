// Here we are going to implement FROM Trait for our custom STRUCT that stores vec of even numbers at 0 location and odd numbers at 1 location.

use std::convert::From;

#[derive(Debug)]
struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(v: Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]];

        for item in v {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }

        Self(even_odd_vec)
    }
}

fn main() {
    let number_vec = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    let even_odd_vec = EvenOddVec::from(number_vec);

    println!("EVEN VEC -> {:?}", even_odd_vec.0[0]); // Accessible 0th element
    println!("ODD VEC -> {:?}", even_odd_vec.0[1]); // Accessible 1st element of the tuple.
}
