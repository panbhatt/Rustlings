fn main() {

    let my_age = 10 ; 
    let myage_ref = &my_age; 
    let myage_ref1 = & myage_ref; 

    println!("{} {}  {}", my_age, *myage_ref, **myage_ref1);

    /*
        & -> single reference
        && -> double reference
        * -> Single Dereferecning
        ** -> Double Dereferencing
    
     */

    println!("{}    ",  my_age_ref == myage_ref1);  // THis is wrong, you can't compare referernces only values. 

}