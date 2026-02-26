//! # AVL Tree
//!
//! A self-balancing binary search tree where the heights of the two child
//! subtrees of every node differ by at most one.

use std::cmp::Ordering;

/// A node in the AVL tree.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: i64,
}

impl<T: Ord> Node<T> {
    fn new(val: T) -> Self {
        Node {
            value: val,
            left: None,
            right: None,
            height: 1,
        }
    }
}

/// Returns the height of an optional node (0 for `None`).
fn height<T>(node: &Option<Box<Node<T>>>) -> i64 {
    node.as_ref().map_or(0, |n| n.height)
}

/// Returns the balance factor of a node (left height - right height).
fn balance_factor<T>(node: &Node<T>) -> i64 {
    height(&node.left) - height(&node.right)
}

/// Recalculates the stored height from children.
fn update_height<T>(node: &mut Node<T>) {
    node.height = 1 + height(&node.left).max(height(&node.right));
}

/// Right rotation (for left-heavy / LL case).
fn right_rotate<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let mut new_root = root.left.take().expect("right_rotate requires left child");
    root.left = new_root.right.take();
    update_height(&mut root);
    new_root.right = Some(root);
    update_height(&mut new_root);
    new_root
}

/// Left rotation (for right-heavy / RR case).
fn left_rotate<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let mut new_root = root.right.take().expect("left_rotate requires right child");
    root.right = new_root.left.take();
    update_height(&mut root);
    new_root.left = Some(root);
    update_height(&mut new_root);
    new_root
}

/// Rebalances a node after insertion/deletion.
fn rebalance<T: Ord>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    update_height(&mut node);
    let bf = balance_factor(&node);

    if bf > 1 {
        // Left-heavy
        if balance_factor(node.left.as_ref().unwrap()) < 0 {
            // LR case: left-rotate left child first
            let left = node.left.take().unwrap();
            node.left = Some(left_rotate(left));
        }
        return right_rotate(node);
    }

    if bf < -1 {
        // Right-heavy
        if balance_factor(node.right.as_ref().unwrap()) > 0 {
            // RL case: right-rotate right child first
            let right = node.right.take().unwrap();
            node.right = Some(right_rotate(right));
        }
        return left_rotate(node);
    }

    node
}

fn insert_node<T: Ord>(node: Option<Box<Node<T>>>, val: T) -> Box<Node<T>> {
    let Some(mut n) = node else {
        return Box::new(Node::new(val));
    };

    match val.cmp(&n.value) {
        Ordering::Less => n.left = Some(insert_node(n.left.take(), val)),
        Ordering::Greater => n.right = Some(insert_node(n.right.take(), val)),
        Ordering::Equal => return n, // duplicate
    }

    rebalance(n)
}

fn search_node<T: Ord>(node: &Option<Box<Node<T>>>, val: &T) -> bool {
    let Some(n) = node else { return false };
    match val.cmp(&n.value) {
        Ordering::Less => search_node(&n.left, val),
        Ordering::Greater => search_node(&n.right, val),
        Ordering::Equal => true,
    }
}

/// A self-balancing AVL tree.
#[derive(Debug)]
pub struct AVLTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> AVLTree<T> {
    /// Creates an empty AVL tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Inserts a value, rebalancing as needed. Duplicates are ignored.
    pub fn insert(&mut self, val: T) {
        let root = self.root.take();
        self.root = Some(insert_node(root, val));
    }

    /// Returns `true` if the tree contains the given value.
    pub fn search(&self, val: &T) -> bool {
        search_node(&self.root, val)
    }

    /// Returns the height of the tree (0 for empty).
    pub fn height(&self) -> i64 {
        height(&self.root)
    }

    /// Returns an in-order traversal (sorted).
    pub fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        fn walk<'a, T>(node: &'a Option<Box<Node<T>>>, out: &mut Vec<&'a T>) {
            if let Some(n) = node {
                walk(&n.left, out);
                out.push(&n.value);
                walk(&n.right, out);
            }
        }
        walk(&self.root, &mut result);
        result
    }
}

impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_avl_is_empty() {
        let tree: AVLTree<i32> = AVLTree::new();
        assert_eq!(tree.height(), 0);
        assert!(tree.inorder().is_empty());
    }

    #[test]
    fn insert_and_search() {
        let mut tree = AVLTree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(5);
        assert!(tree.search(&10));
        assert!(tree.search(&20));
        assert!(tree.search(&5));
        assert!(!tree.search(&99));
    }

    #[test]
    fn rr_rotation() {
        // Inserting 1, 2, 3 in order triggers a left (RR) rotation.
        let mut tree = AVLTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
        assert_eq!(tree.height(), 2); // balanced
    }

    #[test]
    fn ll_rotation() {
        // Inserting 3, 2, 1 triggers a right (LL) rotation.
        let mut tree = AVLTree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn lr_rotation() {
        // 3, 1, 2 triggers LR rotation.
        let mut tree = AVLTree::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn rl_rotation() {
        // 1, 3, 2 triggers RL rotation.
        let mut tree = AVLTree::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn many_insertions_stay_balanced() {
        let mut tree = AVLTree::new();
        for i in 1..=31 {
            tree.insert(i);
        }
        // 31 nodes in a balanced tree should have height 5.
        assert!(tree.height() <= 5);
        let sorted: Vec<&i32> = tree.inorder();
        let expected: Vec<i32> = (1..=31).collect();
        let expected_refs: Vec<&i32> = expected.iter().collect();
        assert_eq!(sorted, expected_refs);
    }

    #[test]
    fn duplicate_ignored() {
        let mut tree = AVLTree::new();
        tree.insert(5);
        tree.insert(5);
        assert_eq!(tree.inorder().len(), 1);
    }
}
