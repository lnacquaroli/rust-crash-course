#![deny(clippy::all)]

fn say_hello_world() -> String {
    return String::from("Hello World!");

    // Last statement without semicolumn return also
    //String::from("Hello World!")
}

// It does not return anything but the print
fn say_hello_world_2() {
    let message = String::from("Hello World!");
    println!("{}", message);
}

// Functions that do not return any value (without specifying the type of the return value), are called unit type
// say_hello_world_2 is the same as say_hello_world_3
fn say_hello_world_3() -> () {
    let message = String::from("Hello World!");
    println!("{}", message);
}

fn print_message(message: &str) -> () {
    println!("{}", message);
}

fn say_hello_to_person(to_person: String) -> String {
    return format!("Hello {} !", to_person);
}

// Functions that takes as argument another function
fn process_name(name: &str, callback: fn(&str) -> ()) -> () {
    callback(name);
}

fn main() {
    let message = say_hello_world();
    println!("{}", message);

    say_hello_world_2();

    say_hello_world_3();

    print_message("Bye bye World!");

    let hello = say_hello_to_person(String::from("Lestat"));
    println!("{}", hello);

    // Inline functions: inside the pipes are the arguments of the function
    let say_hello_to_2 = |name: &str| format!("Hello {}!", name);
    let hello_2 = say_hello_to_2("Lestat");
    println!("{}", hello_2);

    // Inline functions with multiple arguments
    let full_name =
        |first_name: &str, last_name: &str| format!("Hello {}, {}!", first_name, last_name);
    let say_full_name = full_name("Lestat", "The Vampire");
    println!("{}", say_full_name);

    let multiply_by_2 = |x: i32| x * 2;
    let two_by_two = multiply_by_2(2);
    println!("2 x 2 is {}", two_by_two);

    // Inline functions with multiple calculations, need {}
    // let ask_for_age = || {
    //     // ask the user for age
    //     // calculate how old in 10 yrs
    // };

    // Assign a pointer to a function
    let ptr = multiply_by_2;
    let result = ptr(10);
    println!("The value is {}", result);

    let process_name_2 = process_name("Lestat", print_message);
    println!("Printed already :D");
}
