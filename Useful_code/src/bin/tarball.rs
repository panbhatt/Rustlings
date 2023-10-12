use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs::File;
use tar::{self, Archive};

fn main() {
    println!("COMPRESSING A FILE ");
    create_tar("./src".to_owned(), "archive.tar.gz".to_owned()); // This will compress the SRC directory in the archive.tar.gz
    println!("COMPRESSION DONE ");

    println!("=============== STARTING DECOMPRESSION ===================");
    decompress_tar("archive.tar.gz".to_owned(), "./srcclone".to_owned());
    println!("============== UNCOMPRESSION DONE IN srclone dir = ================");
}

fn create_tar(dir: String, archive_name: String) {
    let tar_gz = File::create(archive_name).unwrap();
    let encoder = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(encoder);
    tar.append_dir_all("./", dir);
}

fn decompress_tar(tar_file_name: String, untar_dir: String) {
    let tar_file = File::open(tar_file_name).unwrap();
    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    // THis will print all the files.
    archive
        .entries()
        .unwrap()
        .filter_map(|e| e.ok())
        .for_each(|f| println!("{:?}", f.path().unwrap()));

    archive.unpack(untar_dir);
}
