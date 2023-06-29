// cycle() -> CYCLE thorugh the iterator elements, so like ROUNDED circular linked list.

fn main() {
    let even_odd = vec!["even", "odd"];

    // Here with the values of 0 - 9 "eveN" "odd" iterator are going to clone and cycled from the start.
    let even_odd_vec = (0..10)
        .zip(even_odd.iter().cycle())
        .collect::<Vec<(i32, &&str)>>();

    println!("{:?}", even_odd_vec);

    println!("{:?}", (100..).take(12).collect::<Vec<_>>());

    println!("{:?}", ('a'..).take(26).collect::<Vec<_>>());

    println!("{:?}", ('a'..).skip(8000).take(26).collect::<Vec<char>>()); // Start from 'a' skip 8000 characters and take 26 that lies ahead.
}
