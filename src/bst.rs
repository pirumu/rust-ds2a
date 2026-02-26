//! # Binary Search Tree
//!
//! A binary search tree where each node's left subtree contains only values
//! less than the node, and the right subtree contains only values greater.

/// A node in the BST.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/// A binary search tree.
#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BST<T> {
    /// Creates an empty BST.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Inserts a value into the BST. Duplicates are ignored.
    pub fn insert(&mut self, val: T) {
        fn insert_node<T: Ord>(node: &mut Option<Box<Node<T>>>, val: T) {
            match node {
                None => {
                    *node = Some(Box::new(Node {
                        value: val,
                        left: None,
                        right: None,
                    }));
                }
                Some(n) => {
                    if val < n.value {
                        insert_node(&mut n.left, val);
                    } else if val > n.value {
                        insert_node(&mut n.right, val);
                    }
                    // equal: duplicate, do nothing
                }
            }
        }
        insert_node(&mut self.root, val);
    }

    /// Returns `true` if the BST contains the given value.
    pub fn search(&self, val: &T) -> bool {
        fn find<T: Ord>(node: &Option<Box<Node<T>>>, val: &T) -> bool {
            match node {
                None => false,
                Some(n) => {
                    if *val < n.value {
                        find(&n.left, val)
                    } else if *val > n.value {
                        find(&n.right, val)
                    } else {
                        true
                    }
                }
            }
        }
        find(&self.root, val)
    }

    /// Deletes a value from the BST.
    pub fn delete(&mut self, val: &T) {
        fn remove<T: Ord>(node: &mut Option<Box<Node<T>>>, val: &T) {
            let Some(n) = node else { return };

            if *val < n.value {
                remove(&mut n.left, val);
            } else if *val > n.value {
                remove(&mut n.right, val);
            } else {
                // Found the node to delete.
                match (n.left.take(), n.right.take()) {
                    (None, None) => {
                        *node = None;
                    }
                    (Some(left), None) => {
                        *node = Some(left);
                    }
                    (None, Some(right)) => {
                        *node = Some(right);
                    }
                    (left, Some(mut right)) => {
                        // Two children: replace with in-order successor (min of right subtree).
                        if right.left.is_none() {
                            right.left = left;
                            *node = Some(right);
                        } else {
                            let successor_val = detach_min(&mut right);
                            n.left = left;
                            n.right = Some(right);
                            n.value = successor_val;
                        }
                    }
                }
            }
        }

        /// Detaches the minimum node from a subtree and returns its value.
        fn detach_min<T: Ord>(node: &mut Box<Node<T>>) -> T {
            if node.left.is_some() {
                if node.left.as_ref().unwrap().left.is_none() {
                    let mut min_node = node.left.take().unwrap();
                    node.left = min_node.right.take();
                    min_node.value
                } else {
                    detach_min(node.left.as_mut().unwrap())
                }
            } else {
                unreachable!("detach_min called on a node without left child");
            }
        }

        remove(&mut self.root, val);
    }

    /// Returns a reference to the minimum value, or `None` if empty.
    pub fn min(&self) -> Option<&T> {
        fn find_min<T>(node: &Option<Box<Node<T>>>) -> Option<&T> {
            let n = node.as_ref()?;
            if n.left.is_none() {
                Some(&n.value)
            } else {
                find_min(&n.left)
            }
        }
        find_min(&self.root)
    }

    /// Returns a reference to the maximum value, or `None` if empty.
    pub fn max(&self) -> Option<&T> {
        fn find_max<T>(node: &Option<Box<Node<T>>>) -> Option<&T> {
            let n = node.as_ref()?;
            if n.right.is_none() {
                Some(&n.value)
            } else {
                find_max(&n.right)
            }
        }
        find_max(&self.root)
    }

    /// Returns an in-order traversal of references (sorted order).
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

impl<T: Ord> Default for BST<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bst_is_empty() {
        let bst: BST<i32> = BST::new();
        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);
        assert!(bst.inorder().is_empty());
    }

    #[test]
    fn insert_and_search() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        assert!(bst.search(&5));
        assert!(bst.search(&3));
        assert!(bst.search(&7));
        assert!(!bst.search(&1));
    }

    #[test]
    fn inorder_is_sorted() {
        let mut bst = BST::new();
        for v in [5, 3, 7, 1, 4, 6, 8] {
            bst.insert(v);
        }
        let sorted: Vec<&i32> = bst.inorder();
        assert_eq!(sorted, vec![&1, &3, &4, &5, &6, &7, &8]);
    }

    #[test]
    fn min_and_max() {
        let mut bst = BST::new();
        for v in [5, 3, 7, 1, 9] {
            bst.insert(v);
        }
        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&9));
    }

    #[test]
    fn delete_leaf() {
        let mut bst = BST::new();
        for v in [5, 3, 7] {
            bst.insert(v);
        }
        bst.delete(&3);
        assert!(!bst.search(&3));
        assert!(bst.search(&5));
        assert!(bst.search(&7));
    }

    #[test]
    fn delete_node_with_one_child() {
        let mut bst = BST::new();
        for v in [5, 3, 7, 6] {
            bst.insert(v);
        }
        bst.delete(&7);
        assert!(!bst.search(&7));
        assert!(bst.search(&6));
    }

    #[test]
    fn delete_node_with_two_children() {
        let mut bst = BST::new();
        for v in [5, 3, 7, 1, 4, 6, 8] {
            bst.insert(v);
        }
        bst.delete(&5);
        assert!(!bst.search(&5));
        let sorted: Vec<&i32> = bst.inorder();
        assert_eq!(sorted, vec![&1, &3, &4, &6, &7, &8]);
    }

    #[test]
    fn delete_root_single_node() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.delete(&10);
        assert!(!bst.search(&10));
        assert!(bst.inorder().is_empty());
    }

    #[test]
    fn duplicate_insert_ignored() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(5);
        bst.insert(5);
        assert_eq!(bst.inorder().len(), 1);
    }

    #[test]
    fn delete_nonexistent() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.delete(&99);
        assert!(bst.search(&5));
    }
}
