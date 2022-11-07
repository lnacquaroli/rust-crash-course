#![deny(clippy::all)]

// Collections: Tuples, Vectors, HashMap, Iterators

use std::collections::HashMap;

// Tuples area heterogeneous collections: unnamed of grouped data on the go
struct Point(f32, f32);

fn get_values() -> (String, String, i32) {
    ("Hello".to_string(), "World".to_string(), 30)
}

// Traits for the hashing tables
#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // Tuples
    // Heterogeneous formation
    let values = ("Hello", "World", 30);
    // Unpacking tuples
    let hello = values.0;
    let world = values.1;
    println!("{} {}!", hello, world);

    let (hello, world, age) = values;
    println!("{} {}, you are {} yo!", hello, world, age);
    // Unpack world only
    let (_, world, _) = values;
    println!("Only {} from the tuple", world);

    // From function
    let values2 = get_values();
    let (_, world, _) = get_values();

    // Vectors
    // Vectors are the arrays in other languages
    // They contain elements of the same type
    // Fix size vectors indicating the type and the number of elements
    let vector: [&str; 2] = ["foo", "bar"];
    // Iterrating over a vector
    for v in vector.iter() {
        println!("{}", v);
    }
    // Read values at specific indexes
    let foo = &vector[0];
    println!("Foo is {}", foo);
    // Length of the vector
    let length = vector.len();
    // Map values in the vector
    let values3: [i32; 2] = [10, 20];
    let values3_times_2 = values3.iter().map(|x| x * 2);
    for v in values3_times_2 {
        println!("{}", v);
    }
    // Create a vector with macro
    let vector2 = vec![1, 2, 3, 4, 5];
    // Mutable vectors
    let mut vector3 = vec![1, 2, 3, 4, 5];
    // Add to the end with push and remove last with pop
    vector3.push(6);
    vector3.push(7);
    vector3.pop();
    // Remove all values from a vector
    println!("vector3 values are {:?}", vector3);
    vector3.clear();
    println!("vector3 values are {:?}", vector3);
    // Clone and append to a mutable vector
    let mut vector4 = vec![1, 2, 3];
    println!("vector4 values are {:?}", vector4);
    vector4.extend_from_slice(&[4, 5, 6]);
    println!("vector4 values are {:?}", vector4);
    let mut vector4 = vec![1, 2, 3];
    let mut vector5 = vec![4, 5, 6];
    println!("vector4 = {:?}", vector4);
    println!("vector5 = {:?}", vector5);
    // Append will clear (move) the appended vector
    vector4.append(&mut vector5);
    println!("vector4 = {:?}", vector4);
    println!("vector5 = {:?}", vector5);
    // Test if a vector contains a value (4)
    if vector4.contains(&4) {
        println!("Yes!");
    } else {
        println!("No!");
    };
    // Test if a vector is empty
    vector4.clear();
    if vector4.is_empty() {
        println!("It's empty!");
    } else {
        println!("It's not empty!");
    };

    // HashMaps: dictionaries in other languages (key, value)
    // Need to load the HashMaps to use them
    let mut dict: HashMap<&str, &str> = HashMap::new();
    dict.insert("foo", "bar");
    println!("Hash map - dict is {:?}", dict);
    // Check existence of a key inside the hashmap
    if dict.contains_key("name") {
        println!("name exists!")
    } else {
        println!("name does not exists!")
    }
    // To remove a key
    dict.remove("foo");
    println!("Hash map - dict is {:?}", dict);
    // Unsafely (assuming keys exit) read values from HashMap
    dict.insert("foo", "bar");
    let bar = dict["foo"];
    println!("bar is {}", bar);
    // Safely read values from HashMap
    let bar2 = dict.get("foo");
    println!("bar is {}", bar);
    match dict.get("foo") {
        Some(value) => println!("The value is {}", value),
        None => println!("Key not found..."),
    };
    // Iterate over hashmaps using references
    for (&v, &k) in &dict {
        println!("{} {}", k, v);
    }
    // Retrieve an entry: usually more common to retrive as above though
    // They can be matched after
    let entry = dict.entry("foo");
    // Safely insert a key if it doesn't exists
    dict.entry("name").or_insert("Jane Doe");
    println!("Hash map - dict is {:?}", dict);
    // Insert custom structures inside a hashmap
    let mut dict2: HashMap<Person, &str> = HashMap::new();

    // Iterators: are not collections perse
    // List that you can go through: lazy, need to be consumed to do something
    // Needs a trait for it called iterator
    let values = vec![1, 2, 3, 4, 5];
    for value in values.iter() {
        println!("{}", value);
    }
    let iter = values.iter();
    let sum1: i32 = iter.sum();
    // let sum2 = iter.sum(); // Iterators CANNOT be consumed twice
    // Mapping iterators and collect them
    let values_times_2: Vec<i32> = values.iter().map(|x| x * 2).collect();
    println!("{:?}", values_times_2);
    // Unowned iterators: iter() goes through references
    let names = vec!["Lestat", "Marius", "Queen", "Milena"];
    for name in names.iter() {
        println!("{}", name);
    }
    // Owned values out of the iterator
    for name in names.into_iter() {
        println!("{}", name);
    }
    // Filter the iteration
    let names = vec!["Lestat", "Marius", "Queen", "Milena"];
    for name in names.iter().filter(|name| name.len() == 6) {
        println!("{}", name);
    }
    // Jump over the iterations
    for name in names.iter().filter(|name| name.len() == 6) {
        if name.len() != 6 {
            continue;
        }
        println!("{}", name);
    }
    // Break out of the iteration
    for name in names.iter() {
        if name.len() != 6 {
            break;
        }
        println!("{}", name);
    }
}
