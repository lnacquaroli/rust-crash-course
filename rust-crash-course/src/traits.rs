#![deny(clippy::all)]

use std::fmt;

// Traits
// Shared functionalities, like conformed interfaces or protocols
//

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// Create a display implementation of Person
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}

// We can create a trait our own
trait HasFullName {
    fn full_name(&self) -> String;
}

// Implementing a trait
impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// Trait with "new" function
trait CanInitializeWithFullName {
    fn new(full_name: &str, age: u8) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str, age: u8) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age,
        }
    }
}

// Traits as parameters in a function
fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name())
}

// Trait bound syntax: same as trait as parameter
// This is more julia like
fn print_details<T: HasFullName>(value: &T) {
    println!("{}", value.full_name())
}

fn print_details_run<T: HasFullName + CanRun>(value: &T) {
    println!("{}", value.full_name());
    value.run();
}

// Trailing trait bounds using where: more julia-ish
fn print_details_run_where<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name());
    value.run();
}
// Conforme to multiple traits
trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        // to do
    }
}

// Traits can be implemented in other traits: binding trait to another
// If an object is HasName it's also HasFullName2
trait HasFullName2
where
    Self: HasName,
{
    // One way of doing it
    // fn full_name(&self) -> String {
    //     format!("{} {}", self.first_name(), self.last_name())
    // }

    //Another way is to call the fn and implemented below
    fn full_name_2(&self) -> String;
}

impl<T> HasFullName2 for T
where
    T: HasName,
{
    fn full_name_2(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}

trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.first_name
    }
}
fn main() {
    let person = Person {
        first_name: "Lestat".to_string(),
        last_name: "Vampire".to_string(),
        age: 251,
    };

    // Print signature with Debug trait
    println!("{:?}", person);

    // Own traits
    let person_new = Person::new("Marky Ramone", 57);
    println!(
        "First name : {}, Last name : {}, age: {}",
        person_new.first_name, person_new.last_name, person_new.age
    );

    // Print using the display trait
    println!("{}", person_new);

    // Traits as paramters
    print_full_name_and_age(&person);

    // Traits bound syntax
    print_details(&person_new);

    //
    let full_name = person_new.full_name_2();
    println!("{}", full_name);
}
