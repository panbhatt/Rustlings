use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::time::SystemTime;
use std::fs; 

fn main() {
    let file_name = "abc11.txt";

    append_content(file_name);
    println!("File Appended successfully");
    read_contents(file_name);

}

// This function will append the time of the day to the file.
fn append_content(file_name: &str) -> Result<()> {
    let mut our_file = OpenOptions::new().create(true).read(true).append(true).open(file_name)?;

    let header = SystemTime::now();

    write!(&mut our_file, "\n{:?}", header)?;
    our_file.write_all(b"\nThis is the content that needs to be written\n")?;

    let footer = SystemTime::now();
    write!(&mut our_file, "\n{:?}", header)?;

    Ok(())
}

fn read_contents(file_name : &str) -> Result<()> {

    println!("{:?}", fs::read_to_string(file_name)?);
    Ok(())

}
