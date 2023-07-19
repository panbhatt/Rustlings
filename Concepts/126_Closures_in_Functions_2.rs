// ANother examples of passing closures in function.

#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u16>,
    populations: Vec<u32>,
}

impl City {
    fn new(name: &str, years: Vec<u16>, populations: Vec<u32>) -> City {
        Self {
            name: name.to_string(),
            years,
            populations,
        }
    }

    fn call_function<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Vec<u16>, &mut Vec<u32>),
    {
        println!("Calling function -> ");
        f(&mut self.years, &mut self.populations);
    }
}

fn main() {
    let mut dehradun = City::new(
        "Dehradun",
        vec![2021, 2022, 2023],
        vec![1500000, 2000000, 2500000],
    );
    println!("{:?}", dehradun);

    let print_fn = |yrs: &mut Vec<u16>, pol: &mut Vec<u32>| {
        let new_vec = yrs.iter().zip(pol.iter()).collect::<Vec<(_, _)>>();
        println!("{:?}", new_vec);
    };

    dehradun.call_function(print_fn);

    let year = 2024;
    let popl = 300000;
    let add_data_fn = move |yrs: &mut Vec<u16>, pol: &mut Vec<u32>| {
        yrs.push(year);
        pol.push(popl);
    };

    dehradun.call_function(add_data_fn);

    println!("============= AFTER ADDITION ====================");
    dehradun.call_function(print_fn);
}
