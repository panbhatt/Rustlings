// PANIC
// panic!
// assert_eq!
// assert_ne!
// assert!()

fn main() {
    let my_vec = vec![8,9,10]; 
    print_three_things(my_vec); 

    let my_vec1 = vec![1,2]; 
    print_three_things(my_vec1);  // THis will panic and exit 


    //panic!("CANT DO SHIT"); // EXist the program with the panic, same as in Golang. 
}

fn print_three_things(vector : Vec<i32>) {
    if vector.len() < 3 {
        panic!("Can't use a vector when we have less then 3 elements in Vector"); 
    }
    println!("{} , {} , {}", vector[0], vector[1], vector[2]); 

}