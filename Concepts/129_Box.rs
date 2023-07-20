// Box is a smart pointer that stores it data in the heap, while the pointer is in the stack.  its of 8 bytes

fn main() {
    let i = 100; 
    let i_box = Box::new(i); // 100 will be create on heap and pointer would be given back. 

    println!("{} {}", i, i_box.clone()); 
    //std::mem::drop(i_box); This will drop the pointer. 
    println!("I box = {}", i_box); 

    // COnvert a Large  VEC into Box. 
    let x =vec![80;3_000_000].into_boxed_slice();
    println!(" Length = {}", x.len()); 

    let bx = Box::new([27; 1_000_000]); 

}