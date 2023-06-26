fn main() {
    // Closure

    // Remembr the _or and _or_else function 

    let my_closure = |x| {
        println!("Closure input is ->  {x}");
    };
    my_closure(5);

    // UNWRAP Or.
    let my_option: Option<String> = None;
    // println!(" Option Value = {}", my_option.unwrap());  // This code would panic. as there is none inside the option.

    println!(
        "Option Value = {}",
        my_option.clone().unwrap_or("Pankaj Bhatt".to_string())
    ); // this would work correctly.
    println!(
        "Option Value = {}",
        my_option
            .clone()
            .unwrap_or_else(|| { "Pankaj in Florida".to_string() })   //This takes a CLOSURES. 
    );
    // this would work correctly.
}
