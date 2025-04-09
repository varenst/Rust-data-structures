pub struct AvlNode<T> {
    value: T,
    right: Option<Box<AvlNode<T>>>,
    left: Option<Box<AvlNode<T>>>,
    height: i64
}

impl<T> AvlNode<T> {
 pub fn new(value: T) -> Self {
     AvlNode {
        value,
        right: None,
        left: None,
        height: 0
     }
 }   
}

pub struct AvlTree<T> {
    root: Option<Box<AvlNode<T>>>,
}

impl<T : Eq + Clone> AvlTree<T> {
    pub fn new(value: T) -> Self {
        Self { root: Some(Box::new(AvlNode::new(value))) }
    }

    fn get_height(node: Option<&Box<AvlNode<T>>>) -> i64 {
        match node {
            Some(n) => 1 + std::cmp::max(
                AvlTree::get_height(n.left.as_ref()),
                AvlTree::get_height(n.right.as_ref()),
            ),
            None => -1, // height of empty node is -1
        }
    }

    fn balance() -> i64 {
        0
    }
}