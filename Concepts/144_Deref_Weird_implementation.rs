// This an Weird Implementation of Deref
use std::ops::Deref;

#[derive(Debug)]
struct Player {
    name: String,
    score: u16,
}

impl Deref for Player {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.score
    }
}

fn main() {
    let messi = Player {
        name: "Messi".into(),
        score: 758,
    };
    let ronaldo = Player {
        name: "Ronaldo".into(),
        score: 1001,
    };

    println!("Total Goals = {}", *messi + *ronaldo); // it automatically DEREFS to the Score
    let  mut total_goals = vec![]; 

    total_goals.push(*messi); 
    total_goals.push(*ronaldo); 

    println!("Goals distibutionu : {:?}", total_goals); 


}
