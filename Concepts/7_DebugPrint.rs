

fn main() {
    let age = {
        let (father_age , mothers_age) = (70, 65); 
        let age =  (father_age + mothers_age )/ 2;
        age;
        age
    };

    
    println!(" My Age is = {:?}  - {:#?} ", age, age ) ;   // This is DEBUG PRINT, will print empty () if nothing is returned. if lin no 8 is enabled. it will print 67. 
}