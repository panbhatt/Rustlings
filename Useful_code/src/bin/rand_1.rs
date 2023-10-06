// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random U8 -> {}", n1);
    println!("Random U16 -> {}", n2);
    println!("Random U32 -> {}", rng.gen::<u32>());
    println!("Random i32 -> {}", rng.gen::<i32>());
    println!("Random F64 -> {}", rng.gen::<f64>());

    println!("-------------------------------------------------");
    println!("############# RANDOM NO IN RANGE ################");

    println!("Integer -> {}", rng.gen_range(0..100));
    println!("Integer -> {}", rng.gen_range(0.9..1000.45));

    // we can also use rand_distr crate to generate values of diffierent distribution.
    // you can also generate the RANDOM values for any STRUCT TYPE.

    let rand_str: String = rng
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("AlphaNumeric String -> {}", rand_str);

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    let mut rng1 = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng1.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("Password -> {}", password);
}
