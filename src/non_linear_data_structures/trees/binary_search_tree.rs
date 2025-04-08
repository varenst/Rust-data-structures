use core::fmt;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, left: None, right: None }
    }
}

pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone + fmt::Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            Some(ref mut n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => Self::insert_node(&mut n.left, value),
                    Ordering::Greater => Self::insert_node(&mut n.right, value),
                    Ordering::Equal => {} 
                }
            }
            None => *node = Some(Box::new(Node::new(value))),
        }
    }

    pub fn search(&self, value: T) -> bool {
        Self::search_node(&self.root, &value)
    }

    fn search_node(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => Self::search_node(&n.left, value),
                Ordering::Greater => Self::search_node(&n.right, value),
                Ordering::Equal => true,
            },
            None => false,
        }
    }
    pub fn delete(&mut self, value: T) {
        Self::delete_node(&mut self.root, value);
    }

    fn delete_node(node: &mut Option<Box<Node<T>>>, value: T) {
        if let Some(ref mut n) = node {
            match value.cmp(&n.value) {
                Ordering::Less => Self::delete_node(&mut n.left, value),
                Ordering::Greater => Self::delete_node(&mut n.right, value),
                Ordering::Equal => {
                    *node = match (n.left.take(), n.right.take()) {
                        (None, None) => None,
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (Some(left), Some(right)) => {
                            let min = Self::find_min(&right);
                            let mut new_node = Box::new(Node::new(min));
                            new_node.left = Some(left);
                            new_node.right = Self::delete_min(right);
                            Some(new_node)
                        }
                    };
                }
            }
        }
    }

    fn find_min(node: &Box<Node<T>>) -> T
    where
        T: Clone,
    {
        match &node.left {
            Some(left) => Self::find_min(left),
            None => node.value.clone(),
        }
    }

    fn delete_min(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        if node.left.is_none() {
            return node.right;
        }
        node.left = Self::delete_min(node.left.unwrap());
        Some(node)
    }

    pub fn pretty_print(&self) {
        Self::print_node(&self.root, 0);
    }

    fn print_node(node: &Option<Box<Node<T>>>, depth: usize) {
        if let Some(n) = node {
            Self::print_node(&n.right, depth + 1);
            println!("{}{}", "    ".repeat(depth), n.value);
            Self::print_node(&n.left, depth + 1);
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_search() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        assert!(!bst.search(20));
    }

    #[test]
    fn insert_duplicates() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(10);

        assert!(bst.search(10));
    }

    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::new();
        bst.insert(20);
        bst.insert(10);
        bst.insert(30);
        bst.insert(25);
        bst.insert(35);

        assert!(bst.search(25));
        bst.delete(25);
        assert!(!bst.search(25));

        assert!(bst.search(30));
        bst.delete(30);
        assert!(!bst.search(30));
    }

    #[test]
    fn test_pretty_print() {
        let mut bst = BinarySearchTree::new();
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(12);
        bst.insert(11);
        bst.insert(1);
        bst.insert(14);
        bst.insert(9);

        bst.pretty_print();
    }
}
