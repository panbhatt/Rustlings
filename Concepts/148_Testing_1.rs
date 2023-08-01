// How to run Cargo tests:
//rustc --test 148_Testing_1.rs && ./148_Testing_1

fn is_even(input: i32) -> bool {
    input % 2 == 0
}

fn is_odd(input: i32) -> bool {
    input % 2 == 1
}

fn main() {
    println!("HELLO WORLD "); 
}

#[cfg(test)]  // This ti used to ignore test once u running main function via CTRL_ALT_N or cargo run. 
mod tests {

    use super::*; 

    #[test]
    fn test_is_even() {
        assert!(is_even(4) == true);
        assert_eq!(is_even(10), true);
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(3) == true);
        assert_eq!(is_odd(101), true);
    }
}

// When tests are at the same scope.
#[test]
fn test_is_even() {
    assert!(is_even(4) == true);
    assert_eq!(is_even(10), true);
}

#[test]
fn test_is_odd() {
    assert!(is_odd(3) == true);
    assert_eq!(is_odd(101), true);
}
