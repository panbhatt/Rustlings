// Marco's with different type of inputs, go to cheats.rs/#maros-attributes for checking differnet things.

// IDENT, EXPR and there are tons of others.

macro_rules! check {
    ($item:expr) => {
        // will take any expression.
        println!("You gave me a : {:?} ", $item);
    };
    ($input1 : ident, $input2 : expr) => {
        println!(
            "Is {:?} is equal to {:?} -> {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

fn main() {
    check!([7, 8]);
    check!(65 + 100);

    let x = 6;
    let my_vec = vec![1, 2, 3];

    check!(x, 6);
    check!(my_vec, vec![1, 2, 3]);
}
