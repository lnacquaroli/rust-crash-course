#![deny(clippy::all)]

// Constants are defined at the top in upper case typed
const MY_AGE: u8 = 124;

fn main() {
    // Inmutable variables
    let last_name = "de Lioncourt";
    let first_name = "Lestat";
    println!("The name is {} and last name is {}", first_name, last_name);

    // Mutable variables: for same type of variables
    let mut user_name = "Pancho Villa";
    let mut age = 39;
    println!("The user {} has {} years old", user_name, age);
    user_name = "Zapata";
    age = 45;
    println!("The user {} has {} years old", user_name, age);

    // Other data types
    let red = 0xFA;
    let rgb = 0xF000000;
    let distance_1 = 5.5;
    let distance_2 = 2.1;
    let speed = 9;

    // Operators
    let time = distance_1 + distance_2;
    println!("Time required is {} s", time);

    // Shadowing: change the data (also data type, not recommended) for a certain variable
    let name = "Foo";
    let name = "Bar";
    let data = "Miami";
    let data = 10;
    {
        let name = name.to_string();
    }
    println!("name is now {}", name);

    let my_age = MY_AGE - 120;
    println!("My age is {} years old", my_age);

    // Tuples: collections in a way
    let personal_data = (24, "Ryder");
    let (age, knight) = personal_data;
    let age_0 = personal_data.0; // 0-based index in Rust
    let knight_0 = personal_data.1;
}
