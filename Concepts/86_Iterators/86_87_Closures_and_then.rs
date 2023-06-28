// and_then() https://hermanradtke.com/2016/09/12/rust-using-and_then-and-map-combinators-on-result-type.html/

// This is present in Result and Option and use to process the get

//you can link and and ano_then function so that your pipeline will process if the value exists in the pipeline.  [ Chaining ]

fn main() {
    let months = vec![Some(1), None, Some(3), None, Some(5), Some(6)];

    println!("{:?}", months.get(5).and_then(|value| Some(-1))); // This will print -1 since months[5] has a value which makes and then executes.
    println!(
        "{:?}",
        months
            .get(10)
            .and_then(|value| Some(-1))
            .and_then(|value| Some("Error Occured"))
    ); // Since in this case,.get() returns None so it wont proceed in the pipeline .

    // and_then can make endless chains and only it wil executed if the previous value is not NONE.

    // Use then() when Future successfully resolves.
    // Use and_then() when Future successfully resolves an Ok .
    // Use or_else() when Future successfully resolves an Err.

    // and() - Boolean for Options

    println!("{:?}", months.get(10).and(months.get(4))); // Since 10 is NONE it wont even got forward.
    if months.get(10).and(months.get(40)).is_some() {
        println!("10 & 40 are present");
    } else {
        println!("10 and 40 NOT PRESENT");
    }

    if months.get(1).and(months.get(4)).is_some() {
        //Here since 1 and 4 are present so And returns too like && boolean operator.
        println!("1 & 4 are present");
    } else {
        println!("1 and 4 NOT PRESENT");
    }
}
