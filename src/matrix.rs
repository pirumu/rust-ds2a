//! # Matrix Traversal
//!
//! Spiral order, diagonal traversal, 90-degree rotation, flood fill,
//! island counting (DFS), and shortest path in a binary grid (BFS).

use std::collections::VecDeque;

/// Spiral Order — return elements of a matrix in spiral (clockwise) order.
///
/// Starting from top-left, walk right → down → left → up, shrinking
/// the boundary after each pass.
///
/// **Time:** O(m*n). **Space:** O(m*n).
pub fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut result = Vec::with_capacity(m * n);

    let (mut top, mut bottom) = (0i32, m as i32 - 1);
    let (mut left, mut right) = (0i32, n as i32 - 1);

    while top <= bottom && left <= right {
        // → right along top row
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }
        top += 1;

        // ↓ down along right column
        for row in top..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }
        right -= 1;

        // ← left along bottom row
        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
            bottom -= 1;
        }

        // ↑ up along left column
        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
            left += 1;
        }
    }

    result
}

/// Diagonal Order — group elements that share the same diagonal index
/// (`row + col`).
///
/// Returns a `Vec<Vec<i32>>` where the *k*-th inner vec contains all
/// elements with `row + col == k`.
///
/// **Time:** O(m*n). **Space:** O(m*n).
pub fn diagonal_order(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }
    let (m, n) = (matrix.len(), matrix[0].len());
    let num_diags = m + n - 1;
    let mut diags: Vec<Vec<i32>> = vec![Vec::new(); num_diags];

    for r in 0..m {
        for c in 0..n {
            diags[r + c].push(matrix[r][c]);
        }
    }

    diags
}

/// Rotate 90° clockwise — modify the matrix in-place.
///
/// Strategy: transpose, then reverse each row.
///
/// **Time:** O(m*n). **Space:** O(1) (in-place).
pub fn rotate_90(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();
    if n == 0 {
        return;
    }

    // Transpose.
    for i in 0..n {
        for j in (i + 1)..n {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }

    // Reverse each row.
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

/// Flood Fill — starting from `(sr, sc)`, repaint all connected cells
/// of the same original color with `new_color`.
///
/// Uses DFS (recursive). Connected means 4-directional (up/down/left/right).
///
/// **Time:** O(m*n). **Space:** O(m*n) (call stack).
pub fn flood_fill(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, new_color: i32) {
    let original = image[sr][sc];
    if original == new_color {
        return;
    }
    dfs_fill(image, sr, sc, original, new_color);
}

fn dfs_fill(image: &mut Vec<Vec<i32>>, r: usize, c: usize, original: i32, new_color: i32) {
    if image[r][c] != original {
        return;
    }
    image[r][c] = new_color;
    let (m, n) = (image.len(), image[0].len());
    if r > 0 {
        dfs_fill(image, r - 1, c, original, new_color);
    }
    if r + 1 < m {
        dfs_fill(image, r + 1, c, original, new_color);
    }
    if c > 0 {
        dfs_fill(image, r, c - 1, original, new_color);
    }
    if c + 1 < n {
        dfs_fill(image, r, c + 1, original, new_color);
    }
}

/// Number of Islands — count connected components of `'1'` cells.
///
/// Each island is a group of `'1'`s connected 4-directionally.
/// The grid is modified in-place (visited cells are sunk to `'0'`).
///
/// **Time:** O(m*n). **Space:** O(m*n) (call stack worst case).
pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }
    let (m, n) = (grid.len(), grid[0].len());
    let mut count = 0;

    for r in 0..m {
        for c in 0..n {
            if grid[r][c] == '1' {
                count += 1;
                sink_island(grid, r, c);
            }
        }
    }
    count
}

fn sink_island(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
    let (m, n) = (grid.len(), grid[0].len());
    if grid[r][c] != '1' {
        return;
    }
    grid[r][c] = '0';
    if r > 0 {
        sink_island(grid, r - 1, c);
    }
    if r + 1 < m {
        sink_island(grid, r + 1, c);
    }
    if c > 0 {
        sink_island(grid, r, c - 1);
    }
    if c + 1 < n {
        sink_island(grid, r, c + 1);
    }
}

/// Shortest Path in Binary Grid — find the shortest path from `(0,0)` to
/// `(m-1, n-1)` moving in 8 directions. Only cells with value `0` are passable.
///
/// Returns the path length (number of cells visited), or `-1` if no path exists.
///
/// **Time:** O(m*n). **Space:** O(m*n).
pub fn shortest_path_grid(grid: &[Vec<i32>]) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return -1;
    }
    let (m, n) = (grid.len(), grid[0].len());
    if grid[0][0] != 0 || grid[m - 1][n - 1] != 0 {
        return -1;
    }

    let mut visited = vec![vec![false; n]; m];
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
    queue.push_back((0, 0, 1));
    visited[0][0] = true;

    let dirs: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    while let Some((r, c, dist)) = queue.pop_front() {
        if r == m - 1 && c == n - 1 {
            return dist;
        }
        for &(dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && grid[nr][nc] == 0 {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, dist + 1));
                }
            }
        }
    }

    -1
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- spiral_order ----

    #[test]
    fn spiral_3x3() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(&matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn spiral_3x4() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            spiral_order(&matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn spiral_single_row() {
        let matrix = vec![vec![1, 2, 3]];
        assert_eq!(spiral_order(&matrix), vec![1, 2, 3]);
    }

    #[test]
    fn spiral_single_col() {
        let matrix = vec![vec![1], vec![2], vec![3]];
        assert_eq!(spiral_order(&matrix), vec![1, 2, 3]);
    }

    #[test]
    fn spiral_empty() {
        let matrix: Vec<Vec<i32>> = Vec::new();
        assert_eq!(spiral_order(&matrix), Vec::<i32>::new());
    }

    #[test]
    fn spiral_1x1() {
        let matrix = vec![vec![42]];
        assert_eq!(spiral_order(&matrix), vec![42]);
    }

    // ---- diagonal_order ----

    #[test]
    fn diagonal_3x3() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let diags = diagonal_order(&matrix);
        assert_eq!(
            diags,
            vec![vec![1], vec![2, 4], vec![3, 5, 7], vec![6, 8], vec![9]]
        );
    }

    #[test]
    fn diagonal_2x3() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let diags = diagonal_order(&matrix);
        assert_eq!(
            diags,
            vec![vec![1], vec![2, 4], vec![3, 5], vec![6]]
        );
    }

    #[test]
    fn diagonal_empty() {
        let matrix: Vec<Vec<i32>> = Vec::new();
        assert!(diagonal_order(&matrix).is_empty());
    }

    // ---- rotate_90 ----

    #[test]
    fn rotate_3x3() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_90(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn rotate_2x2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_90(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
    }

    #[test]
    fn rotate_1x1() {
        let mut matrix = vec![vec![1]];
        rotate_90(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }

    #[test]
    fn rotate_four_times_identity() {
        let original = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut matrix = original.clone();
        for _ in 0..4 {
            rotate_90(&mut matrix);
        }
        assert_eq!(matrix, original);
    }

    // ---- flood_fill ----

    #[test]
    fn flood_fill_basic() {
        let mut image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        flood_fill(&mut image, 1, 1, 2);
        assert_eq!(image, vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
    }

    #[test]
    fn flood_fill_same_color() {
        let mut image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        flood_fill(&mut image, 0, 0, 0);
        assert_eq!(image, vec![vec![0, 0, 0], vec![0, 0, 0]]);
    }

    #[test]
    fn flood_fill_single_cell() {
        let mut image = vec![vec![5]];
        flood_fill(&mut image, 0, 0, 9);
        assert_eq!(image, vec![vec![9]]);
    }

    #[test]
    fn flood_fill_partial() {
        // Only the connected region of 1s from (0,0) should be filled.
        let mut image = vec![vec![1, 1, 0], vec![1, 0, 0], vec![0, 0, 1]];
        flood_fill(&mut image, 0, 0, 3);
        assert_eq!(image, vec![vec![3, 3, 0], vec![3, 0, 0], vec![0, 0, 1]]);
    }

    // ---- num_islands ----

    #[test]
    fn islands_basic() {
        let mut grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(num_islands(&mut grid), 1);
    }

    #[test]
    fn islands_three() {
        let mut grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(num_islands(&mut grid), 3);
    }

    #[test]
    fn islands_none() {
        let mut grid = vec![vec!['0', '0'], vec!['0', '0']];
        assert_eq!(num_islands(&mut grid), 0);
    }

    #[test]
    fn islands_all_land() {
        let mut grid = vec![vec!['1', '1'], vec!['1', '1']];
        assert_eq!(num_islands(&mut grid), 1);
    }

    #[test]
    fn islands_empty() {
        let mut grid: Vec<Vec<char>> = Vec::new();
        assert_eq!(num_islands(&mut grid), 0);
    }

    // ---- shortest_path_grid ----

    #[test]
    fn shortest_path_3x3() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(shortest_path_grid(&grid), 4);
    }

    #[test]
    fn shortest_path_diagonal() {
        let grid = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        // Only path: (0,0) -> (1,1) -> (2,2) = 3 cells
        assert_eq!(shortest_path_grid(&grid), 3);
    }

    #[test]
    fn shortest_path_blocked() {
        // With 8-directional movement, (0,0) can reach (1,1) diagonally
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(shortest_path_grid(&grid), 2);
    }

    #[test]
    fn shortest_path_start_blocked() {
        let grid = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(shortest_path_grid(&grid), -1);
    }

    #[test]
    fn shortest_path_1x1() {
        let grid = vec![vec![0]];
        assert_eq!(shortest_path_grid(&grid), 1);
    }

    #[test]
    fn shortest_path_wide_open() {
        // In a 3x3 grid of all 0s, the shortest diagonal path is 3.
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(shortest_path_grid(&grid), 3);
    }

    #[test]
    fn shortest_path_empty() {
        let grid: Vec<Vec<i32>> = Vec::new();
        assert_eq!(shortest_path_grid(&grid), -1);
    }
}
