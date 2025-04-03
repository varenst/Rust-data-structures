use core::fmt;
use std::fmt::Debug;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SingularLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Eq> SingularLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert_at_end(&mut self, data: T) {
        if self.head.is_none() {
            self.insert_at_beginning(data);

            return;
        }

        let new_node = Box::new(Node { data, next: None });

        let mut current = self.head.as_mut().unwrap();

        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }

        current.next = Some(new_node);
    }

    pub fn insert_at_beginning(&mut self, data: T) {
        let node = Box::new(Node {
            data,
            next: self.head.take(), // pushes out of the option the value leaving None there
        });

        self.head = Some(node);
    }

    pub fn insert_after(&mut self, prev_data: T, data: T) -> bool {
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.data == prev_data {
                let new_node = Box::new(Node {
                    data,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return true;
            }
            current = node.next.as_mut();
        }

        false
    }

    pub fn delete(&mut self, data: T) {
        let mut link = &mut self.head;

        while let Some(mut boxed_node) = link.take() {
            if boxed_node.data == data {
                *link = boxed_node.next.take();
                return;
            } else {
                let next = boxed_node.next.take();
                *link = Some(boxed_node);
                link = &mut link.as_mut().unwrap().next;
                *link = next;
            }
        }
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<Box<Node<T>>> = None;
        let mut current = self.head.take();

        while let Some(mut boxed_node) = current {
            let next = boxed_node.next.take();
            boxed_node.next = prev;
            prev = Some(boxed_node);
            current = next;
        }

        self.head = prev;
    }

    pub fn search(&self, data: T) -> bool {
        self.iter().any(|x| *x == data)
    }    

    pub fn length(&self) -> usize {
        self.iter().count()
    }    
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> SingularLinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref(); // advance
            &node.data
        })
    }
}

impl<T: Debug> fmt::Display for SingularLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.as_ref();
        let mut debug_list = f.debug_list();

        if current.is_none() {
            debug_list.entry(&"Empty");
        } else {
            while let Some(node) = current {
                debug_list.entry(&node.data);
                current = node.next.as_ref();
            }
        }

        debug_list.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at_end() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        assert_eq!(format!("{}", list), "[1, 2, 3]");
    }

    #[test]
    fn test_insert_at_beginning() {
        let mut list = SingularLinkedList::new();
        list.insert_at_beginning(10);
        list.insert_at_beginning(20);
        assert_eq!(format!("{}", list), "[20, 10]");
    }

    #[test]
    fn test_insert_after_middle() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        let result = list.insert_after(2, 99);
        println!("{}", list);
        assert!(result);
        assert_eq!(format!("{}", list), "[1, 2, 99, 3]");
    }

    #[test]
    fn test_insert_after_head() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        let result = list.insert_after(1, 42);
        assert!(result);
        assert_eq!(format!("{}", list), "[1, 42, 2]");
    }

    #[test]
    fn test_insert_after_nonexistent() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        let result = list.insert_after(99, 42);
        assert!(!result);
        assert_eq!(format!("{}", list), "[1]");
    }

    #[test]
    fn test_delete_middle_node() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        list.delete(2);

        println!("{}", list);
        assert_eq!(format!("{}", list), "[1, 3]");
    }

    #[test]
    fn test_delete_head_node() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(10);
        list.insert_at_end(20);
        list.delete(10);
        assert_eq!(format!("{}", list), "[20]");
    }

    #[test]
    fn test_search_found() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(100);
        list.insert_at_end(200);
        assert!(list.search(200));
    }

    #[test]
    fn test_search_not_found() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(10);
        assert!(!list.search(99));
    }

    #[test]
    fn test_reverse_list() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        list.reverse();
        assert_eq!(format!("{}", list), "[3, 2, 1]");
    }

    #[test]
    fn test_length_empty() {
        let list: SingularLinkedList<i32> = SingularLinkedList::new();
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_length_nonempty() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_display_empty_list() {
        let list: SingularLinkedList<i32> = SingularLinkedList::new();
        assert_eq!(format!("{}", list), "[\"Empty\"]");
    }

    #[test]
    fn test_iterator_collect() {
        let mut list = SingularLinkedList::new();
        list.insert_at_end(1);
        list.insert_at_end(2);
        list.insert_at_end(3);

        let collected: Vec<_> = list.iter().cloned().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
