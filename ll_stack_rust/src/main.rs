use text_io::read;

#[derive(Debug, Default)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        return Node {
            val: val,
            next: None,
        };
    }
}

fn push(head: &mut Option<Box<Node>>, data: i32) {
    let mut new_node = Box::new(Node::new(data));

    new_node.next = head.take();

    *head = Some(new_node);
}

fn pop(head: &mut Option<Box<Node>>) {
    if head.is_some() {
        let temp = head.as_ref();
        println!("The element popped is {}", temp.unwrap().val);
        *head = head.take().unwrap().next;
    } else {
        println!("The stack is empty!");
    }
}

fn display(head: &mut Option<Box<Node>>) {
    if head.is_some() {
        let mut current = head;
        while current.as_mut().unwrap().next.is_some() {
            print!("{} ->", current.as_mut().unwrap().val);
            current = &mut current.as_mut().unwrap().next;
        }
        println!("\n");
    } else {
        println!("The stack is empty!");
    }
}

fn main() {
    let mut head: Option<Box<Node>> = None;
    let mut ch: i32 = 0;
    while ch != -1 {
        println!("---------------");
        println!("Stack Implementation in Linked List");
        println!("---------------");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Display");
        println!("4. Exit");
        println!("Enter your choice: ");
        println!("---------------");
        ch = read!();
        match ch {
            1 => {
                println!("Enter an element to push into stack: ");
                let ele: i32 = read!();
                push(&mut head, ele);
            }
            2 => {
                pop(&mut head);
            }
            3 => {
                display(&mut head);
            }
            _ => {
                ch = -1;
            }
        }
    }
}
