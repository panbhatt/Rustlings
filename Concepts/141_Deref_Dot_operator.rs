// Dereferencing and the DOT operator. 

// Deref -> . operator can go to any level to dereference and find the actual object. 
// it does auto referencing / auto dereference and coercion until types match. (coercion -> manually dereference))

struct Employee {
    age : u8, 
    name : String, 
}

impl Employee {
    fn getName(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let manager = Employee {
        age : 38, 
        name : "Pankaj Bhatt".into(), 
    };

    let ref_manager_age = &manager.age; 
    let ref_manager = &manager; 
    let ref ref_senior_manager = &ref_manager; 
    let ceo = &&&&&&&&&&&&&&&&&&&&ref_senior_manager; 
    
    println!("Manager age -> {}", *ref_manager_age);
    println!("Manager age -> {}", (*ref_manager).age);
    println!("Manager age -> {}", (*ref_senior_manager).age);
    println!("Manager age -> {}", (&&&&&&&&&&ref_senior_manager).age); //so no matter, how much deep u are it will reference it on its own. 

    println!("Award goes to -> {}  - Age - {} ", ceo.getName(), ceo.age); 
}