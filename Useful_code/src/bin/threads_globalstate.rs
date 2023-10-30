use lazy_static::lazy_static; 
use std::sync::Mutex; 

lazy_static! {
    #[derive(Debug)]
    static ref FRUIT : Mutex<Vec<String>> = Mutex::new(Vec::new()); 
}


fn main()  -> Result<(),()> {
    
    insert("APPLE");
    insert("ORANGE");
    println!("{:#?}",FRUIT );

    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire Mutex Guard").unwrap(); 
        db.iter().enumerate().for_each(|(i, item)| println!("{} -> {} ", i, item)); 

    }
    insert("GRAPE"); 
    Ok(())


}

fn insert(fruit : &str) -> Result<(),()> {

    let mut fruits_vec = FRUIT.lock().map_err(|_| "Failed to acuire MutexGuard" ).unwrap();
    fruits_vec.push(fruit.to_owned()); 
    Ok(())

}