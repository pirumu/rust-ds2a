//! # Graph Patterns
//!
//! Common graph interview patterns: bipartite check, cycle detection
//! (directed & undirected), connected components, and course scheduling
//! (topological sort).

use std::collections::{HashMap, HashSet, VecDeque};

// ---------------------------------------------------------------------------
// Bipartite Check (BFS 2-coloring)
// ---------------------------------------------------------------------------

/// Check whether an undirected graph (given as adjacency list) is bipartite.
///
/// A graph is bipartite if its vertices can be divided into two disjoint sets
/// such that every edge connects a vertex in one set to one in the other.
/// Uses BFS coloring — if we can 2-color the graph without conflict, it is
/// bipartite.
///
/// **Time:** O(V + E).  **Space:** O(V).
pub fn is_bipartite(adj: &[Vec<usize>]) -> bool {
    let n = adj.len();
    // 0 = unvisited, 1 = color A, 2 = color B
    let mut color = vec![0u8; n];

    for start in 0..n {
        if color[start] != 0 {
            continue;
        }
        // BFS from this unvisited node
        color[start] = 1;
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if color[v] == 0 {
                    color[v] = if color[u] == 1 { 2 } else { 1 };
                    queue.push_back(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Cycle Detection — Directed Graph (DFS 3-color)
// ---------------------------------------------------------------------------

/// Detect whether a directed graph contains a cycle.
///
/// Uses DFS with three states per node:
/// - WHITE (0): unvisited
/// - GRAY  (1): in the current DFS path (on the recursion stack)
/// - BLACK (2): fully processed
///
/// A back edge (to a GRAY node) indicates a cycle.
///
/// **Time:** O(V + E).  **Space:** O(V).
pub fn detect_cycle_directed(n: usize, adj: &[Vec<usize>]) -> bool {
    assert_eq!(adj.len(), n);
    let mut state = vec![0u8; n]; // 0=white, 1=gray, 2=black

    for start in 0..n {
        if state[start] == 0 && dfs_cycle_directed(start, adj, &mut state) {
            return true;
        }
    }
    false
}

fn dfs_cycle_directed(u: usize, adj: &[Vec<usize>], state: &mut [u8]) -> bool {
    state[u] = 1; // gray
    for &v in &adj[u] {
        if state[v] == 1 {
            return true; // back edge -> cycle
        }
        if state[v] == 0 && dfs_cycle_directed(v, adj, state) {
            return true;
        }
    }
    state[u] = 2; // black
    false
}

// ---------------------------------------------------------------------------
// Cycle Detection — Undirected Graph (Union-Find)
// ---------------------------------------------------------------------------

/// Detect whether an undirected graph contains a cycle using Union-Find.
///
/// For each edge, if both endpoints are already in the same set, adding the
/// edge creates a cycle.
///
/// **Time:** O(E * alpha(V)) ~ O(E).  **Space:** O(V).
pub fn detect_cycle_undirected(n: usize, edges: &[[usize; 2]]) -> bool {
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0u8; n];

    for edge in edges {
        let (x, y) = (edge[0], edge[1]);
        let rx = uf_find(&mut parent, x);
        let ry = uf_find(&mut parent, y);
        if rx == ry {
            return true;
        }
        uf_union(&mut parent, &mut rank, rx, ry);
    }
    false
}

fn uf_find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = uf_find(parent, parent[x]);
    }
    parent[x]
}

fn uf_union(parent: &mut [usize], rank: &mut [u8], x: usize, y: usize) {
    match rank[x].cmp(&rank[y]) {
        std::cmp::Ordering::Less => parent[x] = y,
        std::cmp::Ordering::Greater => parent[y] = x,
        std::cmp::Ordering::Equal => {
            parent[y] = x;
            rank[x] += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Connected Components (DFS)
// ---------------------------------------------------------------------------

/// Count the number of connected components in an undirected graph.
///
/// **Time:** O(V + E).  **Space:** O(V).
pub fn count_connected_components(n: usize, adj: &[Vec<usize>]) -> usize {
    assert_eq!(adj.len(), n);
    let mut visited = vec![false; n];
    let mut count = 0;

    for i in 0..n {
        if !visited[i] {
            count += 1;
            // DFS using explicit stack
            let mut stack = vec![i];
            while let Some(u) = stack.pop() {
                if visited[u] {
                    continue;
                }
                visited[u] = true;
                for &v in &adj[u] {
                    if !visited[v] {
                        stack.push(v);
                    }
                }
            }
        }
    }
    count
}

// ---------------------------------------------------------------------------
// Course Schedule (Topological Sort — Kahn's BFS)
// ---------------------------------------------------------------------------

/// Determine whether it is possible to finish all courses given prerequisites.
///
/// `prereqs[i] = [a, b]` means "to take course `a` you must first take `b`".
/// This is the classic "Course Schedule" problem — returns `true` if there is
/// no cycle in the prerequisite graph (i.e. a valid topological ordering exists).
///
/// **Time:** O(V + E).  **Space:** O(V + E).
pub fn can_finish_courses(n: usize, prereqs: &[[usize; 2]]) -> bool {
    // Build adjacency list and in-degree array.
    let mut adj = vec![vec![]; n];
    let mut in_degree = vec![0usize; n];

    for p in prereqs {
        let (course, prereq) = (p[0], p[1]);
        adj[prereq].push(course);
        in_degree[course] += 1;
    }

    // Kahn's algorithm: start with all zero in-degree nodes.
    let mut queue: VecDeque<usize> = VecDeque::new();
    for (i, &deg) in in_degree.iter().enumerate() {
        if deg == 0 {
            queue.push_back(i);
        }
    }

    let mut processed = 0;
    while let Some(u) = queue.pop_front() {
        processed += 1;
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    processed == n
}

// ---------------------------------------------------------------------------
// Alien Dictionary (Topological Sort on characters)
// ---------------------------------------------------------------------------

/// Derive character ordering from a sorted list of words in an alien language.
///
/// Builds a DAG by comparing adjacent words, then performs topological sort.
/// Returns `None` if the ordering is contradictory (cycle) or if a shorter
/// word appears after a longer word that is its prefix.
///
/// **Time:** O(total characters).
pub fn alien_dictionary(words: &[&str]) -> Option<String> {
    // Collect all unique characters.
    let mut chars: HashSet<char> = HashSet::new();
    for w in words {
        for c in w.chars() {
            chars.insert(c);
        }
    }

    // Build adjacency list and in-degree map.
    let mut adj: HashMap<char, Vec<char>> = HashMap::new();
    let mut in_degree: HashMap<char, usize> = HashMap::new();
    for &c in &chars {
        adj.entry(c).or_default();
        in_degree.entry(c).or_insert(0);
    }

    // Compare adjacent words to derive ordering constraints.
    for pair in words.windows(2) {
        let (w1, w2) = (pair[0], pair[1]);
        let mut found_diff = false;
        for (c1, c2) in w1.chars().zip(w2.chars()) {
            if c1 != c2 {
                adj.entry(c1).or_default().push(c2);
                *in_degree.entry(c2).or_insert(0) += 1;
                found_diff = true;
                break;
            }
        }
        // Prefix contradiction: "abc" before "ab" is invalid.
        if !found_diff && w1.len() > w2.len() {
            return None;
        }
    }

    // Kahn's topological sort.
    let mut queue: VecDeque<char> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&c, _)| c)
        .collect();
    let mut result = String::new();

    while let Some(c) = queue.pop_front() {
        result.push(c);
        if let Some(neighbors) = adj.get(&c) {
            for &next in neighbors {
                let deg = in_degree.get_mut(&next).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if result.len() == chars.len() {
        Some(result)
    } else {
        None // cycle detected
    }
}

// ---------------------------------------------------------------------------
// Course Schedule II (Topological Sort — return ordering)
// ---------------------------------------------------------------------------

/// Return a valid course ordering given prerequisites, or `None` if impossible.
///
/// `prereqs` are `(course, prerequisite)` pairs. Uses Kahn's algorithm.
///
/// **Time:** O(V + E).
pub fn course_order(num_courses: usize, prereqs: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; num_courses];
    let mut in_degree = vec![0usize; num_courses];

    for &(course, prereq) in prereqs {
        adj[prereq].push(course);
        in_degree[course] += 1;
    }

    let mut queue: VecDeque<usize> = (0..num_courses)
        .filter(|&i| in_degree[i] == 0)
        .collect();
    let mut order = Vec::with_capacity(num_courses);

    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if order.len() == num_courses {
        Some(order)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Clone Graph (deep-copy adjacency list)
// ---------------------------------------------------------------------------

/// Deep-copy an adjacency list.
///
/// **Time:** O(V + E).
pub fn clone_graph(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    adj.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- is_bipartite ----

    #[test]
    fn bipartite_simple_yes() {
        // 0 -- 1
        // |    |
        // 3 -- 2
        let adj = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert!(is_bipartite(&adj));
    }

    #[test]
    fn bipartite_odd_cycle_no() {
        // Triangle: 0-1-2-0
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert!(!is_bipartite(&adj));
    }

    #[test]
    fn bipartite_disconnected() {
        // 0-1 (bipartite)  and  2-3-4-2 (odd cycle)
        let adj = vec![
            vec![1],       // 0
            vec![0],       // 1
            vec![3, 4],    // 2
            vec![2, 4],    // 3
            vec![2, 3],    // 4
        ];
        assert!(!is_bipartite(&adj));
    }

    #[test]
    fn bipartite_single_node() {
        let adj: Vec<Vec<usize>> = vec![vec![]];
        assert!(is_bipartite(&adj));
    }

    #[test]
    fn bipartite_empty_graph() {
        let adj: Vec<Vec<usize>> = vec![vec![], vec![], vec![]];
        assert!(is_bipartite(&adj));
    }

    #[test]
    fn bipartite_complete_bipartite() {
        // K2,2: {0,1} x {2,3}
        let adj = vec![
            vec![2, 3],    // 0
            vec![2, 3],    // 1
            vec![0, 1],    // 2
            vec![0, 1],    // 3
        ];
        assert!(is_bipartite(&adj));
    }

    // ---- detect_cycle_directed ----

    #[test]
    fn directed_cycle_exists() {
        // 0 -> 1 -> 2 -> 0
        let adj = vec![vec![1], vec![2], vec![0]];
        assert!(detect_cycle_directed(3, &adj));
    }

    #[test]
    fn directed_no_cycle() {
        // 0 -> 1 -> 2
        let adj = vec![vec![1], vec![2], vec![]];
        assert!(!detect_cycle_directed(3, &adj));
    }

    #[test]
    fn directed_self_loop() {
        let adj = vec![vec![0]];
        assert!(detect_cycle_directed(1, &adj));
    }

    #[test]
    fn directed_diamond_no_cycle() {
        // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3
        let adj = vec![vec![1, 2], vec![3], vec![3], vec![]];
        assert!(!detect_cycle_directed(4, &adj));
    }

    #[test]
    fn directed_disconnected_with_cycle() {
        // Component 1: 0 -> 1 (no cycle)
        // Component 2: 2 -> 3 -> 2 (cycle)
        let adj = vec![vec![1], vec![], vec![3], vec![2]];
        assert!(detect_cycle_directed(4, &adj));
    }

    #[test]
    fn directed_empty() {
        let adj: Vec<Vec<usize>> = vec![vec![], vec![]];
        assert!(!detect_cycle_directed(2, &adj));
    }

    // ---- detect_cycle_undirected ----

    #[test]
    fn undirected_cycle_triangle() {
        let edges = [[0, 1], [1, 2], [2, 0]];
        assert!(detect_cycle_undirected(3, &edges));
    }

    #[test]
    fn undirected_no_cycle_tree() {
        // 0-1, 1-2, 1-3 (tree)
        let edges = [[0, 1], [1, 2], [1, 3]];
        assert!(!detect_cycle_undirected(4, &edges));
    }

    #[test]
    fn undirected_no_edges() {
        let edges: [[usize; 2]; 0] = [];
        assert!(!detect_cycle_undirected(5, &edges));
    }

    #[test]
    fn undirected_single_edge() {
        let edges = [[0, 1]];
        assert!(!detect_cycle_undirected(2, &edges));
    }

    #[test]
    fn undirected_four_node_cycle() {
        let edges = [[0, 1], [1, 2], [2, 3], [3, 0]];
        assert!(detect_cycle_undirected(4, &edges));
    }

    // ---- count_connected_components ----

    #[test]
    fn components_single_graph() {
        // 0-1-2 fully connected
        let adj = vec![vec![1], vec![0, 2], vec![1]];
        assert_eq!(count_connected_components(3, &adj), 1);
    }

    #[test]
    fn components_three_isolated() {
        let adj: Vec<Vec<usize>> = vec![vec![], vec![], vec![]];
        assert_eq!(count_connected_components(3, &adj), 3);
    }

    #[test]
    fn components_two_groups() {
        // {0,1} and {2,3}
        let adj = vec![vec![1], vec![0], vec![3], vec![2]];
        assert_eq!(count_connected_components(4, &adj), 2);
    }

    #[test]
    fn components_empty() {
        let adj: Vec<Vec<usize>> = vec![];
        assert_eq!(count_connected_components(0, &adj), 0);
    }

    #[test]
    fn components_single_node() {
        let adj: Vec<Vec<usize>> = vec![vec![]];
        assert_eq!(count_connected_components(1, &adj), 1);
    }

    // ---- can_finish_courses ----

    #[test]
    fn courses_simple_yes() {
        // 0 <- 1 (take 1 before 0)
        assert!(can_finish_courses(2, &[[0, 1]]));
    }

    #[test]
    fn courses_cycle_no() {
        // 0 <- 1, 1 <- 0 (circular dependency)
        assert!(!can_finish_courses(2, &[[0, 1], [1, 0]]));
    }

    #[test]
    fn courses_chain() {
        // 0 <- 1 <- 2 <- 3
        assert!(can_finish_courses(4, &[[0, 1], [1, 2], [2, 3]]));
    }

    #[test]
    fn courses_no_prereqs() {
        let empty: [[usize; 2]; 0] = [];
        assert!(can_finish_courses(5, &empty));
    }

    #[test]
    fn courses_diamond() {
        // 3 <- 1, 3 <- 2, 1 <- 0, 2 <- 0
        assert!(can_finish_courses(4, &[[1, 0], [2, 0], [3, 1], [3, 2]]));
    }

    #[test]
    fn courses_complex_cycle() {
        // 0 <- 1, 1 <- 2, 2 <- 0 (cycle among 3)
        assert!(!can_finish_courses(3, &[[0, 1], [1, 2], [2, 0]]));
    }

    #[test]
    fn courses_disconnected_with_cycle() {
        // Component 1: 0 <- 1 (ok)
        // Component 2: 2 <- 3, 3 <- 2 (cycle)
        assert!(!can_finish_courses(4, &[[0, 1], [2, 3], [3, 2]]));
    }

    #[test]
    fn courses_single_course() {
        let empty: [[usize; 2]; 0] = [];
        assert!(can_finish_courses(1, &empty));
    }

    // ---- alien_dictionary ----

    #[test]
    fn alien_dict_basic() {
        let words = ["wrt", "wrf", "er", "ett", "rftt"];
        let order = alien_dictionary(&words).unwrap();
        assert!(order.contains('w'));
        assert!(order.find('t').unwrap() < order.find('f').unwrap());
    }

    #[test]
    fn alien_dict_invalid() {
        let words = ["abc", "ab"];
        assert!(alien_dictionary(&words).is_none());
    }

    // ---- course_order ----

    #[test]
    fn course_order_basic() {
        let order = course_order(4, &[(1, 0), (2, 0), (3, 1), (3, 2)]).unwrap();
        let pos: std::collections::HashMap<usize, usize> =
            order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        assert!(pos[&0] < pos[&1]);
        assert!(pos[&0] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
    }

    #[test]
    fn course_order_cycle() {
        assert!(course_order(2, &[(0, 1), (1, 0)]).is_none());
    }

    // ---- clone_graph ----

    #[test]
    fn clone_graph_basic() {
        let adj = vec![vec![1, 2], vec![0], vec![0, 3], vec![2]];
        let cloned = clone_graph(&adj);
        assert_eq!(adj, cloned);
    }
}
