#![deny(clippy::all)]

use std::f32::consts::PI;

// Structures are useful to pack together related data
// Enumerations are useful to pack related objects, through types of structs
// They are sort of like a an asbtract type in julia

// To compare types inside the enum you have to derive PartialEq trait
#[derive(PartialEq)]
enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

// You can have enums with structures inside
enum Shapes {
    // Point is a tuple for instance
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}

// If you don't want to specify the names of the field inside the enum
struct Size {
    width: f32,
    height: f32,
}

enum Shapes2 {
    Rectangle(f32, f32, Size),
    Circle(f32, f32, f32),
}

// Implementation of enums with unnamed associated values
impl Shapes2 {
    // Method area with self
    fn area(&self) -> f32 {
        match self {
            Shapes2::Rectangle(x, y, size) => size.width * size.height,
            Shapes2::Circle(x, y, radius) => PI * radius * radius,
        }
    }
}

// Match is an expression, it can return a value and evaluate a value
enum Pet {
    Cat { name: String },
    Dog { name: String },
}

fn main() {
    // Creating an instance of enums: Hagrid's dog
    let fluffy = AnimalType::Dog;
    if fluffy == AnimalType::Dog {
        println!("Fluffy is a dog!");
    };

    // Switch cases or nested if-else are packed as a match in Rust
    match fluffy {
        AnimalType::Dog => println!("Wooof!"),
        AnimalType::Cat => println!("Meoww!"),
        AnimalType::Rabbit => println!("Hoot!"),
    };

    // If interested in only Dogs, you can catch all trick for the rest
    let chimera = AnimalType::Rabbit;
    match chimera {
        AnimalType::Dog => println!("Wooof!"),
        _ => println!("It doesn't bark!"),
    };

    // Create instances of Shapes
    let rectangle = Shapes::Rectangle {
        width: 3.0,
        height: 4.0,
    };

    // Comparing enums with associated values
    if let Shapes::Rectangle { width, height } = rectangle {
        println!(
            "wdith = {}, height = {}, area = {}",
            width,
            height,
            width * height
        );
    };

    match rectangle {
        Shapes::Rectangle { width, height } => {
            println!(
                "wdith = {}, height = {}, area = {}",
                width,
                height,
                width * height
            );
        }
        _ => println!("Not a rectangle"),
    };

    // Instance of the Shapes2
    let rectangle_2 = Shapes2::Rectangle(
        1.0,
        2.0,
        Size {
            width: 3.0,
            height: 4.0,
        },
    );

    // Compared unnamed associated values
    if let Shapes2::Rectangle(x, y, Size { width, height }) = rectangle_2 {
        println!("{}, {}, {}, {}", x, y, width, height);
    }

    match rectangle_2 {
        Shapes2::Rectangle(x, y, Size { width, height }) => {
            println!(
                "x = {}, y = {}, width = {}, height = {}",
                x, y, width, height,
            );
        }
        _ => println!("Not a rectangle"),
    };

    let area = rectangle_2.area();
    println!("Area of rectangle: {}", area);

    // Match expressions
    let fluffy2 = Pet::Cat {
        name: "Fluffly".to_string(),
    };
    // Return a value depending on the type
    let name = match fluffy2 {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };
    println!("The name of the pet is {}", name);
}
