struct Adventurer<'a> {
    name: &'a str, // this expects a named lifetime parameter and wont be able to compile properly.
    hit_points: i32,
}

//impl<'a> Adventurer<'a> {  // This also works
impl Adventurer<'_> {
    fn hit_damage(&mut self, damage_points: i32) {
        self.hit_points -= damage_points;
        println!("{} has {} Points left ", self.name, self.hit_points);
    }
}

fn main() {
    println!("HELLO");
}

// THere is also something called <'_> called anonymous lifetime
