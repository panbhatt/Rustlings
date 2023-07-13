use std::borrow::Cow;

//This program will return either the SLICE or the VEC

fn main() {
    let slice = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 100][..]; // thi sis called slice
    println!("SLICE -> {:?}", slice);
    println!("===========================================");
    let x = get_slice_or_vec(&[7, 8, 9, 10]);
    let y = get_slice_or_vec(&[7, 8, 9, 10, 20, 30, 40, 50]);

    println!("SLICE -> {:?}", x);
    println!("VECTOR -> {:?}", y);
}

// Function that either return slice or vec with the help of COW
fn get_slice_or_vec<'a>(input: &'a [u32]) -> Cow<'a, [u32]> {
    match input.len() {
        0..=5 => Cow::Owned(input.to_vec()),
        _ => Cow::Borrowed(input),
    }
}
