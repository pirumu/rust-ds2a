# Thuật toán Dijkstra

> 💡 **Đừng lo lắng:** Dijkstra nghe "hàn lâm" nhưng ý tưởng cực kỳ đơn giản: **BFS + Priority Queue**. BFS dùng Queue (FIFO) → thăm theo số cạnh. Dijkstra dùng Priority Queue (min-distance) → thăm theo tổng weight. Nếu bạn hiểu BFS (chương trước) và Priority Queue (chương Heap), bạn đã hiểu **90% Dijkstra**. Code chỉ khác BFS khoảng 5 dòng. Dijkstra là thuật toán graph **được hỏi nhiều nhất** trong phỏng vấn, và là nền tảng của Google Maps, network routing, game AI pathfinding.

## Đây là gì?

Hãy tưởng tượng bạn mở **Google Maps** để tìm đường từ nhà đến trường. Có nhiều đường đi khác nhau -- đường nào ngắn nhất? Đường nào nhanh nhất?

Đó chính là bài toán mà **thuật toán Dijkstra** giải quyết. Nó tìm **đường đi ngắn nhất** từ một điểm xuất phát đến tất cả các điểm khác trong graph có **trọng số không âm** (non-negative weights).

Tại sao BFS không đủ? BFS tìm đường ít cạnh nhất, nhưng không quan tâm trọng số. Ví dụ: đường đi qua 2 cạnh (trọng số 10 + 10 = 20) có thể dài hơn đường đi qua 5 cạnh (trọng số 1 + 1 + 1 + 1 + 1 = 5).

Ý tưởng cốt lõi của Dijkstra: **tham lam** (greedy). Luôn xử lý đỉnh gần nhất chưa xử lý. Đỉnh đó chắc chắn đã có khoảng cách ngắn nhất rồi, vì mọi đường khác phải đi qua đỉnh xa hơn (trọng số không âm → chỉ có thể dài thêm).

---

## BFS → Dijkstra: chỉ đổi Queue thành Priority Queue

Đặt code BFS và Dijkstra cạnh nhau -- bạn sẽ thấy chúng **gần như giống hệt**:

```
BFS (chương trước):                     Dijkstra:
─────────────────────                    ────────────────────

let mut queue = VecDeque::new();         let mut heap = BinaryHeap::new();
queue.push_back(start);                  heap.push(Reverse((0, start)));
visited[start] = true;                   dist[start] = 0;

while let Some(v) = queue.pop_front() { while let Some(Reverse((d,u))) = heap.pop() {
                                           if d > dist[u] { continue; }
  for &(neighbor, _) in &adj[v] {         for &(v, w) in &adj[u] {
    if !visited[neighbor] {                  let nd = d + w;
      visited[neighbor] = true;              if nd < dist[v] {
      queue.push_back(neighbor);               dist[v] = nd;
    }                                          heap.push(Reverse((nd, v)));
  }                                          }
}                                          }
                                         }
```

So sánh trực tiếp:

| | BFS | Dijkstra |
|---|-----|---------|
| Data structure | Queue (FIFO) | **Priority Queue** (min-distance) |
| Tracking | `visited[]` (bool) | `dist[]` (khoảng cách) |
| Mục tiêu | Tìm **fewest edges** | Tìm **smallest total weight** |
| Thời gian | O(V + E) | O((V+E) log V) |

**Dijkstra = BFS mà thay Queue bằng Priority Queue + thay `visited` bằng `dist[]`. Đó là TOÀN BỘ sự khác biệt.**

---

## Hoạt động như thế nào?

### Trace trên graph nhỏ (4 đỉnh)

```
Graph:
  0 --(4)--> 1 --(1)--> 3
  |                      ^
 (2)                    (3)
  v                      |
  2 ---------(3)-------->+

Tức là:
  Đường 0→1: chi phí 4
  Đường 0→2: chi phí 2
  Đường 1→3: chi phí 1
  Đường 2→3: chi phí 3
```

Tìm đường ngắn nhất từ đỉnh 0:

```
Khởi tạo:  dist = [0, INF, INF, INF]
                    ↑
               Đỉnh 0 cách chính nó = 0, còn lại chưa biết

Bước 1: Xử lý đỉnh 0 (dist=0) ← gần nhất
  ┌─────────────────────────────────────────┐
  │  Xét cạnh 0→1: dist[1] = min(INF, 0+4) = 4  │
  │  Xét cạnh 0→2: dist[2] = min(INF, 0+2) = 2  │
  └─────────────────────────────────────────┘
  dist = [0, 4, 2, INF]

Bước 2: Xử lý đỉnh 2 (dist=2) ← gần nhất chưa xử lý
  ┌──────────────────────────────────────────┐
  │  Xét cạnh 2→3: dist[3] = min(INF, 2+3) = 5  │
  └──────────────────────────────────────────┘
  dist = [0, 4, 2, 5]

Bước 3: Xử lý đỉnh 1 (dist=4)
  ┌──────────────────────────────────────────┐
  │  Xét cạnh 1→3: dist[3] = min(5, 4+1) = 5    │
  │  → Không cải thiện! 0→2→3 (=5) vẫn tốt nhất │
  └──────────────────────────────────────────┘
  dist = [0, 4, 2, 5]

Bước 4: Xử lý đỉnh 3 (dist=5) → Xong!
  Kết quả: dist = [0, 4, 2, 5]
```

**Đọc kết quả:**
- Từ 0 đến 1: chi phí 4 (đi thẳng 0→1)
- Từ 0 đến 2: chi phí 2 (đi thẳng 0→2)
- Từ 0 đến 3: chi phí 5 (đi 0→2→3, vì 2+3=5 < 4+1=5 → hòa, nhưng 0→2→3 được tìm thấy trước)

---

### Trace trên graph lớn hơn (6 đỉnh) -- path update!

Graph nhỏ quá dễ. Thử graph 6 đỉnh -- nơi Dijkstra thực sự "ghi đè" đường cũ bằng đường tốt hơn:

```
Graph (undirected, weighted):

    A --1-- B --6-- D
    |       |       |
    4       2       1
    |       |       |
    C --3-- E --2-- F
```

Dijkstra từ A (dùng A=0, B=1, C=2, E=3, F=4, D=5):

```
Step  Pop      Update                     PQ                        dist[]
──────────────────────────────────────────────────────────────────────────────
 0    --       enqueue A(0)               [(A,0)]                   A=0
 1    A(0)     B:0+1=1, C:0+4=4          [(B,1),(C,4)]             B=1, C=4
 2    B(1)     D:1+6=7, E:1+2=3          [(E,3),(C,4),(D,7)]       E=3
 3    E(3)     C:3+3=6 > 4 skip,         [(C,4),(F,5),(D,7)]       F=5
               F:3+2=5
 4    C(4)     (no better paths)          [(F,5),(D,7)]             --
 5    F(5)     D:5+1=6 < 7 update!       [(D,6),(D,7)]             D=6
 6    D(6)     (D,7) ← stale, skip       []                        --
──────────────────────────────────────────────────────────────────────────────

Final: A=0, B=1, C=4, E=3, F=5, D=6
```

Shortest paths:
```
  A→B: A→B = 1
  A→C: A→C = 4
  A→E: A→B→E = 3
  A→F: A→B→E→F = 5
  A→D: A→B→E→F→D = 6  (KHÔNG PHẢI A→B→D=7!)
```

**Bước 5 là then chốt:** đường A→B→D=7 bị **"ghi đè"** bởi A→B→E→F→D=6. Dijkstra tự động tìm đường tốt hơn nhờ Priority Queue -- đường ngắn hơn được xét trước, và khi tìm thấy update thì push entry mới vào heap.

**Bước 6:** khi pop D(7) ra, ta thấy `7 > dist[D]=6` → entry cũ, bỏ qua. Đây chính là **lazy deletion** -- thay vì xóa entry cũ trong heap (khó), ta chỉ skip nó khi pop ra.

---

### Tại sao KHÔNG dùng được với trọng số âm?

Dijkstra giả sử: khi một đỉnh được xử lý, khoảng cách đến nó là ngắn nhất rồi. Trọng số âm phá vỡ giả sử này.

```
Ví dụ phản biện:
  0 --(1)--> 1
  0 --(5)--> 2 --(-4)--> 1

Dijkstra xử lý đỉnh 1 với dist=1 (đi thẳng 0→1)
Nhưng đường 0→2→1 = 5+(-4) = 1... hoặc nếu cạnh âm là -5,
đường đó = 0, ngắn hơn! Dijkstra không bao giờ phát hiện ra.

→ Dùng Bellman-Ford cho graph có trọng số âm.
```

---

## Path Reconstruction -- tìm đường đi, không chỉ khoảng cách

Code Dijkstra cơ bản chỉ trả về `dist[]` -- khoảng cách ngắn nhất. Nhưng thực tế (Google Maps!), bạn cần biết **đi đường nào**, không chỉ bao xa.

Giải pháp: thêm mảng `parent[]` -- ghi lại "ai dẫn đến tôi".

```
Trace lại ví dụ 6 đỉnh:

parent[] thay đổi qua các bước:
  Bước 1: parent[B] = A, parent[C] = A
  Bước 2: parent[D] = B, parent[E] = B
  Bước 3: parent[F] = E
  Bước 5: parent[D] = F  ← UPDATE! trước đó parent[D] = B

Reconstruct A→D:
  D ← parent[D] = F ← parent[F] = E ← parent[E] = B ← parent[B] = A
  Đảo ngược: A → B → E → F → D ✓
```

**Khác biệt với BFS path reconstruction:** BFS set `parent` đúng **1 lần** (first visit = shortest). Dijkstra `parent` có thể **thay đổi nhiều lần** khi tìm đường tốt hơn.

```rust
pub fn dijkstra_with_path(&self, start: usize) -> (Vec<i64>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist = vec![i64::MAX; self.num_vertices];
    let mut parent = vec![None; self.num_vertices];  // ai dẫn đến tôi?
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0i64, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] { continue; }
        for &(v, w) in &self.adjacency_list[u] {
            let nd = d.saturating_add(w);
            if nd < dist[v] {
                dist[v] = nd;
                parent[v] = Some(u);  // ghi nhớ: u dẫn đến v
                heap.push(Reverse((nd, v)));
            }
        }
    }
    (dist, parent)
}

// Reconstruct path từ parent[]
fn get_path(parent: &[Option<usize>], end: usize) -> Vec<usize> {
    let mut path = vec![end];
    let mut current = end;
    while let Some(p) = parent[current] {
        path.push(p);
        current = p;
    }
    path.reverse();
    path
}
```

---

## Lazy Deletion vs Decrease-Key

Khi Dijkstra tìm đường tốt hơn đến đỉnh V, cần update priority của V trong heap. Có 2 cách:

### Cách 1: Lazy Deletion (code chúng ta dùng -- đơn giản)

```
- Push entry MỚI vào heap (với distance mới)
- KHÔNG xóa entry cũ
- Khi pop entry cũ ra → d > dist[u] → skip (bỏ qua)
- Heap có thể chứa duplicate entries

Ưu điểm: code đơn giản (thêm 1 dòng if)
Nhược điểm: heap lớn hơn (tối đa E entries thay vì V)
Time: O((V+E) log E) ≈ O((V+E) log V) (vì E ≤ V², nên log E ≤ 2 log V)
```

### Cách 2: Decrease-Key (textbook -- phức tạp)

```
- Tìm entry trong heap → giảm priority → sift up
- Heap luôn chỉ có V entries
- Cần indexed priority queue (HashMap + Heap)

Ưu điểm: heap nhỏ hơn, theoretically faster
Nhược điểm: code phức tạp hơn NHIỀU
Time: O((V+E) log V) với binary heap
```

**Thực tế:** lazy deletion gần như **luôn được chọn** vì đơn giản hơn và performance difference rất nhỏ. Competitive programming, production code, interview -- đều dùng lazy deletion.

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub fn dijkstra(&self, start: usize) -> Vec<i64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut dist = vec![i64::MAX; self.num_vertices];
    dist[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0i64, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue; // entry cũ, bỏ qua (lazy deletion)
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
```

Giải thích:

- `Reverse` biến max-heap (mặc định của Rust) thành **min-heap**. Đỉnh có khoảng cách nhỏ nhất được pop trước
- `saturating_add` tránh tràn số khi cộng với `i64::MAX` (đỉnh chưa tìm thấy đường)
- `d > dist[u]` -- **lazy deletion**: khi tìm đường tốt hơn, ta push entry mới vào heap nhưng không xóa entry cũ. Khi pop entry cũ ra, ta phát hiện nó đã lỗi thời và bỏ qua. Đơn giản hơn nhiều so với decrease-key
- Đỉnh không đến được giữ giá trị `i64::MAX`

---

## Gia đình Shortest Path Algorithms

Dijkstra không phải thuật toán duy nhất tìm đường ngắn nhất. Đặt nó trong bức tranh lớn:

| | BFS | Dijkstra | Bellman-Ford | Floyd-Warshall |
|---|-----|---------|-------------|----------------|
| Graph type | Unweighted | **Non-negative** weights | Any weights | Any weights |
| Negative weights | N/A | ❌ | ✅ | ✅ |
| Negative cycles | N/A | N/A | Detects | Detects |
| Source | Single | Single | Single | **All pairs** |
| Time | O(V+E) | **O((V+E)logV)** | O(VE) | O(V³) |
| Dùng khi | Fewest hops | Min total weight | Neg weights | All-to-all |
| Data structure | Queue | **Priority Queue** | Just arrays | Matrix |

**Chọn algorithm:**
```
Unweighted graph?                → BFS
Weighted, no negative weights?   → Dijkstra ✅
Has negative weights?            → Bellman-Ford
All-pairs shortest path?         → Floyd-Warshall
```

---

## A* Preview -- Dijkstra có "la bàn"

Dijkstra explore **mọi hướng** đều nhau -- nó không biết goal ở đâu. A* thêm **heuristic** (ước lượng khoảng cách đến goal) để ưu tiên hướng đúng:

```
Dijkstra:                     A*:
  Explore theo hình tròn        Explore hướng về goal
  (mọi hướng đều nhau)          (biết goal ở đâu → ưu tiên)

       xxxxx                       x
      xxxxxxx                     xxx
     xxxxSxxxx                   xxSxx
      xxxxxxx                     xxxxx
       xxxxx                       xxxxG

  S = start, G = goal
  Dijkstra explore đều          A* hướng về goal → nhanh hơn
```

**A* = Dijkstra + heuristic function.** Nếu heuristic = 0 → A* chính là Dijkstra. Heuristic tốt → A* nhanh hơn nhiều. Game pathfinding luôn dùng A* thay vì Dijkstra vì biết tọa độ goal.

```
f(n) = g(n) + h(n)
  g(n) = Dijkstra distance từ start đến n (đã biết chính xác)
  h(n) = ước lượng distance từ n đến goal (heuristic)
```

---

## Dijkstra trong thực tế

### Google Maps / Navigation
```
Graph: ~1 tỷ giao lộ, weighted by thời gian/khoảng cách
Variant: bidirectional Dijkstra + contraction hierarchies
  Preprocess: precompute "shortcuts" giữa important intersections
  Query: Dijkstra từ CẢ 2 ĐẦU, gặp nhau ở giữa
  Result: sub-second query trên graph 1 tỷ nodes!
```

### Network Routing (OSPF protocol)
```
Router graph: router = vertex, link = edge (weight = latency/cost)
Mỗi router chạy Dijkstra trên link-state graph
→ routing table: mỗi destination → next hop
OSPF (Open Shortest Path First) = Dijkstra-based protocol
```

### Game AI Pathfinding
```
Grid map: cell = vertex, adjacent cells = edges
A* = Dijkstra + heuristic (estimate distance to goal)
Game dùng A* (Dijkstra + la bàn) cho NPC di chuyển
Starcraft, Civilization, mọi RTS game đều dùng variant của A*
```

### Social Network -- Weighted Connections
```
LinkedIn: tìm "strongest path" đến hiring manager
Edge weight = connection strength (inverse)
Dijkstra tìm path with highest total strength
```

---

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian (binary heap) | O((V + E) log V) |
| Thời gian (Fibonacci heap) | O(V log V + E) |
| Bộ nhớ | O(V + E) |
| Xử lý trọng số âm? | Không! |

Mỗi đỉnh xử lý 1 lần, mỗi cạnh gây tối đa 1 lần push vào heap. Mỗi thao tác heap mất O(log V). Tổng: O((V + E) log V).

**Tại sao O((V+E) log V) mà không phải O((V+E) log E)?**

Với lazy deletion, heap có thể chứa tối đa E entries → mỗi operation là O(log E). Nhưng vì E ≤ V² → log E ≤ 2 log V → O(log E) = O(log V). Nên cả 2 cách viết đều đúng.

**Dense graph (E ≈ V²):** Dijkstra với binary heap là O(V² log V). Dùng array thay heap sẽ là O(V²) -- nhanh hơn! Nhưng sparse graph (E << V²) thì binary heap thắng.

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// --- Dijkstra cơ bản: chỉ tìm distance ---
let mut g = Graph::new(4, true);
g.add_edge(0, 1, 4);
g.add_edge(0, 2, 2);
g.add_edge(1, 3, 1);
g.add_edge(2, 3, 3);

let dist = g.dijkstra(0);
assert_eq!(dist[0], 0);  // khoảng cách đến chính mình
assert_eq!(dist[1], 4);  // 0→1 trực tiếp
assert_eq!(dist[2], 2);  // 0→2 trực tiếp
assert_eq!(dist[3], 5);  // 0→2→3 (2+3=5)

// Đỉnh không đến được
let mut g2 = Graph::new(3, true);
g2.add_edge(0, 1, 1);
let dist2 = g2.dijkstra(0);
assert_eq!(dist2[2], i64::MAX); // đỉnh 2 không đến được từ 0

// --- Dijkstra + path reconstruction ---
let mut g3 = Graph::new(6, false);
g3.add_edge(0, 1, 1);  // A-B
g3.add_edge(0, 2, 4);  // A-C
g3.add_edge(1, 3, 2);  // B-E
g3.add_edge(1, 4, 6);  // B-D
g3.add_edge(3, 5, 2);  // E-F
g3.add_edge(3, 2, 3);  // E-C
g3.add_edge(5, 4, 1);  // F-D

let (dist3, parent) = g3.dijkstra_with_path(0);
assert_eq!(dist3[4], 6); // A→B→E→F→D = 1+2+2+1 = 6
```

---

## Những cái bẫy hay gặp

### ❌ Dùng Dijkstra cho graph có negative weights

```rust
// SAI: graph có cạnh âm
g.add_edge(1, 2, -3);
let dist = g.dijkstra(0); // KẾT QUẢ SAI!
```

✅ Luôn check: có negative weight? → dùng Bellman-Ford.

💡 Dijkstra giả sử "đã xử lý = đã tối ưu". Negative weight phá vỡ giả sử này vì đường dài hơn (nhiều cạnh hơn) có thể trở nên ngắn hơn nhờ cạnh âm.

---

### ❌ Quên `Reverse` cho `BinaryHeap` Rust

```rust
// SAI: BinaryHeap Rust là MAX-heap!
heap.push((dist, vertex)); // pop ĐỈNH XA NHẤT → sai hoàn toàn
```

✅ LUÔN wrap trong `Reverse(...)`:

```rust
heap.push(Reverse((dist, vertex))); // pop đỉnh GẦN NHẤT ✓
```

💡 `BinaryHeap` Rust là **max-heap**. Dijkstra cần **min-heap**. Không `Reverse` → pop node xa nhất thay vì gần nhất → kết quả sai hoàn toàn mà không có error nào.

---

### ❌ Quên skip stale entries (lazy deletion)

```rust
// SAI: thiếu check này
while let Some(Reverse((d, u))) = heap.pop() {
    // if d > dist[u] { continue; }  ← QUÊN DÒNG NÀY
    for &(v, w) in &self.adjacency_list[u] {
        // ... process entry cũ → push thêm stale entries
    }
}
```

✅ Luôn có `if d > dist[u] { continue; }` ngay sau `heap.pop()`.

💡 Thiếu dòng này → process entry cũ → có thể push thêm stale entries → performance tệ O(VE) thay vì O((V+E)logV). Kết quả vẫn đúng, nhưng chậm khủng khiếp.

---

### ❌ Nhầm BFS với Dijkstra

```
Đề bài: "Tìm shortest path trong weighted graph"
Sai: dùng BFS → tìm fewest edges, KHÔNG phải minimum weight
```

✅ "Shortest path" trong weighted graph = **Dijkstra**. "Fewest hops" = **BFS**.

💡 BFS tìm đường ít cạnh nhất. Dijkstra tìm đường nhẹ nhất. Chúng **khác nhau** trên weighted graph. Chỉ khi tất cả weights = 1 thì BFS = Dijkstra.

---

## Khi nào dùng Dijkstra?

| Tình huống | Dijkstra? | Thay bằng gì? | Tại sao? |
|------------|----------|---------------|----------|
| Shortest path, non-negative weights | ✅ | -- | Sinh ra cho việc này |
| Navigation / GPS | ✅ (+ A*) | -- | Production: A* hoặc Contraction Hierarchies |
| Network routing (OSPF) | ✅ | -- | Standard protocol |
| Game AI pathfinding | ⚠️ | A* | A* = Dijkstra + heuristic, nhanh hơn |
| Unweighted shortest path | ❌ | BFS | O(V+E) < O((V+E)logV) |
| Negative weights | ❌ | Bellman-Ford | Dijkstra sai với negative |
| All-pairs shortest path | ❌ | Floyd-Warshall | Dijkstra chỉ single-source |
| Dense graph (E ≈ V²) | ⚠️ | Dijkstra with array | Array O(V²) < heap O(V²logV) |

---

## Luyện nhận diện Pattern

### Bài 1: Network Delay Time (LeetCode #743)

> N nodes, edges có delay time, gửi signal từ node K. Bao lâu để **tất cả** nodes nhận signal?

**Gợi ý:** Dijkstra từ K, answer = `max(dist[])`. Nếu có node unreachable → return -1.

### Bài 2: Path With Minimum Effort (LeetCode #1631)

> Grid heights, tìm path từ top-left đến bottom-right sao cho **max difference** giữa 2 cell liên tiếp nhỏ nhất.

**Gợi ý:** Modified Dijkstra. "Distance" = max effort trên path thay vì sum. Relaxation: `new_effort = max(current_effort, abs(height_diff))`.

### Bài 3: Cheapest Flights Within K Stops (LeetCode #787)

> Tìm cheapest flight từ src đến dst với tối đa K stops.

**Gợi ý:** Modified Dijkstra/BFS with state `(node, stops_remaining)`. Hoặc Bellman-Ford chạy K+1 iterations.

---

## Dijkstra trong Rust ecosystem

```rust
// std library -- đủ cho interview và production
use std::collections::BinaryHeap;
use std::cmp::Reverse;
// BinaryHeap + Reverse = min-heap cho Dijkstra

// petgraph crate -- graph algorithms built-in
use petgraph::algo::dijkstra;  // Dijkstra sẵn
use petgraph::algo::astar;     // A* sẵn

// pathfinding crate -- nhiều shortest path algorithms
// Dijkstra, A*, BFS, Bellman-Ford, Floyd-Warshall
```

---

## Preview chương tiếp theo

Dijkstra giải quyết shortest path cho **non-negative weights**. Nhưng đời thực có lúc weight âm -- tiền thưởng, giảm giá, negative latency. Chương tiếp theo: **Bellman-Ford** -- thuật toán chậm hơn Dijkstra nhưng handle được **negative weights** và phát hiện **negative cycles**. Bellman-Ford đơn giản hơn Dijkstra (không cần heap, chỉ 3 vòng for), nhưng chậm hơn: O(VE) vs O((V+E)logV).

---

[← DFS](./03-dfs.md) | [Bellman-Ford →](./05-bellman-ford.md)
