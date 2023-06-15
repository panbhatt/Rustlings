// All Assert examples.

// Change the values of the variable

fn main() {
    let state = "Florida";
    let mut states_vec = vec!["Florida", "Texas", "NY", "CA", "WI"]; 

    assert!(state == "Florida", "State name should be {}", state); // The last two arguments are error message.

    assert_eq!(
        state, "Florida",
        "Assert_eq! State name should be {}",
        state
    ); // same as that of above, it is just it takes two arguments and  both of them should be equal.

    assert_eq!(states_vec.pop(), Some("WI")); // This will also pass, 


    assert_ne!(
        state, "Florida",
        "Assert_ne! State name should not be {}",
        state
    );
}
