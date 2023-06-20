// This program will implement multiple traits to multiple STRUCTS. 

struct Fighter {
    name: String,
    rank: String,
}

struct Ranger {
    name: String,
    rank: String,
}

struct ArmyMen {
    name: String,
    rank: String,
    place_of_work: String,
}

trait HitWithinRange {
    fn hit_distance(&self);
}

trait HitWithWaterAir {
    fn hit_water(&self);
    fn hit_air(&self);
}

trait HitWithHand {
    fn hit_hand(&self);
}

// Implementation of Traits.
impl HitWithinRange for Ranger {
    fn hit_distance(&self) {
        println!(" Name -> {} hitting with a distance ", self.name);
    }
}

impl HitWithWaterAir for ArmyMen {
    fn hit_water(&self) {
        println!(" Name -> {} hitting with WATER ", self.name);
    }
    fn hit_air(&self) {
        println!(" Name -> {} hitting with AIR ", self.name);
    }
}

impl HitWithHand for Fighter {
    fn hit_hand(&self) {
        println!(" Name -> {} hitting with HAND ", self.name);
    }
}

fn main() {
    println!("====================== RANGER =======================");
    let ranger = Ranger {
        name: "Rahul Gupta".to_string(),
        rank: "Subedar".to_string(),
    };

    ranger.hit_distance();
    println!("====================== ARMY MEN =======================");
    let army_men = ArmyMen {
        name: "Pankaj Bhatt".to_string(),
        rank: "Sainik".to_string(),
        place_of_work: "DEHRADUN".to_string(),
    };

    army_men.hit_air();
    army_men.hit_water();

    println!("=================== FIGHTER +++++++++++++++++++++");
    let figther = Fighter {
        name: "FIGHTER".to_string(),
        rank: "NA".to_string(),
    };

    figther.hit_hand();
}
