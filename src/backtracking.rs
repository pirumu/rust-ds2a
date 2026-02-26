//! # Backtracking Algorithms
//!
//! N-Queens, Sudoku solver, permutations, and power set.

/// N-Queens — find all ways to place `n` queens on an n x n board.
///
/// Returns a list of boards where `board[r][c]` is `true` if a queen occupies that cell.
/// **Time:** O(n!).
pub fn n_queens(n: usize) -> Vec<Vec<Vec<bool>>> {
    let mut solutions = Vec::new();
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n]; // row + col
    let mut diag2 = vec![false; 2 * n]; // row - col + n
    let mut placement: Vec<usize> = Vec::with_capacity(n); // column for each row

    fn solve(
        row: usize,
        n: usize,
        placement: &mut Vec<usize>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        solutions: &mut Vec<Vec<Vec<bool>>>,
    ) {
        if row == n {
            // Convert placement to board.
            let mut board = vec![vec![false; n]; n];
            for (r, &c) in placement.iter().enumerate() {
                board[r][c] = true;
            }
            solutions.push(board);
            return;
        }
        for col in 0..n {
            let d1 = row + col;
            let d2 = row + n - col;
            if !cols[col] && !diag1[d1] && !diag2[d2] {
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                placement.push(col);
                solve(row + 1, n, placement, cols, diag1, diag2, solutions);
                placement.pop();
                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }
    }

    if n > 0 {
        solve(0, n, &mut placement, &mut cols, &mut diag1, &mut diag2, &mut solutions);
    }
    solutions
}

/// Sudoku Solver — fill a 9x9 board in-place. Empty cells are represented by `0`.
///
/// Returns `true` if a solution is found.
pub fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    // Find next empty cell.
    let mut empty = None;
    'outer: for (r, row) in board.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 0 {
                empty = Some((r, c));
                break 'outer;
            }
        }
    }
    let (row, col) = match empty {
        Some(pos) => pos,
        None => return true, // No empty cell — solved.
    };

    for num in 1..=9 {
        if is_valid_placement(board, row, col, num) {
            board[row][col] = num;
            if solve_sudoku(board) {
                return true;
            }
            board[row][col] = 0;
        }
    }
    false
}

fn is_valid_placement(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    // Check row and column.
    for i in 0..9 {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }
    // Check 3x3 box.
    let box_r = (row / 3) * 3;
    let box_c = (col / 3) * 3;
    for row in board.iter().skip(box_r).take(3) {
        for &cell in row.iter().skip(box_c).take(3) {
            if cell == num {
                return false;
            }
        }
    }
    true
}

/// Generate all permutations of the input slice.
///
/// **Time:** O(n! * n).  **Space:** O(n! * n).
pub fn permutations<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = arr.to_vec();
    let n = current.len();
    permute_helper(&mut current, 0, n, &mut result);
    result
}

fn permute_helper<T: Clone>(arr: &mut Vec<T>, start: usize, n: usize, result: &mut Vec<Vec<T>>) {
    if start == n {
        result.push(arr.clone());
        return;
    }
    for i in start..n {
        arr.swap(start, i);
        permute_helper(arr, start + 1, n, result);
        arr.swap(start, i);
    }
}

/// Generate all subsets (power set) of the input slice.
///
/// **Time:** O(2^n * n).  **Space:** O(2^n * n).
pub fn subsets<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let mut current: Vec<T> = Vec::new();
    subsets_helper(arr, 0, &mut current, &mut result);
    result
}

fn subsets_helper<T: Clone>(arr: &[T], idx: usize, current: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
    result.push(current.clone());
    for i in idx..arr.len() {
        current.push(arr[i].clone());
        subsets_helper(arr, i + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- n_queens ----
    #[test]
    fn n_queens_4() {
        let solutions = n_queens(4);
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn n_queens_1() {
        let solutions = n_queens(1);
        assert_eq!(solutions.len(), 1);
        assert!(solutions[0][0][0]);
    }

    #[test]
    fn n_queens_0() {
        let solutions = n_queens(0);
        assert_eq!(solutions.len(), 0);
    }

    #[test]
    fn n_queens_8() {
        let solutions = n_queens(8);
        assert_eq!(solutions.len(), 92);
    }

    // ---- sudoku ----
    #[test]
    fn sudoku_solvable() {
        let mut board: [[u8; 9]; 9] = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert!(solve_sudoku(&mut board));
        // Verify no zeros remain.
        for row in &board {
            for &cell in row {
                assert_ne!(cell, 0);
            }
        }
    }

    // ---- permutations ----
    #[test]
    fn permutations_three() {
        let result = permutations(&[1, 2, 3]);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn permutations_empty() {
        let result: Vec<Vec<i32>> = permutations(&[]);
        assert_eq!(result.len(), 1); // one empty permutation
    }

    #[test]
    fn permutations_single() {
        let result = permutations(&[42]);
        assert_eq!(result, vec![vec![42]]);
    }

    // ---- subsets ----
    #[test]
    fn subsets_three() {
        let result = subsets(&[1, 2, 3]);
        assert_eq!(result.len(), 8); // 2^3
    }

    #[test]
    fn subsets_empty() {
        let result: Vec<Vec<i32>> = subsets(&[]);
        assert_eq!(result.len(), 1); // just the empty set
    }
}
