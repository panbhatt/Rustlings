use rayon::prelude::*;

fn main() {
    let mut arr = [01, 2, 3, 4, 5, 6];
    arr.par_iter_mut().for_each(|p| *p += 1);
    println!("{:?}", arr);
}
