use std::fs::File;
use std::io::Error;
use std::io::Write;



fn main() {
    let file_name = "./abc.txt";
    let file_content = " This is the first file content \n. I dont know how i did this \n";

    write_file(file_name, file_content);
    println!("File -> {} successfully written", file_name);
}

fn write_file(file_name: &str, content: &str) -> Result<(), Error> {
    let mut my_file = File::create(file_name)?;
    my_file.write_all(content.as_bytes())?;
    Ok(())
}
