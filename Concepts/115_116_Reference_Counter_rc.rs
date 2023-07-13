// RC -> Reference Counter.

// Only one owner for a piece of data. Its like an Accountant that keeps track of Reference fo the object.

use std::rc::Rc; 

struct Book {
    name: String,
}

impl Book {
    fn print_name(&self) {
        println!("Book Name -> {}", self.name);
    }
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    history: String,
}

#[derive(Debug)]
struct CityHistory {
    names: Vec<String>,
    histories: Vec<String>,
}

// Below two are RC variations of CITY / CityHistory. s
#[derive(Debug)]
struct RCity {
    name: Rc<String>,
    population: u32,
    history: Rc<String>,
}

#[derive(Debug)]
struct RCityHistory {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let my_book = Book {
        name: String::from("HAMLET"),
    };

    // The below two statements are same and we have to use second statement while doing RC.
    my_book.print_name();
    Book::print_name(&my_book);

    println!("==============================================");

    let boca_raton = City {
        name: "Boca Raton".to_string(),
        population: 1_200_00,
        history: "One of the cities where old people love to live".to_string(),
    };

    let c_h = CityHistory {
        names: vec![boca_raton.name],
        histories: vec![boca_raton.history],
    };

    println!("Boca Raton population is {}", boca_raton.population);
    //println!("Boca Raton details are {:?}", boca_raton);  This wont be printed as the value of name and history has been moved.

    println!("==============================RC Reference Counter =============================="); 
    let delray_beach = RCity {
        name: Rc::new("Delray Beach".to_string()),
        population: 1_8000_00,
        history: Rc::new("Best city where I would love to live".to_string()),
    };

    let c_h = RCityHistory {
        names: vec![Rc::clone(&delray_beach.name)],
        histories: vec![Rc::clone(&delray_beach.history)],
    };

    println!("Delray Beach Details -> {:?}", delray_beach); // Now we can use it without problem. 
    // RC is just another smart pointer to the data . 
}
