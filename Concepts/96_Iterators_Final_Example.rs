// This program will break a vec of names by the WORDS present in the name and then create a STRUCT

#[derive(Debug)]
struct Names {
    one_word: Vec<String>,
    two_words: Vec<String>,
    three_words: Vec<String>,
}

fn main() {
    let personalities_names = vec![
        "Mohandas Karamchand Gandhi".to_string(),
        "Nelson Mandel".to_string(),
        "Barak Hussain Obama".to_string(),
        "Charlie Chaplin".to_string(),
        "Tiger".to_string(),
        "Shahrukh khan".to_string(),
    ];

    let mut names = Names {
        one_word: vec![],
        two_words: vec![],
        three_words: vec![],
    };

    let mut personalities_itr = personalities_names.iter().peekable();

    // Here You can also use split_whitespace() to split and check. 

    while personalities_itr.peek().is_some() {
        let name = personalities_itr.peek().unwrap();
        match name.match_indices(' ').collect::<Vec<_>>().len() {
            0 => names.one_word.push(name.to_string()),
            1 => names.two_words.push(name.to_string()),
            2 => names.three_words.push(name.to_string()),
            _ => println!("I DONT KNOW what to do "),
        }
        personalities_itr.next();
    }

    println!("{:?}", names);
}
