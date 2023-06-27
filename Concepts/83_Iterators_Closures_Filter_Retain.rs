// FILTER , RETAIN for ITERATORS.

// CLOSURES.
// Fn, -> takes by reference &self
//FnMut, -> takes by reference mutable &mut self -> this closure will change the value
// FnOnce -> Takes by value (self)

fn main() {
    let mut months = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let months_starts_with_a = months
        .iter()
        .filter(|mth| mth.starts_with("A"))
        .collect::<Vec<_>>(); // You can have filters chain with another filter.

    println!(" MOnths Starts with A -> {:?}", months_starts_with_a);

    println!("======================================================");
    let my_closure = || print!("Least amount of authority "); // this takes FnOnce

    months.retain(|mth| mth.starts_with("A") == false);

    println!("MONTH REMAIN {:?}", months);
}
