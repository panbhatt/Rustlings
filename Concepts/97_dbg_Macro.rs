// dbg! macro

fn main() {
    let my_age = 38;
    println!("{}", my_age);
    dbg!(my_age); // ALways should KEy Value in the output.
    dbg!(8);
    eprintln!("{}", my_age); // this is same as debug, just it print it on th STD ERR.

    let y_name = dbg!("Pankaj Bhatt");
    println!("{y_name}");

    let my_vec = vec![1, 2, 3, 4, 5];
    dbg!(my_vec);
}
