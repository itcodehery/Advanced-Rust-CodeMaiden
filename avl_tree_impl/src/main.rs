// AVL Trees are bound by just three rules:
// - They are BSTs.
// - The Balance Factor, i.e. the difference between the height of the left subtree
//   and the right subtree, always belongs to the set = {-1, 0, 1}
// - No duplicate elements are allowed to be inserted into the tree

use std::io::stdin;

struct AVLNode {
    height: i32,
    value: i32,
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
}

impl AVLNode {
    fn new(height: i32, value: i32) -> Self {
        Self {
            height,
            value,
            left: None,
            right: None,
        }
    }

    fn height(&self) -> i32 {
        return self.height;
    }
}

fn create_avl_tree() -> Option<Box<AVLNode>> {
    println!("Enter a value: ");
    let mut value: String = String::new();
    stdin().read_line(&mut value).unwrap();
    let value: i32 = value.trim().parse().unwrap_or_default();
    Some(Box::new(AVLNode::new(0, value)))
}

fn insert_node(value: i32, parent: &mut AVLNode) {
    if value > parent.value {
        if parent.right.is_some() {
            insert_node(value, &mut parent.right.as_mut().unwrap());
        } else {
            parent.right = Some(Box::new(AVLNode::new(1, value)));
        }
    } else if value < parent.value {
        if parent.left.is_some() {
            insert_node(value, &mut parent.left.as_mut().unwrap());
        } else {
            parent.left = Some(Box::new(AVLNode::new(1, value)));
        }
    } else if value == parent.value {
        println!("Duplicates not allowed!");
    } else {
        println!("Invalid value!");
    }
}

fn display_tree(root: &Option<Box<AVLNode>>) {
    // Perform inorder traversal
    let mut current = root.as_ref();
    while current.is_some() {
        current = current.unwrap().left.as_ref();
        println!("{}", current.unwrap().value);
        current = current.unwrap().right.as_ref();
    }
}

fn main() {
    let mut head = create_avl_tree();
    let vector = vec![2, 3, 51, 6, 12, 6, 23, 3, 4, 9];

    for item in vector {
        insert_node(item, &mut head.unwrap());
    }
}
