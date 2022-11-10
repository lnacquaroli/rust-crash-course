#![deny(clippy::all)]

use std::ops::{Add, AddAssign}; //+=

// Generics
// About reusing code

// This struct accepts i32 but not any other, so you may want to use a generic
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
// Both value types must match
// This PointT accepts strings for instance as well, which might not be useful as a point coordinate
struct PointT<T> {
    x: T,
    y: T,
}

// This will work for any T
// impl<T> PointT<T> {
//     fn move_offset(&mut self, x: T, y: T) {
//         self.x += x; // At compile time it does not know what type is
//         self.y += y;
//     }
// }

// This will work for the right data type that conforms to the specific trait
impl<T> PointT<T> {
    fn move_offset_2(&mut self, x: T, y: T)
    where
        T: AddAssign, // Trait
    {
        self.x += x; // At compile time it does not know what type is
        self.y += y;
    }
}

// Can implement AddAssign on PointT itself
impl<T: AddAssign> AddAssign for PointT<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Implement partialeq for PointT
impl<T: PartialEq> PartialEq for PointT<T> {
    // &self: current instance of the self
    // &Self: current instance of the self type
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// Create a trait CanRun
trait CanRun {
    fn run(&self);
}

trait CanWalk {
    fn walk(&self);
}

// If all the elements of the vector CanRun, the whole vector CanRun
impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run()
        }
    }
}

// If all the elements of the vector CanWalk, the whole vector CanWalk
impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk()
        }
    }
}

struct Person {
    name: String,
}

impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}

struct Elephant {
    name: String,
}

impl CanWalk for Elephant {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

fn main() {
    let p1 = Point { x: 0, y: 7 };

    let p1 = PointT { x: 0, y: 7 };
    let p2 = PointT { x: 0.1, y: 7.1 };
    let p3 = PointT { x: "Foo", y: "Bar" };

    let mut p = PointT { x: 1, y: 2 };
    p.move_offset_2(3, 4);
    println!("{:?}", p);

    let mut point1 = PointT { x: 1.0, y: 2.0 };
    let point2 = PointT { x: 1.0, y: 2.0 };
    // point1 += point2; // Because of the impl AddAssign on PointT
    println!("{:?}", point1);
    if point1 == point2 {
        println!("point1 and point2 are equal")
    } else {
        println!("point1 and point2 are not equal")
    };

    // Traits and generics go hand in hand
    let people = vec![
        Person {
            name: "John".to_string(),
        },
        Person {
            name: "Jane".to_string(),
        },
        Person {
            name: "Joe".to_string(),
        },
    ];
    people.run();
    people.walk();

    let elephants = vec![
        Elephant {
            name: "elephant1".to_string(),
        },
        Elephant {
            name: "elephant2".to_string(),
        },
        Elephant {
            name: "elephant3".to_string(),
        },
    ];
    elephants.walk();
    // elephants.run(); // won't compile, do not conform to CanRun
}
