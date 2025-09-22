use text_io::read;

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    println!("Enter an element: ");
    let new_int: i32 = read!();
    let mut head: Box<Node> = Box::new(Node {
        value: new_int,
        next: None,
    });
    let mut current = &mut head;
    while current.next.is_some() {
        current = current.next.as_mut().unwrap();
    }
    println!("Value of current: {:?}", current);
    let mut choice: bool = true;
    while choice != false {
        // Ask user choice
        println!("Do you want to add another node?");
        choice = read!(); // If Choice is true -> add a new Node
        if choice == true {
            // ask new value
            println!("Enter the value: ");
            let val: i32 = read!();
            // create new node with next empty
            let new_node = Node {
                value: val,
                next: None,
            };
            // if first iteration, set next of head
            current.next = Some(Box::new(new_node));
            if current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
        }
    }
    println!("\nThe Linked List Created: --------------\n");
    let mut current = &mut head;
    while current.next.is_some() {
        print!("{} -> ", current.value);
        if current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
    }
    print!("X");
}
