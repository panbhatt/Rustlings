// This will be same as that of PREVIOUS one, just that we are going to use ENUM as a replacement of TRAIT.

enum GiveName {
    One(One),
    Two(Two),
}

impl GiveName {
    fn give_name(&self) -> &'static str {
        match self {
            GiveName::One(o) => o.give_name(),
            GiveName::Two(t) => t.give_name(),
        }
    }

    fn try_get_one(&self) -> Result<One, String> {
        match self {
            GiveName::One(o) => Ok(o.clone()),
            GiveName::Two(__) => Err("Some Unknown type is being passed in".to_string()),
        }
    }
}

#[derive(Debug,Copy, Clone)]
struct One;
#[derive(Debug,Copy, Clone)]
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
    let o = GiveName::One(One {});
    let t = GiveName::Two(Two {});

    println!("ONE RETURNING -> {}", o.give_name());
    println!("TWO RETURNING -> {}", t.give_name());

    println!("One -> Try One -> {:?}", o.try_get_one());
    println!("Two -> Try TWO -> {:?}", t.try_get_one());
}
