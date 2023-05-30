

const SEASONS : [&str; 4] = ["Spring", "Summer", "fail", "Winter"] ; 

fn main() {
    // Rust Variables are mutable by default. Should be upper case name. 
    const NUM :i32 = 20_i32; 
    println!("{NUM}"); 

    // Static is const but always stay at same part of memory , i.e. fix memory location. 
    static TEMPRATURE :i8 = 100; 
    println!("Static {TEMPRATURE}");

    println!(" SEASONS : {:?}", SEASONS);
}