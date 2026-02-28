# Thuật toán Kruskal (Cây khung nhỏ nhất)

## Đây là gì?

> Kruskal nghe tên lạ nhưng ý tưởng cực kỳ tự nhiên: **sắp xếp tất cả cạnh từ nhỏ đến lớn, rồi lấy từng cạnh nếu không tạo vòng**. Nếu bạn hiểu sorting và Union-Find (chương sau/trước), bạn đã có đủ "nguyên liệu" rồi. Code cũng ngắn đáng ngạc nhiên.

> **Anxiety check:** Kruskal có lẽ là thuật toán MST **trực giác nhất** -- sort cạnh từ nhỏ đến lớn, lấy cạnh nếu không tạo cycle, dừng khi đủ V-1 cạnh. Nếu bạn biết sort (mọi dev đều biết) và Union-Find (sẽ học ngay bên dưới), bạn có tất cả. Code ngắn hơn Prim, ý tưởng giải thích trong 1 câu. Trong phỏng vấn, Kruskal thường được preferred hơn Prim vì **dễ implement** và **dễ giải thích** hơn.

Cùng bài toán nối điện cho làng, nhưng **cách làm khác** so với Prim.

**Prim** bắt đầu từ 1 nhà, mở rộng dần (giống vết dầu loang).
**Kruskal** thì nhìn tất cả đoạn dây, **chọn đoạn ngắn nhất** trước, nối nếu không tạo vòng lặp.

Hãy tưởng tượng bạn có một **đống dây điện** với chiều dài khác nhau. Bạn sắp xếp từ ngắn đến dài. Rồi lần lượt lấy từng sợi: nếu nối 2 nhà chưa cùng nhóm → nối. Nếu nối 2 nhà đã cùng nhóm → bỏ (vì sẽ tạo vòng lặp, lãng phí dây).

Kruskal dùng **Union-Find** để kiểm tra nhanh: "2 nhà này đã cùng nhóm chưa?"

---

## Union-Find mini -- đủ dùng cho Kruskal

Doc hiện tại bảo "xem chương Union-Find" -- nhưng bạn cần hiểu **ngay bây giờ** để hiểu Kruskal. Đừng lo, Union-Find đơn giản hơn bạn tưởng.

**Bài toán:** Bạn có N nhà, ban đầu mỗi nhà tự thành 1 nhóm. Cần 2 thao tác:
- **find(x)**: nhà x thuộc nhóm nào?
- **union(x, y)**: gộp nhóm của x và nhóm của y thành 1

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),  // mỗi node là root của chính nó
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);  // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return false; }  // cùng nhóm → cycle!

        // Union by rank: cây thấp gắn vào cây cao
        if self.rank[rx] < self.rank[ry] { self.parent[rx] = ry; }
        else if self.rank[rx] > self.rank[ry] { self.parent[ry] = rx; }
        else { self.parent[ry] = rx; self.rank[rx] += 1; }
        true  // khác nhóm → đã merge
    }
}
```

**Trace để hiểu:**

```
Ban đầu: parent = [0, 1, 2, 3]  (mỗi node trỏ về chính nó)

union(0, 1): find(0)=0, find(1)=1, khác → merge, parent=[0,0,2,3]
  Nhóm: {0,1} {2} {3}

union(1, 3): find(1)→parent[1]=0→0, find(3)=3, khác → merge, parent=[0,0,2,0]
  Nhóm: {0,1,3} {2}

union(0, 2): find(0)=0, find(2)=2, khác → merge, parent=[0,0,0,0]
  Nhóm: {0,1,2,3}

union(2, 3): find(2)→0, find(3)→0, CÙNG → return false!
  "Nối 2-3 sẽ tạo cycle" ✓
```

Union-Find trả lời "cùng nhóm chưa?" trong **gần O(1)** nhờ path compression + union by rank. Chương Union-Find sẽ giải thích kỹ hơn, nhưng bạn đã hiểu đủ để dùng trong Kruskal.

Quay lại **đống dây điện**: Union-Find chính là cuốn sổ ghi "nhà nào cùng nhóm". Mỗi khi bạn cầm 1 sợi dây, bạn mở sổ check: "2 nhà này cùng nhóm chưa?" Nếu chưa → nối + ghi sổ. Nếu rồi → bỏ dây.

---

## Hoạt động như thế nào?

### Trace 1: Graph nhỏ (4 nodes)

```
Graph (vô hướng, có trọng số):

  0 --(1)-- 1
  |         |
 (3)       (2)
  |         |
  2 --(4)-- 3
```

**Bước 1: Sắp xếp tất cả cạnh theo trọng số:**
```
(0,1,1)  (1,3,2)  (0,2,3)  (2,3,4)
 ngắn     ←───────────────→  dài
```

**Bước 2: Xét từng cạnh:**

```
Cạnh (0,1, w=1):  0 và 1 KHÁC nhóm → NỐI ✓
  Rừng: {0,1}, {2}, {3}

  0 --(1)-- 1



  2         3

Cạnh (1,3, w=2):  1 và 3 KHÁC nhóm → NỐI ✓
  Rừng: {0,1,3}, {2}

  0 --(1)-- 1
             |
            (2)
             |
  2         3

Cạnh (0,2, w=3):  0 và 2 KHÁC nhóm → NỐI ✓
  Rừng: {0,1,2,3} → MST hoàn thành!

  0 --(1)-- 1
  |         |
 (3)       (2)
  |         |
  2         3

Cạnh (2,3, w=4):  2 và 3 CÙNG nhóm → BỎ ✗
  Nối sẽ tạo vòng lặp!
```

**Kết quả:** MST = {(0,1,1), (1,3,2), (0,2,3)}, tổng = 6

---

### Trace 2: Graph lớn hơn (6 nodes)

Bây giờ thử với graph thực tế hơn -- 6 nodes, 9 edges:

```
Graph (6 nodes):

    0 --(4)-- 1
    |       / |
   (2)  (5) (10)
    | /       |
    2 --(3)-- 3
         |    |
        (8)  (1)
         |    |
         4 --(7)-- 5
              |
             (6)
              |
              5

Edges: 0-1(4), 0-2(2), 1-2(5), 1-3(10), 2-3(3), 2-4(8), 3-4(1), 3-5(7), 4-5(6)
```

**Bước 1: Sắp xếp cạnh** (lấy sợi dây ngắn nhất từ đống):
```
3-4(1), 0-2(2), 2-3(3), 0-1(4), 1-2(5), 4-5(6), 3-5(7), 2-4(8), 1-3(10)
```

**Bước 2: Xét từng cạnh:**

```
Step  Edge    Weight  Cùng nhóm?  Action     Groups
───────────────────────────────────────────────────────────────────
 1    3-4     1       Không       ADD ✓      {3,4} {0} {1} {2} {5}
 2    0-2     2       Không       ADD ✓      {0,2} {3,4} {1} {5}
 3    2-3     3       Không       ADD ✓      {0,2,3,4} {1} {5}
 4    0-1     4       Không       ADD ✓      {0,1,2,3,4} {5}
 5    1-2     5       CÓ!         SKIP ✗     (0,1,2 cùng nhóm → cycle)
 6    4-5     6       Không       ADD ✓      {0,1,2,3,4,5}  DONE!
───────────────────────────────────────────────────────────────────

MST: 3-4(1), 0-2(2), 2-3(3), 0-1(4), 4-5(6)
Total: 1 + 2 + 3 + 4 + 6 = 16
Edges: 5 = V-1 ✓

Skipped: 1-2(5) vì tạo cycle
Không cần xét: 3-5(7), 2-4(8), 1-3(10) → dừng sớm sau khi đủ 5 cạnh!
```

Cùng kết quả Prim (tổng 16)! MST edges có thể khác thứ tự nhưng total weight luôn giống.

Để ý: graph có 9 cạnh nhưng Kruskal chỉ xét **6 cạnh** rồi dừng. 3 cạnh còn lại (3-5, 2-4, 1-3) không cần nhìn. Đây là **early termination** -- ưu điểm lớn của Kruskal.

---

## Cycle Property -- tại sao Kruskal đúng?

Prim dùng **Cut Property**: cạnh rẻ nhất qua cut → thuộc MST.

Kruskal ngầm dùng **Cycle Property** (dual của Cut Property):

```
Cycle Property:
  Cạnh ĐẮT NHẤT trong bất kỳ cycle nào → KHÔNG thuộc MST

Cut Property (Prim):
  Cạnh RẺ NHẤT qua cut  → THUỘC MST
```

**Ví dụ:** Trong trace 6 nodes, cạnh 1-2(5) bị skip. Tại sao đúng?

```
Cycle chứa cạnh 1-2:  0 → 1 → 2 → 0
  Edges trong cycle: 0-1(4), 1-2(5), 0-2(2)
  Cạnh đắt nhất: 1-2(5) → KHÔNG thuộc MST ✓

Kruskal sort từ nhỏ → lớn và skip cạnh tạo cycle.
Cạnh bị skip luôn là cạnh đắt nhất trong cycle đó → đúng theo Cycle Property!
```

Nói cách khác: khi bạn cầm sợi dây 1-2 (dài 5), bạn nhìn thấy nhà 1 và nhà 2 **đã nối rồi** (qua 0). Sợi dây này là sợi **dài nhất** trong vòng 0-1-2, nên bỏ nó là hợp lý -- đã có đường ngắn hơn.

---

## Early Termination -- khi đống dây điện rất lớn

Kruskal dừng khi đủ V-1 edges. Với graph lớn, điều này tạo ra lợi thế đáng kể:

```
Graph 10,000 nodes, 1,000,000 edges:
  Sort 1M edges: O(E log E)
  Nhưng MST chỉ cần V-1 = 9,999 edges!

  Nếu 9,999 edges rẻ nhất đều không tạo cycle:
  → Chỉ xét 9,999 / 1,000,000 ≈ 1% edges!
  → Có thể dùng partial sort (nth_element) thay vì full sort

Prim: phải process cho đến khi tất cả nodes trong MST
  → Luôn O(E log V) dù MST form sớm
```

Trong thực tế, nhiều graph "may mắn" -- edges rẻ nhất đủ nối tất cả nodes. Kruskal tận dụng được điều này nhờ dừng sớm, còn Prim thì không.

Code Rust đã có early termination:
```rust
if mst.len() == n - 1 {
    break; // đủ V-1 cạnh → xong, bỏ qua phần còn lại!
}
```

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub fn kruskal_mst(&self) -> Vec<(usize, usize, i64)> {
    let n = self.num_vertices;
    // Thu thập cạnh, tránh trùng lặp cho graph vô hướng
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for u in 0..n {
        for &(v, w) in &self.adjacency_list[u] {
            if self.directed || u < v {
                edges.push((w, u, v));
            }
        }
    }
    edges.sort(); // sắp xếp theo trọng số (phần tử đầu tiên)

    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();

    for (w, u, v) in edges {
        if uf.union(u, v) {
            // u và v khác nhóm → nối an toàn
            mst.push((u, v, w));
            if mst.len() == n - 1 {
                break; // MST có đúng V-1 cạnh → xong!
            }
        }
        // u và v cùng nhóm → bỏ qua (tạo vòng)
    }
    mst
}
```

Giải thích:

- Graph vô hướng: chỉ lấy cạnh với `u < v` để tránh đếm trùng (cạnh A-B và B-A là 1)
- `edges.sort()` sắp theo trọng số vì `w` là phần tử đầu tiên của tuple
- `uf.union(u, v)` trả về `true` nếu u, v khác nhóm → cạnh an toàn, thêm vào MST
- Dừng sớm khi đủ V-1 cạnh (MST của V đỉnh luôn có đúng V-1 cạnh)

**So sánh với ẩn dụ đống dây điện:**

| Code | Đống dây điện |
|------|--------------|
| `edges.sort()` | Sắp dây từ ngắn đến dài |
| `uf.union(u, v)` returns `true` | 2 nhà khác nhóm → nối dây |
| `uf.union(u, v)` returns `false` | 2 nhà cùng nhóm → bỏ dây |
| `mst.len() == n - 1` → `break` | Đủ dây rồi, bỏ đống còn lại |

---

## Kruskal trong thực tế

### a) Lắp đặt cáp mạng (Network Cable Installation)

```
Bài toán: Nối N offices bằng cáp mạng → tổng chiều dài cáp nhỏ nhất

  Liệt kê TẤT CẢ possible connections + cost
  Sort by cost → lấy cheapest → Kruskal
  Natural fit vì input = edge list (danh sách "nối office A-B tốn X đồng")
```

Đây là ứng dụng **đúng nghĩa đen** của ẩn dụ "đống dây điện" -- bạn có danh sách cáp mạng với giá khác nhau, sort chúng, lấy cheapest trước!

### b) Image Segmentation (Thuật toán Felzenszwalb)

```
Pixel = node
Edge weight = mức chênh lệch màu giữa 2 pixel cạnh nhau

Kruskal process edges from small → large:
  Merge similar pixels (chênh lệch nhỏ) vào cùng region
  Stop khi regions đủ khác biệt
= thuật toán phân vùng ảnh hiệu quả

Ứng dụng: computer vision, y tế (phân vùng MRI), tự lái xe (nhận diện vật thể)
```

### c) Clustering bằng MST

Đây có lẽ là ứng dụng **hay nhất**:

```
Build MST → xóa K-1 cạnh nặng nhất → K clusters

Kruskal đặc biệt natural cho bài này:
  Thay vì dừng khi V-1 edges (full MST)
  → Dừng khi V-K edges (K clusters)

Ví dụ: 100 điểm dữ liệu, muốn 3 clusters:
  Kruskal nối dần... dừng khi còn 3 nhóm (V-3 = 97 edges)
  3 nhóm còn lại = 3 clusters!
```

Cách này gọi là **single-linkage clustering**. Ý tưởng: điểm gần nhau được nối bằng cạnh nhẹ. Cạnh nặng = khoảng cách lớn giữa 2 nhóm → cắt đó!

---

## So sánh Prim vs Kruskal

| Tính năng | Prim | Kruskal |
|---|---|---|
| Cách tiếp cận | Mở rộng 1 cây | Gộp nhiều rừng |
| Ẩn dụ | Vết dầu loang | Đống dây điện, chọn ngắn nhất |
| Cấu trúc dữ liệu | Priority queue | Union-Find |
| Thời gian | O(E log V) | O(E log E) |
| Phù hợp với | Graph dày | Graph thưa |
| Bắt đầu từ | 1 đỉnh | Cạnh ngắn nhất toàn cục |
| Property dùng | Cut Property | Cycle Property |

Thực tế, cả hai chạy tốc độ tương tự. Kruskal thường được ưa thích khi cạnh đã được sắp sẵn hoặc graph cho dưới dạng danh sách cạnh.

---

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Sắp xếp cạnh | O(E log E) = O(E log V) |
| Thao tác Union-Find | O(E * α(V)) ≈ O(E) |
| Tổng thời gian | O(E log E) |
| Bộ nhớ | O(V + E) |

Chi phí chủ yếu là sắp xếp cạnh. Union-Find gần như miễn phí nhờ path compression và union by rank.

**Tại sao O(E log E) = O(E log V)?** Vì E ≤ V², nên log E ≤ log V² = 2 log V. Hệ số 2 bỏ trong Big-O → O(E log E) = O(E log V).

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

let mut g = Graph::new(4, false);
g.add_edge(0, 1, 1);
g.add_edge(0, 2, 3);
g.add_edge(1, 3, 2);
g.add_edge(2, 3, 4);

let mst = g.kruskal_mst();
let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
assert_eq!(total, 6);     // giống Prim!
assert_eq!(mst.len(), 3);

// Prim và Kruskal cho cùng tổng trọng số MST
let prim_total: i64 = g.prim_mst().iter().map(|&(_, _, w)| w).sum();
assert_eq!(total, prim_total);
```

**Graph 6 nodes** (từ trace 2):
```rust
let mut g = Graph::new(6, false);
g.add_edge(0, 1, 4);
g.add_edge(0, 2, 2);
g.add_edge(1, 2, 5);
g.add_edge(1, 3, 10);
g.add_edge(2, 3, 3);
g.add_edge(2, 4, 8);
g.add_edge(3, 4, 1);
g.add_edge(3, 5, 7);
g.add_edge(4, 5, 6);

let mst = g.kruskal_mst();
let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
assert_eq!(total, 16);    // 1+2+3+4+6
assert_eq!(mst.len(), 5); // V-1 = 6-1 = 5 cạnh
```

---

## Những cái bẫy hay gặp

### a) Quên xử lý duplicate edges (undirected graph)

❌ Graph vô hướng: cạnh 0-1 xuất hiện ở `adj[0]` VÀ `adj[1]`. Nếu lấy tất cả → sort 2E edges thay vì E, chậm 2x và có thể add duplicate MST edges.

✅ Filter `u < v` khi collect edges:
```rust
if self.directed || u < v {
    edges.push((w, u, v));
}
```

💡 Luôn nhớ: graph vô hướng, mỗi cạnh xuất hiện 2 lần trong adjacency list. Lọc 1 lần bằng `u < v`.

### b) Không implement path compression trong Union-Find

❌ `find()` không có path compression → O(log n) mỗi lần gọi

✅ Path compression -- trỏ thẳng lên root:
```rust
fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]); // ← dòng này!
    }
    self.parent[x]
}
```

💡 Với 1M edges, không có path compression → chậm đáng kể. O(log n) vs O(α(n)) ≈ O(1). Luôn dùng path compression.

### c) Quên check graph connected

❌ Chạy Kruskal xong, tin kết quả là MST mà không check

✅ Check `mst.len() == V - 1` sau khi chạy:
```rust
let mst = g.kruskal_mst();
if mst.len() < n - 1 {
    println!("Graph disconnected! Chỉ tìm được minimum spanning FOREST.");
}
```

💡 Nếu graph disconnected, Kruskal tìm minimum spanning **FOREST** (nhiều trees riêng lẻ). Đây có thể đúng hoặc sai tùy bài toán -- quan trọng là bạn **biết** điều này đang xảy ra.

### d) Dùng Kruskal khi input là adjacency list + dense graph

❌ Cần convert adjacency list → edge list + sort O(E log E). Nếu E ≈ V², sort O(V² log V²) = O(V² · 2 log V)

✅ Dùng Prim cho dense graph: O(E log V) = O(V² log V) -- nhanh hơn

💡 Rule of thumb: **edge list → Kruskal, adjacency list + dense → Prim**. Nếu input đã là edge list và graph thưa, Kruskal win. Nếu input là adjacency list và graph dày, Prim win.

---

## Khi nào Kruskal vs Prim -- bảng quyết định

| Tình huống | Kruskal? | Prim? | Tại sao? |
|------------|---------|-------|----------|
| Input = edge list | ✅ | ⚠️ | Kruskal sort edges natural |
| Input = adjacency list | ⚠️ | ✅ | Prim iterate neighbors |
| Sparse graph (E ≈ V) | ✅ | ⚠️ | Sort E << V² edges, nhanh |
| Dense graph (E ≈ V²) | ⚠️ | ✅ | Prim PQ better for dense |
| Need K clusters (stop early) | ✅ | ⚠️ | Dừng tại V-K edges, tự nhiên |
| Parallel / distributed | ✅ | ⚠️ | Sort easily parallelizable |
| Edge list pre-sorted | ✅ | ⚠️ | Skip sort → O(E·α(V)) ≈ O(E) |
| Phỏng vấn | ✅ | ⚠️ | Easier to explain + shorter code |
| Directed graph | ❌ | ❌ | Cần Edmonds' algorithm |
| Negative weights | ✅ | ✅ | Cả hai handle OK (MST khác shortest path) |

Tóm lại:
- **Dense graph, adjacency list** → Prim
- **Sparse graph, edge list, clustering** → Kruskal
- **Phỏng vấn, không chắc** → Kruskal (code ngắn, giải thích dễ)

---

## Luyện nhận diện Pattern

### Bài 1: Min Cost to Connect All Points (LeetCode #1584)

N points trên 2D plane, cost nối 2 points = Manhattan distance `|x1-x2| + |y1-y2|`. Tìm min cost nối tất cả points.

**Gợi ý:** Generate all edges = O(N²) pairs, mỗi pair có cost = Manhattan distance. Sort → Kruskal. Nhưng N² edges là **dense graph** → Prim có thể nhanh hơn. Cả hai đều accepted trên LeetCode.

### Bài 2: Number of Operations to Make Network Connected (LeetCode #1319)

N computers, connections list. Tìm min moves to connect all computers.

**Gợi ý:** Không cần MST! Chỉ cần đếm **connected components** bằng Union-Find. Answer = `components - 1` nếu đủ edges (≥ N-1), else -1. Union-Find là tool chính -- giống cách Kruskal dùng nó.

### Bài 3: Accounts Merge (LeetCode #721)

`accounts = [[name, email1, email2...]]`. Merge accounts cùng owner (có chung ≥1 email).

**Gợi ý:** Union-Find trên email addresses. Duyệt mỗi account, `union()` tất cả emails trong account đó. Kruskal-like thinking: "nối" emails thuộc cùng account, rồi group by representative.

---

## Rust Ecosystem

- `petgraph::algo::min_spanning_tree` -- Kruskal's (default MST algorithm trong `petgraph`)
- `petgraph::unionfind::UnionFind` -- built-in Union-Find
- `Vec::sort()` trong Rust dùng **pdqsort** (pattern-defeating quicksort) -- O(n log n) worst case, rất nhanh trong thực tế
- `Vec::sort_unstable()` -- nhanh hơn nữa, không cần stable order (perfect cho edges vì ta không care thứ tự cạnh cùng weight)

---

## Chương tiếp theo

Kruskal dùng Union-Find như black box -- "cùng nhóm chưa?" + "gộp 2 nhóm". Chương tiếp theo sẽ deep dive vào **Topological Sort** -- sắp xếp thứ tự trong directed graph. Còn **Union-Find** (Disjoint Set Union) sẽ được explore kỹ ở chương riêng -- data structure cho phép find + union trong gần O(1). Path compression + union by rank là 2 kỹ thuật tạo nên performance gần thần kỳ này. Union-Find là tool powerful cho mọi bài toán connectivity -- từ MST đến cycle detection đến network components.

---

---

[← Prim](./07-prim.md) | [Topological Sort →](./09-topological-sort.md)
