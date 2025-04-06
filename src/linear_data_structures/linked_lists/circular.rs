use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

// Weak and RefCell are used to not create reference cycles
// I did not expect them to be this different

pub struct CircularLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    length: u64,
}

impl<T: Eq + Clone> CircularLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn insert_at_end(&mut self, data: T) {
        match &self.head {
            Some(head_node) => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: None,
                }));

                let tail_node = head_node.borrow().prev.clone();

                tail_node
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .next = Some(new_node.clone());
                new_node.borrow_mut().prev = tail_node;

                new_node.borrow_mut().next = Some(head_node.clone());
                head_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            }
            None => {
                self.insert_at_beginning(data.clone());

                return;
            }
        }

        self.length += 1;
    }

    pub fn insert_at_beginning(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        match self.head.as_ref() {
            Some(head_node) => {
                let head_node_prev = head_node.borrow_mut().prev.clone();

                node.borrow_mut().next = Some(head_node.clone());
                node.borrow_mut().prev = head_node_prev.clone();

                head_node_prev.unwrap().upgrade().unwrap().borrow_mut().next = Some(node.clone());
                head_node.borrow_mut().prev = Some(Rc::downgrade(&node));
            }
            None => {
                node.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(Rc::downgrade(&node));
            }
        }

        self.length += 1;
        self.head = Some(node);
    }

    pub fn insert_after(&mut self, prev_data: T, data: T) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut current_node = self.head.as_ref().unwrap().clone();

        loop {
            if current_node.borrow().data == prev_data {
                let next = current_node.borrow().next.as_ref().cloned();

                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: next.clone(),
                    prev: Some(Rc::downgrade(&current_node)),
                }));

                next.clone().unwrap().borrow_mut().prev = Some(Rc::downgrade(&new_node));
                current_node.borrow_mut().next = Some(new_node);

                self.length += 1;

                return true;
            }

            let next_node = {
                let current_borrow = current_node.borrow();
                current_borrow.next.as_ref().cloned()
            };

            if Rc::ptr_eq(&next_node.clone().unwrap(), self.head.as_ref().unwrap()) {
                return false;
            }

            if let Some(next) = next_node {
                current_node = next;
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

        loop {
            if current.borrow().data == data {
                let prev = current.borrow().prev.clone().unwrap().upgrade().unwrap();
                let next = current.borrow().next.clone().unwrap();

                if Rc::ptr_eq(&current, &next) {
                    self.head = None;
                } else {
                    prev.borrow_mut().next = Some(next.clone());
                    next.borrow_mut().prev = Some(Rc::downgrade(&prev));

                    if Rc::ptr_eq(&current, self.head.as_ref().unwrap()) {
                        self.head = Some(next);
                    }
                }

                self.length -= 1;

                return;
            }

            let next_node = current.borrow().next.clone().unwrap();

            if Rc::ptr_eq(&next_node, self.head.as_ref().unwrap()) {
                break;
            }

            current = next_node;
        }
    }

    pub fn reverse(&mut self) {
        if self.head.is_none() {
            return;
        }

        let mut current = self.head.clone().unwrap();
        let original_head = current.clone();

        #[allow(unused_assignments)]
        let mut last = None;

        loop {
            let next_node = current.borrow().next.clone().unwrap();

            {
                let mut current_borrow = current.borrow_mut();
                let temp_next = current_borrow.next.clone();
                current_borrow.next = current_borrow.prev.as_ref().and_then(|w| w.upgrade());
                current_borrow.prev = temp_next.map(|rc| Rc::downgrade(&rc));
            }

            last = Some(current.clone());
            current = next_node;

            if Rc::ptr_eq(&current, &original_head) {
                break;
            }
        }

        self.head = last;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at_beginning_single_element() {
        let mut list = CircularLinkedList::new();
        list.insert_at_beginning(10);

        assert_eq!(list.length, 1);

        let head = list.head.clone().unwrap();
        assert_eq!(head.borrow().data, 10);

        let next = head.borrow().next.clone().unwrap();
        let prev = head.borrow().prev.clone().unwrap().upgrade().unwrap();

        assert!(Rc::ptr_eq(&head, &next));
        assert!(Rc::ptr_eq(&head, &prev));
    }

    #[test]
    fn test_insert_at_beginning_multiple_elements() {
        let mut list = CircularLinkedList::new();
        list.insert_at_beginning(10);
        list.insert_at_beginning(20);
        list.insert_at_beginning(30);

        assert_eq!(list.length, 3);

        // Expected order: 30 -> 20 -> 10
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![30, 20, 10]);
    }

    #[test]
    fn test_insert_at_end_single_element() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(5);

        assert_eq!(list.length, 1);

        let head = list.head.clone().unwrap();
        assert_eq!(head.borrow().data, 5);

        let next = head.borrow().next.clone().unwrap();
        let prev = head.borrow().prev.clone().unwrap().upgrade().unwrap();

        assert!(Rc::ptr_eq(&head, &next));
        assert!(Rc::ptr_eq(&head, &prev));
    }

    #[test]
    fn test_insert_at_end_multiple_elements() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        assert_eq!(list.length, 3);

        // Expected order: 1 -> 2 -> 3
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_insert_after_success() {
        let mut list = CircularLinkedList::new();

        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        // Insert after node with data 2
        let result = list.insert_after(2, 99);

        assert!(result);
        assert_eq!(list.length, 4);

        // Traverse and collect values
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            println!("Values: {}", current.clone().borrow().data);
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![1, 2, 99, 3]);
    }

    #[test]
    fn test_insert_after_not_found() {
        let mut list = CircularLinkedList::new();

        list.insert_at_end(1);
        list.insert_at_end(2);

        let result = list.insert_after(5, 42);

        assert!(!result);
        assert_eq!(list.length, 2);
    }

    #[test]
    fn test_insert_after_on_empty_list() {
        let mut list = CircularLinkedList::<i32>::new();
        let result = list.insert_after(1, 10);
        assert!(!result);
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_delete_head() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        list.delete(1);

        assert_eq!(list.length, 2);
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![2, 3]);
    }

    #[test]
    fn test_delete_tail() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        list.delete(3);

        assert_eq!(list.length, 2);
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn test_delete_middle() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(10);
        list.insert_at_end(20);
        list.insert_at_end(30);

        list.delete(20);

        assert_eq!(list.length, 2);
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![10, 30]);
    }

    #[test]
    fn test_delete_only_element() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(42);

        list.delete(42);

        assert_eq!(list.length, 0);
        assert!(list.head.is_none());
    }

    #[test]
    fn test_delete_non_existent() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        list.delete(99);

        assert_eq!(list.length, 3);
        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse_multiple_elements() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        list.reverse();

        let mut values = Vec::new();
        let mut current = list.head.clone().unwrap();

        loop {
            values.push(current.clone().borrow().data);
            current = current.clone().borrow().next.clone().unwrap();
            if Rc::ptr_eq(&current, list.head.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_single_element() {
        let mut list = CircularLinkedList::new();
        list.insert_at_end(99);
        list.reverse();

        assert_eq!(list.length, 1);
        let head = list.head.clone().unwrap();
        let next = head.borrow().next.clone().unwrap();
        let prev = head.borrow().prev.clone().unwrap().upgrade().unwrap();

        assert_eq!(head.borrow().data, 99);
        assert!(Rc::ptr_eq(&head, &next));
        assert!(Rc::ptr_eq(&head, &prev));
    }

    #[test]
    fn test_reverse_empty_list() {
        let mut list = CircularLinkedList::<i32>::new();
        list.reverse();

        assert!(list.head.is_none());
        assert_eq!(list.length, 0);
    }
}
