use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::{error::Unspecified, digest, rand::{self, SecureRandom}, pbkdf2};





fn main() -> Result<(),Unspecified> {

    let password = "ABCKDSDSDF&3434;";
    println!("Password = {}", password);

    let salt = getSalt(); 
    let encrypted_password = encrypt_password(password, salt.clone());
    println!("SALT = {} , Entrypted Password = {}",  salt, encrypted_password);

    Ok(())

}

fn getSalt() -> String {
    
    const CREDENTIAL_LEN : usize = digest::SHA512_256_OUTPUT_LEN; 
    let rng = rand::SystemRandom::new(); 
    let mut salt = [0u8; CREDENTIAL_LEN]; 
    rng.fill(&mut salt); 

    return HEXUPPER.encode(&salt);
}

fn encrypt_password(pwd : &str, salt : String)-> String {

    const CREDENTIAL_LEN : usize = digest::SHA512_256_OUTPUT_LEN; 
    let n_iter = NonZeroU32::new(100_000).unwrap(); 


    let mut pbkdf2_hash = [ 0u8; CREDENTIAL_LEN]; 
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA512, n_iter, salt.as_bytes(), pwd.as_bytes(), &mut pbkdf2_hash);


    return HEXUPPER.encode(&pbkdf2_hash);
}

fn decrypt_password() -> String {
    return "".to_string()
}