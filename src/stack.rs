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

/// Tính giá trị biểu thức hậu tố (Reverse Polish Notation).
///
/// Mỗi token là một số nguyên hoặc một toán tử (+, -, *, /).
/// Trả về `Some(kết quả)` nếu biểu thức hợp lệ, `None` nếu không.
pub fn evaluate_rpn(tokens: &[&str]) -> Option<i64> {
    let mut stack = Stack::new();

    for &token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop()?; // lấy toán hạng phải
                let a = stack.pop()?; // lấy toán hạng trái
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 {
                            return None;
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num_str => {
                let num: i64 = num_str.parse().ok()?;
                stack.push(num);
            }
        }
    }

    // Biểu thức hợp lệ: stack còn đúng 1 phần tử
    if stack.size() == 1 {
        stack.pop()
    } else {
        None
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
    fn rpn_basic() {
        // 3 4 + = 7
        assert_eq!(super::evaluate_rpn(&["3", "4", "+"]), Some(7));
    }

    #[test]
    fn rpn_complex() {
        // 3 4 + 2 * = (3+4)*2 = 14
        assert_eq!(super::evaluate_rpn(&["3", "4", "+", "2", "*"]), Some(14));
    }

    #[test]
    fn rpn_subtraction_and_division() {
        // 10 3 - = 7
        assert_eq!(super::evaluate_rpn(&["10", "3", "-"]), Some(7));
        // 20 4 / = 5
        assert_eq!(super::evaluate_rpn(&["20", "4", "/"]), Some(5));
    }

    #[test]
    fn rpn_division_by_zero() {
        assert_eq!(super::evaluate_rpn(&["5", "0", "/"]), None);
    }

    #[test]
    fn rpn_invalid_expression() {
        // Too many operators
        assert_eq!(super::evaluate_rpn(&["3", "+", "+"]), None);
        // Too many operands
        assert_eq!(super::evaluate_rpn(&["3", "4"]), None);
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
