// Enums

#[derive(Debug)]
enum ChoiceOfThings {
    Up,
    Down,
    Left,
    Right,
}

// ENUMS can hold data too
#[derive(Debug)]
enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_choice_of_things(time: u32) -> ChoiceOfThings {
    match time {
        1 => ChoiceOfThings::Up,
        2 => ChoiceOfThings::Down,
        3 => ChoiceOfThings::Left,
        _ => ChoiceOfThings::Right,
    }
}

fn perform_action(action: &ChoiceOfThings) {
    match action {
        ChoiceOfThings::Left => println!("LEFT"),
        ChoiceOfThings::Down => println!("Down"),
        ChoiceOfThings::Up => println!("UP"),
        _ => println!("RIGHT"),
    }
}

fn check_sky_things (things : &ThingsInTheSky) {
    match things {
        ThingsInTheSky::Sun(desc) => println!("SUN -> {}", desc),
        ThingsInTheSky::Stars(desc) => println!("Stars -> {}", desc),
    }
}

fn main() {
    let direction = ChoiceOfThings::Down;
    println!("{:?}", direction);
    println!("================================");
    let my_choice = create_choice_of_things(1);
    perform_action(&my_choice);

    println!("============ENUMS with Data ====================");

    let sunny_day = ThingsInTheSky::Sun("dayTime".to_string());
    println!("BLACK NIGHT = {:#?}", sunny_day);
    check_sky_things(&sunny_day); 

    println!("============MOOD ENUMS====================");
    let my_mood = Mood::HIGH; 
    match_mood(&my_mood);

}

enum Mood {
    HIGH,
    SLEEPY,
    HAPPY ,
    SAD,
}

fn match_mood(md : &Mood)  {
    use Mood::*;  // you can use this to import every 
    match md {
        HIGH =>println!("HIGH"), 
        SLEEPY =>println!("SLEEPY"), 
        HAPPY =>println!("HAPPY"), 
        SAD =>println!("SAD"), 
    }
}
