// WHILTE LET

// This way we can constantly iterate over a VEC ( or something ) that will gives us RESULT/OPTION.

fn main() {
    let weather_vec = vec![
        vec!["DELHI", "HOT", "HUMID", "33", "38", "42"],
        vec!["FLORIDA", "HOT", "44", "46", "48"],
    ];

    for mut city in weather_vec {
        println!("STATE {:?}", city[0]);
        while let Some(info) = city.pop() {
            if let Ok(temp) = info.parse::<i32>() {
                println!("\tTemperature {}", temp);
            }
        }
    }
}
