// TRAIT BOunds in which we can shorten the trait writing from longer version to smaller version

use std::clone::Clone;
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Ranger {
    name: String,
    rank: String,
}

trait HitWithinRange {}

impl HitWithinRange for Ranger {}

fn hit_distance<T: HitWithinRange + Debug>(rgr: &T) {
    // This means, that only a struct that implements both HitWithRange and Debug can use this trait.
    println!(" I am a  {:?} and hitting with a distance ", rgr);
}

fn main() {
    println!("====================== RANGER =======================");
    let mut ranger = Ranger {
        name: "Rahul Gupta".to_string(),
        rank: "Subedar".to_string(),
    };

    hit_distance(&mut ranger);
}

// Practically, this is just another way in which we can take the function at another level and do not bound it to the TRAIT
// and use TRAIT just nomination purposes.
