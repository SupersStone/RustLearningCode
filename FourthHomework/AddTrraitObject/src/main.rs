use std::ops::Add;

// Define a simple struct Point, containing two integer fields x and y
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Create a trait for the operation we want to perform
trait Operate {
    fn operate(&self, other: Point) -> Point;
}

// Implement the trait for Point, enabling addition operation
impl Operate for Point {
    fn operate(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implement a function that accepts Trait Object as a parameter
fn add_points(a: &dyn Operate, b: Point) -> Point {
    a.operate(b)
}

fn main() {
    let point1 = Point { x: 10, y: 20 };
    let point2 = Point { x: 30, y: 40 };

    // Use Trait Object to call the operation method
    let result = add_points(&point1, point2);

    println!("Result: {:?}", result); // Output: Result: Point { x: 40, y: 60 }
}
