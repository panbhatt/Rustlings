// FILTER MAP .
// Make sure that the filter_map returns OPTION, otherwise it is going to give error. 
// or we can hae an VEC of MONTHS. 

fn main() {
    let  months = vec![
        "Jan", "Febuary", "Mar", "April", "May", "Jun", "July", "Aug", "Sep", "October", "Nov",
        "Dec",
    ];

    let months_nam_greater_then_4 = months
        .iter()
        .filter_map(|mth| {
            if mth.len() > 4 {
                Option::Some(mth)
            } else {
                Option::None
            }})
        .collect::<Vec<_>>();

    println!(" {:?}", months_nam_greater_then_4);
}
