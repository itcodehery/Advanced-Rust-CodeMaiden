// Comprehensive Program to learn about Memory Addressing
// and Pointers in Rust
// Types (4):
// - Referencing and Borrowing
// - Box Smart Pointer
// - Reference Counting Values
// - Reference Cell

use std::{cell::RefCell, rc::Rc};

fn main() {
    // Referencing and Dereferencing
    let mut y = 10;
    // Only one mutable reference at a time or
    // multiple immutable references at a time
    // is allowed
    let x = &mut y;
    *x = 20;
    println!("{}", x);

    // Smart Pointers => Box<T>
    // Single Owner, Mutable or Immutable Access
    let z: Box<i32> = Box::new(5);
    println!("{}", z); // Automatic Dereferencing, a feature of Box
    // Automatic Dereferencing is also known as
    // Deref Coercion

    // Smart Pointers => Rc<T> (aka Reference Counting)
    // Multiple Owners, Immutable Access
    let a: Rc<i32> = Rc::new(12); // Creating a reference counting pointer
    let x: Rc<i32> = Rc::clone(&a); // Creating a new reference for an existing pointer
    println!("Multiple owners, Immutable Access: {} and {}", a, x);

    // Smart Pointers => RefCell<T>
    // Multiple Owners, Mutable Access
    let b = Rc::new(RefCell::new(42));
    // We can create as many shared owners as we want
    let c = Rc::clone(&b);
    *c.borrow_mut() = 20;
    let d = Rc::clone(&b);
    println!("Multiple owners, Mutable Access: {:?} and {:?}", c, d);
}
