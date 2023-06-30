// FOLD Method

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];

    let sumOfAll = numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number); // Just like its javascript counterpart.

    println!("Total sum is -> {}", sumOfAll);

    let my_name = "Pankaj Bhatt";

    let new_name = my_name.chars().fold("-".to_string(), |mut string_so_far, next_char| {
        string_so_far.push(next_char);
        string_so_far.push('-');
        string_so_far
    });

    println!("New String is -> {}", new_name);
}
