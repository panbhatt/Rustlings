// Chaining Methods.

fn main() {
    // The Below piece of logic as it is can be implemented via a single line
    let my_vec = (1..=10).collect::<Vec<u32>>();

    /*let mut my_vec = vec![];

    for i in 1..=10 {
        my_vec.push(i);
    } */

    println!("{:?}", my_vec);

    println!("================================================");
    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<u32>>(); // This will print 4/5/6/7
    println!("{:?}", new_vec);
}
