// .take_while()
// skip_while()
// .cloned()
// by_ref()
// sum()

fn main() {
    let my_vec = (1..=10).skip(2).take(5).collect::<Vec<_>>();
    println!("SKIP TAKE {:?}", my_vec);

    let my_vec = (1..=100)
        .skip_while(|&x| x < 10)
        .take_while(|&x| x < 50 && x % 2 == 0)
        .collect::<Vec<_>>();
    println!("SKIP_WHILE TAKE_WHILE{:?}", my_vec);

    // CLONED, it clones the iterator.
    let my_numbers = vec![1, 2, 3, 4];
    println!(
        "CLONE ITERATOR VALUES -> {:?}",
        my_numbers.iter().cloned().collect::<Vec<_>>()
    );

    // By REF, takes an ITERATOR by REF, so that It wont take the ownership.
    println!(
        "By Ref -> {:?}",
        my_numbers.iter().by_ref().collect::<Vec<_>>()
    );

    // sum() Sums everything together. Alternative to FOLD()
    println!("SUM-> {:?}", my_numbers.iter().sum::<i32>());
    println!("SUM 0..1000 -> {:?}", (0..1000).sum::<i32>());
}
