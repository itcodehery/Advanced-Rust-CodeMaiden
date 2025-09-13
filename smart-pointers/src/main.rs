// Comprehensive Program to learn about Memory Addressing
// and Pointers in Rust
// Types (4):
// - Referencing and Borrowing
// - Box Smart Pointer
// - Reference Counting Values
// - Reference Cell

use std::{cell::RefCell, rc::Rc, sync::Arc};

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
    //    ### Arc: Atomic Reference Counting

    // Arc is a smart pointer that stands for Atomically Reference-Counted. Its main purpose
    // is to enable shared ownership of a value across multiple threads safely.

    //  * Shared Ownership: Normally, Rust's ownership rules allow only one variable to own
    //    a piece of data. Arc<T> allows multiple variables (potentially in different    //    threads) to "own" the data T.
    // Analogy: Think of Arc as giving out keycards to a read-only library room. Many
    // people (threads) can have a keycard (Arc) to the same room (data), and they can all
    // read from it concurrently. The room is only closed down when the very last keycard
    // is returned.
    let mut atomic = Arc::new(20);
    let b_atom = &atomic;
    let mut c_atom = &mut atomic;
    c_atom = 20;

    // ### Mutex: Mutual Exclusion

    // Mutex is a concurrency primitive that stands for Mutual Exclusion. It ensures that
    // only one thread can access a piece of data at any given time.

    //  * Exclusive Access: Before a thread can access the data inside a Mutex, it must
    //    first "lock" it. This lock grants the thread exclusive access.
    // Analogy: Think of a Mutex as a "talking stick" for a piece of data. Only the thread
    // holding the stick (MutexGuard) is allowed to modify the data. Any other thread must
    // wait for the stick to be free.

    // ### The Common Pattern: Arc<Mutex<T>>

    // You will very often see Arc and Mutex used together like this: Arc<Mutex<T>>. This
    // combination allows you to have a value T that can be both shared across multiple
    // threads and mutated safely.

    //  * Arc allows the Mutex itself to be owned by multiple threads.
    // Combined Analogy: Imagine a single whiteboard (T) in a room.
    // *   The Arc is like a system that gives keycards to the room to many different
    // people (threads).
    // *   The Mutex is a rule that only one person is allowed to be in the room writing
    // on the whiteboard at any one time. To enter, you must acquire the lock. When you
    // leave, you release the lock so someone else can enter.
}
