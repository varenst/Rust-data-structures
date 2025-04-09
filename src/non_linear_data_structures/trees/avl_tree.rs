#[derive(Clone)]
pub struct AvlNode<T> {
    value: T,
    right: Option<Box<AvlNode<T>>>,
    left: Option<Box<AvlNode<T>>>,
    height: i64,
}

impl<T> AvlNode<T> {
    pub fn new(value: T) -> Self {
        AvlNode {
            value,
            right: None,
            left: None,
            height: 0,
        }
    }
}

pub struct AvlTree<T> {
    root: Option<Box<AvlNode<T>>>,
}

impl<T: Eq + Clone + Ord> AvlTree<T> {
    pub fn new(value: T) -> Self {
        Self {
            root: Some(Box::new(AvlNode::new(value))),
        }
    }

    fn get_height(node: Option<&Box<AvlNode<T>>>) -> i64 {
        match node {
            Some(n) => {
                1 + std::cmp::max(
                    AvlTree::get_height(n.left.as_ref()),
                    AvlTree::get_height(n.right.as_ref()),
                )
            }
            None => -1, // height of empty node is -1
        }
    }

    fn get_balance(node: Option<&Box<AvlNode<T>>>) -> i64 {
        match node {
            Some(n) => {
                let num_left = AvlTree::get_height(n.left.as_ref());
                let num_right = AvlTree::get_height(n.right.as_ref());
                num_left - num_right
            }
            None => 0,
        }
    }

    fn rotate_right(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut left_node = node.left.take().unwrap();
        let right_node = left_node.right.take();

        left_node.right = Some(node);
        left_node.right.as_mut().unwrap().left = right_node;

        left_node.right.as_mut().unwrap().height = 1 + std::cmp::max(
            AvlTree::get_height(left_node.right.as_ref().unwrap().left.as_ref()),
            AvlTree::get_height(left_node.right.as_ref().unwrap().right.as_ref()),
        );

        left_node.height = 1 + std::cmp::max(
            AvlTree::get_height(left_node.left.as_ref()),
            AvlTree::get_height(left_node.right.as_ref()),
        );

        left_node
    }

    fn rotate_left(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        let mut right_node = node.right.take().unwrap();
        let left_node = right_node.left.take();

        right_node.left = Some(node);
        right_node.left.as_mut().unwrap().right = left_node;

        right_node.left.as_mut().unwrap().height = 1 + std::cmp::max(
            AvlTree::get_height(right_node.left.as_ref().unwrap().left.as_ref()),
            AvlTree::get_height(right_node.left.as_ref().unwrap().right.as_ref()),
        );

        right_node.height = 1 + std::cmp::max(
            AvlTree::get_height(right_node.left.as_ref()),
            AvlTree::get_height(right_node.right.as_ref()),
        );

        right_node
    }

    fn rotate_left_right(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        node.left = Some(AvlTree::rotate_left(node.left.take().unwrap()));
        AvlTree::rotate_right(node)
    }

    fn rotate_right_left(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        node.right = Some(AvlTree::rotate_right(node.right.take().unwrap()));
        AvlTree::rotate_left(node)
    }

    fn min_value_node(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
        while let Some(left) = node.left.take() {
            node = left;
        }
        node
    }
    fn insert_node(mut node: Box<AvlNode<T>>, value: T) -> Box<AvlNode<T>> {
        if value < node.value {
            if let Some(left_node) = node.left.take() {
                node.left = Some(AvlTree::insert_node(left_node, value.clone()));
            } else {
                node.left = Some(Box::new(AvlNode::new(value.clone())));
            }
        } else if value > node.value {
            if let Some(right_node) = node.right.take() {
                node.right = Some(AvlTree::insert_node(right_node, value.clone()));
            } else {
                node.right = Some(Box::new(AvlNode::new(value.clone())));
            }
        } else {
            return node;
        }

        node.height = 1 + std::cmp::max(
            AvlTree::get_height(node.left.as_ref()),
            AvlTree::get_height(node.right.as_ref()),
        );

        let balance = AvlTree::get_balance(Some(&node));

        // Left Heavy
        if balance > 1 {
            if value < node.left.as_ref().unwrap().value {
                return AvlTree::rotate_right(node); // LL
            } else {
                return AvlTree::rotate_left_right(node); // LR
            }
        }

        // Right Heavy
        if balance < -1 {
            if value > node.right.as_ref().unwrap().value {
                return AvlTree::rotate_left(node); // RR
            } else {
                return AvlTree::rotate_right_left(node); // RL
            }
        }

        node
    }

    pub fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        if let Some(root) = self.root.take() {
            self.root = Some(AvlTree::insert_node(root, value));
        } else {
            self.root = Some(Box::new(AvlNode::new(value)));
        }
    }

    fn delete_node(mut node: Box<AvlNode<T>>, value: T) -> Option<Box<AvlNode<T>>> {
        if value < node.value {
            if let Some(left) = node.left.take() {
                node.left = AvlTree::delete_node(left, value);
            }
        } else if value > node.value {
            if let Some(right) = node.right.take() {
                node.right = AvlTree::delete_node(right, value);
            }
        } else {
            // Node to be deleted found

            // Case 1: Only one child or no child
            if node.left.is_none() {
                return node.right;
            } else if node.right.is_none() {
                return node.left;
            }

            // Case 2: Two children
            let right_subtree = node.right.take().unwrap();
            let successor = AvlTree::min_value_node(right_subtree.clone());
            node.value = successor.value.clone();
            node.right = AvlTree::delete_node(right_subtree, successor.value);
            
        }

        node.height = 1 + std::cmp::max(
            AvlTree::get_height(node.left.as_ref()),
            AvlTree::get_height(node.right.as_ref()),
        );

        let balance = AvlTree::get_balance(Some(&node));

        if balance > 1 {
            if AvlTree::get_balance(node.left.as_ref()) >= 0 {
                return Some(AvlTree::rotate_right(node)); // LL
            } else {
                return Some(AvlTree::rotate_left_right(node)); // LR
            }
        }

        // Right Heavy
        if balance < -1 {
            if AvlTree::get_balance(node.right.as_ref()) <= 0 {
                return Some(AvlTree::rotate_left(node)); // RR
            } else {
                return Some(AvlTree::rotate_right_left(node)); // RL
            }
        }

        Some(node)
    }

    pub fn delete(&mut self, value: T) {
        if let Some(root) = self.root.take() {
            self.root = AvlTree::delete_node(root, value);
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avl_tree_creation() {
        let tree = AvlTree::new(10);
        assert!(tree.root.is_some());

        if let Some(root) = &tree.root {
            assert_eq!(root.value, 10);
            assert!(root.left.is_none());
            assert!(root.right.is_none());
            assert_eq!(root.height, 0);
        }
    }

    #[test]
    fn test_single_insertion() {
        let mut tree = AvlTree::new(20);
        tree.insert(10);

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 20);
        assert!(root.left.is_some());
        assert!(root.right.is_none());
    }

    #[test]
    fn test_ll_rotation() {
        let mut tree = AvlTree::new(30);
        tree.insert(20);
        tree.insert(10); // Triggers LL rotation

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 20);
        assert_eq!(root.left.as_ref().unwrap().value, 10);
        assert_eq!(root.right.as_ref().unwrap().value, 30);
    }

    #[test]
    fn test_rr_rotation() {
        let mut tree = AvlTree::new(10);
        tree.insert(20);
        tree.insert(30); // Triggers RR rotation

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 20);
        assert_eq!(root.left.as_ref().unwrap().value, 10);
        assert_eq!(root.right.as_ref().unwrap().value, 30);
    }

    #[test]
    fn test_lr_rotation() {
        let mut tree = AvlTree::new(30);
        tree.insert(10);
        tree.insert(20); // Triggers LR rotation

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 20);
        assert_eq!(root.left.as_ref().unwrap().value, 10);
        assert_eq!(root.right.as_ref().unwrap().value, 30);
    }

    #[test]
    fn test_rl_rotation() {
        let mut tree = AvlTree::new(10);
        tree.insert(30);
        tree.insert(20); // Triggers RL rotation

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 20);
        assert_eq!(root.left.as_ref().unwrap().value, 10);
        assert_eq!(root.right.as_ref().unwrap().value, 30);
    }

    #[test]
    fn test_balanced_tree_heights() {
        let mut tree = AvlTree::new(15);
        tree.insert(10);
        tree.insert(20);
        tree.insert(5);
        tree.insert(12);
        tree.insert(18);
        tree.insert(25);

        fn assert_balanced<T>(node: &Option<Box<AvlNode<T>>>)
        where
            T: std::fmt::Debug + Clone + Ord,
        {
            if let Some(n) = node {
                let bf = AvlTree::get_balance(Some(n));
                assert!(
                    (-1..=1).contains(&bf),
                    "Unbalanced at node with value {:?}",
                    n.value
                );
                assert_balanced(&n.left);
                assert_balanced(&n.right);
            }
        }

        assert_balanced(&tree.root);
    }

    #[test]
    fn test_avl_deletion_balanced() {
        let mut tree = AvlTree::new(20);
        tree.insert(10);
        tree.insert(30);
        tree.insert(5);
        tree.insert(15);
        tree.insert(25);
        tree.insert(35);

        tree.delete(10); // node with two children

        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.value, 20);
        assert!(root.left.is_some());
        assert!(root.right.is_some());

        // Optional: use assert_balanced() from earlier to check full balance
    }
}
