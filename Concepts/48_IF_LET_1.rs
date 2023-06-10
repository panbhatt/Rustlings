// IF let is used, since in MATCH, we have to write all the cases for the OPTION/RESULT so it can be replaced via IF LET.

fn main() {
    let vecs = vec![10, 20, 30, 40];

    for i in 0..10 {
        let result = vecs.get(i);
        match result {
            Some(v) => println!("Found value {} at index {} ", result.unwrap(), i),
            None => {} // THis is what we are going to replace.
        }
    }

    // REPLACEMENT. read it backwards.
    println!("==================IF LET WITH OPTION=======================");
    for i in 0..10 {
        let result = vecs.get(i);
        if let Some(v) = result {  // This is how it is written. 
            println!("Found Number {} at index {} ", result.unwrap(), i);
        }
    }

    // IF LET with RESULT
    println!("==================IF LET with RESULT====================");
    let my_number : Result<i32,()> = Ok(15); 
    //let my_number : Result<i32,&str> = Err("Error");    // This will go in the ELSE CASE. 
    if let Ok(v) = my_number {
        println!("Found Number {} ", v);
    } else {
        println!("Did not found the number "); 
    }

    // There is a WHILE LET too
}
