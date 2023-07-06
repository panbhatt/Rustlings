// peekable() -> makes an iterator that you can peek it.
// peek() -> This is used to peek at the next item.

fn main() {
    let my_numbers = vec![1, 2, 3, 4, 5, 6, 7];
    // my_numbers.pop() will take the item i.e. remove it.

    let mut peek_itr = my_numbers.iter().peekable();
    /*for i in peek_itr { // either we can iterate in this way or via peekable iterator.
        println!("{}", i);
    }*/

    println!("=============== Printing via NEXT =====================");
    for _ in 0..7 {
        print!(" {} -  ", peek_itr.peek().unwrap());
        print!(" {} | ", peek_itr.peek().unwrap()); // You can take the value or PEEK into it more then once.
        peek_itr.next();
    }

    let mut another_itr = my_numbers.iter().peekable();
    println!("\n================== Iterating while another way =====================\n");
    while another_itr.peek().is_some() {
        let num = another_itr.peek().unwrap();
        print!("{} - {} | ", num, num);
        another_itr.next();
    }
}
