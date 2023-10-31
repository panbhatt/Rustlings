use num_cpus;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use walkdir::WalkDir;

//This program will walk in the src directory and print the SHA of every FILE that is going to encounter. 

fn main() {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel::<String>();

    let list_of_files = WalkDir::new("./src")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file());

    for entry in list_of_files {
        
        println!("File Name -> {:?}/{:?} " , entry.path().as_os_str(), entry.file_name()); 




    }
}
