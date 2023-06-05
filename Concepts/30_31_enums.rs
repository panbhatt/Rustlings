// ENUM Start with 0 internally.

enum SEASON {
    SUMMER,
    AUTUMN,
    WINTER = 10,
    SPRING,
}

#[derive(Debug)]
enum Number {
    U32(u32),
    I32(i32),
}

fn main() {
    use SEASON::*;

    let seasons = vec![SUMMER, AUTUMN, WINTER, SPRING];
    /*for season in seasons {   // Printing the default value.
        println!(" SEASON -> {}", season as u32);
    }*/

    for season in seasons {
        match season as u32 {
            size if size > 0 && size <= 50 => println!("{}", size),
            size if size > 50 && size <= 100 => println!("{}", size),
            size if size > 100 && size <= 150 => println!("{}", size),
            _ => println!("HUMONGOUS"),
        }
    }

    println!("============================================");

    let number1 = -100 as i32;
    let mut num = Number::U32(32);
    if number1.is_positive() {
        num = Number::U32(number1 as u32);
    } else {
        num = Number::I32(number1 as i32);
    }

    println!("{:?}", num);
}
