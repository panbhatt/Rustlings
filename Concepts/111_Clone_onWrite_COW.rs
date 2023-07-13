// COW -> Clone on write
// either hve to use BORROWED or OWNED

// ALTHOUGH I dont understand the usage of COW BORROWED or CLONED.

use std::borrow::Cow;

fn check_remainder(n: u8) -> Cow<'static, str> {
    // Remember no & is needed.
    match n % 3 {
        0 => "Remainder is 0".into(),
        1 => Cow::Borrowed("Remainder is 1"), // Same as that of above
        remainder => format!("Remainder is {}", remainder).into(),
    }
}

fn main() {
    for num in 1..=10 {
        match check_remainder(num) {
            Cow::Borrowed(msg) => println!("{num} is input. Cow is BORROWED msg = {msg}"),
            Cow::Owned(msg) => println!("{num} is input. Cow is OWNED.  msg = {msg}"),
        }
    }
}
