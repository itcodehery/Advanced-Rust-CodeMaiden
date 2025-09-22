use std::io;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut str = String::new();
    println!("Enter an element: ");
    io::stdin().read_line(&mut str).unwrap();
    let str = str.trim();
    let str: i32 = match str.parse() {
        Ok(res) => res,
        Err(r) => {
            println!("Couldn't parse error!");
            return;
        }
    };
    let head: Node = Node {
        value: str,
        next: None,
    };
    for i in 1..5 {
        if i != 4 {
            println!("Do you want to add another node?");
        }
    }
}
