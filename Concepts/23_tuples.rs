// TUPLES ->  ()

fn main() {
    let random_tuple = (10, 80, "Pankaj", [8, 1, 90], 34.33);
    //// Every tuple has its own type random_tuple.try_me_to_chk_my_type() ; // Just call this function to check the type of the tuple. Just like an Array but different data types.
    println!(" {:?} ", random_tuple);

    println!("{}", random_tuple.2); // Refers to the second element.

    let (my_name, friend_name) = ("Pankaj", "Rahul "); // Creating multiple variables and assigning different values of the tupe. Kindaa destructuring.

    println!("{} {}", friend_name, my_name);

    // Destructing in one line via using vector
    let vec_names = vec!["Pankaj", "Rahul", "Neeraj"];
    let (m_name, f_name, b_name) = (vec_names[0], vec_names[1], vec_names[2]);

    println!("{} {} {}", m_name, f_name, b_name);

    let (m_name1, f_name1, _) = (vec_names[0], vec_names[1], vec_names[2]); // Ignore the other extra values.

    println!("{} {} ", m_name1, f_name1);
}
