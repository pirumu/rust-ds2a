//! # Red-Black Tree
//!
//! A self-balancing BST where each node is colored red or black, ensuring that
//! the tree remains approximately balanced after insertions.

use std::cmp::Ordering;

/// Node color.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

/// A node in the red-black tree.
#[derive(Debug)]
struct Node<T> {
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            value: val,
            color: Color::Red,
            left: None,
            right: None,
        }
    }
}

fn is_red<T>(node: &Option<Box<Node<T>>>) -> bool {
    node.as_ref().is_some_and(|n| n.color == Color::Red)
}

/// Left-leaning red-black tree implementation (LLRB).
/// This simplifies the standard red-black tree by maintaining the invariant
/// that red links always lean left.
fn rotate_left<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    let mut right = node.right.take().unwrap();
    node.right = right.left.take();
    right.color = node.color;
    node.color = Color::Red;
    right.left = Some(node);
    right
}

fn rotate_right<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    let mut left = node.left.take().unwrap();
    node.left = left.right.take();
    left.color = node.color;
    node.color = Color::Red;
    left.right = Some(node);
    left
}

fn flip_colors<T>(node: &mut Node<T>) {
    node.color = match node.color {
        Color::Red => Color::Black,
        Color::Black => Color::Red,
    };
    if let Some(ref mut left) = node.left {
        left.color = match left.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
    if let Some(ref mut right) = node.right {
        right.color = match right.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
}

fn fixup<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    // Right-leaning red link -> rotate left.
    if is_red(&node.right) && !is_red(&node.left) {
        node = rotate_left(node);
    }
    // Two consecutive left red links -> rotate right.
    if is_red(&node.left) && is_red(&node.left.as_ref().unwrap().left) {
        node = rotate_right(node);
    }
    // Both children red -> flip colors.
    if is_red(&node.left) && is_red(&node.right) {
        flip_colors(&mut node);
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

    fixup(n)
}

fn search_node<T: Ord>(node: &Option<Box<Node<T>>>, val: &T) -> bool {
    let Some(n) = node else { return false };
    match val.cmp(&n.value) {
        Ordering::Less => search_node(&n.left, val),
        Ordering::Greater => search_node(&n.right, val),
        Ordering::Equal => true,
    }
}

/// A left-leaning red-black tree.
#[derive(Debug)]
pub struct RedBlackTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> RedBlackTree<T> {
    /// Creates an empty red-black tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Inserts a value into the tree. Duplicates are ignored.
    pub fn insert(&mut self, val: T) {
        let root = self.root.take();
        let mut new_root = insert_node(root, val);
        new_root.color = Color::Black; // root is always black
        self.root = Some(new_root);
    }

    /// Returns `true` if the tree contains the given value.
    pub fn search(&self, val: &T) -> bool {
        search_node(&self.root, val)
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

    /// Checks that all root-to-leaf paths have the same black height.
    /// Returns `None` if the property is violated, otherwise the black height.
    fn black_height(node: &Option<Box<Node<T>>>) -> Option<usize> {
        match node {
            None => Some(1), // nil nodes are black
            Some(n) => {
                let lh = Self::black_height(&n.left)?;
                let rh = Self::black_height(&n.right)?;
                if lh != rh {
                    return None;
                }
                Some(lh + if n.color == Color::Black { 1 } else { 0 })
            }
        }
    }

    /// Returns `true` if the red-black tree properties hold.
    pub fn is_valid(&self) -> bool {
        // Property 1: root is black
        if is_red(&self.root) {
            return false;
        }
        // Property 2: no red node has a red child (checked via LLRB invariants)
        fn no_red_red<T>(node: &Option<Box<Node<T>>>) -> bool {
            let Some(n) = node else { return true };
            if n.color == Color::Red
                && (is_red(&n.left) || is_red(&n.right))
            {
                return false;
            }
            no_red_red(&n.left) && no_red_red(&n.right)
        }
        if !no_red_red(&self.root) {
            return false;
        }
        // Property 3: uniform black height
        Self::black_height(&self.root).is_some()
    }
}

impl<T: Ord> Default for RedBlackTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rbt_is_empty() {
        let tree: RedBlackTree<i32> = RedBlackTree::new();
        assert!(tree.inorder().is_empty());
        assert!(tree.is_valid());
    }

    #[test]
    fn insert_and_search() {
        let mut tree = RedBlackTree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(5);
        assert!(tree.search(&10));
        assert!(tree.search(&20));
        assert!(tree.search(&5));
        assert!(!tree.search(&99));
        assert!(tree.is_valid());
    }

    #[test]
    fn sorted_insertion_stays_valid() {
        let mut tree = RedBlackTree::new();
        for i in 1..=20 {
            tree.insert(i);
        }
        let sorted: Vec<&i32> = tree.inorder();
        let expected: Vec<i32> = (1..=20).collect();
        let expected_refs: Vec<&i32> = expected.iter().collect();
        assert_eq!(sorted, expected_refs);
        assert!(tree.is_valid());
    }

    #[test]
    fn reverse_insertion_stays_valid() {
        let mut tree = RedBlackTree::new();
        for i in (1..=20).rev() {
            tree.insert(i);
        }
        assert!(tree.is_valid());
    }

    #[test]
    fn many_insertions_valid() {
        let mut tree = RedBlackTree::new();
        let values = [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9];
        for v in values {
            tree.insert(v);
        }
        assert!(tree.is_valid());
        for v in values {
            assert!(tree.search(&v));
        }
    }

    #[test]
    fn duplicate_ignored() {
        let mut tree = RedBlackTree::new();
        tree.insert(5);
        tree.insert(5);
        tree.insert(5);
        assert_eq!(tree.inorder().len(), 1);
        assert!(tree.is_valid());
    }

    #[test]
    fn root_is_black() {
        let mut tree = RedBlackTree::new();
        tree.insert(1);
        // Root should be black -- validated by is_valid.
        assert!(tree.is_valid());
    }
}
