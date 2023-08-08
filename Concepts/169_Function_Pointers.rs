// Function Pointers.

fn sum(i: usize, j: usize) -> usize {
    i + j
}

fn sub(i: usize, j: usize) -> usize {
    i - j
}

fn mul(i: usize, j: usize) -> usize {
    i * j
}

fn div(i: usize, j: usize) -> usize {
    i / j
}

// This is a place where we are able to pass a function as the argument.

fn calc(arth_fun: fn(usize, usize) -> usize, i: usize, j: usize) {
    println!("Calc = {}", arth_fun(i, j))
}

fn main() {
    // sum is the function pointer to a function that receives two input and one output
    calc(sum, 10, 20); // This will print 20 .
    calc(sub, 20, 10); // This will print 10
    calc(mul, 20, 10); // This will print 200
    calc(div, 20, 10); // This will print 2
}
