fn main() {
    //   a(); // Enable this line and run via RUST_BACKTRACE=1 cargo run and you will see a stack trace of failure. 
}

fn a() {
    b();
}

fn b() {
    c(0);
}

fn c(num :i32) {
    if num == 0 {
        panic!("you are never supposed to pass zero to this function."); 
    }
}
