
use bitflags::bitflags;
use std::fmt; 

// https://rust-lang-nursery.github.io/rust-cookbook/data_structures/bitfield.html#define-and-operate-on-a-type-represented-as-a-bitfield 

bitflags!{
    #[derive(Debug)]
    struct MyFlags : u32 {
        const FLAG_A = 0b00000001;
        const FLAG_B = 0b00000010;
        const FLAG_C = 0b00000100;
        const FLAG_ABC =  Self::FLAG_A.bits() | Self::FLAG_B.bits() | Self::FLAG_C.bits();


    }
}

impl MyFlags {
    pub fn clear(&mut self) -> &mut MyFlags {
        self.set(MyFlags::FLAG_ABC , false);
        
        self
    }

}

impl fmt::Display for MyFlags {
    fn fmt(&self , f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}



fn main() {
    let mut e1 = MyFlags::FLAG_ABC; 
    println!("{:?}", e1.bits()); // This will print 7. 
    println!("Before Clear -> {}", e1);   // This will print the bit pattern. 

    e1.clear(); 
    println!("After Clear ->  {}", e1); 

    println!("AND ->          {}", MyFlags::FLAG_A & MyFlags::FLAG_B);   // Bitwise AND
    println!("OR ->           {}", MyFlags::FLAG_A | MyFlags::FLAG_B);  // Bitwise OR. 

    

    
}