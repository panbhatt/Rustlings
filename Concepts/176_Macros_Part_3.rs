// Print ANything Macro. 

macro_rules! print_anything {
    ($input : tt) => {
        let output = stringify!($input);
        println!("{}", output);
    };
    ($($input : tt), * ) => {  // This way you can take multiple inputs. * means any number of inputs. , + means at least one
        let output = stringify!($($input)*); 
        println!("Coming here with value => {}",output); 
    };
}

fn main() {
    println!("Stringify example -> {}", stringify!(1233u64 + 45.65));  // -> This stringifies everything. 
    print_anything!("pankaj Bhatt");

    print_anything!(Pankaj); 
    print_anything!('a'); 

    print_anything!("Pankaj", "Bhatt" , 89898);
}