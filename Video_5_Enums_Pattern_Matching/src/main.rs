// ENUMS

#[derive(Debug)]
enum IPAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddressKind1 {
    // Yes it can multiple storage with different data items.
    OLD(String),
    New(String),
    V6One(i8, i8, i8, i8),
}

impl IpAddressKind1 {
    // Methodd and functions can be defined over it.
    fn is_old(&self) -> bool {
        return true;
    }
}

// There aren't any NULL/NIL values in RUST. instead we are going to use Option and Result.

#[derive(Debug)]
struct IpAddr {
    kind: IPAddressKind,
    address: String,
}

fn main() {
    let ip4_addr = IpAddr {
        kind: IPAddressKind::V4,
        address: "127.0.0.1".to_string(),
    };

    let ip_old_addr = IpAddressKind1::OLD("localhost".to_string());

    println!("IP ADDR = {:?}", ip4_addr);
    println!("IP ADDR = {:?}", ip_old_addr);

    println!("================================ OPTIONS ENUM =============================");

    let name: Option<&str> = Some("Pankaj Bhatt");
    let friend_name: Option<String> = Some("Pankaj Bhatt".to_string());
    let no_name: Option<String> = None;

    let age: Option<u8> = Some(38);
    let no_number: Option<u8> = None;

    println!("{:#?} - {:#?} - {:#?}", name, no_name, age);

    // UNWRAPPING.
    println!("{:?} ", age.unwrap_or(0) + no_number.unwrap_or(0)); // Get the actual number in the Object.

    println!("==============================");
    match name {
        Some(nm) => println!("VALUE -> {:#?} ", nm),
        None => println!("NOTHING PRESENT "),
    };

    // IF LET PATTERN
    println!("================IF LET PATTERN =======================");

    if let Some(38) = age {
        println!("IF LET AGE = {}", age.unwrap());
    } else {
        println!(" The values are not equal ");
    }
}
