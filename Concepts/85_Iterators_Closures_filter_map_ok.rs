// OK() that converts from Result<T,E> to an Option<T>, discarding the error.

// FILTER MAP but with OK.

// Same as unwrap
// _ok_or()
// _ok_or_else() uuuuu

fn main() {
    let numbers = vec!["one", "two", "3", "4", "five", "6", "7"];

    let numbers_correct = numbers
        .iter()
        .filter_map(|nmber| {
            //nmber.parse::<i32>().ok() // This gives a result ok() convert it into an Option
            //nmber.parse::<i32>().ok().or(Some(-1)) // This will gives -1 where there is an Error.
            nmber.parse::<i32>().ok()
        })
        .collect::<Vec<_>>();

    println!(" {:?}", numbers_correct);
}

// Similarly we can use .ok_or() to return an Error too.
