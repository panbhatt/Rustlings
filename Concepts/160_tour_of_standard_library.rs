// Implementation of the ADD trait

use std::fmt::*;
use std::ops::Add;

#[derive(Debug, Clone)]
struct Country {
    name: String,
    gdp: u32,
    population: u32,
}

impl Country {
    fn new(name: String, gdp: u32, population: u32) -> Self {
        Self {
            name,
            gdp,
            population,
        }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, other: Country) -> Self {
        Self {
            name: format!(" {} + {} ", self.name, other.name),
            gdp: other.gdp + self.gdp,
            population: other.population + self.population,
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Country = {}, GDP = {}, Population = {}",
            self.name, self.gdp, self.population
        )
    }
}

fn main() {
    let sri_lanka = Country::new("Sri Lanka".into(), 230_000_0, 150_000_00);
    let banglaesh = Country::new("Bangledesh".into(), 130_000_0, 270_000_00);
    let india = Country::new("India".into(), 113430_000_0, 190_00_000_00);
    println!("{:?}", sri_lanka);
    println!("{:?}", banglaesh);
    println!("{:?}", india);

    println!("{:?}", sri_lanka + banglaesh);
}
