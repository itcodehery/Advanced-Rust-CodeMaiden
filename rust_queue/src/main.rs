// Queue Implementation in Rust
use text_io::read;

#[derive(Debug, Default)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val: val,
            next: None,
        }
    }
}

fn enqueue(head: &mut Option<Box<Node>>, ele: i32) {
    let new_node = Box::new(Node::new(ele));
    let mut current = head;
    while let Some(node) = current {
        current = &mut node.next;
    }
    *current = Some(new_node);
}

fn dequeue(head: &mut Option<Box<Node>>) {
    if let Some(old_head) = head.take() {
        println!("Dequeued element: {}", old_head.val);
        *head = old_head.next;
    } else {
        println!("Queue is empty.");
    }
}

fn display(head: &Option<Box<Node>>) {
    let mut current = head.as_ref();
    print!("Queue: ");
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next.as_ref();
    }
    println!("X");
}

fn main_loop(head: &mut Option<Box<Node>>) {
    let mut ch: i32 = 0;

    while ch != -1 {
        println!("-----------");
        println!("Queue Implementation");
        println!("-----------");
        println!("1. Enque");
        println!("2. Deque");
        println!("3. Display");
        println!("4. Exit");
        println!("-----------");
        println!("Enter your choice: ");
        ch = read!();
        match ch {
            1 => {
                println!("Enter an element to enqueue: ");
                let ele: i32 = read!();
                enqueue(head, ele);
            }
            2 => {
                dequeue(head);
            }
            3 => {
                display(head);
            }
            4 => {
                ch = -1;
            }
            _ => {}
        }
    }
}

fn main() {
    let mut head: Option<Box<Node>> = None;
    main_loop(&mut head);
}
