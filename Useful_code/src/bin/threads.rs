use std::{thread, time::Duration};

use crossbeam::channel::{bounded, unbounded};

// https://rust-lang-nursery.github.io/rust-cookbook/concurrency/threads.html

fn main() {
    let arr = [1, 25, -1, 34, 02, 300];
    let max = find_max(&arr);
    println!("{:?}", max);
    assert_eq!(max, Some(300));

    println!("============= PARALLEL PROCESSING  ===========");
    parallel_processing();

    println!("=========== PASSING DATA BETWEEN TWO THREADS =============");
    passing_data_between_threads();
}

/**
 * THis function will find the max by breaking the array in half.
 */
fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    let result = crossbeam::scope(|s| {
        let thread_1 = s.spawn(|_| find_max(left));
        let thread_2 = s.spawn(|_| find_max(right));

        let max_l = thread_1.join().unwrap();
        let max_r = thread_2.join().unwrap();

        (max_l.max(max_r)) // Since this results i an OPTION
    });

    result.unwrap()
}

fn parallel_processing() {
    let (sender1, receiver1) = bounded::<i32>(1);
    let (sender2, receiver2) = bounded::<i32>(1);

    const NO_OF_MSGS: i32 = 4;
    const NO_OF_WORKERS: i32 = 2;

    crossbeam::scope(|s| {
        s.spawn(move |_| {
            for i in 0..NO_OF_MSGS {
                sender1.send(i + 1).unwrap();
                println!("SOURCE SENT -> {}", i + 1);
            }

            drop(sender1);
        });

        // Parallel PRocessing by 2 threads.

        for i in 0..NO_OF_WORKERS {
            let (sn, rc) = (sender2.clone(), receiver1.clone());
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in rc.iter() {
                    println!("WORKER : {:?} RECEIVED : {}", thread::current().id(), msg);
                    sn.send(msg * 2).unwrap();
                }
            });
        }

        drop(sender2);

        for msg in receiver2.iter() {
            println!("SINK RECEIVED -> {}", msg);
        }
    });
}

fn passing_data_between_threads() {
    let (sndr, recr) = unbounded::<i32>();
    let n_msgs = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                sndr.send(i + 1).unwrap();
                println!(" SOURCE SEND -> {}", i + 1);
                thread::sleep(Duration::from_millis(1000));
            }
            drop(sndr);
        });
    })
    .unwrap();

    for _i in 0..n_msgs {
        let msg = recr.recv().unwrap();
        println!(" RECEIVER GET  = {msg}");
    }
}
