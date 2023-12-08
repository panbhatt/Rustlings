
use bitflags::bitflags; 



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
        self.set(MyFlags::FLAG_A, false);
        self
    }
}

fn main() {
    let e1 = MyFlags::FLAG_ABC; 
    println!("{:?}", e1.bits()); // This will print 7. 
    

    
}