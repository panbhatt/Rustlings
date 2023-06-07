#[derive(Debug, Copy, Clone)]
enum AnimalType {
    WATER,
    AMPHIBIAN,
    LAND,
}

#[derive(Debug, Copy, Clone)]
struct Animal {
    color: (u8, u8, u8),
    atype: AnimalType,
}

impl Animal {
    fn new(c: (u8, u8, u8), atype: AnimalType) -> Self {
        Self { color: c, atype }
    }

    fn change_type(&mut self, atype: AnimalType) {
        self.atype = atype;
    }

    fn check_type(&self) -> bool {
        match self.atype {
            AnimalType::WATER => {
                println!("Animal Type is WATER");
                return true;
            }
            AnimalType::AMPHIBIAN => {
                println!("Animal Type is AMPHIBIAN");
                return true;
            }
            AnimalType::LAND => {
                println!("Animal Type is LAND");
                return true;
            }
        }
    }
}

fn main() {
    let dog = Animal::new((255, 200, 200), AnimalType::LAND);
    let frog = Animal::new((255, 200, 200), AnimalType::AMPHIBIAN);
    let fish = Animal::new((255, 200, 200), AnimalType::WATER);

    dog.check_type();
    frog.check_type();
    fish.check_type();

    println!("{:?}", dog);
    println!("{:?}", frog);
    println!("{:?}", fish);
}
