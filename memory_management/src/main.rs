use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    value: i32,
    neighbours: Vec<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Node with value {} is being dropped!", self.value);
    }
}

fn main() {
    println!("Creating a cyclic graph");
    let a = Rc::new(RefCell::new(Node {
        value: 20,
        neighbours: vec![],
    }));

    let b = Rc::new(RefCell::new(Node {
        value: 30,
        neighbours: vec![],
    }));

    let c = Rc::new(RefCell::new(Node {
        value: 40,
        neighbours: vec![],
    }));

    a.borrow_mut().neighbours.push(Rc::clone(&b));
    b.borrow_mut().neighbours.push(Rc::clone(&c));
    c.borrow_mut().neighbours.push(Rc::clone(&a));

    println!("Created cyclic graph!");

    println!(
        "After connections: a={}, b={}, c={}",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::strong_count(&c)
    );
}
