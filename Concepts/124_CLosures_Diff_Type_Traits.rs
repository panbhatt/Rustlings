// This pRoram will show different type of Traits of closures.

// Fn -> Most powerful and generally choose this. can take by reference. Also include FnMut and FnOnce - Octopus Paw
// FnMut -> Can modify it.  - Squirrel paw
// FnOnce -> u can only use it once. what is passed in and drop it i..e destroys it. - Cat PAW

// This code will show different types of CLosures.

fn main() {
    let mut my_string = String::from("Pankaj Bhatt");

    let refs_it = || println!("REFS IT -> {}", my_string); // This is FNOnce
    refs_it();
    let mut muts_it = || {
        my_string.push('!');
        println!("MUTS IT -> {}", my_string);
    }; // FN Mut

    muts_it();

    let drops_it = || {
        println!("Dropping String -> {}", my_string);
        std::mem::drop(my_string);
    }; // Fn
    drops_it();

    /*refs_it();
    muts_it();
    drops_it(); */

    // if we remove the above comments and remove calling refs_it() and muts_it() it will give us an error.
}
