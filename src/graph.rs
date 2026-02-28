//! # Graph Data Structures and Algorithms
//!
//! An adjacency-list graph supporting directed/undirected, weighted/unweighted
//! edges, with classic graph algorithms: BFS, DFS, Dijkstra, Bellman-Ford,
//! Floyd-Warshall, Prim's MST, Kruskal's MST, topological sort, and cycle
//! detection.  Also includes a Union-Find (disjoint set) structure.

use std::collections::VecDeque;

// ---------------------------------------------------------------------------
// Union-Find
// ---------------------------------------------------------------------------

/// Disjoint-set (Union-Find) with path compression and union by rank.
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// Creates a Union-Find with `n` singleton sets `{0}, {1}, ..., {n-1}`.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Returns the representative of the set containing `x` (with path compression).
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merges the sets containing `x` and `y`.
    /// Returns `true` if they were in different sets (i.e. a merge happened).
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Graph
// ---------------------------------------------------------------------------

/// Adjacency-list graph with optional direction and `i64` edge weights.
#[derive(Debug, Clone)]
pub struct Graph {
    adjacency_list: Vec<Vec<(usize, i64)>>,
    directed: bool,
    num_vertices: usize,
}

impl Graph {
    // -- construction -------------------------------------------------------

    /// Creates a graph with `num_vertices` vertices (labeled `0..num_vertices`)
    /// and no edges.
    pub fn new(num_vertices: usize, directed: bool) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); num_vertices],
            directed,
            num_vertices,
        }
    }

    /// Adds a weighted edge from `from` to `to`.
    /// For undirected graphs, the reverse edge is also added.
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i64) {
        self.adjacency_list[from].push((to, weight));
        if !self.directed {
            self.adjacency_list[to].push((from, weight));
        }
    }

    /// Adds an unweighted edge (weight = 1).
    pub fn add_unweighted_edge(&mut self, from: usize, to: usize) {
        self.add_edge(from, to, 1);
    }

    /// Returns the neighbors of `vertex` as a slice of `(neighbor, weight)`.
    pub fn neighbors(&self, vertex: usize) -> &[(usize, i64)] {
        &self.adjacency_list[vertex]
    }

    /// Returns the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    // -- BFS ----------------------------------------------------------------

    /// Breadth-first search from `start`. Returns the visit order.
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.num_vertices];
        let mut order = Vec::new();
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            order.push(v);
            for &(neighbor, _) in &self.adjacency_list[v] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        order
    }

    // -- BFS shortest path --------------------------------------------------

    /// BFS shortest path from `start` to `end`. Returns the path as a vector
    /// of vertex indices, or `None` if no path exists.
    /// Only correct for unweighted graphs (all edges treated as weight 1).
    pub fn bfs_shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let mut visited = vec![false; self.num_vertices];
        let mut parent: Vec<Option<usize>> = vec![None; self.num_vertices];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            if v == end {
                let mut path = vec![end];
                let mut current = end;
                while let Some(p) = parent[current] {
                    path.push(p);
                    current = p;
                }
                path.reverse();
                return Some(path);
            }
            for &(neighbor, _) in &self.adjacency_list[v] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    parent[neighbor] = Some(v);
                    queue.push_back(neighbor);
                }
            }
        }
        None
    }

    // -- BFS by level -------------------------------------------------------

    /// BFS that returns vertices grouped by level (distance from start).
    pub fn bfs_by_level(&self, start: usize) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.num_vertices];
        let mut levels: Vec<Vec<usize>> = Vec::new();
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::new();

            for _ in 0..level_size {
                let v = queue.pop_front().unwrap();
                current_level.push(v);
                for &(neighbor, _) in &self.adjacency_list[v] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
            levels.push(current_level);
        }
        levels
    }

    // -- DFS ----------------------------------------------------------------

    /// Depth-first search from `start` (iterative). Returns the visit order.
    pub fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.num_vertices];
        let mut order = Vec::new();
        let mut stack = vec![start];

        while let Some(v) = stack.pop() {
            if visited[v] {
                continue;
            }
            visited[v] = true;
            order.push(v);
            // Push neighbors in reverse so the smallest-index neighbor is
            // visited first (matches a natural left-to-right ordering).
            for &(neighbor, _) in self.adjacency_list[v].iter().rev() {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
        order
    }

    // -- Dijkstra -----------------------------------------------------------

    /// Dijkstra's single-source shortest paths from `start`.
    /// Returns a vector of distances; unreachable vertices have `i64::MAX`.
    ///
    /// Uses a simple priority queue (binary heap via `BinaryHeap` from std).
    /// All edge weights must be non-negative for correct results.
    pub fn dijkstra(&self, start: usize) -> Vec<i64> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut dist = vec![i64::MAX; self.num_vertices];
        dist[start] = 0;

        // Min-heap of (distance, vertex).
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, start)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue; // stale entry
            }
            for &(v, w) in &self.adjacency_list[u] {
                let nd = d.saturating_add(w);
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        dist
    }

    // -- Dijkstra with path reconstruction ------------------------------------

    /// Dijkstra's single-source shortest paths from `start`, also tracking
    /// the predecessor of each vertex so that shortest paths can be
    /// reconstructed.
    ///
    /// Returns `(dist, parent)` where `parent[v]` is the vertex that leads
    /// to `v` on the shortest path from `start` (or `None` if `v` is the
    /// start or unreachable).
    pub fn dijkstra_with_path(&self, start: usize) -> (Vec<i64>, Vec<Option<usize>>) {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut dist = vec![i64::MAX; self.num_vertices];
        let mut parent: Vec<Option<usize>> = vec![None; self.num_vertices];
        dist[start] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0i64, start)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &self.adjacency_list[u] {
                let nd = d.saturating_add(w);
                if nd < dist[v] {
                    dist[v] = nd;
                    parent[v] = Some(u);
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        (dist, parent)
    }

    /// Reconstructs the shortest path from a `parent` array (as returned by
    /// [`dijkstra_with_path`]) to `end`. Returns `None` if `end` is
    /// unreachable.
    pub fn reconstruct_path(parent: &[Option<usize>], start: usize, end: usize) -> Option<Vec<usize>> {
        if start == end {
            return Some(vec![start]);
        }
        if parent[end].is_none() {
            return None; // unreachable
        }
        let mut path = vec![end];
        let mut current = end;
        while let Some(p) = parent[current] {
            path.push(p);
            current = p;
        }
        path.reverse();
        Some(path)
    }

    // -- Bellman-Ford -------------------------------------------------------

    /// Bellman-Ford single-source shortest paths from `start`.
    /// Returns `None` if a negative-weight cycle is reachable from `start`.
    pub fn bellman_ford(&self, start: usize) -> Option<Vec<i64>> {
        let n = self.num_vertices;
        let mut dist = vec![i64::MAX; n];
        dist[start] = 0;

        // Collect all edges.
        let edges: Vec<(usize, usize, i64)> = (0..n)
            .flat_map(|u| self.adjacency_list[u].iter().map(move |&(v, w)| (u, v, w)))
            .collect();

        // Relax V-1 times.
        for _ in 0..n.saturating_sub(1) {
            for &(u, v, w) in &edges {
                if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                }
            }
        }

        // Check for negative cycles.
        for &(u, v, w) in &edges {
            if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                return None;
            }
        }

        Some(dist)
    }

    // -- Floyd-Warshall -----------------------------------------------------

    /// All-pairs shortest paths. Returns an `n x n` matrix where
    /// `result[i][j]` is the shortest distance from `i` to `j`
    /// (`i64::MAX` if unreachable).
    pub fn floyd_warshall(&self) -> Vec<Vec<i64>> {
        let n = self.num_vertices;
        let mut dist = vec![vec![i64::MAX; n]; n];

        for (i, row) in dist.iter_mut().enumerate() {
            row[i] = 0;
        }
        for (u, edges) in self.adjacency_list.iter().enumerate() {
            for &(v, w) in edges {
                if w < dist[u][v] {
                    dist[u][v] = w;
                }
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                        let through_k = dist[i][k] + dist[k][j];
                        if through_k < dist[i][j] {
                            dist[i][j] = through_k;
                        }
                    }
                }
            }
        }
        dist
    }

    // -- Floyd-Warshall with path reconstruction -----------------------------

    /// All-pairs shortest paths with path reconstruction.
    /// Returns `(dist, next)` where `next[i][j]` is the next vertex on the
    /// shortest path from `i` to `j`.
    pub fn floyd_warshall_with_path(
        &self,
    ) -> (Vec<Vec<i64>>, Vec<Vec<Option<usize>>>) {
        let n = self.num_vertices;
        let mut dist = vec![vec![i64::MAX; n]; n];
        let mut next = vec![vec![None; n]; n];

        for i in 0..n {
            dist[i][i] = 0;
        }
        for u in 0..n {
            for &(v, w) in &self.adjacency_list[u] {
                if w < dist[u][v] {
                    dist[u][v] = w;
                    next[u][v] = Some(v);
                }
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                        let through_k = dist[i][k] + dist[k][j];
                        if through_k < dist[i][j] {
                            dist[i][j] = through_k;
                            next[i][j] = next[i][k];
                        }
                    }
                }
            }
        }
        (dist, next)
    }

    /// Reconstructs the shortest path from `i` to `j` using the `next` matrix
    /// returned by [`floyd_warshall_with_path`].
    /// Returns an empty vector if no path exists.
    pub fn get_floyd_path(
        next: &[Vec<Option<usize>>],
        i: usize,
        j: usize,
    ) -> Vec<usize> {
        if next[i][j].is_none() {
            return vec![];
        }
        let mut path = vec![i];
        let mut current = i;
        while current != j {
            current = next[current][j].unwrap();
            path.push(current);
        }
        path
    }

    /// Checks whether the graph contains a negative-weight cycle.
    /// Runs Floyd-Warshall and checks the diagonal for negative values.
    pub fn has_negative_cycle(&self) -> bool {
        let dist = self.floyd_warshall();
        dist.iter().enumerate().any(|(i, row)| row[i] < 0)
    }

    /// Computes the transitive closure of the graph.
    /// `result[i][j]` is `true` if there is a path from `i` to `j`.
    pub fn transitive_closure(&self) -> Vec<Vec<bool>> {
        let n = self.num_vertices;
        let mut reach = vec![vec![false; n]; n];

        for i in 0..n {
            reach[i][i] = true;
        }
        for u in 0..n {
            for &(v, _) in &self.adjacency_list[u] {
                reach[u][v] = true;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    reach[i][j] = reach[i][j] || (reach[i][k] && reach[k][j]);
                }
            }
        }
        reach
    }

    // -- Prim's MST ---------------------------------------------------------

    /// Prim's minimum spanning tree (for undirected graphs).
    /// Returns the MST as a list of `(from, to, weight)` edges.
    /// Starts from vertex 0.
    pub fn prim_mst(&self) -> Vec<(usize, usize, i64)> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = self.num_vertices;
        if n == 0 {
            return Vec::new();
        }

        let mut in_mst = vec![false; n];
        let mut mst_edges = Vec::new();
        // Min-heap of (weight, from, to).
        let mut heap = BinaryHeap::new();

        in_mst[0] = true;
        for &(v, w) in &self.adjacency_list[0] {
            heap.push(Reverse((w, 0, v)));
        }

        while let Some(Reverse((w, from, to))) = heap.pop() {
            if in_mst[to] {
                continue;
            }
            in_mst[to] = true;
            mst_edges.push((from, to, w));
            for &(next, nw) in &self.adjacency_list[to] {
                if !in_mst[next] {
                    heap.push(Reverse((nw, to, next)));
                }
            }
        }
        mst_edges
    }

    // -- Kruskal's MST ------------------------------------------------------

    /// Kruskal's minimum spanning tree (for undirected graphs).
    /// Returns the MST as a list of `(from, to, weight)` edges.
    pub fn kruskal_mst(&self) -> Vec<(usize, usize, i64)> {
        let n = self.num_vertices;
        // Collect all edges (avoid duplicates for undirected: keep u < v).
        let mut edges: Vec<(i64, usize, usize)> = Vec::new();
        for u in 0..n {
            for &(v, w) in &self.adjacency_list[u] {
                if self.directed || u < v {
                    edges.push((w, u, v));
                }
            }
        }
        edges.sort();

        let mut uf = UnionFind::new(n);
        let mut mst = Vec::new();

        for (w, u, v) in edges {
            if uf.union(u, v) {
                mst.push((u, v, w));
                if mst.len() == n - 1 {
                    break;
                }
            }
        }
        mst
    }

    // -- Topological Sort ---------------------------------------------------

    /// Kahn's algorithm (BFS-based topological sort) for directed graphs.
    /// Returns `None` if the graph contains a cycle.
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.num_vertices;
        let mut in_degree = vec![0usize; n];

        for u in 0..n {
            for &(v, _) in &self.adjacency_list[u] {
                in_degree[v] += 1;
            }
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&v| in_degree[v] == 0).collect();
        let mut order = Vec::with_capacity(n);

        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &(v, _) in &self.adjacency_list[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if order.len() == n {
            Some(order)
        } else {
            None // cycle detected
        }
    }

    // -- DFS-based Topological Sort -----------------------------------------

    /// DFS-based topological sort using reverse post-order.
    /// Returns `None` if the graph contains a cycle (detected via 3-color DFS).
    pub fn topological_sort_dfs(&self) -> Option<Vec<usize>> {
        let n = self.num_vertices;
        // 0 = white (unvisited), 1 = gray (in current path), 2 = black (done)
        let mut color = vec![0u8; n];
        let mut stack = Vec::with_capacity(n);

        for v in 0..n {
            if color[v] == 0 {
                if !self.topo_dfs_visit(v, &mut color, &mut stack) {
                    return None; // cycle detected
                }
            }
        }

        stack.reverse();
        Some(stack)
    }

    fn topo_dfs_visit(&self, u: usize, color: &mut [u8], stack: &mut Vec<usize>) -> bool {
        color[u] = 1; // gray
        for &(v, _) in &self.adjacency_list[u] {
            if color[v] == 1 {
                return false; // back edge → cycle
            }
            if color[v] == 0 && !self.topo_dfs_visit(v, color, stack) {
                return false;
            }
        }
        color[u] = 2; // black
        stack.push(u); // post-order
        true
    }

    // -- Kahn's with level tracking -----------------------------------------

    /// Kahn's algorithm that returns vertices grouped by "wave" / level.
    /// All vertices in the same level have in-degree 0 simultaneously and
    /// can be processed in parallel.
    /// Returns `None` if the graph contains a cycle.
    pub fn topo_levels(&self) -> Option<Vec<Vec<usize>>> {
        let n = self.num_vertices;
        let mut in_degree = vec![0usize; n];

        for u in 0..n {
            for &(v, _) in &self.adjacency_list[u] {
                in_degree[v] += 1;
            }
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&v| in_degree[v] == 0).collect();
        let mut levels = Vec::new();
        let mut count = 0;

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level = Vec::with_capacity(level_size);
            for _ in 0..level_size {
                let u = queue.pop_front().unwrap();
                level.push(u);
                count += 1;
                for &(v, _) in &self.adjacency_list[u] {
                    in_degree[v] -= 1;
                    if in_degree[v] == 0 {
                        queue.push_back(v);
                    }
                }
            }
            levels.push(level);
        }

        if count == n {
            Some(levels)
        } else {
            None // cycle detected
        }
    }

    // -- Cycle Detection ----------------------------------------------------

    /// Returns `true` if the graph contains a cycle.
    ///
    /// For directed graphs: uses DFS coloring (white/gray/black).
    /// For undirected graphs: uses DFS with parent tracking.
    pub fn has_cycle(&self) -> bool {
        if self.directed {
            self.has_cycle_directed()
        } else {
            self.has_cycle_undirected()
        }
    }

    fn has_cycle_directed(&self) -> bool {
        // 0 = white (unvisited), 1 = gray (in current path), 2 = black (done)
        let mut color = vec![0u8; self.num_vertices];

        for start in 0..self.num_vertices {
            if color[start] == 0 && self.dfs_cycle_directed(start, &mut color) {
                return true;
            }
        }
        false
    }

    fn dfs_cycle_directed(&self, u: usize, color: &mut [u8]) -> bool {
        color[u] = 1;
        for &(v, _) in &self.adjacency_list[u] {
            if color[v] == 1 {
                return true; // back edge -> cycle
            }
            if color[v] == 0 && self.dfs_cycle_directed(v, color) {
                return true;
            }
        }
        color[u] = 2;
        false
    }

    fn has_cycle_undirected(&self) -> bool {
        let mut visited = vec![false; self.num_vertices];

        for start in 0..self.num_vertices {
            if !visited[start]
                && self.dfs_cycle_undirected(start, usize::MAX, &mut visited)
            {
                return true;
            }
        }
        false
    }

    fn dfs_cycle_undirected(
        &self,
        u: usize,
        parent: usize,
        visited: &mut [bool],
    ) -> bool {
        visited[u] = true;
        for &(v, _) in &self.adjacency_list[u] {
            if !visited[v] {
                if self.dfs_cycle_undirected(v, u, visited) {
                    return true;
                }
            } else if v != parent {
                return true;
            }
        }
        false
    }

    // -- 0-1 BFS ------------------------------------------------------------

    /// 0-1 BFS from `src`. Edges with weight 0 are pushed to the front of the
    /// deque, edges with weight 1 to the back. Returns shortest distances
    /// (`i64::MAX` if unreachable). Only correct when all weights are 0 or 1.
    ///
    /// **Time:** O(V + E).
    pub fn bfs_01(&self, src: usize) -> Vec<i64> {
        let n = self.num_vertices;
        let mut dist = vec![i64::MAX; n];
        dist[src] = 0;
        let mut deque = VecDeque::new();
        deque.push_back(src);

        while let Some(u) = deque.pop_front() {
            for &(v, w) in &self.adjacency_list[u] {
                let nd = dist[u] + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    if w == 0 {
                        deque.push_front(v);
                    } else {
                        deque.push_back(v);
                    }
                }
            }
        }
        dist
    }

    // -- Multi-source BFS ---------------------------------------------------

    /// BFS from multiple sources simultaneously. All sources start at distance 0.
    /// Edge weights are ignored (treated as 1). Returns distances; unreachable
    /// vertices have -1.
    ///
    /// **Time:** O(V + E).
    pub fn multi_source_bfs(&self, sources: &[usize]) -> Vec<i64> {
        let n = self.num_vertices;
        let mut dist = vec![-1i64; n];
        let mut queue = VecDeque::new();

        for &s in sources {
            dist[s] = 0;
            queue.push_back(s);
        }

        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.adjacency_list[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        dist
    }

    // -- Tarjan's SCC -------------------------------------------------------

    /// Tarjan's algorithm for finding Strongly Connected Components.
    /// Returns a list of SCCs (each SCC is a `Vec<usize>` of vertex indices).
    /// Only meaningful for directed graphs.
    ///
    /// **Time:** O(V + E).
    pub fn tarjan_scc(&self) -> Vec<Vec<usize>> {
        let n = self.num_vertices;
        let mut index_counter = 0usize;
        let mut indices = vec![usize::MAX; n];
        let mut lowlinks = vec![0usize; n];
        let mut on_stack = vec![false; n];
        let mut stack = Vec::new();
        let mut sccs = Vec::new();

        for v in 0..n {
            if indices[v] == usize::MAX {
                self.tarjan_dfs(
                    v,
                    &mut index_counter,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut stack,
                    &mut sccs,
                );
            }
        }
        sccs
    }

    fn tarjan_dfs(
        &self,
        u: usize,
        index_counter: &mut usize,
        indices: &mut [usize],
        lowlinks: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<usize>,
        sccs: &mut Vec<Vec<usize>>,
    ) {
        indices[u] = *index_counter;
        lowlinks[u] = *index_counter;
        *index_counter += 1;
        stack.push(u);
        on_stack[u] = true;

        for &(v, _) in &self.adjacency_list[u] {
            if indices[v] == usize::MAX {
                self.tarjan_dfs(v, index_counter, indices, lowlinks, on_stack, stack, sccs);
                lowlinks[u] = lowlinks[u].min(lowlinks[v]);
            } else if on_stack[v] {
                lowlinks[u] = lowlinks[u].min(indices[v]);
            }
        }

        if lowlinks[u] == indices[u] {
            let mut scc = Vec::new();
            while let Some(w) = stack.pop() {
                on_stack[w] = false;
                scc.push(w);
                if w == u {
                    break;
                }
            }
            sccs.push(scc);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Graph basics -------------------------------------------------------

    #[test]
    fn test_new_graph() {
        let g = Graph::new(5, false);
        assert_eq!(g.num_vertices(), 5);
        assert!(g.neighbors(0).is_empty());
    }

    #[test]
    fn test_add_edges_undirected() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 10);
        g.add_unweighted_edge(2, 3);
        // Undirected: both directions
        assert_eq!(g.neighbors(0), &[(1, 10)]);
        assert_eq!(g.neighbors(1), &[(0, 10)]);
        assert_eq!(g.neighbors(2), &[(3, 1)]);
        assert_eq!(g.neighbors(3), &[(2, 1)]);
    }

    #[test]
    fn test_add_edges_directed() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 5);
        assert_eq!(g.neighbors(0), &[(1, 5)]);
        assert!(g.neighbors(1).is_empty());
    }

    #[test]
    fn test_bfs_order() {
        //  0 -- 1 -- 3
        //  |    |
        //  2    4
        let mut g = Graph::new(5, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 3);
        g.add_unweighted_edge(1, 4);
        let order = g.bfs(0);
        assert_eq!(order[0], 0);
        assert_eq!(order.len(), 5);
        // Level 1 before level 2.
        let pos = |v: usize| order.iter().position(|&x| x == v).unwrap();
        assert!(pos(1) < pos(3));
        assert!(pos(1) < pos(4));
        assert!(pos(2) < pos(3));
    }

    #[test]
    fn test_dfs_order() {
        let mut g = Graph::new(5, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 3);
        g.add_unweighted_edge(1, 4);
        let order = g.dfs(0);
        assert_eq!(order[0], 0);
        assert_eq!(order.len(), 5);
    }

    // -- Shortest path algorithms -------------------------------------------

    #[test]
    fn test_dijkstra() {
        //  0 --(4)--> 1 --(1)--> 3
        //  |                      ^
        //  (2)                   (3)
        //  v                      |
        //  2 ---------(3)------> 3  (already there, edge from 2)
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 4);
        g.add_edge(0, 2, 2);
        g.add_edge(1, 3, 1);
        g.add_edge(2, 3, 3);
        let dist = g.dijkstra(0);
        assert_eq!(dist, vec![0, 4, 2, 5]);
    }

    #[test]
    fn test_bellman_ford_no_negative_cycle() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -3);
        g.add_edge(0, 2, 4);
        g.add_edge(2, 3, 2);
        let dist = g.bellman_ford(0).expect("no negative cycle");
        assert_eq!(dist, vec![0, 1, -2, 0]);
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 0, -1); // cycle sum = -1
        assert!(g.bellman_ford(0).is_none());
    }

    #[test]
    fn test_floyd_warshall() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 3);
        g.add_edge(0, 2, 6);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 1);
        g.add_edge(1, 3, 8);
        let dist = g.floyd_warshall();
        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][1], 3);
        assert_eq!(dist[0][2], 5); // 0->1->2
        assert_eq!(dist[0][3], 6); // 0->1->2->3
        assert_eq!(dist[1][3], 3); // 1->2->3
    }

    #[test]
    fn test_dijkstra_with_path() {
        // Same graph as basic dijkstra test
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 4);
        g.add_edge(0, 2, 2);
        g.add_edge(1, 3, 1);
        g.add_edge(2, 3, 3);
        let (dist, parent) = g.dijkstra_with_path(0);
        assert_eq!(dist, vec![0, 4, 2, 5]);
        assert_eq!(parent[0], None); // start
        assert_eq!(parent[1], Some(0));
        assert_eq!(parent[2], Some(0));
        assert_eq!(parent[3], Some(2)); // 0→2→3 is shorter than 0→1→3
        let path = Graph::reconstruct_path(&parent, 0, 3).unwrap();
        assert_eq!(path, vec![0, 2, 3]);
    }

    #[test]
    fn test_dijkstra_with_path_6_nodes() {
        // A(0)--1--B(1)--6--D(4)
        // |        |        |
        // 4        2        1
        // |        |        |
        // C(2)--3--E(3)--2--F(5)
        let mut g = Graph::new(6, false);
        g.add_edge(0, 1, 1); // A-B
        g.add_edge(0, 2, 4); // A-C
        g.add_edge(1, 4, 6); // B-D
        g.add_edge(1, 3, 2); // B-E
        g.add_edge(2, 3, 3); // C-E
        g.add_edge(3, 5, 2); // E-F
        g.add_edge(4, 5, 1); // D-F
        let (dist, parent) = g.dijkstra_with_path(0);
        assert_eq!(dist[0], 0); // A
        assert_eq!(dist[1], 1); // B
        assert_eq!(dist[2], 4); // C
        assert_eq!(dist[3], 3); // E
        assert_eq!(dist[5], 5); // F
        assert_eq!(dist[4], 6); // D via A→B→E→F→D, not A→B→D=7
        // Path to D should be A→B→E→F→D
        let path = Graph::reconstruct_path(&parent, 0, 4).unwrap();
        assert_eq!(path, vec![0, 1, 3, 5, 4]);
    }

    #[test]
    fn test_dijkstra_with_path_unreachable() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        let (dist, parent) = g.dijkstra_with_path(0);
        assert_eq!(dist[2], i64::MAX);
        assert!(Graph::reconstruct_path(&parent, 0, 2).is_none());
    }

    #[test]
    fn test_reconstruct_path_start_equals_end() {
        let parent = vec![None; 3];
        let path = Graph::reconstruct_path(&parent, 0, 0).unwrap();
        assert_eq!(path, vec![0]);
    }

    #[test]
    fn test_floyd_warshall_with_path() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 3);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 1);
        let (dist, next) = g.floyd_warshall_with_path();
        assert_eq!(dist[0][3], 6);
        let path = Graph::get_floyd_path(&next, 0, 3);
        assert_eq!(path, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_floyd_warshall_with_path_no_path() {
        let g = Graph::new(3, true);
        let (_, next) = g.floyd_warshall_with_path();
        let path = Graph::get_floyd_path(&next, 0, 2);
        assert!(path.is_empty());
    }

    #[test]
    fn test_has_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -2);
        g.add_edge(2, 0, -1);
        assert!(g.has_negative_cycle());
    }

    #[test]
    fn test_no_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 2);
        assert!(!g.has_negative_cycle());
    }

    #[test]
    fn test_transitive_closure() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 3, 1);
        let reach = g.transitive_closure();
        assert!(reach[0][3]); // 0→1→2→3
        assert!(!reach[3][0]); // no path back
        assert!(reach[0][0]); // self
    }

    // -- MST algorithms -----------------------------------------------------

    #[test]
    fn test_prim_mst() {
        //  0 --(1)-- 1
        //  |         |
        // (3)       (2)
        //  |         |
        //  2 --(4)-- 3
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 3);
        g.add_edge(1, 3, 2);
        g.add_edge(2, 3, 4);
        let mst = g.prim_mst();
        let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
        assert_eq!(total, 6); // edges 0-1(1), 1-3(2), 0-2(3)
        assert_eq!(mst.len(), 3);
    }

    #[test]
    fn test_kruskal_mst() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 3);
        g.add_edge(1, 3, 2);
        g.add_edge(2, 3, 4);
        let mst = g.kruskal_mst();
        let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
        assert_eq!(total, 6);
        assert_eq!(mst.len(), 3);
    }

    #[test]
    fn test_kruskal_mst_larger() {
        //    0
        //   / \
        //  1   4
        //  |   |
        //  2   5
        //   \ /
        //    3
        let mut g = Graph::new(6, false);
        g.add_edge(0, 1, 2);
        g.add_edge(0, 4, 7);
        g.add_edge(1, 2, 3);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 5, 4);
        g.add_edge(4, 5, 5);
        g.add_edge(1, 4, 6);
        let mst = g.kruskal_mst();
        let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
        // MST: 2-3(1), 0-1(2), 1-2(3), 3-5(4), 4-5(5) = 15
        assert_eq!(total, 15);
        assert_eq!(mst.len(), 5);
    }

    // -- Topological sort & cycles ------------------------------------------

    #[test]
    fn test_topological_sort_dag() {
        // 0 -> 1 -> 3
        // 0 -> 2 -> 3
        let mut g = Graph::new(4, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 3);
        g.add_unweighted_edge(2, 3);
        let order = g.topological_sort().expect("DAG");
        let pos = |v: usize| order.iter().position(|&x| x == v).unwrap();
        assert!(pos(0) < pos(1));
        assert!(pos(0) < pos(2));
        assert!(pos(1) < pos(3));
        assert!(pos(2) < pos(3));
    }

    #[test]
    fn test_topological_sort_cycle() {
        let mut g = Graph::new(3, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        g.add_unweighted_edge(2, 0);
        assert!(g.topological_sort().is_none());
    }

    #[test]
    fn test_topological_sort_dfs_dag() {
        let mut g = Graph::new(5, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 3);
        g.add_unweighted_edge(2, 3);
        g.add_unweighted_edge(1, 4);
        let order = g.topological_sort_dfs().expect("DAG");
        let pos = |v: usize| order.iter().position(|&x| x == v).unwrap();
        assert!(pos(0) < pos(1));
        assert!(pos(0) < pos(2));
        assert!(pos(1) < pos(3));
        assert!(pos(2) < pos(3));
        assert!(pos(1) < pos(4));
    }

    #[test]
    fn test_topological_sort_dfs_cycle() {
        let mut g = Graph::new(3, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        g.add_unweighted_edge(2, 0);
        assert!(g.topological_sort_dfs().is_none());
    }

    #[test]
    fn test_topological_sort_dfs_single() {
        let g = Graph::new(1, true);
        assert_eq!(g.topological_sort_dfs(), Some(vec![0]));
    }

    #[test]
    fn test_topological_sort_dfs_disconnected() {
        let mut g = Graph::new(4, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(2, 3);
        let order = g.topological_sort_dfs().expect("DAG");
        let pos = |v: usize| order.iter().position(|&x| x == v).unwrap();
        assert!(pos(0) < pos(1));
        assert!(pos(2) < pos(3));
    }

    #[test]
    fn test_topo_levels_basic() {
        // 0→2, 1→2, 2→3, 2→4, 3→5, 4→5
        let mut g = Graph::new(6, true);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 2);
        g.add_unweighted_edge(2, 3);
        g.add_unweighted_edge(2, 4);
        g.add_unweighted_edge(3, 5);
        g.add_unweighted_edge(4, 5);
        let levels = g.topo_levels().expect("DAG");
        assert_eq!(levels.len(), 4);
        // Level 0: {0, 1} in some order
        assert_eq!(levels[0].len(), 2);
        assert!(levels[0].contains(&0));
        assert!(levels[0].contains(&1));
        // Level 1: {2}
        assert_eq!(levels[1], vec![2]);
        // Level 2: {3, 4} in some order
        assert_eq!(levels[2].len(), 2);
        assert!(levels[2].contains(&3));
        assert!(levels[2].contains(&4));
        // Level 3: {5}
        assert_eq!(levels[3], vec![5]);
    }

    #[test]
    fn test_topo_levels_cycle() {
        let mut g = Graph::new(3, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        g.add_unweighted_edge(2, 0);
        assert!(g.topo_levels().is_none());
    }

    #[test]
    fn test_has_cycle_directed() {
        let mut g = Graph::new(3, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        assert!(!g.has_cycle());
        g.add_unweighted_edge(2, 0);
        assert!(g.has_cycle());
    }

    #[test]
    fn test_has_cycle_undirected() {
        let mut g = Graph::new(4, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        assert!(!g.has_cycle());
        g.add_unweighted_edge(2, 0);
        assert!(g.has_cycle());
    }

    // -- Union-Find ---------------------------------------------------------

    #[test]
    fn test_uf_basic() {
        let mut uf = UnionFind::new(5);
        assert_ne!(uf.find(0), uf.find(1));
        assert!(uf.union(0, 1));
        assert_eq!(uf.find(0), uf.find(1));
    }

    #[test]
    fn test_uf_already_same_set() {
        let mut uf = UnionFind::new(3);
        uf.union(0, 1);
        assert!(!uf.union(0, 1)); // already same set
    }

    #[test]
    fn test_uf_path_compression() {
        let mut uf = UnionFind::new(6);
        // Build chain: 0-1-2-3-4-5
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);
        uf.union(4, 5);
        // After find with path compression, all should point to same root.
        let root = uf.find(5);
        for i in 0..6 {
            assert_eq!(uf.find(i), root);
        }
    }

    #[test]
    fn test_uf_multiple_components() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 5);
        // Three components.
        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(2), uf.find(3));
        assert_eq!(uf.find(4), uf.find(5));
        assert_ne!(uf.find(0), uf.find(2));
        assert_ne!(uf.find(0), uf.find(4));
        assert_ne!(uf.find(2), uf.find(4));

        // Merge two components.
        uf.union(1, 3);
        assert_eq!(uf.find(0), uf.find(2));
        assert_ne!(uf.find(0), uf.find(4));
    }

    // -- BFS shortest path & BFS by level -----------------------------------

    #[test]
    fn test_bfs_shortest_path() {
        //  0 -- 1 -- 3 -- 5
        //  |         |
        //  2         4
        let mut g = Graph::new(6, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 3);
        g.add_unweighted_edge(3, 4);
        g.add_unweighted_edge(3, 5);
        let path = g.bfs_shortest_path(0, 5).unwrap();
        assert_eq!(path, vec![0, 1, 3, 5]);
    }

    #[test]
    fn test_bfs_shortest_path_same_vertex() {
        let g = Graph::new(3, false);
        let path = g.bfs_shortest_path(0, 0).unwrap();
        assert_eq!(path, vec![0]);
    }

    #[test]
    fn test_bfs_shortest_path_no_path() {
        let mut g = Graph::new(4, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(2, 3);
        assert!(g.bfs_shortest_path(0, 3).is_none());
    }

    #[test]
    fn test_bfs_by_level() {
        //       0
        //      / \
        //     1   2
        //    / \   \
        //   3   4   5
        let mut g = Graph::new(6, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(0, 2);
        g.add_unweighted_edge(1, 3);
        g.add_unweighted_edge(1, 4);
        g.add_unweighted_edge(2, 5);
        let levels = g.bfs_by_level(0);
        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0], vec![0]);
        assert_eq!(levels[1], vec![1, 2]);
        assert_eq!(levels[2], vec![3, 4, 5]);
    }

    #[test]
    fn test_bfs_by_level_single() {
        let g = Graph::new(1, false);
        let levels = g.bfs_by_level(0);
        assert_eq!(levels, vec![vec![0]]);
    }

    // -- 0-1 BFS, Multi-source BFS, Tarjan SCC -----------------------------

    #[test]
    fn bfs_01_basic() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 0);
        g.add_edge(2, 3, 1);
        g.add_edge(1, 3, 0);
        let dist = g.bfs_01(0);
        assert_eq!(dist, vec![0, 1, 0, 1]);
    }

    #[test]
    fn multi_source_bfs_basic() {
        let mut g = Graph::new(5, false);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        g.add_unweighted_edge(3, 4);
        let dist = g.multi_source_bfs(&[0, 3]);
        assert_eq!(dist, vec![0, 1, 2, 0, 1]);
    }

    #[test]
    fn tarjan_scc_basic() {
        let mut g = Graph::new(5, true);
        g.add_unweighted_edge(0, 1);
        g.add_unweighted_edge(1, 2);
        g.add_unweighted_edge(2, 0);
        g.add_unweighted_edge(2, 3);
        g.add_unweighted_edge(3, 4);
        g.add_unweighted_edge(4, 3);
        let sccs = g.tarjan_scc();
        assert_eq!(sccs.len(), 2);
        let mut sorted: Vec<Vec<usize>> = sccs.into_iter().map(|mut s| { s.sort(); s }).collect();
        sorted.sort();
        assert_eq!(sorted, vec![vec![0, 1, 2], vec![3, 4]]);
    }
}
