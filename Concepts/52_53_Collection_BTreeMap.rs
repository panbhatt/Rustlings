// BREEMAP, exactly same as that of HASHMAP, just it stores the data in the ASCENDING ORDER of KEYS.

use std::collections::BTreeMap;

#[derive(Debug)]
struct City {
    name: String,
    population: BTreeMap<u16, u32>,
}

fn main() {
    let hMap: BTreeMap<String, u32> = BTreeMap::new();

    let mut delhi = City {
        name: "Delhi".to_string(),
        population: BTreeMap::new(),
    };

    delhi.population.insert(2000, 123456);
    delhi.population.insert(2010, 2123456);
    delhi.population.insert(2020, 1423456);

    println!("DELHI IS -> {:?}", delhi);

    for (year, popu) in &delhi.population {
        println!("In Year {year} -> {popu} People lives in delhi ");
    }

    println!("=================RANDOM GET ========================");

    if let Some(popu) = &delhi.population.get(&2010) {
        println!(" RANDOM GET -> {:?} ", popu)
    }

    if let Some(popu) = &delhi.population.get(&2015) {
        println!(" RANDOM GET -> {:?} ", popu)
    } else {
        println!(" CENSUS NOT AVAILABLE for year 2015");
    }

    let delhi_2017_population = delhi.population[&2020]; // If the Key is not found that it is going to PANIC and exit.
    println!(" DELHI POPULATION -> {:?}", delhi_2017_population);
}
