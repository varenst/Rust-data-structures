// Same here really easy to implement you rather understand the idea
// Key difference between linked lists as stacks and arrays are in when you use them
// linked list stacks are great when size changes frequently and you need speed with O(1)
// But array based do better with predictable size and catch storing, but their speed is
// O(1) amortized

use std::collections::LinkedList;

pub struct LinkedListStack<T> {
    list: LinkedList<T>,
}

impl<T> LinkedListStack<T> {
    pub fn new() -> Self {
        LinkedListStack {
            list: LinkedList::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.list.push_front(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.front()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }
}
