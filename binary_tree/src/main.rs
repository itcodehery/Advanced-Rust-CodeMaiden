#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
fn inorder_traversal(root: &mut TreeNode) -> Vec<i32> {
    let mut res_vector: Vec<i32> = Vec::new();
    let mut current = root;
    res_vector.push(current.val);
    while current.right.is_some() {
        current = current.right.as_mut().unwrap();
        res_vector.push(current.val);
    }
    res_vector
}

fn tree_builder(root: &mut TreeNode) {
    let mut current = root;
    print!("{}", current.val);
    if current.left.is_some() {
        print!("󱞩 {}", current.left.as_mut().unwrap().val);
    }
    while current.right.is_some() {
        current = current.right.as_mut().unwrap();
        print!("-> {}", current.val);
        if current.left.is_some() {
            print!("󱞩 {}", current.left.as_mut().unwrap().val);
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    root.right = Some(Box::new(TreeNode::new(1)));
    root.left = Some(Box::new(TreeNode::new(12)));
    let mut current = &mut root;
    current = current.right.as_mut().unwrap();
    current.right = Some(Box::new(TreeNode::new(2)));
    current.left = Some(Box::new(TreeNode::new(23)));

    tree_builder(&mut root);
    // println!("{:?}", inorder_traversal(&mut root));
}
