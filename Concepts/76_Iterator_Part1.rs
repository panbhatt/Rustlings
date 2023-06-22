// Iterators .
// COllection of things that gives you one item at a time when you call .next() on it.

/*
Three type of Iterators.
.into_iter() -> iterator that owns its values = taking self.
.iter() -> iterator that references &self
.iter_mut() -> iterator of mutable references &mut self
*/

/*
.map()-> I
.for_each()  -> TO MODIFY IN PLACE
 */

fn main() {
    let odd_num = vec![1, 3, 5];
    //let odd_num_double = odd_num.iter().map(|x| x * 2 ); // This is not going to be an VECTOR too. it returns an iteator
    let odd_num_double = odd_num.iter().map(|x| x * 2).collect::<Vec<i32>>(); // This is not going to be an iterator as we have to collect the values. .

    println!("Vec is {:?} \nDouble is {:?}", odd_num, odd_num_double);

    let odd_num_triple = odd_num.into_iter().map(|x| x * 3).collect::<Vec<i32>>();
    println!("-> TRIPLE Vec is {:?}", odd_num_triple); // After this odd_num is not going to be available.

    //println!(" {:?}", odd_num); // This line will give an ERROR
    let mut even_vec = vec![2,4,6]; 
    even_vec.iter_mut().map(|x| *x *= 2); // This will take a mutable reference, so every change will be reflected. 

    println!("EVEN VEC Double = {:?}", even_vec); 



}
