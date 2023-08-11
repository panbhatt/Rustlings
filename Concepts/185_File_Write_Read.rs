use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;

// This program will write the file.
fn main() {
    let file_name = "./abc11.txt";
    let file_content = " This is the first file content \n. I dont know how i did this \n";

    //   write_file(file_name, file_content);  // Single line file write.

    fs::write(file_name, file_content);
    println!("File {} successfully written", file_name);

    println!("================== FILE CONTENTS IN UPPERCASE================");
    println!("{}", read_file_contents(file_name).unwrap());
}

// Not Used here.
fn write_file(file_name: &str, content: &str) -> Result<(), Error> {
    let mut my_file = File::create(file_name)?.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file_contents(file_name: &str) -> Result<String, Error> {
    let mut my_file = File::open(file_name)?;
    let mut file_contents = String::new();
    my_file.read_to_string(&mut file_contents)?;
    let my_str = file_contents
        .split_whitespace()
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>()
        .join(" ");

    println!("MODIFIED CONTENTS => \n{}\n\n ", my_str); 

    // with MAP you can collect, but you can't collect with for_each()
    Ok(file_contents)
}
