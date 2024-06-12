// Define a binary tree node structure
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example binary tree
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let mut right = TreeNode::new(3);
    let left_left = TreeNode::new(4);
    let left_right = TreeNode::new(5);

    left.left = Some(Box::new(left_left));
    left.right = Some(Box::new(left_right));

    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));

    let depth = max_depth(Some(Box::new(root)));
    println!("Maximum depth of the binary tree: {}", depth);
}
