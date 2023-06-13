// HASHSET
// Similarly we have BTREESET, just replace HashSet with BTreeSet. 

use std::collections::HashSet;

fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8];

    let mut num_set = HashSet::new();
    for item in items {
        num_set.insert(item);
    }

    println!("{}", num_set.len()); // it will give 10, as items with same hash i.e. same number has been removed.
    println!("==============PRINTING HASHSET ITEMS ==================");
    for item in &num_set {
        print!("{} -", item);
    }

    // Get first element.   as it will returns.
    if num_set.get(&3).is_some() {
        println!(
            "\n GET THE VALUE ->  VALUE IS {} ",
            num_set.get(&3).unwrap()
        );
    }
}
