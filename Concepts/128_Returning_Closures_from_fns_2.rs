// SIMON's game and how his FEAR changes as the Day passed by.

enum TimeOfDay {
    Dawn,
    Day,
    Sunset,
    Night,
}

fn change_fear(input: TimeOfDay) -> impl Fn(f64) -> f64 {
    use TimeOfDay::*;

    match input {
        Dawn => |x| {
            println!("Finally its Dawn, my fear is reducing. Fear = {}", x * 0.5);
            x * 0.5
        },
        Day => |x| {
            println!(
                "Finally its DAY TIME, my fear is almost gone. Fear = {}",
                x * 0.2
            );
            x * 0.2
        },
        Sunset => |x| {
            println!("Suncset is here, my fear is increasing.  {}", x * 0.8);
            x * 0.8
        },
        Night => |x| {
            println!("Night is here. My fear is high  {}", x * 1.5);
            x * 1.5
        },
    }
}

fn main() {
    use TimeOfDay::*;

    let mut simon_fear = 10.0;
    let day_fear = change_fear(Day);
    let sunset_fear = change_fear(Sunset);
    let night_fear = change_fear(Night);
    let dawn_fear = change_fear(Dawn);

    simon_fear = day_fear(simon_fear);
    simon_fear = sunset_fear(simon_fear);
    simon_fear = night_fear(simon_fear);
    simon_fear = dawn_fear(simon_fear);
}
