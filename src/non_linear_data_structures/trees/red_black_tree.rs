use core::fmt;
use std::{
    cell::RefCell,
    cmp::Ordering,
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub struct Node<T> {
    value: T,
    is_red: bool,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            is_red: true,
            parent: None,
        }
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
            self.insert_node(root_node.clone(), value);
        } else {
            let root = Rc::new(RefCell::new(Node::new(value)));
            root.borrow_mut().is_red = false;
            self.root = Some(root);
        }
    }

    fn insert_node(&mut self, node: Rc<RefCell<Node<T>>>, value: T) {
        let ordering = {
            let node_ref = node.borrow();
            value.cmp(&node_ref.value)
        };

        match ordering {
            Ordering::Less => {
                let left_child = {
                    let nref = node.borrow();
                    nref.left.clone()
                };

                if let Some(left_node) = left_child {
                    self.insert_node(left_node, value);
                } else {
                    let new_node = Rc::new(RefCell::new(Node::new(value)));
                    
                    {
                        let mut node_mut = node.borrow_mut();
                        new_node.borrow_mut().parent = Some(Rc::downgrade(&node));

                        node_mut.left = Some(Rc::clone(&new_node));
                    }

                    self.insert_fix(new_node);
                }
            }
            Ordering::Greater => {
                let right_child = {
                    let nref = node.borrow();
                    nref.right.clone()
                };

                if let Some(right_node) = right_child {
                    self.insert_node(right_node, value);
                } else {
                    let new_node = Rc::new(RefCell::new(Node::new(value)));
                    
                    {
                        let mut node_mut = node.borrow_mut();
                        new_node.borrow_mut().parent = Some(Rc::downgrade(&node));
                        
                        node_mut.right = Some(Rc::clone(&new_node));
                    }

                    self.insert_fix(new_node);
                }
            }
            Ordering::Equal => {
                return;
            }
        }
    }

    fn insert_fix(&mut self, mut new_node: Rc<RefCell<Node<T>>>) {
        while let Some(parent_weak) = new_node.clone().borrow().parent.clone() {
            let parent_rc = match parent_weak.upgrade() {
                None => break, 
                Some(rc) => rc,
            };
      
            if !parent_rc.borrow().is_red {
                break;
            }

            let grand_weak = parent_rc.borrow().parent.clone();

            let grand_rc = match grand_weak {
                None => break,
                Some(g) => match g.upgrade() {
                    None => break,
                    Some(rc) => rc,
                },
            };

            let is_parent_left = if let Some(ref left_rc) = grand_rc.borrow().left {
                Rc::ptr_eq(left_rc, &parent_rc)
            } else {
                false
            };

            if is_parent_left {
                let uncle_opt = grand_rc.borrow().right.clone();
  
                if let Some(ref uncle_rc) = uncle_opt {
                    if uncle_rc.borrow().is_red {
                        parent_rc.borrow_mut().is_red = false;
                        uncle_rc.borrow_mut().is_red = false;
                        grand_rc.borrow_mut().is_red = true;
                        
                        new_node = grand_rc.clone();
                        
                        continue;
                    }
                }

                if let Some(ref parent_right) = parent_rc.borrow().right {
                    if Rc::ptr_eq(parent_right, &new_node) {
                        self.left_rotation(parent_rc.clone());
                        new_node = parent_rc.clone();
                    }
                }

                parent_rc.borrow_mut().is_red = false;
                grand_rc.borrow_mut().is_red = true;
               
                self.right_rotation(grand_rc.clone());
            } else {
                let uncle_opt = grand_rc.borrow().left.clone();

                if let Some(ref uncle_rc) = uncle_opt {
                    if uncle_rc.borrow().is_red {
                        parent_rc.borrow_mut().is_red = false;
                        uncle_rc.borrow_mut().is_red = false;
                        grand_rc.borrow_mut().is_red = true;
                        
                        new_node = grand_rc.clone();
                        
                        continue;
                    }
                }
              
                if let Some(ref parent_left) = parent_rc.borrow().left {
                    if Rc::ptr_eq(parent_left, &new_node) {
                        self.right_rotation(parent_rc.clone());
                        new_node = parent_rc.clone();
                    }
                }

                parent_rc.borrow_mut().is_red = false;
                grand_rc.borrow_mut().is_red = true;

                self.left_rotation(grand_rc.clone());
            }
        }

        if let Some(ref r) = self.root {
            r.borrow_mut().is_red = false;
        }
    }

    fn right_rotation(&mut self, node: Rc<RefCell<Node<T>>>) {
        let left_node = node.borrow().left.as_ref().unwrap().clone();
        node.borrow_mut().left = left_node.borrow().right.clone();

        if let Some(ref right_subtree) = left_node.borrow().right {
            right_subtree.borrow_mut().parent = Some(Rc::downgrade(&node));
        }

        let parent_opt = node.borrow().parent.clone();
        left_node.borrow_mut().parent = parent_opt.clone();

        if let Some(parent_weak) = parent_opt {
            let parent_rc = parent_weak.upgrade().unwrap();
            if Rc::ptr_eq(&node, parent_rc.borrow().right.as_ref().unwrap()) {
                parent_rc.borrow_mut().right = Some(Rc::clone(&left_node));
            } else {
                parent_rc.borrow_mut().left = Some(Rc::clone(&left_node));
            }
        } else {
            self.root = Some(Rc::clone(&left_node));
        }

        left_node.borrow_mut().right = Some(Rc::clone(&node));
        node.borrow_mut().parent = Some(Rc::downgrade(&left_node));
    }

    fn left_rotation(&mut self, node: Rc<RefCell<Node<T>>>) {
        let right_node = node.borrow().right.as_ref().unwrap().clone();
        node.borrow_mut().right = right_node.borrow().left.clone();

        if let Some(ref left_subtree) = right_node.borrow().left {
            left_subtree.borrow_mut().parent = Some(Rc::downgrade(&node));
        }

        let parent_opt = node.borrow().parent.clone();
        right_node.borrow_mut().parent = parent_opt.clone();

        if let Some(parent_weak) = parent_opt {
            let parent_rc = parent_weak.upgrade().unwrap();
            if Rc::ptr_eq(&node, parent_rc.borrow().left.as_ref().unwrap()) {
                parent_rc.borrow_mut().left = Some(Rc::clone(&right_node));
            } else {
                parent_rc.borrow_mut().right = Some(Rc::clone(&right_node));
            }
        } else {
            self.root = Some(Rc::clone(&right_node));
        }

        right_node.borrow_mut().left = Some(Rc::clone(&node));
        node.borrow_mut().parent = Some(Rc::downgrade(&right_node));
    }

    pub fn delete(&mut self, value: T) {
        self.delete_node(self.root.clone(), value);
    }

    fn transplant(&mut self, u: Option<Rc<RefCell<Node<T>>>>, v: Option<Rc<RefCell<Node<T>>>>) {
        let (parent_rc_opt, is_left_child) = {
            if let Some(ref u_rc) = u {
                let u_borrow = u_rc.borrow();
                let parent_weak = u_borrow.parent.clone();
                
                drop(u_borrow); 

                if let Some(weak_parent) = parent_weak {
                    let parent_rc = weak_parent.upgrade().unwrap();

                    let is_left = {
                        let p_borrow = parent_rc.borrow();
                        
                        if let Some(ref left_child) = p_borrow.left {
                            Rc::ptr_eq(left_child, u_rc)
                        } else {
                            false
                        }
                    };
                    (Some(parent_rc), is_left)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            }
        };

        match parent_rc_opt {
            Some(parent_rc) => {
                if is_left_child {
                    parent_rc.borrow_mut().left = v.clone();
                } else {
                    parent_rc.borrow_mut().right = v.clone();
                }
            }
            None => {
                self.root = v.clone();
            }
        }

        if let Some(ref v_rc) = v {
            if let Some(ref u_rc) = u {
                v_rc.borrow_mut().parent = u_rc.borrow().parent.clone();
            }
        }
    }

    fn delete_node(&mut self, mut node: Option<Rc<RefCell<Node<T>>>>, value: T) {
        let mut target: Option<Rc<RefCell<Node<T>>>> = None;

        while let Some(current) = node.clone() {
            match value.cmp(&current.borrow().value) {
                Ordering::Less => node = current.borrow().left.clone(),
                Ordering::Greater => node = current.borrow().right.clone(),
                Ordering::Equal => {
                    target = Some(current.clone());
                    break;
                }
            }
        }

        let z = match target {
            Some(n) => n,
            None => return,
        };

        let mut y = z.clone();
        let y_original_red = y.borrow().is_red;
        let x_opt: Option<Rc<RefCell<Node<T>>>>;

        if z.borrow().left.is_none() {
            x_opt = z.borrow().right.clone();
            self.transplant(Some(z.clone()), x_opt.clone());
        }
        else if z.borrow().right.is_none() {
            x_opt = z.borrow().left.clone();
            self.transplant(Some(z.clone()), x_opt.clone());
        }

        else {
            let mut y_opt = z.borrow().right.clone();
            
            while let Some(ref y_node) = y_opt.clone() {
                if y_node.borrow().left.is_none() {
                    break;
                }
                y_opt = y_node.borrow().left.clone();
            }

            y = y_opt.unwrap();
            let mut y_right = y.borrow().right.clone();
            let y_parent = y.borrow().parent.clone().unwrap().upgrade().unwrap();

            if !Rc::ptr_eq(&y_parent, &z) {
                self.transplant(Some(y.clone()), y_right.clone());
                
                y.borrow_mut().right = z.borrow().right.clone();
                
                if let Some(ref right) = y.borrow().right {
                    right.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
            } else {
                if let Some(ref mut right) = y_right {
                    right.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
            }

            self.transplant(Some(z.clone()), Some(y.clone()));
            y.borrow_mut().left = z.borrow().left.clone();
            
            if let Some(ref left) = y.borrow().left {
                left.borrow_mut().parent = Some(Rc::downgrade(&y));
            }

            y.borrow_mut().is_red = z.borrow().is_red;
            x_opt = y_right;
        }

        if y_original_red == false {
            self.delete_fix(x_opt);
        }
    }

    fn delete_fix(&mut self, mut x_opt: Option<Rc<RefCell<Node<T>>>>) {
        while let Some(x) = x_opt.clone() {
            if let Some(ref root_rc) = self.root {
                if Rc::ptr_eq(&x, root_rc) {
                    break;
                }
            }

            let parent_weak = x.borrow().parent.clone();
            if parent_weak.is_none() {
                break;
            }
            let parent = parent_weak.unwrap().upgrade().unwrap();

            let is_left_child = if let Some(ref left) = parent.borrow().left {
                Rc::ptr_eq(&x, left)
            } else {
                false
            };

            let mut sibling_opt = if is_left_child {
                parent.borrow().right.clone()
            } else {
                parent.borrow().left.clone()
            };

            if let Some(sibling) = sibling_opt.clone() {
                if sibling.borrow().is_red {
                    sibling.borrow_mut().is_red = false;
                    parent.borrow_mut().is_red = true;
                   
                    if is_left_child {
                        self.left_rotation(Rc::clone(&parent));
                        sibling_opt = parent.borrow().right.clone();
                    } else {
                        self.right_rotation(Rc::clone(&parent));
                        sibling_opt = parent.borrow().left.clone();
                    }
                }

                let sibling = sibling_opt.clone().unwrap();

                let sibling_left_red = sibling
                    .borrow()
                    .left
                    .as_ref()
                    .map_or(false, |left| left.borrow().is_red);
                
                let sibling_right_red = sibling
                    .borrow()
                    .right
                    .as_ref()
                    .map_or(false, |right| right.borrow().is_red);

                if !sibling_left_red && !sibling_right_red {
                    sibling.borrow_mut().is_red = true;
                    x_opt = parent.borrow().parent.as_ref().and_then(|p| p.upgrade());
                }

                else {
                    if is_left_child {
                        if !sibling_right_red && sibling_left_red {
                            sibling.borrow().left.as_ref().unwrap().borrow_mut().is_red = false;
                            sibling.borrow_mut().is_red = true;
                            
                            self.right_rotation(Rc::clone(&sibling));
                            
                            sibling_opt = parent.borrow().right.clone();
                        }

                        let sibling = sibling_opt.unwrap();
                        sibling.borrow_mut().is_red = parent.borrow().is_red;
                        parent.borrow_mut().is_red = false;
                        
                        if let Some(ref right) = sibling.borrow().right {
                            right.borrow_mut().is_red = false;
                        }
                        
                        self.left_rotation(Rc::clone(&parent));
                        x_opt = self.root.clone();
                    } else {
                        if !sibling_left_red && sibling_right_red {
                            sibling.borrow().right.as_ref().unwrap().borrow_mut().is_red = false;
                            sibling.borrow_mut().is_red = true;
                           
                            self.left_rotation(Rc::clone(&sibling));
                          
                            sibling_opt = parent.borrow().left.clone();
                        }

                        let sibling = sibling_opt.unwrap();
                        sibling.borrow_mut().is_red = parent.borrow().is_red;
                        parent.borrow_mut().is_red = false;
                        
                        if let Some(ref left) = sibling.borrow().left {
                            left.borrow_mut().is_red = false;
                        }
                       
                       self.right_rotation(Rc::clone(&parent));
                        x_opt = self.root.clone();
                    }
                }
            } else {
                x_opt = parent.borrow().parent.as_ref().and_then(|p| p.upgrade());
            }
        }

        if let Some(x) = x_opt {
            x.borrow_mut().is_red = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_order_values<T: Ord + Clone + fmt::Display>(
        node: &Option<Rc<RefCell<Node<T>>>>,
        vals: &mut Vec<T>,
    ) {
        if let Some(n) = node {
            in_order_values(&n.borrow().left, vals);
            vals.push(n.borrow().value.clone());
            in_order_values(&n.borrow().right, vals);
        }
    }

    #[test]
    fn test_insertion() {
        let mut tree = RedBlackTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(12);
        tree.insert(1);

        let mut vals = vec![];
        in_order_values(&tree.root, &mut vals);
        assert_eq!(vals, vec![1, 5, 10, 12, 15]);
    }

    #[test]
    fn test_deletion_leaf_node() {
        let mut tree = RedBlackTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        tree.delete(5); // Leaf
        let mut vals = vec![];
        in_order_values(&tree.root, &mut vals);
        assert_eq!(vals, vec![10, 15]);
    }

    #[test]
    fn test_deletion_node_with_one_child() {
        let mut tree = RedBlackTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(2); // 5 has one child

        tree.delete(5);
        let mut vals = vec![];
        in_order_values(&tree.root, &mut vals);
        assert_eq!(vals, vec![2, 10]);
    }

    #[test]
    fn test_deletion_node_with_two_children() {
        let mut tree = RedBlackTree::new();
        tree.insert(20);
        tree.insert(10);
        tree.insert(30);
        tree.insert(25);
        tree.insert(35);

        tree.delete(30); // has two children
        let mut vals = vec![];
        in_order_values(&tree.root, &mut vals);
        assert_eq!(vals, vec![10, 20, 25, 35]);
    }

    #[test]
    fn test_deletion_root_node() {
        let mut tree = RedBlackTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        tree.delete(10); // root
        let mut vals = vec![];
        in_order_values(&tree.root, &mut vals);
        assert_eq!(vals, vec![5, 15]);
    }
}
