use std::io::stdin;

// Approach: create Adjacency List
struct Node {
    value: i32,
    #[allow(dead_code)]
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { value, next: None }
    }
}

fn display_adj_list(list: &Vec<Box<Node>>) {
    for item in list {
        print!("{} -> ", item.value);
    }
}

fn main() {
    let mut adj_list: Vec<Box<Node>> = Vec::new();
    let mut ch: String = String::new();
    let mut ch2: String = String::new();
    while ch.to_lowercase() != "n" {
        println!("Enter a variable: ");
        stdin().read_line(&mut ch).unwrap_or_default();
        let val: i32 = ch.trim().parse().unwrap();
        while ch2.to_lowercase() != "n" {}
    }

    display_adj_list(&adj_list);
}
