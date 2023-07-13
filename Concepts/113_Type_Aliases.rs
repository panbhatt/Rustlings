

type MyString = String ; // This is exactly same as that of String



fn get_name() -> MyString {
    String::from("Panakj Bhatt")
}

fn main() {
    println!("Getting MyString same as that of String ->{}", get_name()); 

    // As to convert from one type to another. 
    println!("{} " , 23 as u64 as i32);

    // As while renaming import variables. 
    {
        use FileOperation::ReadOperation as ROP; 
        use FileOperation::{
            WriteOperation as WOP
        }; 

        let operation = FileOperation::ReadOperation; 
        match operation {
            ROP => println!("READ OPERATION CALLED"), 
            WOP => println!("WRITE OPERATION CALLED"),
            _ => println!("OPERATION CALLED UNKNOWN"), 
        }
    }



}

enum FileOperation {
    ReadOperation,
    WriteOperation,
    ReadWriteOperation, 
    AppedOnlyOperation,
}