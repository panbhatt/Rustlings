// Demonstrating the use of external crates. 

use rand::*; 

fn main() {
    println!(" Random I32 -> {} ", rand::random::<i32>()); 
    println!("Random Bool -> {}", rand::random::<bool>()); 

    // Generate Number in the range. 
    let mut rng = rand::thread_rng();
    println!("\t\t10 Random Numbers\t\t"); 
    for i in 0..10 {
        print!("{}\t", rng.gen_range(1..=100)); 
    }

}