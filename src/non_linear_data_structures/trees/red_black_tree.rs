use core::fmt;
use std::{cell::{Ref, RefCell}, cmp::Ordering, rc::{Rc, Weak}};

#[derive(Clone)]
pub struct Node<T> {
    value: T,
    is_red: bool,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, left: None, right: None, is_red: true, parent: None }
    }
}

pub struct RedBlackTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord + Clone + fmt::Display> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(root_node) = self.root.as_ref() {
            RedBlackTree::insert_node(root_node, value);
        } else {
            self.root = Some(Rc::new(RefCell::new(Node::new(value))));
        }
    }

    fn insert_node(node: &Rc<RefCell<Node<T>>>, value: T) {
        match value.cmp(&node.borrow().value) {
            Ordering::Less => {
                if let Some(left_node) = node.borrow().left.as_ref() {
                    Self::insert_node(&left_node, value);    
                } else {
                    let mut new_node = Node::new(value);         
                  
                    new_node.parent = Some(Rc::downgrade(node)); 
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));
                }
            },
            Ordering::Greater => {
                if let Some(right_node) = node.borrow().right.as_ref() {
                    Self::insert_node(&right_node, value);    
                } else {
                    let mut new_node = Node::new(value);         
                  
                    new_node.parent = Some(Rc::downgrade(node)); 
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
                }
            },
            Ordering::Equal => {} 
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
    
}