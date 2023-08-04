// BOOL.

fn main() {
    let tfValue = (true, false);

    println!("TRUE - {}, FALSE - {}", tfValue.0 as i32, tfValue.1 as i32);

    let tfLValue: (u128, u128) = (true.into(), false.into()); // This time it gives u128 bytes, we can't convert 1/0 to bool though.
    println!(
        "Large -> TRUE - {}, FALSE - {}",
        tfLValue.0 as i32, tfLValue.1 as i32
    );

    // then() for bool to turn it an Option.
    let (tru, fal) = (true.then(|| 8), false.then(|| 100));
    println!("{:?}", tru); // This would be Some(8)
    println!("{:?}", fal); // This would be NONE

    // Turn a VEC of BOOLS in a VEC of Options.
    let vec_bools = vec![true, false, false, true, true, true];

    let vec_bools_modified = vec_bools
        .iter()
        .map(|b| {
            b.then(|| {
                println!("Got a POsition value ");
                "POsitive Value"
            })
        })
        .filter_map(|item| item) // only pass those which are SOME i.e. TRUE>
        .collect::<Vec<_>>();

    println!("MOdified VEC of Bools = {:?}", vec_bools_modified);
}
