# Tìm kiếm theo chiều rộng (Breadth-First Search - BFS)

> 💡 **Đừng lo lắng:** BFS nghe có vẻ abstract nhưng bạn đã **dùng BFS rồi** -- level-order traversal trong chương Binary Tree chính là BFS trên tree! Giờ chỉ mở rộng sang graph (có cycle → cần `visited`). Code BFS ngắn hơn hầu hết data structures trong series -- chỉ ~15 dòng. Và nó dùng Queue mà bạn đã học từ chương 2. BFS + DFS giải được **80% bài graph** trong phỏng vấn. Hiểu 2 thuật toán này = foundation cho Dijkstra, topological sort, connected components.

## Đây là gì?

Hãy tưởng tượng bạn **thả một viên đá xuống mặt hồ**. Sóng lan ra theo từng vòng tròn -- vòng 1 gần nhất, vòng 2 xa hơn, vòng 3 xa hơn nữa... Mỗi vòng sóng lan đều ra mọi hướng trước khi vòng tiếp theo bắt đầu.

Đó chính là cách **BFS** (Breadth-First Search -- tìm kiếm theo chiều rộng) hoạt động. Nó khám phá graph **theo từng lớp**, từ gần đến xa.

Tại sao cần BFS? Vì BFS tự nhiên tìm được **đường đi ngắn nhất** (ít cạnh nhất) trong graph không trọng số. Vòng sóng đầu tiên chạm tới đâu = con đường ngắn nhất từ điểm xuất phát đến đó.

BFS dùng **queue** (hàng đợi) -- ai đến trước phục vụ trước, giống xếp hàng mua vé.

---

## Queue → BFS, Level-Order → BFS

Nhớ chương 2 (Queue) và chương 3 (Binary Tree)?

```
Level-order traversal (Binary Tree):    BFS (Graph):
  Dùng Queue                            Dùng Queue
  Thăm theo tầng                        Thăm theo "vòng sóng"
  Không cần visited (tree no cycle)     Cần visited (graph có cycle)
  Code gần giống nhau!
```

So sánh code:

```
Binary Tree level-order:               Graph BFS:
  queue.push(root)                       queue.push(start)
  while queue not empty:                 visited[start] = true  ← THÊM
    node = queue.pop()                   while queue not empty:
    process(node)                          v = queue.pop()
    if node.left:                          process(v)
      queue.push(node.left)                for neighbor in v.neighbors:
    if node.right:                           if !visited[neighbor]:  ← THÊM
      queue.push(node.right)                   visited[neighbor] = true
                                               queue.push(neighbor)
```

BFS = Level-Order Traversal + visited array. Đó là **toàn bộ** sự khác biệt. Bạn đã biết 90% BFS từ chương Binary Tree.

---

## Hoạt động như thế nào?

### Trace cơ bản -- graph dạng cây

Cho graph sau, bắt đầu từ đỉnh 0:

```
       0          ← Vòng 0: bắt đầu (thả đá)
      / \
     1   2        ← Vòng 1: vòng sóng đầu tiên
    / \   \
   3   4   5      ← Vòng 2: vòng sóng thứ hai
```

**Quá trình thực hiện (theo dõi queue):**

```
Bước 0: Thăm 0       Queue: [1, 2]           Thứ tự: [0]
  ↳ 0 kể cho 1 và 2

Bước 1: Thăm 1       Queue: [2, 3, 4]        Thứ tự: [0, 1]
  ↳ 1 kể cho 3 và 4

Bước 2: Thăm 2       Queue: [3, 4, 5]        Thứ tự: [0, 1, 2]
  ↳ 2 kể cho 5

Bước 3: Thăm 3       Queue: [4, 5]           Thứ tự: [0, 1, 2, 3]
  ↳ 3 không có ai mới để kể

Bước 4: Thăm 4       Queue: [5]              Thứ tự: [0, 1, 2, 3, 4]

Bước 5: Thăm 5       Queue: []               Thứ tự: [0, 1, 2, 3, 4, 5]
```

Chú ý: Tất cả đỉnh ở vòng 1 (đỉnh 1, 2) được thăm **trước** mọi đỉnh ở vòng 2 (đỉnh 3, 4, 5). Đó là bản chất của BFS -- **hết lớp này rồi mới sang lớp tiếp**.

### Thuật toán

1. Đánh dấu đỉnh bắt đầu là "đã thăm", đưa vào queue
2. Lặp cho đến khi queue rỗng:
   a. Lấy đỉnh `v` ra khỏi queue (đầu hàng đợi)
   b. Với mỗi hàng xóm `u` chưa thăm của `v`:
      - Đánh dấu `u` là "đã thăm"
      - Đưa `u` vào queue (cuối hàng đợi)

```
Queue hoạt động giống xếp hàng mua vé:
  ┌───┬───┬───┬───┐
  │ 1 │ 2 │ 3 │ 4 │  → Vào cuối, ra đầu (FIFO)
  └───┴───┴───┴───┘
  ↑ ra                ↑ vào
```

### Tại sao BFS tìm đường ngắn nhất?

BFS thăm đỉnh **theo thứ tự khoảng cách** từ gần đến xa. Lần đầu tiên BFS gặp một đỉnh, đó chắc chắn là con đường ngắn nhất (tính theo số cạnh). Giống vòng sóng mặt hồ -- vòng sóng đầu tiên chạm bờ = khoảng cách ngắn nhất. DFS không có tính chất này.

---

## Graph có cycle -- tại sao cần visited?

Graph trên giống tree (không có cycle). Nhưng graph thực tế thường có cycle. Xem chuyện gì xảy ra:

```
Graph có cycle:
  0 --- 1
  |   / |
  |  /  |
  2 --- 3
```

**BFS từ 0 (CÓ visited):**

```
Bước 0: Visit 0, enqueue neighbors [1, 2]
  Queue: [1, 2]     Visited: {0, 1, 2}

Bước 1: Dequeue 1, neighbors = [0, 2, 3]
  0 đã visited → skip
  2 đã visited → skip
  3 chưa → enqueue
  Queue: [2, 3]     Visited: {0, 1, 2, 3}

Bước 2: Dequeue 2, neighbors = [0, 1, 3]
  0 đã visited → skip
  1 đã visited → skip
  3 đã visited → skip
  Queue: [3]

Bước 3: Dequeue 3, neighbors = [1, 2]
  All visited → skip
  Queue: []          ← Done!

Order: [0, 1, 2, 3]
```

**Nếu KHÔNG có visited:**

```
0 → enqueue 1, 2
1 → enqueue 0, 2, 3   ← 0 lại vào queue!
2 → enqueue 0, 1, 3   ← 0 lại vào queue LẦN NỮA!
0 → enqueue 1, 2      ← lặp lại...
→ Vòng lặp vô tận! Queue không bao giờ rỗng.
```

**Bài học:** Graph có cycle → **bắt buộc** phải có `visited`. Đây là khác biệt lớn nhất giữa BFS trên tree và BFS trên graph.

---

## Shortest Path Reconstruction

BFS tìm shortest path, nhưng làm sao **lấy ra** đường đi cụ thể? Dùng mảng `parent` -- ghi nhớ "ai dẫn đến tôi":

```rust
pub fn bfs_shortest_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
    let mut visited = vec![false; self.num_vertices];
    let mut parent: Vec<Option<usize>> = vec![None; self.num_vertices];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        if v == end {
            // Truy vết từ end về start
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
                parent[neighbor] = Some(v);  // ghi nhớ: v dẫn đến neighbor
                queue.push_back(neighbor);
            }
        }
    }
    None  // không có đường đi
}
```

**Trace:**

```
Graph:  0 -- 1 -- 3 -- 5
        |         |
        2         4

BFS từ 0 đến 5:
  Visit 0: parent[1]=0, parent[2]=0
  Visit 1: parent[3]=1
  Visit 2: (no new neighbors)
  Visit 3: parent[4]=3, parent[5]=3
  Visit 5 == end → truy vết:
    5 ← parent[5]=3 ← parent[3]=1 ← parent[1]=0
    Path: [0, 1, 3, 5] (length = 3 edges)
```

Tưởng tượng mỗi đỉnh ghi lại "ai giới thiệu tôi". Khi đến đích, ta đi ngược chuỗi giới thiệu để tìm đường về nguồn. Giống lần theo vòng sóng -- mỗi vòng sóng biết nó đến từ vòng nào.

---

## Level-by-Level BFS

Pattern cực kỳ quan trọng cho LeetCode -- BFS nhưng **biết đang ở tầng nào**:

```rust
pub fn bfs_by_level(&self, start: usize) -> Vec<Vec<usize>> {
    let mut visited = vec![false; self.num_vertices];
    let mut levels: Vec<Vec<usize>> = Vec::new();
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let level_size = queue.len();  // ← KEY: snapshot size trước khi process
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

// Output: [[0], [1, 2], [3, 4, 5]] -- rõ ràng tầng nào có node nào
```

Trick `level_size = queue.len()` trước inner loop cho phép biết chính xác đang ở tầng nào. Pattern này dùng trong: shortest distance, level-order tree traversal, minimum steps problems.

**Trace:**

```
       0
      / \
     1   2
    / \   \
   3   4   5

Queue ban đầu: [0]

Tầng 0: level_size = 1
  Process 0 → enqueue 1, 2
  Queue sau: [1, 2]
  levels = [[0]]

Tầng 1: level_size = 2
  Process 1 → enqueue 3, 4
  Process 2 → enqueue 5
  Queue sau: [3, 4, 5]
  levels = [[0], [1, 2]]

Tầng 2: level_size = 3
  Process 3, 4, 5 → no new
  Queue sau: []
  levels = [[0], [1, 2], [3, 4, 5]]
```

---

## Multi-Source BFS

Bài toán: cho grid, tìm khoảng cách ngắn nhất từ MỖI ô đến ô "lửa" gần nhất. Có nhiều ô lửa.

```
Naive: BFS từ từng ô lửa riêng → O(K × V) với K ô lửa
Multi-source: enqueue TẤT CẢ ô lửa vào queue ban đầu → 1 BFS duy nhất!
```

Tưởng tượng thả **nhiều viên đá** xuống hồ cùng lúc. Mỗi viên đá tạo vòng sóng riêng. Vòng sóng nào chạm đến 1 điểm trước = khoảng cách ngắn nhất đến viên đá gần nhất.

```rust
pub fn multi_source_bfs(&self, sources: &[usize]) -> Vec<i64> {
    let n = self.num_vertices;
    let mut dist = vec![-1i64; n];
    let mut queue = VecDeque::new();

    for &s in sources {
        dist[s] = 0;               // tất cả sources bắt đầu ở distance 0
        queue.push_back(s);         // enqueue TẤT CẢ cùng lúc
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
```

**Trace:**

```
Graph: 0 -- 1 -- 2 -- 3 -- 4
Sources: [0, 3]

Queue ban đầu: [0, 3]     dist = [0, -1, -1, 0, -1]

Process 0: neighbor 1 → dist[1]=1    Queue: [3, 1]
Process 3: neighbor 2 → dist[2]=1
           neighbor 4 → dist[4]=1    Queue: [1, 2, 4]
Process 1: neighbor 2 → skip (đã có) Queue: [2, 4]
Process 2: skip                       Queue: [4]
Process 4: skip                       Queue: []

dist = [0, 1, 1, 0, 1]
  → Mỗi ô biết khoảng cách đến source gần nhất!
```

Time: O(V + E) -- giống 1 BFS thường!

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
use std::collections::VecDeque;

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
```

Giải thích:

- `visited`: Mảng đánh dấu ai đã thăm rồi. Tránh thăm lại (đặc biệt quan trọng với graph có cycle!)
- Đánh dấu "đã thăm" **khi đưa vào queue**, không phải khi lấy ra. Tại sao? Để tránh thêm cùng một đỉnh vào queue nhiều lần
- `_` bỏ qua trọng số cạnh -- BFS coi mọi cạnh như nhau
- `VecDeque` cho phép thêm cuối / lấy đầu đều O(1)

---

## BFS trong thực tế

### a) Social Network -- "Người bạn có thể biết"

```
Facebook "People You May Know":
  BFS từ bạn, depth 2-3
  Bạn (depth 0) → Bạn bè (depth 1) → Bạn của bạn bè (depth 2)
  Recommend: người ở depth 2 mà bạn chưa kết bạn

  Bạn ──── An ──── Bình    ← Bình ở depth 2 = "có thể biết"
   │              /
   └──── Cúc ───┘           ← Cúc ở depth 1 = đã là bạn
```

### b) Shortest Path (unweighted)

```
Maze solving: tìm đường ngắn nhất từ start đến exit
  S . . # .
  # . # . .
  . . . . #
  . # # . E

BFS từ S, mỗi ô = 1 node, 4 hướng = edges
→ Vòng sóng đầu tiên chạm E = đường ngắn nhất!
```

Thêm: GPS (số trạm ít nhất), network hop count (bao nhiêu router để đến đích).

### c) Web Crawler

```
Crawl BFS: thăm tất cả page ở depth 1 trước, rồi depth 2...
  homepage → about, products, blog
           → about/team, products/item1, blog/post1, ...

Đảm bảo không miss page quan trọng gần root.
```

### d) Bipartite Check

```
Graph 2 màu (bipartite): mỗi cạnh nối 2 node khác màu.
BFS + tô màu xen kẽ. Nếu conflict → không bipartite.

  🔴 ── 🔵 ── 🔴 ── 🔵    ← OK, bipartite!
  🔴 ── 🔵 ── 🔴 ── 🔵
         └──────┘            ← 🔵 gặp 🔵 = conflict!

Use case: matching problems (job↔candidate, course↔timeslot)
```

---

## BFS vs DFS -- preview

| | BFS | DFS |
|---|-----|-----|
| Dùng gì? | Queue (FIFO) | Stack (LIFO) / Recursion |
| Thăm theo | Tầng (level by level) | Nhánh (đi sâu hết mới quay lại) |
| Tìm shortest path (unweighted)? | **Có** | Không |
| Memory | O(width of graph) | O(depth of graph) |
| Khi nào chọn? | Shortest path, level-order | Cycle detect, topological sort, backtracking |
| Ẩn dụ | Vòng sóng mặt hồ | Mê cung -- đi hết 1 ngõ rồi quay lại |

```
BFS (vòng sóng):              DFS (mê cung):
       0                            0
      / \                          /
     1   2    ← thăm hết tầng    1
    / \   \                      /
   3   4   5  ← rồi mới xuống  3 → 4 → 2 → 5  ← đi sâu 1 nhánh
```

DFS chi tiết ở chương sau!

---

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V + E) |
| Bộ nhớ | O(V) cho mảng visited + queue |
| Tìm đường ngắn nhất (không trọng số)? | Có |

Mỗi đỉnh vào queue đúng 1 lần, ra đúng 1 lần. Mỗi cạnh được xét đúng 1 lần (2 lần nếu vô hướng). Tổng cộng O(V + E).

**Ý nghĩa thực tế:** Với graph 1 triệu đỉnh và 5 triệu cạnh, BFS chạy khoảng 6 triệu thao tác. Rất nhanh!

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// Xây graph đơn giản
//   0 -- 1 -- 3
//   |    |
//   2    4
let mut g = Graph::new(5, false);
g.add_unweighted_edge(0, 1);
g.add_unweighted_edge(0, 2);
g.add_unweighted_edge(1, 3);
g.add_unweighted_edge(1, 4);

let order = g.bfs(0);
// 0 đầu tiên, rồi hàng xóm của 0 (1, 2), rồi lớp tiếp theo (3, 4)
assert_eq!(order[0], 0);
assert_eq!(order.len(), 5);

// BFS từ đỉnh khác
let order2 = g.bfs(3);
assert_eq!(order2[0], 3);  // luôn bắt đầu từ đỉnh xuất phát

// Shortest path reconstruction
//   0 -- 1 -- 3 -- 5
//   |         |
//   2         4
let mut g2 = Graph::new(6, false);
g2.add_unweighted_edge(0, 1);
g2.add_unweighted_edge(0, 2);
g2.add_unweighted_edge(1, 3);
g2.add_unweighted_edge(3, 4);
g2.add_unweighted_edge(3, 5);

let path = g2.bfs_shortest_path(0, 5);
assert_eq!(path, Some(vec![0, 1, 3, 5]));

// BFS by level
let levels = g2.bfs_by_level(0);
assert_eq!(levels[0], vec![0]);       // tầng 0
assert_eq!(levels[1], vec![1, 2]);    // tầng 1

// Multi-source BFS
let dist = g2.multi_source_bfs(&[0, 5]);
// dist[0]=0, dist[5]=0 (sources), các node khác = distance đến source gần nhất

// Graph không liên thông -- BFS chỉ thăm được phần kết nối
// Giống hai hòn đảo riêng biệt, không có cầu nối
let mut g3 = Graph::new(4, false);
g3.add_unweighted_edge(0, 1);  // đảo 1
g3.add_unweighted_edge(2, 3);  // đảo 2
let order3 = g3.bfs(0);
assert_eq!(order3, vec![0, 1]); // chỉ thăm được đảo 1
```

---

## Những cái bẫy hay gặp

### a) Đánh dấu visited khi dequeue thay vì enqueue

❌ **Sai:** Mark visited khi `pop` ra khỏi queue

```rust
// SAI!
while let Some(v) = queue.pop_front() {
    visited[v] = true;  // ← quá muộn!
    for neighbor in &adj[v] {
        if !visited[neighbor] {
            queue.push_back(neighbor);  // neighbor có thể bị push nhiều lần
        }
    }
}
```

✅ **Đúng:** Mark visited khi `push` vào queue

```rust
visited[start] = true;
queue.push_back(start);
while let Some(v) = queue.pop_front() {
    for neighbor in &adj[v] {
        if !visited[neighbor] {
            visited[neighbor] = true;   // ← mark ngay lúc push!
            queue.push_back(neighbor);
        }
    }
}
```

💡 **Tại sao?** Nếu mark khi dequeue, node A có thể được push bởi nhiều neighbors trước khi A được pop. Queue phình to, performance tệ, thậm chí sai kết quả shortest path.

### b) Quên handle graph không connected

❌ BFS từ 1 node rồi nghĩ đã thăm hết graph

✅ Loop qua mọi node, BFS từ mỗi unvisited node:

```rust
for v in 0..n {
    if !visited[v] {
        bfs(v);  // thăm component mới
    }
}
```

💡 BFS từ 1 node chỉ thăm connected component chứa node đó. Graph có thể có nhiều "hòn đảo" riêng biệt.

### c) Dùng BFS cho weighted graph shortest path

❌ `bfs(0)` trên graph có cạnh weight 1, 5, 10 rồi nghĩ tìm được shortest

✅ BFS chỉ tìm path **ít cạnh nhất**, KHÔNG phải path **tổng weight nhỏ nhất**. Weighted graph → dùng Dijkstra (chương sau).

💡 BFS coi mọi cạnh "bằng nhau". Nếu cạnh có weight khác nhau, BFS không biết phân biệt.

### d) Dùng DFS khi cần shortest path

❌ Dùng DFS tìm "minimum steps" rồi thắc mắc sao sai

✅ DFS có thể tìm **một** path nhưng KHÔNG đảm bảo **ngắn nhất**. BFS đảm bảo shortest (unweighted).

💡 Nếu đề hỏi "minimum steps / moves / jumps" → nghĩ BFS ngay. DFS đi sâu 1 nhánh, có thể lòng vòng trước khi tìm đích.

---

## Khi nào dùng BFS?

| Tình huống | BFS? | Tại sao? |
|------------|------|----------|
| Shortest path (unweighted) | ✅ | BFS tự nhiên tìm shortest |
| Level-order traversal | ✅ | BFS = level by level |
| "Minimum steps/moves" problems | ✅ | Mỗi step = 1 edge = BFS optimal |
| Bipartite check | ✅ | BFS + 2-coloring |
| Connected components | ✅ (hoặc DFS) | Cả hai đều OK |
| Multi-source distance | ✅ | Enqueue tất cả sources |
| Cycle detection (directed) | ❌ dùng DFS | DFS + back edge dễ hơn |
| Topological sort | ❌ dùng DFS (hoặc Kahn's BFS) | DFS standard hơn |
| Backtracking / permutation | ❌ dùng DFS | DFS + backtrack pattern |
| Weighted shortest path | ❌ dùng Dijkstra | BFS chỉ count edges |

---

## Luyện nhận diện Pattern

### a) Binary Tree Level Order Traversal (LeetCode #102)

Cho binary tree, trả list of lists -- mỗi list là nodes ở cùng level.

```
Input:     3
          / \
         9   20
            / \
           15  7

Output: [[3], [9, 20], [15, 7]]
```

**Gợi ý:** BFS with `level_size` trick. Bạn đã biết level-order từ chương Binary Tree -- giờ code nó với trick `level_size = queue.len()`!

### b) Rotting Oranges (LeetCode #994)

Grid có 3 trạng thái: 0 (trống), 1 (cam tươi), 2 (cam thối). Mỗi phút, cam thối làm cam tươi **kề** thối. Bao nhiêu phút tất cả cam thối?

```
Input:  [[2,1,1],     Output: 4
         [1,1,0],     (4 phút để tất cả cam thối)
         [0,1,1]]
```

**Gợi ý:** Multi-source BFS! Enqueue tất cả cam thối ban đầu (nhiều viên đá cùng thả), count levels = số phút.

### c) Word Ladder (LeetCode #127)

Cho `beginWord = "hit"`, `endWord = "cog"`, `wordList = ["hot","dot","dog","lot","log","cog"]`. Tìm shortest transformation (đổi 1 ký tự mỗi bước).

```
hit → hot → dot → dog → cog  (length = 5)
```

**Gợi ý:** Mỗi word = 1 node. Hai word khác nhau đúng 1 ký tự = 1 edge. BFS tìm shortest path từ beginWord đến endWord!

---

## BFS trong Rust ecosystem + KaCrab

- **`VecDeque`** (std) -- queue cho BFS, đã quen từ chương Queue/Deque
- **`petgraph::visit::Bfs`** -- BFS iterator built-in trong petgraph
- **`petgraph::algo::dijkstra`** -- khi cần weighted shortest path (chương sau)

**KaCrab** dùng BFS cho:
- **Consumer group rebalance discovery:** BFS từ group coordinator để tìm tất cả members trong partition assignment graph
- **Broker cluster health check:** BFS từ controller broker để verify tất cả brokers reachable. Nếu BFS không thăm hết = có broker bị isolate

---

## Tiếp theo -- DFS

BFS thăm theo tầng, tìm shortest path. Nhưng nhiều bài toán cần **đi sâu** trước khi quay lại: detect cycle, topological sort, backtracking. Chương tiếp theo giới thiệu **DFS** (Depth-First Search) -- dùng Stack thay Queue, đi hết 1 nhánh rồi mới quay lại. DFS là nền tảng cho topological sort (build order), cycle detection, strongly connected components. Cùng với BFS, DFS hoàn thiện bộ "vũ khí" graph traversal của bạn.

---

---

[← Biểu diễn đồ thị](./01-graph-representations.md) | [DFS →](./03-dfs.md)
