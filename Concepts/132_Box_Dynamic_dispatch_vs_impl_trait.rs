// we can use Box dynamic Dispatch rather then impl trait, as it will give us error

// it is better to return Box<dyn Error> rather then impl trait

use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct SocketError {}

#[derive(Debug)]
struct TcpError {}

impl Error for TcpError {}
impl Error for SocketError {}

impl Display for TcpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TCP Error ")
    }
}

impl Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket Error ")
    }
}

/* The below code wont work as impl Error unable to determine the type of RETURN SOCKETError or TcpError
fn return_err(input : u8 ) -> Result<String, impl Error> {
    match input % 3 {
        1 => Err(SocketError{}),
        2 => Err(TcpError{}),
        _ => Ok(format!("All is Good. Correct input - {}", input)),
     }
}*/

fn return_err(input: u8) -> Result<String, Box<dyn Error>> {
    match input % 3 {
        1 => Err(Box::new(SocketError {})),
        2 => Err(Box::new(TcpError {})),
        _ => Ok(format!("All is Good. Correct input - {}", input)),
    }
}

fn main() {
    let vecs_numbers = vec![1, 2, 3, 4, 5, 6, 7];
    for number in vecs_numbers {
        match return_err(number) {
            Ok(number) => println!("Received a number {} ", number),
            Err(err) => println!("Error -> {}", err),
        }
    }
}
