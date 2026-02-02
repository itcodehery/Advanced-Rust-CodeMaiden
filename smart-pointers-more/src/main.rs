// pub struct Cell<T> {
//     value: T,
// }
//
// impl<T> Cell<T> {
//     pub fn new(value: T) -> Self {
//         Cell { value }
//     }
//
//     pub fn set(&self, value: T) {
//         self.value = value;
//     }
//
//     pub fn get(&self) -> T {
//         self.value
//     }
// }

use std::cell::Cell;

fn main() {
    let x: Cell<i32> = Cell::new(32);
    println!("Hello, {}!", x.get());
    x.set(34);
    println!("Hello, {}!", x.get());
}
