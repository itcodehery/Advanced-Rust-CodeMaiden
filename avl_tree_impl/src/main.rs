// AVL Trees are bound by just three rules:
// - They are BSTs.
// - The Balance Factor, i.e. the difference between the height of the left subtree
//   and the right subtree, always belongs to the set = {-1, 0, 1}
// - No duplicate elements are allowed to be inserted into the tree

use std::io::stdin;

struct AVLNode {
    value: i32,
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
    height: i32,
}

impl AVLNode {
    fn new(height: i32, value: i32) -> Self {
        Self {
            value,
            height: height,
            left: None,
            right: None,
        }
    }

    fn get_balance_factor(&self) -> i32 {
        let factor: i32;
        // If node is a leaf
        if self.left.is_none() && self.right.is_none() {
            factor = 0;
        }
        // If node has only right
        else if self.left.is_none() {
            factor = self.right.as_ref().unwrap().get_balance_factor();
        }
        // If node has only left
        else if self.right.is_none() {
            factor = self.left.as_ref().unwrap().get_balance_factor();
        }
        // If node has both left and right
        else {
            factor = self.left.as_ref().unwrap().get_balance_factor()
                - self.right.as_ref().unwrap().get_balance_factor();
        }
        factor
    }
}

fn create_avl_tree() -> Option<Box<AVLNode>> {
    println!("Enter a value: ");
    let mut value: String = String::new();
    stdin().read_line(&mut value).unwrap();
    let value: i32 = value.trim().parse().unwrap_or_default();
    Some(Box::new(AVLNode::new(0, value)))
}

fn perform_left_rotation(node: AVLNode) {}

fn insert_node(value: i32, parent: &mut AVLNode) {}

// fn display_tree(root: &Option<Box<AVLNode>>) {
//     // Perform inorder traversal
//     let mut current = root.as_ref();
//     while current.is_some() {
//         current = current.unwrap().left.as_ref();
//         println!("{}", current.unwrap().value);
//         current = current.unwrap().right.as_ref();
//     }
// }

fn main() {
    let mut head = create_avl_tree();
    let vector = vec![2, 3, 51, 6, 12, 6, 23, 3, 4, 9];

    for item in vector {
        insert_node(item, &mut head.as_mut().unwrap());
    }

    // display_tree(&head);
}
