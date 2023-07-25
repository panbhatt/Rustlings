// Distribute a work via a channel amongst multiple receivers.

// it would change the 0 at every position with the position number E.g. 0 with 0 , 1 with 1 etc.

use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel();
    let huge_vec: Vec<u32> = vec![0; 1__000__000];
    let mut handle_vec = vec![];
    let mut new_vec = vec![]; // This is where it will put the final results.
    let no_of_workers = 10;

    for i in 0..no_of_workers {
        let sender_c = sender.clone();
        let mut work: Vec<u32> = Vec::with_capacity(huge_vec.len() / no_of_workers);
        work.extend(&huge_vec[i * 1_00_000..(i + 1) * 1_00_000]);
        let handle = spawn(move || {
            //let mut cntr = 0;
            //for item in work.iter_mut() {
            for (j, item) in work.iter_mut().enumerate() {
                // *item = (i * 1_00_000 + cntr) as u32 ;
                //     cntr +=1;
                //*item += 1;
                *item = (i * 1_00_000 + j) as u32;
            }
            sender_c.send(work).unwrap(); // send the item.
        });

        handle_vec.push(handle);
    }

    for handle in handle_vec {
        handle.join().unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        new_vec.push(results);
    }

    println!("FINAL VEC LENGTH = {}", new_vec.len());

    // Now weneed to flatten the vec.

    let result_vec = new_vec.iter().flatten().collect::<Vec<&u32>>();
    println!(
        "Element at 38 = {} and 45678 = {} ",
        result_vec[38], result_vec[45678]
    );

    // check first 10 vales and last 10 values.
    println!(
        "{:?}  Last : {:?}",
        &result_vec[0..10],
        &result_vec[result_vec.len() - 10..result_vec.len()]
    )
}
