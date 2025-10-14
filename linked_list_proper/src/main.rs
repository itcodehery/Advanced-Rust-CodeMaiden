// Singly Linked List Implementation in Rust
use text_io::read;

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value: value,
            next: None,
        }
    }
}

fn count(head: &mut Node) -> u32 {
    let mut count = 1;
    let mut current = head;
    while current.next.is_some() {
        count += 1;
        current = current.next.as_mut().unwrap();
    }
    count
}

fn kth_from_end(head: &mut Node, k: u32) {
    let count = count(head);
    let mut current = head;
    for _ in 0..(count - k) {
        current = current.next.as_mut().unwrap();
    }
    println!("\nThe kth element from the end is {}", &current.value);
}

fn insert_at_beginning(head: &mut Node) {
    println!("\nEnter the element to be inserted: ");
    let inp: i32 = read!();
    let mut new_node = Node::new(inp);
    new_node.next = Some(Box::new(head.clone()));
    *head = new_node;
    println!("The element {} has been inserted!", inp);
}

/*
fn delete_at_position(head: &mut Node) {
    println!("\nEnter the position to delete the node: ");
    let inp: i32 = read!();
    let mut current = head;
    for _ in 0..=inp {
        current = current.next.as_mut().unwrap();
    }
    let next = &mut current.next;
    current = next.as_mut().unwrap();
}
*/

fn read_ll(head: &mut Node) {
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
            let new_node = Node::new(int);
            current.next = Some(Box::new(new_node));
            current = current.next.as_mut().unwrap();
        }
    }
}

fn display_ll(head: &mut Node) {
    let mut current = head;
    while current.next.is_some() {
        print!("{} -> ", current.value);
        current = current.next.as_mut().unwrap();
    }
    print!("{} -> X", current.value);
}

fn main() {
    let mut head = Node::new(0);
    read_ll(&mut head);
    display_ll(&mut head);
    insert_at_beginning(&mut head);
    display_ll(&mut head);
    kth_from_end(&mut head, 2);
}
