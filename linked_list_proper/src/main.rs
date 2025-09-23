use text_io::read;

#[derive(Debug, Clone)]
struct Node {
    next: Option<Box<Node>>,
    value: String,
}

impl Node {
    fn new(value: String) -> Self {
        Self { value, next: None }
    }
}

fn main() {
    // Variables
    let mut choice: i32 = 1;
    let mut head = Node::new(String::from(""));
    let mut current = &mut head;
    // IO Initial Input
    println!("Enter a string: ");
    let mut str: String = read!();
    // Move to the end
    while current.next.is_some() {
        current = current.next.as_mut().unwrap();
    }
    // Set Head Value
    current.value = str.clone();
    while choice == 1 {
        str.clear();
        // IO Choice Input
        println!("Do you want to add another node? ");
        choice = read!();
        println!("Choice is {}", choice);
        if choice == 0 {
            current.next = None;
            break;
        } else {
            // IO Iterative Input
            println!("Enter a string: ");
            str = read!();
            let new_node = Node::new(str.clone());
            current.next = Some(Box::new(new_node));
            current = current.next.as_mut().unwrap();
        }
    }

    // Display nodes
    display_nodes(&mut head);
}

fn display_nodes(head: &mut Node) {
    let mut current = head;
    while current.next.is_some() {
        print!("{} -> ", current.value);
        current = current.next.as_mut().unwrap();
    }
    print!("X");
}
