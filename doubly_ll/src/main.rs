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

    let mut choice = 1;
    println!("Enter an integer value: ");
    let mut inp: i32 = read!();
    head.prev = None;
    head.value = inp;
    head.next = None;
    let mut current = &mut head;
    let prev = &head;
    while current.next.is_some() {
        current = current.next.as_mut().unwrap();
    }
    while choice == 1 {
        println!("Do you want to add another node? ");
        choice = read!();
        if choice == 0 {
            current.next = None;
            break;
        } else {
            println!("Enter an integer: ");
            let int: i32 = read!();
            let new_node = DNode::new(int);
            current.next = Some(Box::new(new_node));
            current = current.next.as_mut().unwrap();
        }
    }
}
