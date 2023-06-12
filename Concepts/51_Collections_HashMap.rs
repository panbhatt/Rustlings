// HASHMAP

use std::collections::HashMap;

#[derive(Debug)]
struct City {
    name: String,
    population: HashMap<u16, u32>,
}

fn main() {
    let hMap: HashMap<String, u32> = HashMap::new();

    let mut delhi = City {
        name: "Delhi".to_string(),
        population: HashMap::new(),
    };

    delhi.population.insert(2000, 123456);
    delhi.population.insert(2010, 2123456);
    delhi.population.insert(2020, 1423456);

    println!("DELHI IS -> {:?}", delhi);

    for (year, popu) in delhi.population {
        println!("In Year {year} -> {popu} People lives in delhi ");
    }
}
