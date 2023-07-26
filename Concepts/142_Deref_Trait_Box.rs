// Deref trait for custom types. 

struct Employee {
    age : u8, 
    name : String, 
}

impl Employee {
    fn print_age(&self)  {
        println!("{}", self.age);
    }

}

fn main() {
    // * = thing.deref() 

        let emp_box = Box::new(Employee {
            name : "Pankaj bhatt".into(), 
            age : 38
        }); 

        // BOX will auto dereference. 
        emp_box.print_age();
}