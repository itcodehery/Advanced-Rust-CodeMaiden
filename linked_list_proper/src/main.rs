// Singly Linked List Implementation in Rust
use text_io::read;

#[derive(Debug, Clone)]
struct DNode {
    value: i32,
    next: Option<Box<DNode>>,
}

impl DNode {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            next: None,
        }
    }
}

fn insert_at_beginning(head: &mut DNode) {
    println!("\nEnter the element to be inserted: ");
    let inp: i32 = read!();
    let mut new_node = DNode::new(inp);
    new_node.next = Some(Box::new(head.clone()));
    *head = new_node;
    println!("The element {} has been inserted!", inp);
}

fn read_ll(head: &mut DNode) {
    let mut choice = 1;
    println!("Enter an integer value: ");
    let inp: i32 = read!();
    head.value = inp;
    head.next = None;
    let mut current = head;
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

fn display_ll(head: &mut DNode) {
    let mut current = head;
    while current.next.is_some() {
        print!("{} -> ", current.value);
        current = current.next.as_mut().unwrap();
    }
    print!("{} -> X", current.value);
}

fn main() {
    let mut head = DNode::new(0);
    read_ll(&mut head);
    display_ll(&mut head);
    insert_at_beginning(&mut head);
    display_ll(&mut head);
}
