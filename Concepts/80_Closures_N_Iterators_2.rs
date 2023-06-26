// .enumerate() over iterator.

fn main() {
    let my_cars = vec!["Bugati", "Rolls Royce", "AUDI", "BMW"];

    my_cars
        .iter()
        .enumerate()
        .for_each(|(index, car)| println!("{index}   {car}"));

        let new_cars = my_cars
        .iter()
        .enumerate()
        .map(|(index, car)| (index + 1 ,car)).collect::<Vec<_>>(); // Usuage of closures with Iterators

    println!("{:?}", new_cars); 
}
