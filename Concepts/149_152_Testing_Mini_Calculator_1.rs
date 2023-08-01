// How to run Cargo tests:
//rustc --test 149_152_Testing_Mini_Calculator_1.rs && ./149_152_Testing_Mini_Calculator_1

// This program basically takes an INPUT like "22 + 35 - 10 " and returns the return as i32.

const ALLOWED_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if !input
        .chars()
        .all(|character| ALLOWED_CHARACTERS.contains(character))
    {
        panic!("Please enter valid characters");
    }

    let input = input
        .trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|x| *x != ' ')
        .collect::<String>();

    // CHeck if the starting character is number. 
    if !input.chars().take(2).any(|c| c.is_numeric()) {
        panic!("Invalid characters at the start of the input"); 
    }

    println!(" Expression {} ", input);

    let mut result_vec = vec![];
    let mut push_string = String::new();

    for character in input.chars() {
        match character {
            '+' => {
                if !push_string.is_empty() {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                }
            }
            '-' => {
                if push_string.is_empty() || push_string.contains("-") {
                    push_string.push(character);
                } else {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(character);
                }
            }
            number => {
                if push_string.contains('-') {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(number);
                } else {
                    push_string.push(number);
                }
            }
        };
    }
    result_vec.push(push_string.clone()); 

    println!("{:?} \n {:?}", result_vec, push_string); 

    // Now Iterate through the VEC
    let mut total = 0 ; 
    let  mut operation_add = true ; 

    for entry in result_vec {
        if entry.contains('-') {
            if entry.chars().count() % 2 != 0 {
                operation_add = false  // change the operation. 
            }
            continue; 
        }
        
        if operation_add {
            total += entry.parse::<i32>().unwrap();
        } else {
            total -= entry.parse::<i32>().unwrap();
            operation_add = true ; 
        }

    }
    println!("= {}", total );
    total 
}

fn main() {
    math("1 - 25 ++++ 3 ++++");
    math("1 + 20 - 3 ++++-----  ");
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }

    #[test]
    fn test_25_minues_1() {
        assert_eq!(math("25 -  1"), 24);
    }

    #[test]
    #[should_panic]
    fn test_invalid_characters_at_start() {
        assert_eq!(math("25 *  1"), 24);
    }

    #[test]
    #[should_panic]
    fn test_invalid_characters() {
        assert_eq!(math("---2325 *  1"), 24);
    }

    #[test]
    fn test_Nine_plus_nine_minus_nine_minus_nine() {
        assert_eq!(math("9+9-9-9"), 0 ); 
    }
}
