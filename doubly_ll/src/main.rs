// Doubly Linked List Implementation in Rust
use text_io::read;

#[derive(Debug, Clone)]
struct DNode {
    prev: Option<Box<DNode>>,
    value: i32,
    next: Option<Box<DNode>>,
}

impl DNode {
    fn new(value: i32) -> Self {
        Self {
            prev: None,
            value: value,
            next: None,
        }
    }
}

fn main() {
    let mut head = DNode::new(0);
    let mut current = &mut head;
    while current.next.is_some() {
        current = current.next.as_mut().unwrap();
    }
    let mut choice = 1;
    println!("Enter an integer value: ");
    let mut inp: i32 = read!();
    head.prev = None;
    head.value = inp;
    head.next = None;
    while choice == 1 {
        println!("Do you want to add another node? ");
        choice = read!();
        if choice == 0 {
            current.next = None;
            break;
        }
    }
}
