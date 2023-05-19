
fn main() {

    let my_age = 37;
    println!("  {} " , my_age); 

    let my_age = 39 ; 
    println!(" {my_age} " ); 

    {
        let my_age = 100 ; 
        println!(" {my_age} " );   // This will print 100 ; 
    }

    println!(" {my_age} " );  // This will print 39 , as this variable overshadowed the previous one. 

}