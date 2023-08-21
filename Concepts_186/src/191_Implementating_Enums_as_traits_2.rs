
// This will be same as that of PREVIOUS one, just that we are going to use ENUM as a replacement of TRAIT. 

enum GiveName {
    One(One), 
    Two(Two)
}

impl GiveName {

    fn give_name(&self) -> &'static str {
        match self {
            GiveName::One(o) => o.give_name(), 
            GiveName::Two(t) => t.give_name() , 
        }

    }
}

#[derive(Debug)]
struct One; 
#[derive(Debug)]
struct Two; 

impl One {
    fn give_name(&self) -> &'static str {
        "ONE"
    }
}

impl Two {
    fn give_name(&self) -> &'static str {
        "TWO"
    }
}

fn main() {
    let o = GiveName::One(One{}); 
    let t = GiveName::Two(Two{}); 

    println!("ONE RETURNING -> {}", o.give_name())  ; 
    println!("TWO RETURNING -> {}", t.give_name())  ; 

}