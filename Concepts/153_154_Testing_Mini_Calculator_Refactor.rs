// How to run Cargo tests:
//rustc --test 153_154_Testing_Mini_Calculator_Refactor.rs && ./153_154_Testing_Mini_Calculator_Refactor

// This program basically takes an INPUT like "22 + 35 - 10 " and returns the return as i32.

//This is struct based implementation of the calculator . 

#[derive(Debug  )]
struct Calculator {
    results : Vec<String> ,
    current_input : String, 
    total : i32, 
    operation_add : bool, 
}

impl Calculator {
    fn new() -> Self {
        Self {
            results : vec![],
            current_input : String::from(""), 
            total : 0, 
            operation_add : true, 
        }
    }

    fn clear(&mut self) {
        self.current_input.clear(); 
    }

    fn push_char(&mut self, ch : char) {
        self.current_input.push(ch);
    }
}

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

    let mut calculator = Calculator::new(); 

    for character in input.chars() {
        match character {
            '+' => {
                if !calculator.current_input.is_empty() {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                }
            }
            '-' => {
                if calculator.current_input.is_empty() || calculator.current_input.contains("-") {
                    calculator.push_char(character);
                } else {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                    calculator.push_char(character);
                }
            }
            number => {
                if calculator.current_input.contains('-') {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.clear();
                    calculator.push_char(number);
                } else {
                    calculator.push_char(number);
                }
            }
        };
    }
    calculator.results.push(calculator.current_input.clone()); 

    println!("{:?} \n {:?}", calculator.results, calculator.current_input); 


    for entry in calculator.results {
        if entry.contains('-') {
            if entry.chars().count() % 2 != 0 {
                calculator.operation_add = false  // change the operation. 
            }
            continue; 
        }
        
        if calculator.operation_add {
            calculator.total += entry.parse::<i32>().unwrap();
        } else {
            calculator.total -= entry.parse::<i32>().unwrap();
            calculator.operation_add = true ; 
        }

    }
    println!("= {}", calculator.total );
    calculator.total 
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
