

fn main() {
     
     let mut my_ages = vec![50,20,30,40,10]; 
     my_ages.sort(); 
     println!("Ages = {:?}", my_ages);
     my_ages.sort_by(|a,b| b.partial_cmp(a).unwrap()); 
     println!("Reverse Ages = {:?}", my_ages);

     let mut my_salaries = vec![10.5,23.4,34.4, 11.2]; 
     my_salaries.sort_by(|a,b| a.partial_cmp(b).unwrap()); 
     println!("My Salaries = {:?}", my_salaries); 
}