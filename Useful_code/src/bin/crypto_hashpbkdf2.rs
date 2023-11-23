use std::num::NonZeroU32;

use data_encoding::HEXUPPER;
use ring::{error::Unspecified, digest, rand::{self, SecureRandom}, pbkdf2};





fn main() -> Result<(),Unspecified> {

    let password = "ABCKDSDSDF&3434;";
    println!("Password = {}", password);

    let encrypted_password = hash_password_with_salt(password);
    println!("Entrypted Password = {}", encrypted_password);

    Ok(())

}

fn hash_password_with_salt(pwd : &str)-> String {

    const CREDENTIAL_LEN : usize = digest::SHA512_256_OUTPUT_LEN; 
    let n_iter = NonZeroU32::new(100_000).unwrap(); 
    let rng = rand::SystemRandom::new(); 

    let mut salt = [0u8; CREDENTIAL_LEN]; 
    rng.fill(&mut salt); 

    println!("SALT = {}", HEXUPPER.encode(&salt)); 

    let mut pbkdf2_hash = [ 0u8; CREDENTIAL_LEN]; 
    pbkdf2::derive(pbkdf2::PBKDF2_HMAC_SHA512, n_iter, &salt, pwd.as_bytes(), &mut pbkdf2_hash);


    return HEXUPPER.encode(&pbkdf2_hash);
}