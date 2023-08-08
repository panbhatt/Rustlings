// Transmute function. This is highly unsafe and should be used only when it is the last option for us.

use std::mem::transmute;

fn main() {
    // Adding two bytes.
    let thing1 = unsafe { transmute::<[u8; 2], u16>([25, 200]) };

    println!("U16 i.e. 2 bytes of u8 (25) +u8(200) = {:?}  ", thing1);

    // Break 64 in a pair of 32 bytes.
    let thing2 = unsafe { transmute::<u32, [u16; 2]>(65537) };

    println!("u32 in a pair of two bytes : {:?}", thing2); // This would be shown as 1, 1, , Think why.

    // Treat three u8 in a String. This will fail as it cant' translate between different sizes.
    //let thing3 = unsafe { transmute::<[u16; 4], String>([24, 44565,3434, 343]) };

    println!("{:?}", "HELLO".as_bytes()); // give the byts and U can do reverse to via transmute

    // There are some crates available that helps to make this safer.
}
