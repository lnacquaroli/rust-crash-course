#![deny(clippy::all)]

// Structs are the primary way of grouping data, structuring your code

// Debug trait here is a debug description of a struct or tuple
// Need to add the line below for it
#[derive(Debug)]
// Implementations blocks for structs
struct Point(f64, f64, f64);

// Add describe function to point to the Point struct implementation
impl Point {
    // Functions with self argument are methods (python classes resemblance)
    fn describe(&self) {
        println!(
            "Point is at: x1 = {}, x2 = {}, x3 = {}",
            self.0, self.1, self.2
        );
    }

    // Non-methods associated with functions don't need self, they work at a higher level
    // Non-method associated function to creates all zeros Point
    // (staticmethod in Python)
    fn origin_point() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

// Can organize implementations in different blocks depending on their content
impl Point {
    // This function does not mutate the input Point
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }

    // This function does mutate the input Point
    fn twice_inplace(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
}

fn main() {
    // Create an instance of Point
    let mut point = Point(0.0, 1.2, -2.4);
    point.describe();

    let twice_point = point.twice();
    twice_point.describe();

    //Debug the point
    point.twice_inplace();
    println!("Debug trait: {:?}", point);

    // mut and immut
    let mut point_mut = Point(1.0, 2.0, 3.0);
    let point_not_mut = point.twice();
    point_mut.twice_inplace();
    point_not_mut.describe();

    // Non-method funcs: staticmethod in Python
    let point_0 = Point::origin_point();
    let point_1 = Point::origin_point();
    let point_2 = Point::origin_point();

    point_0.describe();
    point_1.describe();
    point_2.describe();
}
