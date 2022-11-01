#![deny(clippy::all)]

// Ownership is a topic related to strings mostly
// Look at stack and heap memmory
// A String object is both a stack (LIFO) and heap (Random)
// Stack holds the pointer (ptr), length (len) and capacity
// Heap holds the data in pointer (ptr)
// Pointers are not like in C

// Carefull to the &String and not String
fn greet(name: &String) {
    println!("Hello, {}!", name);
}

// You cannot clear from a reference (borrowed) variable, so you need &mut to get a mutable reference that we can clear.
fn empty_string(value: &mut String) {
    value.clear();
}

// Dangling reference: lifetime specifier missing. You can't do this.
fn get_name() -> &String {
    &"John".to_string()
}

fn main() {
    let name1 = String::from("John");
    let name2: String = name1;

    // name2 takes ownership on name1, and destroys it
    // Both variables cannot free data (deallocate), so Rust prevents (automatic memmory allocation) this by taking ownership of name1 into name2. Cannot have two variables pointing at the same data in the heap. Concept of moving using a trait called Copy: name1 is moved to name2.
    // Stack values copy (move) by default.

    // println!("Hello {}", name1);
    println!("Hello {}", name2);

    // Block of code: Rust drops variables out of scope
    {
        let name: String = "John".to_string();
        println!("Hello {}", name);
    }
    // After the block there is no access to variable name

    // To prevent moving we can use references, which points to a valid data (data in the heap that sits in the stack) as read-only view. Looks like a @view from Julia.
    let s1 = String::from("Johnny");
    let s2: &String = &s1;
    println!("Hello {}", s1);
    println!("Hello {}", s2);
    // References don't drop variables name, So s1 is still valid now on.

    // This works fine with integers
    let age1 = 10;
    let age2 = age1;
    println!("Your age is {}", age1);
    println!("Your age is {}", age2);

    // Using the function
    let s1 = String::from("JJ");
    let s2: &String = &s1;
    greet(&s1);
    greet(s2);

    // References can be mutable and immutable
    let mut name_to_empty = String::from("Lestat");
    empty_string(&mut name_to_empty);

    // You can have at most ONE mutable reference at a time to a variable
    // This prevent data races
    let mut name2 = &mut name_to_empty;
    //let mut name3 = &mut name_to_empty;
    empty_string(name2);

    // Don't need mutable references while immutable references are there
    // Do to memmory management
    let mut name1 = String::from("Marius");
    let name2 = &name1;
    let mut name3 = &mut name1;
    println!("Hello {}", name1);
    println!("Hello {}", name2);
    println!("Hello {}", name3);
}
