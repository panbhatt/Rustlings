// SOme other useful macros.

// unreachable! -> This Macro shows that the place where u put this, is still unreahable.
// column! , line!, file!, module_path!

fn send_msg_except_5(msg: String) {
    match msg.chars().count() {
        1 => println!(" 1  {}", msg),
        2 => println!(" 2  {}", msg),
        3 => println!(" 3  {}", msg),
        4 => println!(" 4  {}", msg),
        _ => unreachable!("You can't reached here, something is wrong in the input"),
    }
}

fn main() {
    send_msg_except_5("hell".into());
    //send_msg_except_5("hell0".into()); // This will panic, as this points to an unreachable code.

    println!("COLUMN -> {}", column!());
    println!("LINE -> {}", line!());
    println!("FILE -> {}", file!());
    println!("MODULE -> {}", module_path!());

    dbg!("Program finishd");

    if cfg!(target_os = "windows") {
        println!("You are on windows");
    } else {
        println!("You are not on WINDOWS");
    }
}
