//! # Stack
//!
//! A last-in, first-out (LIFO) stack backed by a `Vec`.

/// A generic LIFO stack backed by a `Vec<T>`.
#[derive(Debug, Clone)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Pushes a value onto the top of the stack. O(1) amortized.
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    /// Removes and returns the value on top of the stack, or `None` if empty. O(1).
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns a reference to the top element without removing it, or `None` if empty. O(1).
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns `true` if the stack contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the stack.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek_returns_top() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.size(), 2); // peek does not remove
    }

    #[test]
    fn peek_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn size_tracks_correctly() {
        let mut stack = Stack::new();
        assert_eq!(stack.size(), 0);
        stack.push('a');
        stack.push('b');
        assert_eq!(stack.size(), 2);
        stack.pop();
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn lifo_order() {
        let mut stack = Stack::new();
        for i in 0..5 {
            stack.push(i);
        }
        let mut result = Vec::new();
        while let Some(v) = stack.pop() {
            result.push(v);
        }
        assert_eq!(result, vec![4, 3, 2, 1, 0]);
    }
}
