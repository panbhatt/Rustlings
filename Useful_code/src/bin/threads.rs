

fn main() {
    let arr = [1,25,-1,34,02,300]; 
    let max= find_max(&arr); 
    println!("{:?}", max);
    assert_eq!(max, Some(300)); 
}

/**
 * THis function will find the max by breaking the array in half. 
 */
fn find_max(arr : &[i32]) -> Option<i32> {
    const THRESHOLD : usize = 2; 

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2; 
    let (left, right) = arr.split_at(mid); 

    let result  =crossbeam::scope(|s| {
        let thread_1 = s.spawn(|_| find_max(left)); 
        let thread_2 = s.spawn(|_| find_max(right)); 

        let max_l = thread_1.join().unwrap(); 
        let max_r = thread_2.join().unwrap(); 

        (max_l.max(max_r))  // Since this results i an OPTION
    });

    result.unwrap()

}


    


