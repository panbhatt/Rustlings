// GENERIC Types.

fn main() {
    let my_numbers = vec![10, 50, 90, 20, 100, 500, 3223, 12];
    let my_chars = vec!['A', 'B', 'C', 'D', 'E', 'F'];

    println!("Largest of Numbers = {}", get_largest(my_numbers));
    println!("Largest of CHARACTERS = {}", get_largest(my_chars));

    println!("======================= ANOTHER EXAMPLE ====================");
    let p = Point { x: 10, y: 20 };
    println!("x = {} , y = {}", p.x, p.y);
    let char_p = Point { x: 'P', y: 'B' };
    println!("Point as String = {} ", char_p.str()); // But this method is not available on the Point P.

    let p3 = p.mix_up(char_p);
    println!("P3 x = {} y = {}", p3.x, p3.y);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for i in 1..list.len() {
        if largest < list[i] {
            largest = list[i];
        }
    }
    return largest;
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// THis method is not available on POINT<char,char>
impl Point<char, char> {
    fn str(&self) -> String {
        let point_as_string = format!("({},{})", &self.x, &self.y);
        point_as_string
    }
}
