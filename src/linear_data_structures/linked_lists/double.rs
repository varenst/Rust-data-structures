use core::fmt;
use std::{cell::RefCell, fmt::Debug, rc::{Rc, Weak}};


#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>, // Allows to mutateit safely
    prev: Option<Weak<RefCell<Node<T>>>> // Weak has no ownership
}

// Weak and RefCell are used to not create reference cycles
// I did not expect them to be this different

pub struct DoubleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Eq + Clone> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert_at_end(&mut self, data: T) {
        if self.head.is_none() {
            self.insert_at_beginning(data);
            return;
        }
    
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));
    
        let mut current = self.head.as_ref().unwrap().clone();

        loop {
            let next: Option<Rc<RefCell<Node<T>>>> = {
                let current_borrow = current.borrow();
                current_borrow.next.as_ref().cloned()
            };
        
            if let Some(next_node) = next {
                current = next_node;
            } else {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&current));
                
                break;
            }
        }        

        new_node.borrow_mut().prev = Some(Rc::downgrade(&current));
        current.borrow_mut().next = Some(new_node);
    }
    
    pub fn insert_at_beginning(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.clone(),
            prev: None
        }));

        if let Some(head_node) = self.head.as_ref() {
            head_node.borrow_mut().prev = Some(Rc::downgrade(&node));
        }

        self.head = Some(node);
    }

    pub fn insert_after(&mut self, prev_data: T, data: T) -> bool { 
        let mut current = self.head.as_ref().unwrap().clone();
    
        loop {
            if current.borrow().data == prev_data {
                let next = current.borrow().next.as_ref().cloned();
    
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: next.clone(),
                    prev: Some(Rc::downgrade(&current)),
                }));
    
                if let Some(ref next_node) = next {
                    next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                }
    
                current.borrow_mut().next = Some(new_node);
    
                return true;
            }
    
            let next_node = {
                let current_borrow = current.borrow();
                current_borrow.next.as_ref().cloned()
            };
    
            if let Some(next) = next_node {
                current = next;
            } else {
                return false;
            }
        }        
    }

    pub fn delete(&mut self, data: T) {
        if self.head.is_none() {
            return;
        }

        let mut current = self.head.as_ref().unwrap().clone();

        if current.borrow().data == data {
            let next = current.borrow_mut().next.take();
 
            if let Some(ref next_node) = next {
                next_node.borrow_mut().prev = None;
            }
 
            self.head = next;
 
            return;
        }

        loop {
            let next: Option<Rc<RefCell<Node<T>>>> = {
                let current_borrow = current.borrow();
                current_borrow.next.as_ref().cloned()
            };
            
            let next_node = match next {
                Some(node) => node,
                None => break,  
            };

            if next_node.borrow().data == data {
                let mut node_to_remove = next_node.borrow_mut();
                
                let next = node_to_remove.next.take();
                let prev = node_to_remove.prev.take();

                if let Some(next) = next.clone() {
                    next.borrow_mut().prev = prev.clone();
                }   

                if let Some(prev) = prev.clone() {
                    prev.upgrade().unwrap().borrow_mut().next = next;
                }

                return;
            } else {
                current = next_node;
            }
        }        
    }

    pub fn reverse(&mut self) {
        let mut current = self.head.take();
        let mut new_head = None; 

        while let Some(node_rc) = current {
            let mut node = node_rc.borrow_mut();
    
            let next = node.next.take();
            let prev = node.prev.take().and_then(|w| w.upgrade());

            node.next = prev.clone();
            node.prev = next.as_ref().map(|rc| Rc::downgrade(rc));
    
            drop(node);

            current = next;
            new_head = Some(node_rc);
        }

        self.head = new_head;
    }
    
    pub fn search(&self, data: T) -> bool {
        self.iter().any(|x| x == data)
    }    

    pub fn length(&self) -> usize {
        self.iter().count()
    }    
    
}

pub struct Iter<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|current_rc| {
            let current_ref = current_rc.borrow();
            self.next = current_ref.next.clone();
            current_ref.data.clone()
        })
    }
}


impl<T: Debug> fmt::Display for DoubleLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self.head.clone();
        let mut debug_list = f.debug_list();

        if current.is_none() {
            debug_list.entry(&"Empty");
        } else {
            while let Some(rc_node) = current {
                let node_ref = rc_node.borrow();
                debug_list.entry(&node_ref.data);
                current = node_ref.next.clone();
            }
        }

        debug_list.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_backward<T: Clone>(tail: Rc<RefCell<Node<T>>>) -> Vec<T> {
        let mut result = vec![];
        let mut current = Some(tail);
    
        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            result.push(node.data.clone());
            current = node.prev.as_ref().and_then(|weak| weak.upgrade());
        }
    
        result.reverse(); // so it matches forward order
        result
    }

    #[test]
    fn test_insert_at_end_with_prev() {
        let mut list = DoubleLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
    
        assert_eq!(format!("{}", list), "[1, 2, 3]");
    
        // Traverse to tail
        let mut tail = list.head.clone().unwrap();

        while let Some(next) = tail.clone().borrow().next.clone() {
            tail = next;
        }
    
        // Backward traversal should yield [1, 2, 3]
        let collected = collect_backward(tail);
        assert_eq!(collected, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_insert_at_beginning_with_prev() {
        let mut list = DoubleLinkedList::new();
        list.insert_at_end(10);
        list.insert_at_beginning(20);
        list.insert_at_beginning(30);
    
        assert_eq!(format!("{}", list), "[30, 20, 10]");
    
        // Traverse to tail
        let mut tail = list.head.clone().unwrap();
        while let Some(next) = tail.clone().borrow().next.clone() {
            tail = next;
        }
    
        let collected = collect_backward(tail);
        assert_eq!(collected, vec![30, 20, 10]);
    }
    
    #[test]
    fn test_insert_after_preserves_prev_links() {
        let mut list = DoubleLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        assert!(list.insert_after(2, 99));
    
        assert_eq!(format!("{}", list), "[1, 2, 99, 3]");
    
        // Traverse to tail
        let mut tail = list.head.clone().unwrap();
        while let Some(next) = tail.clone().borrow().next.clone() {
            tail = next;
        }
    
        let collected = collect_backward(tail);
        assert_eq!(collected, vec![1, 2, 99, 3]);
    }
    
    #[test]
    fn test_delete_preserves_prev_links() {
        let mut list = DoubleLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        list.insert_at_end(4);
        list.delete(3);
    
        assert_eq!(format!("{}", list), "[1, 2, 4]");
    
        // Traverse to tail
        let mut tail = list.head.clone().unwrap();
        while let Some(next) = tail.clone().borrow().next.clone() {
            tail = next;
        }
    
        let collected = collect_backward(tail);
        assert_eq!(collected, vec![1, 2, 4]);
    }
    
    #[test]
    fn test_reverse_preserves_prev_links() {
        let mut list = DoubleLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        list.reverse();
    
        assert_eq!(format!("{}", list), "[3, 2, 1]");
    
        // Traverse to tail
        let mut tail = list.head.clone().unwrap();
        while let Some(next) = tail.clone().borrow().next.clone() {
            tail = next;
        }
    
        let collected = collect_backward(tail);
        assert_eq!(collected, vec![3, 2, 1]);
    }    
}
