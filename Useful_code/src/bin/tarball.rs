use std::fs::File;
use flate2::{write::GzEncoder, Compression};
use tar; 



fn main() {
    println!("COMPRESSING A FILE "); 
    create_tar("./src".to_owned(), "archive.tar.gz".to_owned());  // This will compress the SRC directory in the archive.tar.gz 
    println!("COMPRESSION DONE ");

}



fn create_tar(dir : String, archive_name : String) {

    let tar_gz = File::create(archive_name).unwrap();
    let encoder = GzEncoder::new(tar_gz, Compression::default()); 
    let mut tar = tar::Builder::new(encoder); 
    tar.append_dir_all("*", dir); 


}