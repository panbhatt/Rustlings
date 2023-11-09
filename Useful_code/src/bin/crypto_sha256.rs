use std::{fs::File, io::{BufReader, Read}};

use ring::digest::{Context, Digest, SHA256};

use data_encoding::HEXUPPER; 


fn main()  {
    let path = "Cargo.toml"; 

    let input = File::open(path).unwrap(); 
    let reader = BufReader::new(input); 

    let dg = sha256_digest(reader); 
    
    if dg.is_ok() {

        let dg_string = HEXUPPER.encode(dg.unwrap().as_ref())  ;    
        println!("HEX STRING = {}", dg_string); 
    }


 
}

fn sha256_digest<R:Read>(mut reader:R) -> Result<Digest,()> {

    let mut context = Context::new(&SHA256); 
    let mut buffer = [0;1024]; 

    loop {
        let count = reader.read(&mut buffer).unwrap(); 
        if count == 0 {
            break; 
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())

}


