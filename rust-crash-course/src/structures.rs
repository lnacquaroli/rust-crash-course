#![deny(clippy::all)]

// Rust used to have classes, but not anymore. Now structs and traits

// Define a structure with two fields, similar to Julia
struct Person {
    name: String,
    age: u8,
}

// Field init shorthand: not use the field names if the variables have the same names
fn create_person(name: String, age: u8) -> Person {
    return Person { name, age };
}

// Tuples are created with the struct keyword with parenthesis
struct Point(f64, f64, f64);

fn main() {
    // Create an instance of Person
    let person_0 = Person {
        name: "John".to_string(),
        age: 30,
    };
    println!("{} is {} years old.", person_0.name, person_0.age);

    let person_1 = create_person("Louis".to_string(), 143);
    println!(
        "{} the vampire is {} years old.",
        person_1.name, person_1.age
    );

    // Struct update fields: if you want to use a field value from another instance you can pass a ..variable_name and it will update the new instance with the remaining fields after the one introduced. Tricky with strings.
    let person_2 = Person {
        name: "Marius".to_string(),
        ..person_1
    };
    println!(
        "{} the vampire is {} years old.",
        person_2.name, person_2.age
    );

    // An instance of a tuple is also created with parenthesis
    let point = Point(0.0, -1.0, 2.0);
    println!(
        "Point coordinatess: x = {}, y = {}, z = {}",
        point.0, point.1, point.2
    );
}
