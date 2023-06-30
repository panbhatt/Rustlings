// chunks()
// windows()

fn main() {
    // chunks will give a batch of elements once at a time .
    let my_numbers = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("==================== CHUNKS =========================");
    for chunk in my_numbers.chunks(3) {
        println!("{:?}", chunk);
    }

    println!("==================== WINDOWS ========================="); 
    for window in my_numbers.windows(3) {
        println!("{:?}", window);
    }
}
