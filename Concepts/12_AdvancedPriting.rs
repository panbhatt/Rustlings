fn main() {
    let myName = "Lebron";
    let famName = " James";
    let daugther_name = "Keyla";

    println!(
        "My Name = {0} {1}. Daughter Name = {2} {1}",
        myName, famName, daugther_name
    );

    println!(
        "{city1} is in {country} ",
        city1 = "New Delhi",
        country = "India"
    );

    // Printing is like
    //{variable :padding alignment minimum.maximum}
    println!("{:_^11}", "*"); // Center aligned
    println!("{:_<11}", "*"); // Left aligned
    println!("{:_>11}", "*"); // Right aligned
}
