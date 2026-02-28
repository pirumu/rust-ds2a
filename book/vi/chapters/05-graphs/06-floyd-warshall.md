# Thuật toán Floyd-Warshall

## Đây là gì?

> **Đừng sợ cái tên dài.** Floyd-Warshall có tên nghe hàn lâm, nhưng code **ngắn nhất** trong tất cả shortest path algorithms bạn đã học -- chỉ 3 vòng `for` lồng nhau và 1 phép so sánh `min`. Không cần Queue, Stack, hay Priority Queue. Không cần data structure phức tạp nào. Chỉ cần **1 ma trận 2D**. Nếu bạn hiểu câu hỏi: *"đi từ i đến j, đi thẳng hay đi qua k?"* -- bạn đã hiểu Floyd-Warshall. Đây là chương **cuối cùng** về shortest path, và cũng là lần đầu tiên bạn gặp **Dynamic Programming (DP)** -- một kỹ thuật sẽ xuất hiện rất nhiều trong thuật toán nâng cao.

Bạn đã từng thấy **bảng khoảng cách giữa các thành phố** trên bản đồ chưa? Kiểu như:

```
         Hà Nội   Đà Nẵng   Sài Gòn
Hà Nội      0       764       1726
Đà Nẵng    764       0        962
Sài Gòn   1726      962        0
```

Bảng này cho ta khoảng cách ngắn nhất giữa **TẤT CẢ các cặp** thành phố cùng lúc. Đó chính là bài toán mà **Floyd-Warshall** giải quyết.

Dijkstra và Bellman-Ford trả lời: "Đường ngắn nhất **từ 1 điểm** đến mọi nơi?"
Floyd-Warshall trả lời: "Đường ngắn nhất giữa **MỌI cặp** điểm?"

Ý tưởng rất đẹp -- dùng **dynamic programming** (quy hoạch động): Với mỗi đỉnh trung gian `k`, thử xem đi qua `k` có rút ngắn đường từ `i` đến `j` không?

---

## Từ Single-Source đến All-Pairs

Hãy nhìn lại hành trình shortest path bạn đã đi qua:

```
BFS:           1 nguồn → tất cả,  không trọng số,    O(V+E)
Dijkstra:      1 nguồn → tất cả,  trọng số ≥ 0,     O((V+E)logV)
Bellman-Ford:  1 nguồn → tất cả,  trọng số bất kỳ,  O(VE)
Floyd-Warshall: TẤT CẢ → TẤT CẢ, trọng số bất kỳ,  O(V³)
```

Mỗi bước, bạn **mở rộng thêm khả năng**. Floyd-Warshall là bước cuối: giải cho **tất cả các cặp** cùng lúc.

**"Nhưng chạy Dijkstra V lần cũng được mà?"**

Đúng! Cách naive: chạy Dijkstra từ mỗi đỉnh = O(V × (V+E)logV).

| Loại graph | Chạy Dijkstra V lần | Floyd-Warshall |
|---|---|---|
| Thưa (E ≈ V) | O(V² logV) ✅ nhanh hơn | O(V³) |
| Trung bình (E ≈ V^1.5) | O(V^2.5 logV) | O(V³) ≈ tương đương |
| Dày (E ≈ V²) | O(V³ logV) | O(V³) ✅ nhanh + đơn giản hơn |
| Có trọng số âm | ❌ Dijkstra không xử lý được | ✅ |

Kết luận: Floyd-Warshall **thắng** khi graph dày hoặc có trọng số âm. Và luôn thắng về **độ đơn giản của code**.

---

## DP Insight -- tại sao thuật toán đúng?

Phần này hơi "toán" một chút, nhưng rất quan trọng để hiểu **tại sao 3 vòng for hoạt động**.

### Trạng thái DP

```
dist_k[i][j] = đường ngắn nhất từ i đến j
               chỉ dùng các đỉnh {0, 1, ..., k} làm trung gian
```

### Base case

```
dist_{-1}[i][j] = trọng số cạnh trực tiếp (hoặc INF nếu không có cạnh)
```

Khi chưa được dùng đỉnh nào làm trung gian, chỉ có cạnh trực tiếp.

### Công thức chuyển trạng thái

```
dist_k[i][j] = min(
    dist_{k-1}[i][j],                          // không qua k
    dist_{k-1}[i][k] + dist_{k-1}[k][j]        // qua k
)
```

Mỗi khi thêm đỉnh `k` vào tập trung gian, ta chỉ cần hỏi: **"Có nên đi qua k không?"**

### Tại sao k phải ở vòng ngoài cùng?

Vì mỗi giá trị k **mở rộng tập đỉnh trung gian được phép dùng**. Khi xét k=2, ta cần kết quả đã tính xong cho k=0 và k=1. Nếu đặt k ở vòng trong → kết quả chưa hoàn chỉnh → sai!

### Tại sao cập nhật in-place (không cần copy ma trận)?

Câu hỏi hay: nếu ta ghi đè `dist[i][j]`, liệu `dist[i][k]` và `dist[k][j]` có bị ảnh hưởng?

**Không.** Vì:
- Khi xét k, `dist[i][k]` = đường ngắn nhất từ i đến k qua {0..k-1}. Giá trị này **không thay đổi** khi thêm k làm trung gian (đi từ i qua k... đến k = vẫn là đi đến k).
- Tương tự cho `dist[k][j]`.

Nên ta an toàn ghi đè trực tiếp, tiết kiệm bộ nhớ.

---

## Hoạt động như thế nào?

### Công thức DP

```
dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])

Nghĩa là: đi thẳng từ i→j hay đi vòng qua k (i→k→j), cái nào ngắn hơn?
```

Ta thử với **mọi** đỉnh trung gian k, từ 0 đến V-1. Sau khi xét hết, `dist[i][j]` chính là khoảng cách ngắn nhất thật sự.

### Từng bước trên graph nhỏ

```
Graph:
  0 --(3)--> 1
  0 --(6)--> 2
  1 --(2)--> 2
  2 --(1)--> 3
  1 --(8)--> 3
```

**Ma trận khoảng cách ban đầu** (INF = không có cạnh trực tiếp):

```
     0    1    2    3
0 [  0    3    6  INF ]    ← 0 đến 1 = 3, đến 2 = 6
1 [INF    0    2    8 ]    ← 1 đến 2 = 2, đến 3 = 8
2 [INF  INF    0    1 ]    ← 2 đến 3 = 1
3 [INF  INF  INF    0 ]    ← 3 không đi đâu được
```

**Sau k=0** (thử đi qua đỉnh 0):
Không cải thiện gì. Vì không ai đi qua 0 được rút ngắn (0 không có cạnh vào từ 1, 2, 3).

**Sau k=1** (thử đi qua đỉnh 1):
```
dist[0][2] = min(6, 3+2) = 5   ← Đi 0→1→2 ngắn hơn 0→2 trực tiếp!
dist[0][3] = min(INF, 3+8) = 11

     0    1    2    3
0 [  0    3    5   11 ]
1 [INF    0    2    8 ]
2 [INF  INF    0    1 ]
3 [INF  INF  INF    0 ]
```

**Sau k=2** (thử đi qua đỉnh 2):
```
dist[0][3] = min(11, 5+1) = 6   ← Đi 0→1→2→3 ngắn hơn!
dist[1][3] = min(8, 2+1) = 3    ← Đi 1→2→3 ngắn hơn 1→3 trực tiếp!

     0    1    2    3
0 [  0    3    5    6 ]
1 [INF    0    2    3 ]
2 [INF  INF    0    1 ]
3 [INF  INF  INF    0 ]
```

**Sau k=3**: Không cải thiện.

Kết quả cuối cùng cho ta **bảng khoảng cách** hoàn chỉnh giữa tất cả các cặp đỉnh -- giống bảng khoảng cách thành phố lúc đầu!

### Tại sao 3 vòng lặp lồng nhau?

```
for k in 0..V {          ← Vòng ngoài: thử mỗi đỉnh trung gian
    for i in 0..V {      ← Vòng giữa: mỗi đỉnh xuất phát
        for j in 0..V {  ← Vòng trong: mỗi đỉnh đích
            // Đi qua k có rút ngắn i→j không?
        }
    }
}
```

Đơn giản nhưng mạnh mẽ. O(V³) -- chấp nhận được khi V không quá lớn.

---

## Path Reconstruction -- tìm đường đi cụ thể

Floyd-Warshall cơ bản chỉ cho **khoảng cách**. Nhưng nhiều khi ta cần biết **đi qua đâu**. Giải pháp: thêm ma trận `next[i][j]` = đỉnh tiếp theo trên đường từ i đến j.

### Ý tưởng

```
Ban đầu:   next[i][j] = j  (đi thẳng đến j)
Khi cập nhật qua k:  next[i][j] = next[i][k]  (đi đến k trước)
```

### Trace

```
Graph: 0→1(3), 1→2(2), 2→3(1)

Sau Floyd-Warshall:
  dist[0][3] = 6    (0→1→2→3)
  next[0][3] = 1    (từ 0, đi đến 1 trước)
  next[1][3] = 2    (từ 1, đi đến 2)
  next[2][3] = 3    (từ 2, đi đến 3)

Reconstruct path 0→3:
  start = 0
  → next[0][3] = 1
  → next[1][3] = 2
  → next[2][3] = 3  (= đích!)
  Path: [0, 1, 2, 3] ✓
```

### Code Rust

```rust
// Trong src/graph.rs
pub fn floyd_warshall_with_path(&self)
    -> (Vec<Vec<i64>>, Vec<Vec<Option<usize>>>)
{
    let n = self.num_vertices;
    let mut dist = vec![vec![i64::MAX; n]; n];
    let mut next = vec![vec![None; n]; n];  // next hop

    for i in 0..n { dist[i][i] = 0; }
    for u in 0..n {
        for &(v, w) in &self.adjacency_list[u] {
            if w < dist[u][v] {
                dist[u][v] = w;
                next[u][v] = Some(v);  // đi thẳng đến v
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
                        next[i][j] = next[i][k];  // đi đến k trước
                    }
                }
            }
        }
    }
    (dist, next)
}

// Reconstruct path i → j
pub fn get_floyd_path(
    next: &[Vec<Option<usize>>], i: usize, j: usize
) -> Vec<usize> {
    if next[i][j].is_none() { return vec![]; }
    let mut path = vec![i];
    let mut current = i;
    while current != j {
        current = next[current][j].unwrap();
        path.push(current);
    }
    path
}
```

---

## Negative Cycle Detection -- phát hiện chu trình âm

Bellman-Ford phát hiện negative cycle từ **1 nguồn**. Floyd-Warshall phát hiện từ **mọi nơi** -- chỉ cần check đường chéo ma trận.

### Ý tưởng

```
Sau khi chạy Floyd-Warshall, kiểm tra:

for i in 0..n {
    if dist[i][i] < 0 {
        // Node i nằm trên negative cycle!
        // dist[i][i] < 0 = đi từ i quay về i tốn chi phí âm
        // → có thể lặp vô hạn, giảm chi phí mãi
    }
}
```

### Trace

```
Graph: 0→1(1), 1→2(-2), 2→0(-1)
Cycle: 0→1→2→0, tổng = 1 + (-2) + (-1) = -2

Sau Floyd-Warshall:
  dist[0][0] = -2  (< 0! → negative cycle qua 0)
  dist[1][1] = -2  (< 0! → negative cycle qua 1)
  dist[2][2] = -2  (< 0! → negative cycle qua 2)
```

Tất cả đỉnh trên chu trình đều có `dist[i][i] < 0`. Đơn giản và mạnh mẽ.

```rust
// Trong src/graph.rs
pub fn has_negative_cycle(&self) -> bool {
    let dist = self.floyd_warshall();
    dist.iter().enumerate().any(|(i, row)| row[i] < 0)
}
```

---

## Transitive Closure -- biến thể thú vị

Floyd-Warshall không chỉ tìm đường ngắn nhất. Thay đổi phép toán một chút, ta giải bài toán **reachability** (khả năng đến được):

> Node i có thể đến node j (qua bất kỳ đường nào) không?

### Ý tưởng

Thay `min` + `+` bằng `OR` + `AND`:

```rust
for k in 0..n {
    for i in 0..n {
        for j in 0..n {
            reach[i][j] = reach[i][j] || (reach[i][k] && reach[k][j]);
        }
    }
}
```

Cùng 3 vòng for, chỉ thay phép toán! Đây gọi là **Transitive Closure** (bao đóng bắc cầu).

### Ứng dụng

- **Email forwarding**: "Có thể gửi email từ A đến B qua chuỗi forward?"
- **Course prerequisites**: "Course A là prerequisite (trực tiếp/gián tiếp) của course B?"
- **Type system**: "Type A có thể convert sang type B?" (compiler sử dụng)

```rust
// Trong src/graph.rs
pub fn transitive_closure(&self) -> Vec<Vec<bool>> {
    let n = self.num_vertices;
    let mut reach = vec![vec![false; n]; n];

    for i in 0..n { reach[i][i] = true; }
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
```

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub fn floyd_warshall(&self) -> Vec<Vec<i64>> {
    let n = self.num_vertices;
    let mut dist = vec![vec![i64::MAX; n]; n];

    // Khoảng cách đến chính mình = 0
    for i in 0..n {
        dist[i][i] = 0;
    }
    // Khởi tạo từ các cạnh trực tiếp
    for u in 0..n {
        for &(v, w) in &self.adjacency_list[u] {
            if w < dist[u][v] {
                dist[u][v] = w;
            }
        }
    }

    // 3 vòng lặp thần thánh
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
```

Giải thích:

- Khởi tạo đường chéo = 0 (khoảng cách đến chính mình), cạnh trực tiếp = trọng số, còn lại = INF
- Kiểm tra `!= i64::MAX` tránh tràn số khi cộng với "vô cực"
- Nếu có cạnh song song giữa 2 đỉnh, lấy cạnh có trọng số nhỏ nhất

Vẻ đẹp của Floyd-Warshall: **toàn bộ thuật toán chỉ 15 dòng code**. So với Dijkstra cần Priority Queue, Bellman-Ford cần collect edges -- Floyd-Warshall đơn giản đến bất ngờ.

---

## Floyd-Warshall trong thực tế

### Bảng khoảng cách (Distance Table)

```
Airline route planning: khoảng cách giữa tất cả cặp sân bay
Logistics: chi phí vận chuyển giữa tất cả cặp kho hàng
Precompute 1 lần → mỗi query O(1)
```

Đây chính là ẩn dụ "bảng khoảng cách thành phố" -- tính 1 lần, tra cứu mãi mãi.

### Network Latency Matrix

```
N servers, đo latency giữa từng cặp
Floyd-Warshall tìm đường có tổng latency thấp nhất giữa mọi cặp
→ Routing table cho toàn bộ network
```

### Game AI -- All-Pairs Movement Cost

```
Grid map: precompute đường ngắn nhất giữa mọi cặp ô
NPC cần di chuyển từ bất kỳ đâu đến bất kỳ đâu
Floyd-Warshall precompute → mỗi query O(1) lookup
Chỉ khả thi cho map nhỏ (V ≤ 500)
```

---

## Tổng kết Shortest Path Family

Đây là chương cuối về shortest path. Hãy nhìn lại toàn bộ hành trình:

```
BFS → Dijkstra → Bellman-Ford → Floyd-Warshall

Mỗi bước mở rộng khả năng:
  BFS:           unweighted, single-source
  Dijkstra:      + weighted (non-negative)
  Bellman-Ford:  + negative weights + cycle detection
  Floyd-Warshall: + all-pairs
```

### Decision Tree -- chọn algorithm nào?

```
┌─ Không trọng số? ───────────────→ BFS  O(V+E)
│
├─ 1 nguồn, trọng số ≥ 0? ───────→ Dijkstra  O((V+E)logV)
│
├─ 1 nguồn, có trọng số âm? ─────→ Bellman-Ford  O(VE)
│
├─ Tất cả cặp, graph dày? ───────→ Floyd-Warshall  O(V³)
│
└─ Tất cả cặp, graph thưa, ≥ 0? → Chạy Dijkstra V lần
```

| Algorithm | Source | Weights | Time | Space |
|---|---|---|---|---|
| BFS | 1 nguồn | Không trọng số | O(V+E) | O(V) |
| Dijkstra | 1 nguồn | ≥ 0 | O((V+E)logV) | O(V) |
| Bellman-Ford | 1 nguồn | Bất kỳ | O(VE) | O(V) |
| Floyd-Warshall | Tất cả | Bất kỳ | O(V³) | O(V²) |

---

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V³) |
| Bộ nhớ | O(V²) cho ma trận khoảng cách |
| Xử lý trọng số âm? | Có |
| Phát hiện negative cycle? | Có (kiểm tra đường chéo < 0) |
| Path reconstruction? | Có (thêm ma trận next) |
| Transitive closure? | Có (thay min/+ bằng OR/AND) |

### Giới hạn thực tế

| V (số đỉnh) | Thời gian ước tính |
|---|---|
| 100 | ~1 ms |
| 500 | ~125 ms |
| 1,000 | ~1 giây |
| 5,000 | ~2 phút |
| 10,000 | ~17 phút ❌ |

Floyd-Warshall **khả thi** cho V ≤ ~500. Trên 1,000 đỉnh, cần cân nhắc dùng Dijkstra V lần.

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// --- Floyd-Warshall cơ bản ---
let mut g = Graph::new(4, true);
g.add_edge(0, 1, 3);
g.add_edge(0, 2, 6);
g.add_edge(1, 2, 2);
g.add_edge(2, 3, 1);
g.add_edge(1, 3, 8);

let dist = g.floyd_warshall();
assert_eq!(dist[0][1], 3);        // trực tiếp
assert_eq!(dist[0][2], 5);        // 0→1→2
assert_eq!(dist[0][3], 6);        // 0→1→2→3
assert_eq!(dist[1][3], 3);        // 1→2→3
assert_eq!(dist[3][0], i64::MAX); // không có đường từ 3 về 0

// --- Path reconstruction ---
let (dist, next) = g.floyd_warshall_with_path();
let path = Graph::get_floyd_path(&next, 0, 3);
assert_eq!(path, vec![0, 1, 2, 3]);  // đường đi cụ thể

// --- Negative cycle detection ---
let mut g2 = Graph::new(3, true);
g2.add_edge(0, 1, 1);
g2.add_edge(1, 2, -2);
g2.add_edge(2, 0, -1);
assert!(g2.has_negative_cycle());  // cycle 0→1→2→0 = -2

// --- Transitive closure ---
let mut g3 = Graph::new(4, true);
g3.add_edge(0, 1, 1);
g3.add_edge(1, 2, 1);
g3.add_edge(2, 3, 1);
let reach = g3.transitive_closure();
assert!(reach[0][3]);   // 0 có thể đến 3 (qua 1, 2)
assert!(!reach[3][0]);  // 3 không thể về 0
```

---

## Những cái bẫy hay gặp

### ❌ Dùng Floyd-Warshall cho graph lớn

```rust
// V = 10,000 → 10^12 operations → hàng giờ!
let dist = huge_graph.floyd_warshall(); // 💥
```

✅ Floyd-Warshall chỉ khả thi cho V ≤ ~500. Graph lớn + cần all-pairs → chạy Dijkstra V lần.

💡 Kiểm tra `num_vertices()` trước khi chọn algorithm.

---

### ❌ Sai thứ tự vòng lặp k-i-j

```rust
// SAI! k phải ở vòng ngoài cùng
for i in 0..n {
    for j in 0..n {
        for k in 0..n {  // ← k ở trong = sai!
            // ...
        }
    }
}
```

✅ `k` **PHẢI** ở vòng ngoài cùng: `k → i → j`.

💡 Lý do: mỗi k mở rộng tập intermediate vertices. Phải hoàn thành xét tất cả (i,j) cho k hiện tại trước khi sang k+1.

---

### ❌ Quên initialize dist[i][i] = 0

```rust
// Nếu dist[i][i] = INF → negative cycle detection sai
// INF + negative ≠ negative (do overflow check)
```

✅ Luôn set `dist[i][i] = 0` trước khi chạy thuật toán.

💡 Khoảng cách từ 1 đỉnh đến chính nó luôn = 0 (khi chưa có negative cycle).

---

### ❌ Quên check overflow khi cộng

```rust
// dist[i][k] + dist[k][j] có thể overflow!
let bad = dist[i][k] + dist[k][j];  // 💥 nếu cả hai lớn
```

✅ Luôn check `!= i64::MAX` trước khi cộng:

```rust
if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
    let through_k = dist[i][k] + dist[k][j];
    // safe!
}
```

💡 Hoặc dùng `saturating_add` để tránh overflow hoàn toàn.

---

## Khi nào dùng Floyd-Warshall?

| Tình huống | FW? | Thay bằng gì? | Tại sao? |
|---|---|---|---|
| All-pairs, V ≤ 500 | ✅ | -- | Code đơn giản, O(V³) OK |
| All-pairs, dense, negative weights | ✅ | -- | FW handle cả trọng số âm |
| Negative cycle detection (toàn bộ graph) | ✅ | -- | Check diagonal < 0 |
| Transitive closure | ✅ | -- | OR/AND variant |
| All-pairs, V > 1000 | ❌ | V × Dijkstra | O(V³) quá chậm |
| Single-source only | ❌ | Dijkstra/BF | FW overkill |
| Sparse graph, non-negative | ❌ | V × Dijkstra | O(V(V+E)logV) < O(V³) |
| Cần đường đi, không chỉ khoảng cách | ⚠️ | FW + next matrix | Thêm next[][] |

---

## Luyện nhận diện Pattern

### Bài 1: Find the City With the Smallest Number of Neighbors (LeetCode #1334)

**Đề bài**: n cities, các cạnh có distance. Tìm city mà số cities reachable within distance threshold nhỏ nhất.

**Gợi ý**: Floyd-Warshall tính all-pairs distance, rồi đếm cities within threshold cho mỗi city.

```
Ví dụ: threshold = 4
dist[0] = [0, 3, 5, 6]  → 1 city within threshold (city 1)
dist[1] = [INF, 0, 2, 3] → 2 cities within threshold
→ Answer: city 0 (ít neighbors nhất)
```

### Bài 2: Course Schedule IV (LeetCode #1462)

**Đề bài**: n courses, prerequisites. Trả lời queries "course A là prerequisite (trực tiếp/gián tiếp) của course B?"

**Gợi ý**: Đây là **transitive closure**! Dùng OR/AND variant của Floyd-Warshall.

### Bài 3: Optimal Server Location

**Đề bài**: network N nodes, latency matrix giữa adjacent nodes. Tìm node mà max latency đến mọi node khác nhỏ nhất (= optimal server location).

**Gợi ý**: Floyd-Warshall all-pairs, rồi cho mỗi node tính `max(dist[node][j] for all j)`. Node có max nhỏ nhất = answer.

---

## Trong Rust ecosystem

- `petgraph` crate không có built-in Floyd-Warshall, nhưng implement chỉ mất ~15 dòng
- Floyd-Warshall thường **implement from scratch** vì code quá đơn giản -- không cần thư viện
- `ndarray` crate cho matrix operations nếu cần optimize cho V lớn

### Ứng dụng trong hệ thống phân tán

- **Broker latency matrix**: precompute đường có latency thấp nhất giữa tất cả broker pairs → optimal routing cho cross-datacenter messages
- **Resource assignment**: model "cost" gán resource cho consumer → all-pairs cost matrix → tìm assignment tối ưu

---

## Tổng kết

Floyd-Warshall kết thúc chặng đường **graph shortest path algorithms**. Bạn đã master: biểu diễn đồ thị, BFS, DFS, Dijkstra, Bellman-Ford, Floyd-Warshall -- đủ để giải hầu hết bài graph trong phỏng vấn và thực tế.

Nhưng graph chưa hết! Chương tiếp theo: **Prim's algorithm** -- tìm cây khung nhỏ nhất (Minimum Spanning Tree). Bài toán khác hoàn toàn: không phải "đường ngắn nhất" mà là "nối tất cả với chi phí thấp nhất".

---

---

[← Bellman-Ford](./05-bellman-ford.md) | [Prim →](./07-prim.md)
