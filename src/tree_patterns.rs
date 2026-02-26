use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn wrap(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}

/// Build a binary tree from level-order representation.
/// `None` entries represent null nodes.
pub fn from_level_order(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }

    let root = TreeNode::wrap(vals[0].unwrap());
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());

    let mut i = 1;
    while i < vals.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = TreeNode::wrap(v);
                    current.borrow_mut().left = left.clone();
                    queue.push_back(left.unwrap());
                }
            }
            i += 1;

            // Right child
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = TreeNode::wrap(v);
                    current.borrow_mut().right = right.clone();
                    queue.push_back(right.unwrap());
                }
            }
            i += 1;
        }
    }

    root
}

/// Lowest Common Ancestor: find the LCA of nodes with values `p` and `q`.
/// Returns `Some(val)` of the LCA node, or `None` if either p or q is not found.
pub fn lca(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<i32> {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = node.as_ref()?;
        let val = node.borrow().val;

        if val == p || val == q {
            return Some(node.clone());
        }

        let left = dfs(&node.borrow().left, p, q);
        let right = dfs(&node.borrow().right, p, q);

        match (&left, &right) {
            (Some(_), Some(_)) => Some(node.clone()),
            (Some(_), None) => left,
            (None, Some(_)) => right,
            (None, None) => None,
        }
    }

    dfs(root, p, q).map(|n| n.borrow().val)
}

/// Serialize a binary tree to a comma-separated string using BFS level-order.
/// Trailing nulls are trimmed.
pub fn serialize(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
    if root.is_none() {
        return String::new();
    }

    let mut result: Vec<String> = Vec::new();
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    queue.push_back(root.clone());

    while let Some(front) = queue.pop_front() {
        match front {
            Some(node) => {
                result.push(node.borrow().val.to_string());
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
            None => {
                result.push("null".to_string());
            }
        }
    }

    // Trim trailing nulls
    while result.last().map_or(false, |s| s == "null") {
        result.pop();
    }

    result.join(",")
}

/// Deserialize a comma-separated string back into a binary tree.
pub fn deserialize(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
    if s.is_empty() {
        return None;
    }

    let tokens: Vec<&str> = s.split(',').collect();
    if tokens.is_empty() || tokens[0] == "null" {
        return None;
    }

    let root_val: i32 = tokens[0].parse().ok()?;
    let root = TreeNode::wrap(root_val);
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());

    let mut i = 1;
    while i < tokens.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if i < tokens.len() && tokens[i] != "null" {
                if let Ok(v) = tokens[i].parse::<i32>() {
                    let left = TreeNode::wrap(v);
                    current.borrow_mut().left = left.clone();
                    queue.push_back(left.unwrap());
                }
            }
            i += 1;

            // Right child
            if i < tokens.len() && tokens[i] != "null" {
                if let Ok(v) = tokens[i].parse::<i32>() {
                    let right = TreeNode::wrap(v);
                    current.borrow_mut().right = right.clone();
                    queue.push_back(right.unwrap());
                }
            }
            i += 1;
        }
    }

    root
}

/// Maximum path sum through any nodes in the tree.
/// A path can start and end at any node.
pub fn max_path_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, global_max: &mut i32) -> i32 {
        let node = match node.as_ref() {
            Some(n) => n,
            None => return 0,
        };

        let val = node.borrow().val;
        let left_gain = dfs(&node.borrow().left, global_max).max(0);
        let right_gain = dfs(&node.borrow().right, global_max).max(0);

        // Path through this node as the "turning point"
        let path_sum = val + left_gain + right_gain;
        *global_max = (*global_max).max(path_sum);

        // Return max gain if we continue upward (can only pick one branch)
        val + left_gain.max(right_gain)
    }

    let mut global_max = i32::MIN;
    dfs(root, &mut global_max);
    global_max
}

/// Diameter of a binary tree: the longest path (in edges) between any two nodes.
pub fn diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut usize) -> usize {
        let node = match node.as_ref() {
            Some(n) => n,
            None => return 0,
        };

        let left_depth = dfs(&node.borrow().left, best);
        let right_depth = dfs(&node.borrow().right, best);

        *best = (*best).max(left_depth + right_depth);

        1 + left_depth.max(right_depth)
    }

    let mut best = 0;
    dfs(root, &mut best);
    best
}

/// Validate whether a binary tree is a valid BST.
/// Uses i64 bounds to handle i32 edge cases.
pub fn is_valid_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn validate(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        let node = match node.as_ref() {
            Some(n) => n,
            None => return true,
        };

        let val = node.borrow().val as i64;
        if val <= min || val >= max {
            return false;
        }

        validate(&node.borrow().left, min, val) && validate(&node.borrow().right, val, max)
    }

    validate(root, i64::MIN, i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lca_basic() {
        let tree = from_level_order(&[Some(3), Some(5), Some(1), Some(6), Some(2)]);
        assert_eq!(lca(&tree, 5, 1), Some(3));
        assert_eq!(lca(&tree, 6, 2), Some(5));
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let tree = from_level_order(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
        let s = serialize(&tree);
        let tree2 = deserialize(&s);
        assert_eq!(serialize(&tree2), s);
    }

    #[test]
    fn max_path_sum_basic() {
        let tree =
            from_level_order(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_path_sum(&tree), 42);
    }

    #[test]
    fn diameter_basic() {
        let tree = from_level_order(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter(&tree), 3);
    }

    #[test]
    fn valid_bst_true() {
        let tree = from_level_order(&[Some(2), Some(1), Some(3)]);
        assert!(is_valid_bst(&tree));
    }

    #[test]
    fn valid_bst_false() {
        let tree = from_level_order(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!is_valid_bst(&tree));
    }
}
