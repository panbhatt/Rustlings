//inspect()

// This method to inspect the iter() in the pipeline to see individual elements.

fn main() {
    let my_nums = vec![1, 2, 3, 4, 5, 6, 7];

    let double_nums = my_nums
        .iter()
        .inspect(|x| { 
            dbg!(x);
            print!("{x} - ") }
    )
        .map(|x| x * x)
        .inspect(|x| println!("{x} "))
        .collect::<Vec<i32>>();

    println!(" ================= FINAL OUTPUT ===================");
    println!("Double Vector is = {:#?}", double_nums);
}

// dbg() -> output will come first, as it print to the STDERR. 
