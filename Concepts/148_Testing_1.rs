
// How to run Cargo tests: 
//rustc --test 148_Testing_1.rs && ./148_Testing_1   


fn is_even(input :i32) -> bool {
    input % 2 == 0 
}

fn is_odd(input :i32) -> bool {
    input % 2 == 1
}

#[test]
fn test_is_even(){
    assert!(is_even(4) == true ); 
}

#[test]
fn test_is_odd(){
    assert!(is_odd(3) == true); 
}