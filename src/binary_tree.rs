//! # Binary Tree
//!
//! A general-purpose binary tree with level-order insertion and standard traversals.

use std::collections::VecDeque;

/// A node in the binary tree holding a value and optional children.
#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// A generic binary tree that inserts elements in level-order (breadth-first).
#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Clone + std::fmt::Debug> BinaryTree<T> {
    /// Creates an empty binary tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Inserts a value using level-order (breadth-first) placement.
    ///
    /// The new node is placed in the first available position when
    /// traversing the tree level by level, left to right.
    pub fn insert(&mut self, val: T) {
        let new_node = Box::new(Node {
            value: val,
            left: None,
            right: None,
        });

        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        // BFS to find first node with an empty child slot.
        let mut queue: VecDeque<*mut Node<T>> = VecDeque::new();
        // SAFETY: We hold &mut self so no aliasing occurs. The raw pointers
        // are only used within this function scope while &mut self is held.
        queue.push_back(self.root.as_deref_mut().unwrap() as *mut Node<T>);

        while let Some(ptr) = queue.pop_front() {
            let node = unsafe { &mut *ptr };
            if node.left.is_none() {
                node.left = Some(new_node);
                return;
            }
            queue.push_back(node.left.as_deref_mut().unwrap() as *mut Node<T>);

            if node.right.is_none() {
                node.right = Some(new_node);
                return;
            }
            queue.push_back(node.right.as_deref_mut().unwrap() as *mut Node<T>);
        }
    }

    /// Returns an in-order traversal (left, root, right).
    pub fn inorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        fn walk<T: Clone>(node: &Option<Box<Node<T>>>, out: &mut Vec<T>) {
            if let Some(n) = node {
                walk(&n.left, out);
                out.push(n.value.clone());
                walk(&n.right, out);
            }
        }
        walk(&self.root, &mut result);
        result
    }

    /// Returns a pre-order traversal (root, left, right).
    pub fn preorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        fn walk<T: Clone>(node: &Option<Box<Node<T>>>, out: &mut Vec<T>) {
            if let Some(n) = node {
                out.push(n.value.clone());
                walk(&n.left, out);
                walk(&n.right, out);
            }
        }
        walk(&self.root, &mut result);
        result
    }

    /// Returns a post-order traversal (left, right, root).
    pub fn postorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        fn walk<T: Clone>(node: &Option<Box<Node<T>>>, out: &mut Vec<T>) {
            if let Some(n) = node {
                walk(&n.left, out);
                walk(&n.right, out);
                out.push(n.value.clone());
            }
        }
        walk(&self.root, &mut result);
        result
    }

    /// Returns a level-order (breadth-first) traversal.
    pub fn level_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            let mut queue: VecDeque<&Node<T>> = VecDeque::new();
            queue.push_back(root);
            while let Some(node) = queue.pop_front() {
                result.push(node.value.clone());
                if let Some(ref left) = node.left {
                    queue.push_back(left);
                }
                if let Some(ref right) = node.right {
                    queue.push_back(right);
                }
            }
        }
        result
    }

    /// Returns the height of the tree (0 for empty, 1 for single node).
    pub fn height(&self) -> usize {
        fn h<T>(node: &Option<Box<Node<T>>>) -> usize {
            match node {
                None => 0,
                Some(n) => 1 + h(&n.left).max(h(&n.right)),
            }
        }
        h(&self.root)
    }

    /// Returns the total number of nodes in the tree.
    pub fn count(&self) -> usize {
        fn c<T>(node: &Option<Box<Node<T>>>) -> usize {
            match node {
                None => 0,
                Some(n) => 1 + c(&n.left) + c(&n.right),
            }
        }
        c(&self.root)
    }
}

impl<T: Clone + std::fmt::Debug> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree_is_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.count(), 0);
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn insert_single() {
        let mut tree = BinaryTree::new();
        tree.insert(1);
        assert_eq!(tree.count(), 1);
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn level_order_insertion() {
        let mut tree = BinaryTree::new();
        for i in 1..=7 {
            tree.insert(i);
        }
        // Level-order insertion produces a complete binary tree:
        //        1
        //       / \
        //      2   3
        //     / \ / \
        //    4  5 6  7
        assert_eq!(tree.level_order(), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn inorder_traversal() {
        let mut tree = BinaryTree::new();
        for i in 1..=7 {
            tree.insert(i);
        }
        assert_eq!(tree.inorder(), vec![4, 2, 5, 1, 6, 3, 7]);
    }

    #[test]
    fn preorder_traversal() {
        let mut tree = BinaryTree::new();
        for i in 1..=7 {
            tree.insert(i);
        }
        assert_eq!(tree.preorder(), vec![1, 2, 4, 5, 3, 6, 7]);
    }

    #[test]
    fn postorder_traversal() {
        let mut tree = BinaryTree::new();
        for i in 1..=7 {
            tree.insert(i);
        }
        assert_eq!(tree.postorder(), vec![4, 5, 2, 6, 7, 3, 1]);
    }

    #[test]
    fn height_of_complete_tree() {
        let mut tree = BinaryTree::new();
        for i in 1..=7 {
            tree.insert(i);
        }
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn count_nodes() {
        let mut tree = BinaryTree::new();
        for i in 1..=10 {
            tree.insert(i);
        }
        assert_eq!(tree.count(), 10);
    }

    #[test]
    fn single_node_traversals() {
        let mut tree = BinaryTree::new();
        tree.insert(42);
        assert_eq!(tree.inorder(), vec![42]);
        assert_eq!(tree.preorder(), vec![42]);
        assert_eq!(tree.postorder(), vec![42]);
        assert_eq!(tree.level_order(), vec![42]);
    }
}
