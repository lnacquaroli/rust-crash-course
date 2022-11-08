#![deny(clippy::all)]

// Optionals: are expressed using Option enum
// They can have either a value or not
// Options and optionals are used interchangeably in Rust (same thing)

fn main() {
    let _value = Some(10);
    let _name = Option::<&str>::None;

    // Unwrapping options safely
    let name: Option<&str> = None;
    match name {
        Some(name) => println!("Hello, {}", name),
        None => println!("There is no name"),
    };
    // Unsafely unwrap a value: it will panic with the msg provided
    // let unwrapped_name = name.expect("Was not provided");
    //println!("{}", unwrapped_name);
    // Unsafely unwrap a value without a msg if it's None
    //let unwrapped_value_2 = name.unwrap();

    // Mutating option values
    let mut age = Some(20);
    match age.as_mut() {
        Some(age) => *age += 1,
        None => println!("No age"), // we can use None => {},
    };
    println!("Age is: {}", age.unwrap());

    // Unwrapping multiple optionals with tuples
    let age1 = Some(20);
    let age2 = Some(22);
    let age3 = Some(24);
    if let (Some(age1), Some(age2), Some(age3)) = (age1, age2, age3) {
        println!("Sum of ages is: {}", age1 + age2 + age3);
    };

    // Unwrap with default value if it's None
    let name: Option<&str> = None;
    let unwrap_name = name.unwrap_or("Milena Doe");
    println!("The name is {}", unwrap_name);

    // Unwrap with functions, below it will complain because the fn is just like a default
    let name: Option<&str> = None;
    let _unwrap_name = name.unwrap_or_else(|| "Annalisa");

    // Check if Option is None or Some
    let name: Option<&str> = None;
    if name.is_some() {
        println!("The is a value");
    } else {
        println!("The is no value");
    }
    if name.is_none() {
        println!("The is no value");
    } else {
        println!("The is a value");
    };

    // Unwrap with default value
    let age: Option<i32> = None;
    let age = age.unwrap_or_default();
    println!("The age is: {}", age);

    // Mapping an option
    let age: Option<i32> = Some(20);
    let age_times_2 = age.map(|age| age * 2);
    println!("The age times 2 is: {}", age_times_2.unwrap_or_default());

    // Functions can return an Option as well
}
