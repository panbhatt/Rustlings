// Type Alias and New Types.

// Type Alias = new Name for the exact same type

type MyString = String;

type MyVec = Vec<Vec<Vec<Vec<[i32; 4]>>>>; // This is real type alias.

#[derive(Debug)]
struct MyCustomString(String); // This is going to be a TUPLE.

fn main() {
    let name = MyString::from("hello world");
    let name_str = String::from("hello world");
    println!("{}", name);

    if name == name_str {
        println!("Both Strings are equal");
    }

    println!("=============== CUSTOM STRING ====================");

    let my_str = MyCustomString(String::from("hello world"));
    println!("Custom String -> {:?}", my_str);
    println!("First Element -> {}", my_str.0)
}
