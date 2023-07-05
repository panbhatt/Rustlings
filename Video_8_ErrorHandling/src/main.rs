use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    //   a(); // Enable this line and run via RUST_BACKTRACE=1 cargo run and you will see a stack trace of failure.

    // Handling Result : Recorable Gracefully using RESULT.
    let f = File::open("./sample.txt");

    let fl = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./sample.txt") {
                Ok(fl) => fl,
                Err(err) => panic!("Unable to create a new file"),
            },
            other_error => panic!("Unable to open the file "),
        },
    };

    println!("Length of file is = {}", fl.metadata().unwrap().len());

    // Result operations
    // unwrap()
    // expect(): fail with this error message.

    println!("FILE CONTENTS -> \n{}", read_username_from_file().unwrap());
    println!(
        "FILE CONTENTS VER 1  -> \n{}",
        read_username_from_file_ver1().unwrap()
    );
    println!(
        "FILE CONTENTS VER 3  -> \n{}",
        read_username_from_file_ver2().unwrap()
    );
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("./sample.txt")?; // ? will return error if there is an  otherwise it will unwrap and mvoe ahead.
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_ver1() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("./sample.txt")?.read_to_string(&mut s);

    Ok(s)
}

fn read_username_from_file_ver2() -> Result<String, io::Error> {
    fs::read_to_string("./sample.txt")
}

fn a() {
    b();
}

fn b() {
    c(0);
}

fn c(num: i32) {
    if num == 0 {
        panic!("you are never supposed to pass zero to this function.");
    }
}
