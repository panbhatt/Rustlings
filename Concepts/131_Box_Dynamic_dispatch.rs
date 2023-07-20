// Dynamic Dispatch of a trait object.
// Dynamic Dispatch happens at runtime, like interface reference and object present implements that interface.

//impl trait and dyn trait are almost same. it. when we specificy interface at the return tyep but return any concrete implementation of it.

use std::fmt::Display;

trait JustATrait {}

#[derive(Debug)]
enum WeatherCondition {
    Hot(u32),
    Cold(u32),
    Messy(u32),
}

impl JustATrait for WeatherCondition {}

#[derive(Debug)]
struct Employee {
    name: String,
}

impl JustATrait for Employee {}

fn return_a_trait(input: &str) -> impl JustATrait { // This is another. but we can't return two concrete implementation at the same time from the code. 
    WeatherCondition::Hot(32)
}

fn return_a_trait_box(input: &str) -> Box<dyn JustATrait> {    // This is one way.
    match input {
        "weather" => Box::new(WeatherCondition::Hot(32)),
        _ => Box::new(Employee {
            name: "Pankaj Bhatt".to_string(),
        }),
    }
}

fn main() {
    // First way to get a interface or trait.
    let just_t = return_a_trait_box("weather");
    let just_2 = return_a_trait("weather");

    // This is ok, as we must have to make sure that what we are returning is a BOX i.e. of Fixed size in bytes. 8 bytes in this case. 
    println!("We successfully return two variation of the same functions ")
}
