

fn main(){


    let my_number = 10 ; 

    //my_number = 100 ;   // You can't do this without putting the mut keyword in the declaration. Everything is mutable by default. you have ot use MUT
    println!("{}", my_number); 

    let my_number = "Pankaj Bhatt age is 37" ;   // This is redeclaration of the same variable. this is called SHADOWING. 
    println!("{my_number}");
}