// BinaryHeap

use std::collections::BinaryHeap;

fn main() {
    let mut my_heap : BinaryHeap<i32> = BinaryHeap::new();

    my_heap.push(100);
    my_heap.push(-1000);
    my_heap.push(100090);
    my_heap.push(1000);
    my_heap.push(-100);

    while let Some(number) = my_heap.pop() {
        println!("{}", number);
    }

    // When this wil print , it would print in DESCENDING ORDER i.e. from 100090 to -1000
    // Binary Heap returns the largest number at one point of time, but that doesn't mean, it is going to store it in that order. 
}
