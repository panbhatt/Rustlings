// LOOPS

fn main() {
    let mut i = 1;
    loop {
        // Indefinite loop.
        println!("{}", i);
        if i >= 5 {
            break;
        } else {
            i += 1;
        }
    }

    // Named LOOP
    let mut i = 0;
    'outer: loop {
        let mut j = 0;
        println!("");
        'inner: loop {
            j += 1;
            print!(" i = {}  j = {} <->", i, j);
            if j >= 5 {
                break 'outer; // This will break the OUTER Loop. so the outer loop is only going to run once.
            }
        }
        i += 1;
    }
}
