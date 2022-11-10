#![deny(clippy::all)]

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

// Pointers: Box, Rc, Cell, RefCell
// Stack vs heap
// Stack is LIFO, while heap is random access

// Implement a Box
struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

// Implement Deref for BoxedValue
// Deref reads the pointer internal value stored by the Box itself
impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn print_integer(value: &i32) {
    println!("{}", value)
}

// Cell allows internal mutability: usually avoided by Rust people (unsafe)
struct Person {
    name: String,
    age: Cell<u8>, // kind of a pointer, bit dangerous to use for mutability
}

impl Person {
    fn increment_age(&self) -> u8 {
        self.age.set(self.age.get() + 1);
        self.age.get()
    }
}

fn main() {
    // Box data type: stores values in the HEAP
    let age = Box::new(22);
    let twice = *age * 2;
    println!("{}", twice);

    // Point to the underlying dereferenced value
    let age2 = BoxedValue::new(22);
    // Actual value
    let actual_age = *age2;
    // *ptr is a shorthand for *(ptr.deref())
    let _other = *(age2.deref());
    println!("Value is: {}", actual_age);

    // Pointing to the dereferenced value as reference type
    let actual_age = age.deref(); // returns a reference type value
    println!("Value is: {}", actual_age);

    // Creating a reference to the Box value is passing the reference to the type
    let value2 = BoxedValue::new(10);
    print_integer(&value2);
    let value2_ref = &value2;
    print_integer(value2_ref);

    // Rc pointer: reference counter
    // Rc disallows mutation of the wrapped value
    // It can modify the reference
    // Not suitable for multithreading env
    let array = vec!["John".to_string(), "Jane".to_string()];
    let rc = Rc::new(array);
    let rc2 = rc.clone(); //Rc::clone
                          // Weak reference to the vector: once Rc loses its value you cannot access the reference
    let weak = Rc::downgrade(&rc);
    // drop removes the value
    //drop(rc);
    // This will panic because weak cannot hold on the underlying data
    //let value3 = weak.upgrade().unwrap();
    //println!("{:?}", value3);
    match weak.upgrade() {
        Some(rc) => println!("{:?}", rc),
        None => println!("None"),
    }

    // Cloning an Rc creates a new Rc
    println!("{:?}", rc2);

    // Mutability of Rc: Cell
    let person = Person {
        name: "Zack".to_string(),
        age: Cell::new(20),
    };
    let newage = person.increment_age();
    let person_age = person.age.get();
    println!("{}, {}", newage, person_age);

    // RefCell is a safer version of Cell
    // For multiple immutable and 1 mutable value
    // Inforced at runtime not at compile time (unlike Box) -> panic show
    // Allow single thread env
    // RefCell can be borrowed inmmutably or mutably
    let ref_cell = RefCell::new(vec![1, 2, 3]);
    // Get a mutable reference to the vector
    let mut mut_ref = ref_cell.borrow_mut();
    // Get a immutable reference to the vector
    let len = ref_cell.borrow().len();
    mut_ref.push(100);
    println!("Length: {}", len);

    // You can combine pointers
}
