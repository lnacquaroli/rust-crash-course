#![deny(clippy::all)]

// Lifetimes
// Works with references to types
// Fundamental to know how Rust manages memory
// How Rust is trying not to be like C: it will deallocate autom and will not double allocations

// Instance of string works
fn _get_full_name_string() -> String {
    "John Doe".to_string()
}

// Function that won't compile: returning a reference to string
// The string is created somewhere in the heap and we are trying to returning reference
// fn get_full_name() -> &str {
//     "John Doe"
// }
// Need to tell Rust how long &str lives: &'static str will tell Rust that the string will live for the entire application in the heap or data stack
fn get_full_name() -> &'static str {
    "John Doe"
}

// As long as a and b are living, need to tell Rust that the return value also lives on
// Do this with generic lifetime specifiers <'ls_a, 'ls_b>
// Output lifetimes need to match with the signature of fn
fn get_random_name<'ls_a, 'ls_b>(a: &'ls_a str, _b: &'ls_b str) -> &'ls_a str {
    a
}

// Can use one lifetime specifier for all
fn get_random_name_2<'ls>(_a: &'ls str, b: &'ls str) -> &'ls str {
    b
}

// Missing lifetimes annotations in struct
// struct Person {
//     name: &str,
// }

// Specify lifetimes annotations in struct
struct Person<'ls> {
    name: &'ls str,
}
// Also valid, but less recommended(?)
struct Person2 {
    name: &'static str,
}

// Lifetime elision

// Lifetime #1: Compiler assigns lifetime to every parameter that is a reference
// Lifetime #2: Single input lifetime is assigned to all outputs
// Lifetime #3: If &self and &mut self in input arguments, lifetime or self is assigned to output

// Rust automatically applies the lifetime specifiers to the argument and signature, but they are hidden
fn _get_full_name_elision(full_name: &str) -> &str {
    full_name
}

// PersonName instance and the &str should live on for as long the instance lives on
struct PersonName<'ls_a> {
    first_name: &'ls_a str,
    last_name: &'ls_a str,
}

// Create an implementation with life speficier
impl<'ls_a> PersonName<'ls_a> {
    // Lifetime #3
    fn first_char_or_first_name(&self) -> &str {
        &self.first_name[0..1]
    }

    // Can't use &str to return full name
    fn return_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// Lifetime in enums
enum Animal<'ls_a> {
    Dog { name: &'ls_a str },
}

fn main() {
    let full_name = get_full_name();
    println!("Name is: {}", full_name);

    // main fn owns a and b
    let random_name = get_random_name("Marie", "Curie");
    println!("Name is: {}", random_name);
    let random_name = get_random_name_2("Marie", "Curie");
    println!("Name is: {}", random_name);

    let person = Person { name: "Faraday" };
    println!("Name is: {}", person.name);
    let person2 = Person2 { name: "Gauss" };
    println!("Name is: {}", person2.name);
}
