// Box as we know always used 8 bytes and we will prove in this. Also you can't create a linked list without Box. , otherwise we have to use VecDEcque

struct Linked_List {
    item: i32,
    next: Option<Box<Linked_List>>,
}

fn main() {
    let i = Box::new(10);
    println!("Size of i = {}", std::mem::size_of::<Box<u32>>());

    println!(
        "Size of Box of String  = {}",
        std::mem::size_of::<Box<Vec<String>>>()
    );
    println!(
        "Size of Box of &str  = {}",
        std::mem::size_of::<Box<Vec<&str>>>()
    );

    // we can refer a box value directly, in println as it implements the Display Trait, and in operations via using *.
    println!("Box i = {}", i);
    println!("Box * 5 = {}", *i * 5);
}
