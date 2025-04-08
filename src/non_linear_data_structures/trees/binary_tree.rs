pub struct BinaryTreeNode {
    value: String,
    left: Option<Box<BinaryTreeNode>>,
    right: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: &str) -> Self {
        let final_value = value.to_owned();

        BinaryTreeNode {
            value: final_value,
            left: None,
            right: None,
        }
    }

    // Root -> Left -> Right
    pub fn preorder(&self) {
        println!("{:?}", self.value);

        if let Some(ref left_node) = self.left {
            left_node.preorder();
        }

        if let Some(ref right_node) = self.right {
            right_node.preorder();
        }
    }

    // Left -> Root -> Right
    pub fn inorder(&self) {
        if let Some(ref right_node) = self.right {
            right_node.inorder();
        }

        println!("{:?}", self.value);

        if let Some(ref left_node) = self.left {
            left_node.inorder();
        }
    }

    pub fn postorder(&self) {
        if let Some(ref right_node) = self.right {
            right_node.preorder();
        }

        if let Some(ref left_node) = self.left {
            left_node.preorder();
        }

        println!("{:?}", self.value);
    }

    pub fn pretty_print(&self) {
        fn helper(node: &Option<Box<BinaryTreeNode>>, prefix: String, is_left: bool) {
            if let Some(n) = node {
                println!(
                    "{}{}{}",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    n.value
                );
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                helper(&n.left, new_prefix.clone(), true);
                helper(&n.right, new_prefix, false);
            }
        }

        println!("{}", self.value);
        helper(&self.left, "".to_string(), true);
        helper(&self.right, "".to_string(), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_expression_tree() -> BinaryTreeNode {
        let node3 = BinaryTreeNode::new("3");
        let node4 = BinaryTreeNode::new("4");
        let node5 = BinaryTreeNode::new("5");

        let mut mult = BinaryTreeNode::new("*");
        mult.left = Some(Box::new(node4));
        mult.right = Some(Box::new(node5));

        let mut plus = BinaryTreeNode::new("+");
        plus.left = Some(Box::new(node3));
        plus.right = Some(Box::new(mult));

        plus
    }

    #[test]
    fn test_preorder() {
        let root = build_expression_tree();
        root.preorder(); // Should print: "+ 3 * 4 5"
    }

    #[test]
    fn test_inorder() {
        let root = build_expression_tree();
        root.inorder(); // Should print: "3 + 4 * 5"
    }

    #[test]
    fn test_postorder() {
        let root = build_expression_tree();
        root.postorder(); // Should print: "3 4 5 * +"
    }

    #[test]
    fn test_visual_tree() {
        let root = build_expression_tree();
        root.pretty_print();
    }
}
