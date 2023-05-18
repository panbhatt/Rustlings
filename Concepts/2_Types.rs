

fn main(){

/*
    i8, i16, i32, i64, i128, isize
    u8, u16, u32, u64, u128, usize

 */

// Chars
let letter = 'A';
println!("{}", letter as u32 as u8 as char);

// Character Length
println!("{}", "₹".len() );  // Len is 3 
println!("{}", "₹".chars().count() );  // Character is just one 

}
