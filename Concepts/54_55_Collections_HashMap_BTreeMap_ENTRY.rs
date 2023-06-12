// https://doc.rust-lang.org/beta/std/collections/struct.HashMap.html

// Entry MEthod of HashMap, BTreeMap, it is just like Java.

fn main() {
    let mut my_string = "I loved Florida, Best state to live in";
    let mut hMap = std::collections::HashMap::new();

    for ch in my_string.chars() {
        let counter = hMap.entry(ch).or_insert(0); // This means it will check for the ENTRY (OCCUPIED ENTRY or vacant entry) and return the value
        // This insert ZERO above can be any type like VECTOR, Another HASHMAP. OR_INSERT it belongs to ENTRY and not the hashmap or btreemap. 
        *counter += 1;
    }

    for (k, v) in hMap {
        println!("{k} = {v}");
    }
}
I