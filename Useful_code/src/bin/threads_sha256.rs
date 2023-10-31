use num_cpus;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use walkdir::WalkDir;

//This program will walk in the src directory and print the SHA of every FILE that is going to encounter.

fn main() {
    let pool = ThreadPool::new(num_cpus::get());

    println!("Total CPUs = {}", num_cpus::get());

    let (tx, rx) = channel::<(Digest, PathBuf)>();

    let list_of_files = WalkDir::new("./src")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file());

    for entry in list_of_files {
        //println!("File Name -> {:?}/{:?} " , entry.path().as_os_str(), entry.file_name());

        let path = entry.path().to_owned();
        let sender_clone = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);

            if digest.is_ok() {
                sender_clone
                    .send(digest.unwrap())
                    .expect("Could not send data!");
            }
        })
    }

    drop(tx);
    for rc in rx.into_iter() {
        let (sha, fPath) = rc;
        println!("File : {:?} , SHA = {:?}", fPath, sha);
    }
}

fn compute_digest<P: AsRef<Path>>(filePath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filePath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count <= 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok((context.finish(), filePath))
}
