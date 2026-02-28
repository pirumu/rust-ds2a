# Thuật toán Prim (Cây khung nhỏ nhất)

> 💡 **Đừng lo lắng:** Nếu bạn hiểu Dijkstra, bạn **đã hiểu 90% Prim** -- code gần như giống nhau, chỉ khác **MỘT dòng**. Dijkstra push `(dist_from_source + weight, node)` vào heap (tổng khoảng cách cộng dồn). Prim push `(weight, node)` (chỉ trọng số cạnh, KHÔNG cộng dồn). Bài toán khác -- MST thay vì shortest path -- nhưng kỹ thuật GIỐNG. MST xuất hiện trong network design, clustering, image segmentation, và phỏng vấn thường hỏi "nối tất cả nodes với minimum cost".

## Đây là gì?

Hãy tưởng tượng bạn là **kỹ sư điện** cần nối điện cho tất cả các nhà trong một ngôi làng. Mỗi nhà cần được kết nối vào mạng lưới điện. Kéo dây giữa hai nhà tốn chi phí khác nhau (tùy khoảng cách, địa hình). Bạn muốn **nối tất cả các nhà** với **chi phí thấp nhất**.

Đó chính là bài toán **Minimum Spanning Tree** (MST -- cây khung nhỏ nhất).

**Thuật toán Prim** giải bài này bằng cách: bắt đầu từ 1 nhà, nối đến **nhà gần nhất** (rẻ nhất), rồi từ mạng lưới hiện tại, nối tiếp đến nhà gần nhất chưa nối, cứ mở rộng dần cho đến khi tất cả nhà đều có điện.

---

## Dijkstra vs Prim: side-by-side

Đặt code Dijkstra và Prim cạnh nhau -- bạn sẽ thấy chúng **gần như giống hệt**:

```rust
// Dijkstra:                           Prim:
heap.push(Reverse((                    heap.push(Reverse((
    dist[u] + w,  // ← tổng từ source     w,           // ← CHỈ weight cạnh
    v                                      from, to
)));                                   )));

if new_dist < dist[v] {               if !in_mst[to] {
    dist[v] = new_dist;                   in_mst[to] = true;
    heap.push(...)                        mst_edges.push(...)
}                                      }
```

So sánh trực tiếp:

```
Dijkstra:                    Prim:
  Mục tiêu: shortest PATH     Mục tiêu: cheapest TREE
  Cộng dồn distance            Chỉ xét weight cạnh hiện tại
  dist[] lưu total distance    in_mst[] lưu "đã kết nối?"
  Output: distance array        Output: list of MST edges
  Directed hoặc undirected      Undirected only (MST)
```

Cách nhớ:
- **Dijkstra hỏi:** "đường ĐẾN node này **tổng** bao nhiêu?"
- **Prim hỏi:** "cạnh NỐI đến node này **nặng** bao nhiêu?"

Dijkstra quan tâm **tổng quãng đường** (cộng dồn từ source). Prim chỉ quan tâm **cạnh rẻ nhất** nối vào mạng lưới hiện tại. Đó là khác biệt DUY NHẤT.

---

## MST Properties -- trước khi hiểu algorithm, hiểu bài toán

**MST** là gì?
- **Spanning** (khung): nối tất cả các đỉnh
- **Tree** (cây): không có vòng lặp, đúng V-1 cạnh
- **Minimum** (nhỏ nhất): tổng trọng số các cạnh nhỏ nhất có thể

```
MST properties:
1. Đúng V-1 cạnh (tree với V nodes luôn có V-1 edges)
2. Connected (nối TẤT CẢ nodes)
3. No cycle (thêm bất kỳ cạnh nào → tạo cycle)
4. Tổng weight NHỎ NHẤT trong mọi spanning tree
5. Không unique! Có thể có nhiều MST (khi có cạnh cùng weight)
6. Bất kỳ node nào đều có thể là root (MST giống nhau)
```

Quay lại ẩn dụ kéo dây điện: MST là cách nối điện cho TẤT CẢ nhà trong làng, dùng ÍT DÂY NHẤT (không thừa dây nào), và TỔNG CHIỀU DÀI DÂY nhỏ nhất.

### Tại sao Prim đúng? -- Cut Property

Nhờ **tính chất lát cắt** (cut property):

```
Cut Property:
  Chia V thành 2 nhóm: S (đã trong MST) và V\S (chưa trong)
  Cạnh rẻ nhất nối S và V\S CHẮC CHẮN thuộc 1 MST nào đó.

  Ví dụ:
    S = {0, 1}     V\S = {2, 3}
    Cạnh nối: 0-2(w=3), 1-3(w=2), 2-3(w=4)
    Rẻ nhất: 1-3(w=2) → chắc chắn trong MST ✓
```

**Chứng minh (đơn giản):**

```
Tại sao cạnh rẻ nhất qua lát cắt luôn thuộc MST?

  Giả sử MST tối ưu KHÔNG chứa cạnh rẻ nhất e.
  → MST đó phải dùng cạnh KHÁC nối S và V\S (vì tree phải connected)
  → Cạnh đó đắt hơn e (vì e là rẻ nhất)
  → Thay cạnh đó bằng e → tổng GIẢM → MST cũ KHÔNG tối ưu
  → Mâu thuẫn! → e phải thuộc MST ✓
```

Prim luôn chọn cạnh rẻ nhất nối 2 nhóm (đã nối / chưa nối). Cut property đảm bảo mỗi lần chọn đều đúng. Greedy works!

---

## Hoạt động như thế nào?

### Trace 1: Graph nhỏ (4 nodes)

```
Graph (vô hướng, có trọng số):

  0 --(1)-- 1        Nhà 0 và 1: nối tốn 1 triệu
  |         |        Nhà 0 và 2: nối tốn 3 triệu
 (3)       (2)       Nhà 1 và 3: nối tốn 2 triệu
  |         |        Nhà 2 và 3: nối tốn 4 triệu
  2 --(4)-- 3
```

Bắt đầu từ nhà 0:

```
Bước 0: MST = {0}                  ← mới chỉ có nhà 0
  Dây có thể nối: 0-1 (w=1), 0-2 (w=3)
  Chọn rẻ nhất: 0-1 (w=1) ✓

Bước 1: MST = {0, 1}              ← nhà 0 và 1 có điện
  Dây có thể nối: 0-2 (w=3), 1-3 (w=2)
  Chọn rẻ nhất: 1-3 (w=2) ✓

Bước 2: MST = {0, 1, 3}           ← 3 nhà có điện
  Dây có thể nối: 0-2 (w=3), 3-2 (w=4)
  Chọn rẻ nhất: 0-2 (w=3) ✓

Bước 3: MST = {0, 1, 2, 3}        ← Tất cả nhà có điện!

Các dây đã nối: (0,1,1), (1,3,2), (0,2,3)
Tổng chi phí: 1 + 2 + 3 = 6 triệu
```

```
Kết quả MST:

  0 --(1)-- 1        Cạnh 2-3 (w=4) KHÔNG được chọn
  |         |        vì sẽ tạo vòng lặp và đắt hơn
 (3)       (2)
  |         |
  2         3
```

### Dùng priority queue cho nhanh

Thay vì quét tất cả cạnh ứng viên mỗi bước, dùng **min-heap** để tìm cạnh rẻ nhất nhanh chóng:

```
Min-heap chứa các cạnh ứng viên:
  ┌──────────────────────┐
  │ (w=1, 0→1)           │ ← rẻ nhất, lấy ra trước!
  │ (w=2, 1→3)           │
  │ (w=3, 0→2)           │
  │ (w=4, 2→3)           │
  └──────────────────────┘
```

---

### Trace 2: Graph lớn hơn (6 nodes)

```
Graph (6 nodes):
          4
  0 ─────────── 1
  │             │ \
  │2          5 │  10
  │             │    \
  2 ─────────── 3 ─── 1 ── 3
  │      3      │          │
  │             │1         │7
  │8            │          │
  └──── 4 ──────┘    5 ───┘
              6

  Cạnh: 0-1(4), 0-2(2), 1-2(5), 1-3(10), 2-3(3), 2-4(8), 3-4(1), 3-5(7), 4-5(6)
```

Prim từ nhà 0:

```
Step  Pop           Add edge      MST nodes       Heap (top entries)
─────────────────────────────────────────────────────────────────────
 0    start at 0    --            {0}             [(2,0→2),(4,0→1)]
 1    (2, 0→2)      0-2(2)        {0,2}           [(3,2→3),(4,0→1),(5,2→1),(8,2→4)]
 2    (3, 2→3)      2-3(3)        {0,2,3}         [(1,3→4),(4,0→1),(7,3→5),...]
 3    (1, 3→4)      3-4(1)        {0,2,3,4}       [(4,0→1),(6,4→5),(7,3→5),...]
 4    (4, 0→1)      0-1(4)        {0,1,2,3,4}     [(6,4→5),(7,3→5),...]
 5    (6, 4→5)      4-5(6)        {0,1,2,3,4,5}   done!

MST edges: 0-2(2), 2-3(3), 3-4(1), 0-1(4), 4-5(6)
Total: 2 + 3 + 1 + 4 + 6 = 16
Edges: 5 = V-1 = 6-1 ✓
```

Từng bước chi tiết:
- **Step 1:** Từ nhà 0, dây rẻ nhất là 0→2 (w=2). Nối nhà 2!
- **Step 2:** Từ mạng {0,2}, dây rẻ nhất là 2→3 (w=3). Nối nhà 3!
- **Step 3:** Từ mạng {0,2,3}, dây rẻ nhất là 3→4 (w=1). Nối nhà 4!
- **Step 4:** Tiếp theo là 0→1 (w=4). Nối nhà 1!
- **Step 5:** Cuối cùng là 4→5 (w=6). Nối nhà 5! Tất cả nhà có điện!

---

## Prim vs Kruskal: hai cách giải cùng bài toán

Cùng bài toán kéo dây điện cho làng, nhưng **2 cách nghĩ khác nhau**:

```
Prim:                                   Kruskal:
  Bắt đầu từ 1 nhà, mở rộng dần         Nhìn TẤT CẢ dây, chọn rẻ nhất trước
  "Xây từ trung tâm, lan ra"             "Chọn dây rẻ nhất, kết nối dần"
  Dùng: Priority Queue                   Dùng: Sort + Union-Find
  Time: O(E log V)                       Time: O(E log E) = O(E log V)
  Best for: dense graph (E ≈ V²)         Best for: sparse graph (E ≈ V)
```

Trace Kruskal trên cùng graph 4 nodes:

```
Edges sorted: (0-1, w=1), (1-3, w=2), (0-2, w=3), (2-3, w=4)

Take (0-1, w=1): 0 và 1 khác component → ADD ✓
Take (1-3, w=2): 1 và 3 khác component → ADD ✓
Take (0-2, w=3): 0 và 2 khác component → ADD ✓
Skip (2-3, w=4): 2 và 3 CÙNG component → SKIP (tạo cycle)

MST = {(0-1,1), (1-3,2), (0-2,3)}, total = 6 ✓
Giống kết quả Prim!
```

| | Prim | Kruskal |
|---|------|---------|
| Approach | Grow tree từ 1 node | Add cheapest edge |
| Data structure | Priority Queue | Sort + Union-Find |
| Time | O(E log V) | O(E log E) |
| Best for | Dense graph | Sparse graph |
| Input format | Adjacency list | Edge list |
| Partial MST? | Yes (stop early) | No (need all edges sorted) |

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub fn prim_mst(&self) -> Vec<(usize, usize, i64)> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = self.num_vertices;
    if n == 0 {
        return Vec::new();
    }

    let mut in_mst = vec![false; n];
    let mut mst_edges = Vec::new();
    let mut heap = BinaryHeap::new();

    // Bắt đầu từ nhà 0
    in_mst[0] = true;
    for &(v, w) in &self.adjacency_list[0] {
        heap.push(Reverse((w, 0, v)));
    }

    while let Some(Reverse((w, from, to))) = heap.pop() {
        if in_mst[to] {
            continue; // nhà này đã có điện rồi, bỏ qua
        }
        in_mst[to] = true;
        mst_edges.push((from, to, w));
        // Thêm các dây nối từ nhà mới vào heap
        for &(next, nw) in &self.adjacency_list[to] {
            if !in_mst[next] {
                heap.push(Reverse((nw, to, next)));
            }
        }
    }
    mst_edges
}
```

Giải thích:

- Bắt đầu từ đỉnh 0. MST giống nhau bất kể bắt đầu từ đâu (với graph liên thông)
- `Reverse` biến max-heap thành min-heap → cạnh rẻ nhất được pop trước
- Cạnh đến đỉnh đã trong MST bị bỏ qua (lazy deletion) -- giống Dijkstra
- Kết quả là danh sách các cạnh `(từ, đến, trọng số)` tạo thành MST
- So với Dijkstra: heap push `(w, from, to)` thay vì `(dist[u] + w, v)`. Chỉ khác đó thôi!

---

## MST trong thực tế

### a) Network Design -- nối mạng cho office

```
Bài toán: Nối N offices bằng cáp mạng → tổng chiều dài cáp nhỏ nhất
  Office = node
  Cost to connect = edge weight
  MST = minimum total cable

  Ví dụ: 5 offices, kéo cáp quang giữa các cặp
  → MST cho biết nối cặp nào để tổng dây ngắn nhất
  → Tiết kiệm hàng triệu đồng tiền cáp!
```

### b) Clustering -- phân nhóm bằng MST

Đây là ứng dụng HAY NHẤT của MST:

```
K clusters từ N points:
  1. Build MST (nối tất cả points)
  2. Xóa K-1 cạnh NẶNG NHẤT
  → N nodes chia thành K connected components = K clusters

Ví dụ: 6 điểm, muốn 2 clusters:
  MST: A-B(1), B-C(2), C-D(8), D-E(1), E-F(3)
                        ^^^
                     cạnh nặng nhất!

  Xóa cạnh nặng nhất: C-D(8)
  → Cluster 1: {A, B, C}    Cluster 2: {D, E, F}
```

Cách này gọi là **single-linkage clustering**. Ý tưởng: những điểm "gần nhau" được MST nối bằng cạnh nhẹ. Cạnh nặng = khoảng cách lớn giữa 2 nhóm → cắt đó!

### c) Image Segmentation

```
Pixel = node, similarity giữa 2 pixel cạnh nhau = edge weight (inverted)
MST-based segmentation: cut heavy edges → tách vùng ảnh
→ Ứng dụng trong computer vision, y tế (phân vùng ảnh MRI)
```

### d) Approximation cho TSP

```
Traveling Salesman Problem (NP-hard) -- đi thăm tất cả thành phố:
  MST weight ≤ optimal TSP tour ≤ 2 × MST weight
  → MST là lower bound và basis cho 2-approximation algorithm
  → Khi bài toán NP-hard, MST cho lời giải "đủ tốt"
```

---

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian (binary heap) | O(E log V) |
| Thời gian (Fibonacci heap) | O(E + V log V) |
| Bộ nhớ | O(V + E) |

Mỗi cạnh được push vào heap tối đa 1 lần. Mỗi thao tác heap mất O(log V). Tổng: O(E log V).

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

let mut g = Graph::new(4, false);
g.add_edge(0, 1, 1);
g.add_edge(0, 2, 3);
g.add_edge(1, 3, 2);
g.add_edge(2, 3, 4);

let mst = g.prim_mst();
let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
assert_eq!(total, 6);     // 1 + 2 + 3 = 6
assert_eq!(mst.len(), 3); // V-1 = 4-1 = 3 cạnh

// Graph lớn hơn
let mut g2 = Graph::new(5, false);
g2.add_edge(0, 1, 2);
g2.add_edge(0, 3, 6);
g2.add_edge(1, 2, 3);
g2.add_edge(1, 3, 8);
g2.add_edge(2, 4, 5);
g2.add_edge(3, 4, 7);
let mst2 = g2.prim_mst();
let total2: i64 = mst2.iter().map(|&(_, _, w)| w).sum();
assert_eq!(mst2.len(), 4); // 5-1 = 4 cạnh

// So sánh Prim và Kruskal -- cùng kết quả!
let mut g3 = Graph::new(4, false);
g3.add_edge(0, 1, 1);
g3.add_edge(0, 2, 3);
g3.add_edge(1, 3, 2);
g3.add_edge(2, 3, 4);
let prim_total: i64 = g3.prim_mst().iter().map(|&(_, _, w)| w).sum();
let kruskal_total: i64 = g3.kruskal_mst().iter().map(|&(_, _, w)| w).sum();
assert_eq!(prim_total, kruskal_total); // cùng total weight!
```

---

## Những cái bẫy hay gặp

### a) Nhầm Prim với Dijkstra

❌ Push `dist[u] + w` vào heap (Dijkstra) khi đang làm MST

✅ Push `w` (chỉ weight cạnh) vào heap cho Prim

💡 Kiểm tra: bạn đang tìm **shortest path** hay **minimum spanning tree**? Dijkstra cộng dồn, Prim không cộng dồn. Nhầm → kết quả sai hoàn toàn.

### b) Dùng Prim cho directed graph

❌ Chạy Prim trên directed graph rồi thắc mắc "sao kết quả sai?"

✅ MST chỉ defined cho **undirected** graph

💡 Directed graph có "minimum spanning arborescence" (Edmonds' algorithm) -- hoàn toàn khác Prim. Nếu graph là directed, bạn đang giải **bài toán khác**.

### c) Quên handle disconnected graph

❌ Chạy Prim xong, tin kết quả là MST của toàn bộ graph

✅ Check `mst_edges.len() == V - 1` sau khi chạy

💡 Nếu graph không connected, Prim từ 1 node chỉ tìm MST của **connected component chứa node đó**. Nếu `mst_edges.len() < V - 1` → graph disconnected, không có MST.

### d) Nghĩ MST luôn unique

❌ Cho rằng chỉ có đúng 1 MST

✅ Nếu nhiều cạnh cùng weight, có thể có **nhiều MST khác nhau**

💡 Tổng weight giống nhau nhưng cạnh có thể khác. Prim và Kruskal có thể cho MST khác nhau (nhưng cùng total weight). MST unique khi mọi cạnh có weight khác nhau.

---

## Khi nào dùng Prim / Kruskal / neither?

| Tình huống | Prim? | Kruskal? | Tại sao? |
|------------|-------|----------|----------|
| Dense graph (E ≈ V²) | ✅ | ⚠️ | Prim O(E log V) tốt hơn sort O(E log E) |
| Sparse graph (E ≈ V) | ⚠️ | ✅ | Kruskal edge-sort hiệu quả |
| Input = adjacency list | ✅ | ⚠️ | Prim iterate neighbors natural |
| Input = edge list | ⚠️ | ✅ | Kruskal sort edges natural |
| Need partial MST (k edges) | ✅ | ⚠️ | Prim stop early dễ |
| Clustering (cut K-1 heaviest) | ⚠️ | ✅ | Kruskal tự nhiên build components |
| Directed graph | ❌ | ❌ | Cần Edmonds' algorithm |
| Negative weights | ✅ | ✅ | Cả hai handle OK (MST khác shortest path) |
| Shortest path, not MST | ❌ | ❌ | Dùng Dijkstra / Bellman-Ford / Floyd-Warshall |

Tóm lại:
- **Dense graph, adjacency list** → Prim
- **Sparse graph, edge list, clustering** → Kruskal
- **Không chắc** → cả hai cho cùng kết quả, chọn cái nào quen hơn

---

## Luyện nhận diện Pattern

Ba bài tập MST kinh điển:

### Bài 1: Min Cost to Connect All Points (LeetCode #1584)

Cho N points trên 2D plane, cost nối 2 points = Manhattan distance `|x1-x2| + |y1-y2|`. Tìm min cost nối tất cả points.

**Gợi ý:** Đây là complete graph (mỗi cặp points đều có cạnh). Dense graph → Prim tốt hơn Kruskal. Build adjacency list với N² cạnh, chạy Prim.

### Bài 2: Connecting Cities With Minimum Cost (LeetCode #1135)

N cities, danh sách connections có cost. Min cost nối tất cả cities. Nếu không thể → return -1.

**Gợi ý:** MST cơ bản. Chạy Prim hoặc Kruskal. Check `mst_edges.len() == N-1` để biết graph connected không. Nếu không → return -1.

### Bài 3: MST-based Clustering

Cho N points, chia thành K clusters sao cho maximum inter-cluster distance lớn nhất.

**Gợi ý:** Build MST, xóa K-1 cạnh nặng nhất. Đây chính là ứng dụng clustering đã nói ở trên. Kruskal tự nhiên hơn cho bài này vì build MST = build components dần dần.

---

## Rust Ecosystem

- `petgraph::algo::min_spanning_tree` -- Kruskal's built-in trong thư viện `petgraph`
- Prim's thường implement from scratch (vì giống Dijkstra, code ngắn)
- `petgraph::unionfind::UnionFind` -- cho Kruskal's
- Trong Rust std: `BinaryHeap` + `Reverse` là đủ cho Prim (giống code trong `src/graph.rs`)

---

## Chương tiếp theo

Prim giải MST bằng cách **grow tree từ 1 node**. Kruskal giải cùng bài toán nhưng bằng cách **sort edges + Union-Find**. Chương tiếp theo sẽ giới thiệu **Kruskal's Algorithm** cùng **Union-Find** (Disjoint Set) -- data structure cho phép merge và check 2 sets trong gần O(1). Union-Find là tool mạnh mẽ cho mọi bài toán "connectivity" -- từ MST đến cycle detection đến network components.

---

---

[← Floyd-Warshall](./06-floyd-warshall.md) | [Kruskal →](./08-kruskal.md)
