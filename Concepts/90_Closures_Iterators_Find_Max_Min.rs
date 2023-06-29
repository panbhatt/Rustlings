// Find() -> Return first item in an OPtion.
// Position() -> I will try to find it for you and tell you where it is.

fn main() {
    let numbers = vec![10, 20, 30, 40, 50, 60, 80, 100];

    // THis will return the object, where it matches the condition.
    let num_80 = numbers.iter().find(|&&x| x == 80);
    if num_80.is_some() {
        println!("{}", num_80.unwrap());
    } else {
        println!("80 is not found in the vector ");
    }

    // This will give the position
    let position = numbers.iter().position(|&x| x == 80);
    if position.is_some() {
        println!("Position is -> {}", position.unwrap());
    } else {
        println!("80 is not found in the vector ");
    }

    println!("=============EVERY NUMBER TYPE HAS MIN/mAX====================");
    println!("{} - {}", u8::MAX, u8::MIN);
    println!("{} - {}", i32::MAX, i32::MIN);
    println!("{} - {}", f32::MAX, f32::MIN);
}
