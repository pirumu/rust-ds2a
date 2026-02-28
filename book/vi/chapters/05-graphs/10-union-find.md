# Union-Find (Tập hợp rời rạc)

## Đây là gì?

> **Đừng lo!** Union-Find là data structure có **tỷ lệ power/complexity cao nhất** trong toàn bộ series này. Chỉ 1 mảng `parent[]`, 2 function (`find` mỗi cái 3 dòng, `union` mỗi cái 8 dòng), performance gần O(1). Bạn đã dùng nó trong Kruskal (chương trước) -- giờ hiểu sâu hơn thôi. Không có recursion phức tạp, không cần tree rotation như AVL/Red-Black, không cần hash function. Chỉ cần hiểu **"mỗi node trỏ về cha, root trỏ về chính mình"**. Union-Find xuất hiện **cực nhiều** trong phỏng vấn (Number of Islands variant, Accounts Merge, Redundant Connection) và production (network connectivity, image processing, social networks).

Union-Find trông như một cấu trúc "mới hoàn toàn", nhưng thực ra nó chỉ là **một mảng** -- `parent[i]` cho biết "cha" của phần tử `i`. Hai thao tác `find` và `union` mỗi cái chỉ vài dòng code. Đây là một trong những cấu trúc có **tỷ lệ sức mạnh / độ phức tạp code** cao nhất mà bạn sẽ học.

Hãy tưởng tượng trường học có nhiều **nhóm bạn**. Ban đầu ai cũng riêng lẻ. Khi 2 người kết bạn, nhóm của họ **hợp nhất** thành 1.

Bạn cần trả lời nhanh 2 câu hỏi:
1. **"An và Bình có cùng nhóm không?"** → thao tác **Find** (tìm)
2. **"Gộp nhóm của An và nhóm của Bình lại"** → thao tác **Union** (hợp)

Đó chính là **Union-Find** (còn gọi là **Disjoint Set Union** -- DSU -- cấu trúc tập hợp rời rạc).

Tại sao quan trọng? Union-Find là xương sống của thuật toán Kruskal (tìm MST). Nó trả lời "2 đỉnh có cùng thành phần liên thông không?" trong gần O(1) -- nhanh không tưởng.

Với 2 tối ưu hóa (**path compression** + **union by rank**), mỗi thao tác mất O(α(n)) -- α là hàm ngược Ackermann, luôn ≤ 4 với mọi n thực tế. Gần như O(1).

---

## Tại sao cần Union-Find khi đã có BFS/DFS?

Bạn đã biết BFS/DFS có thể check "A và B connected không?" bằng cách traverse từ A, xem có đến được B không. Vậy tại sao cần Union-Find?

**Bài toán:** Dynamic graph -- edges thêm dần theo thời gian, không bao giờ xóa. Liên tục có queries "A và B connected không?"

```
Cách 1: BFS/DFS mỗi query → O(V+E) per query
  1M queries × graph 1M nodes = O(10^12) operations. Chậm!

Cách 2: Union-Find → O(α(n)) ≈ O(1) per query
  1M queries = ~4M operations. Gần instant!
```

Hãy nghĩ như thế này: BFS/DFS giống như mỗi lần muốn biết "An và Bình cùng nhóm không?", bạn phải **đi hỏi từng người một** cho đến khi tìm thấy. Union-Find giống như mỗi nhóm có **1 trưởng nhóm** -- hỏi trưởng nhóm là biết ngay.

```
Trade-off:
  BFS/DFS: general purpose (traverse, shortest path, connected components...)
  Union-Find: CHỈ answer "connected?" nhưng NHANH hơn rất nhiều

Union-Find WIN khi:
  ✅ Chỉ cần "connected or not" (không cần biết path)
  ✅ Edges thêm dần (union operations)
  ✅ Nhiều queries liên tục
  ✅ Kruskal MST (check cycle = check connected)

BFS/DFS WIN khi:
  ✅ Cần tìm actual path A→B
  ✅ Cần traverse toàn bộ graph
  ✅ Static graph, chỉ 1 query
  ✅ Cần shortest path
```

Nhớ lại Kruskal: mỗi khi xét 1 cạnh, ta hỏi "2 đỉnh này đã connected chưa?" Nếu dùng BFS mỗi lần → O(V+E) per edge × E edges = O(E(V+E)). Dùng Union-Find → O(E·α(n)) ≈ O(E). Nhanh hơn **nhiều lần**.

---

## Hoạt động như thế nào?

### Biểu diễn bằng rừng cây

Mỗi tập hợp là một cây. Mỗi phần tử trỏ đến cha. Gốc cây là **đại diện** (representative) của tập hợp.

```
Ban đầu (5 người, mỗi người 1 nhóm):
  0   1   2   3   4        ← mỗi người tự làm gốc

Sau union(0, 1) -- An kết bạn với Bình:
  0       2   3   4
  |
  1                         ← nhóm {0,1}, gốc là 0

Sau union(2, 3) -- Cường kết bạn với Dũng:
  0       2       4
  |       |
  1       3                 ← nhóm {0,1} và nhóm {2,3}

Sau union(0, 2) -- gộp 2 nhóm:
      0           4
     / \
    1   2                   ← nhóm {0,1,2,3}
        |
        3
```

**Find(3):** Đi từ 3 → 2 → 0. Gốc là 0.
**Find(1):** Đi từ 1 → 0. Gốc là 0.
**Cùng gốc → cùng nhóm!**

Theo dõi mảng `parent[]` qua từng bước:

```
Ban đầu:      parent = [0, 1, 2, 3, 4]   ← mỗi người trỏ đến chính mình

union(0, 1):  parent = [0, 0, 2, 3, 4]   ← parent[1] = 0

union(2, 3):  parent = [0, 0, 2, 2, 4]   ← parent[3] = 2

union(0, 2):  parent = [0, 0, 0, 2, 4]   ← parent[2] = 0  (gắn root 2 vào root 0)
```

### Path compression (nén đường đi)

Vấn đề: cây có thể dài, `find` phải đi nhiều bước. Giải pháp: khi `find(3)`, **nối thẳng** mọi đỉnh trên đường đi đến gốc.

```
Trước find(3):              Sau find(3):
      0                          0
     / \                       / | \
    1   2                     1  2  3
        |
        3                   ← 3 bây giờ nối thẳng đến gốc!

parent[3]: 2 → 0            ← path compression!
Lần sau find(3) chỉ cần 1 bước!
```

Giống như: thay vì hỏi "trưởng nhóm của bạn là ai?" rồi bạn hỏi tiếp trưởng nhóm, bạn **ghi nhớ luôn** ai là trưởng nhóm cao nhất → lần sau hỏi là biết ngay.

### Union by rank (hợp theo hạng)

Khi gộp 2 cây, gắn cây **thấp hơn** vào gốc cây **cao hơn**. "Rank" (hạng) là giới hạn trên của chiều cao cây.

```
rank 1:      rank 0:        Kết quả (cây thấp gắn dưới cây cao):
  0            2                0
  |                            / \
  1                           1   2

Nếu ngược lại (gắn cây cao dưới cây thấp):
    2
    |
    0         ← cây cao hơn không cần thiết!
    |
    1
```

Union by rank giữ cây **cân bằng**, tránh cây suy biến thành danh sách dài.

### Kết hợp 2 tối ưu hóa

```
Không tối ưu:      Chỉ path compression:   Cả hai:
  O(n) mỗi find     O(log n) amortized       O(α(n)) ~ O(1)

α(n) là hàm ngược Ackermann:
  α(10^80) = 4    ← số nguyên tử trong vũ trụ → vẫn chỉ là 4!
  Gần như hằng số.
```

---

## Union by Size (variant)

Doc bên trên dùng **union by rank** (gắn cây thấp dưới cây cao). Có 1 variant phổ biến: **union by size** -- gắn cây **ít phần tử** vào cây **nhiều phần tử**.

```
Union by Rank:                    Union by Size:
  Theo CHIỀU CAO (rank)            Theo SỐ LƯỢNG phần tử (size)
  rank[] track height bound        size[] track element count
```

```rust
// Union by SIZE (thay vì rank):
struct UnionFindBySize {
    parent: Vec<usize>,
    size: Vec<usize>,    // size thay vì rank
}

impl UnionFindBySize {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],    // mỗi nhóm ban đầu có 1 phần tử
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return false; }

        // Gắn cây NHỎ vào cây LỚN
        if self.size[rx] < self.size[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];  // cập nhật size
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        }
        true
    }

    // Bonus: biết ngay nhóm có bao nhiêu người!
    fn group_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}
```

Ví dụ nhóm bạn trường học:

```
Ban đầu: 5 người, mỗi người 1 nhóm
  size = [1, 1, 1, 1, 1]

union(0, 1): nhóm {0,1} có 2 người
  size = [2, 1, 1, 1, 1]      ← size[0] = 2

union(2, 3): nhóm {2,3} có 2 người
  size = [2, 1, 2, 1, 1]      ← size[2] = 2

union(0, 2): nhóm {0,1,2,3} có 4 người
  size = [4, 1, 2, 1, 1]      ← size[0] = 4

group_size(3) → find(3) = 0 → size[0] = 4
"Nhóm bạn của Dũng có 4 người!"
```

So sánh 2 variant:

| | Union by Rank | Union by Size |
|---|-------------|-------------|
| Extra array | `rank[]` (height bound) | `size[]` (element count) |
| Merge rule | Cây thấp gắn dưới cây cao | Cây nhỏ gắn vào cây lớn |
| Bonus info | -- | Biết **SIZE** mỗi nhóm! |
| Performance | O(α(n)) | O(α(n)) (giống nhau) |
| Dùng khi | Default, đơn giản | Cần biết group size |

**Chọn cái nào?** Nếu không cần biết nhóm có bao nhiêu phần tử → dùng rank (đơn giản hơn). Nếu cần biết size → dùng size (bonus thông tin miễn phí).

---

## Connected Components Counting

Một ứng dụng rất phổ biến: **đếm số nhóm (connected components)**. Chỉ cần thêm 1 biến `num_components`.

```
Ý tưởng:
  Ban đầu: N người, N nhóm riêng biệt
  Mỗi union thành công: giảm 1 nhóm (2 nhóm gộp thành 1)
  Sau K successful unions → N-K nhóm

  Không cần DFS/BFS để đếm! Chỉ cần track 1 biến.
```

```rust
struct UnionFindWithCount {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_components: usize,  // track số nhóm
}

impl UnionFindWithCount {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            num_components: n,  // ban đầu N nhóm
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return false; }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        self.num_components -= 1;  // mỗi union thành công giảm 1 nhóm!
        true
    }
}
```

Ví dụ nhóm bạn:

```
5 người, ban đầu 5 nhóm
  num_components = 5

union(0, 1) → true  → num_components = 4
union(2, 3) → true  → num_components = 3
union(0, 2) → true  → num_components = 2
union(0, 1) → false → num_components = 2  (đã cùng nhóm, không đổi!)

Kết quả: 2 nhóm -- {0,1,2,3} và {4}
```

**Khi nào dùng?** Bài toán "Number of Islands", "Number of Provinces", network partition detection -- bất cứ khi nào cần đếm connected components trong dynamic graph.

---

## Weighted Union-Find (nâng cao)

> Phần này nâng cao hơn -- bạn có thể bỏ qua nếu chỉ cần basic Union-Find.

Union-Find bình thường chỉ trả lời "A và B cùng nhóm không?" Weighted Union-Find lưu thêm **quan hệ giữa các phần tử** (weight/distance/ratio).

```
Bài toán: A nặng hơn B 3kg, B nặng hơn C 2kg. A nặng hơn C bao nhiêu?

Weighted Union-Find: mỗi edge lưu "weight difference to parent"

  A →(3)→ B →(2)→ C(root)
  find(A) = (C, 5)  -- A nặng hơn C tổng cộng 5kg

Tự động tích lũy weight dọc đường đi!
```

Ứng dụng:
- **LeetCode #399 Evaluate Division**: `a/b = 2.0, b/c = 3.0` → `a/c = ?` Answer: `6.0`
- Relative ranking/scoring systems
- Currency exchange rate chains

Ý tưởng core: `find(x)` không chỉ trả về root, mà còn trả về **tổng weight từ x đến root**. Path compression cũng phải cập nhật weight khi nén đường đi.

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // mỗi phần tử tự trỏ đến chính mình
            rank: vec![0; n],         // ban đầu tất cả rank = 0
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false; // đã cùng nhóm rồi
        }
        // Union by rank: gắn cây thấp dưới cây cao
        match self.rank[rx].cmp(&self.rank[ry]) {
            Ordering::Less => self.parent[rx] = ry,
            Ordering::Greater => self.parent[ry] = rx,
            Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
        true
    }
}
```

Giải thích:

- `parent`: Mảng trỏ đến cha. Ban đầu `parent[i] = i` (tự trỏ đến mình = mình là trưởng nhóm)
- `find`: Đệ quy tìm gốc. **Path compression** = gán `parent[x]` thẳng đến gốc, nén toàn bộ đường đi
- `union` trả về `false` nếu đã cùng nhóm -- rất hữu ích cho Kruskal (phát hiện vòng lặp)
- **Union by rank**: cây có rank nhỏ hơn gắn dưới cây có rank lớn hơn. Nếu bằng nhau, chọn 1 làm gốc và tăng rank

**Rust-specific:** `find(&mut self)` cần **mutable reference** vì path compression modify `parent[]`. Đây là design choice đúng đắn -- nếu dùng immutable find (không compress) thì performance chỉ O(log n) thay vì O(α(n)).

---

## Union-Find trong thực tế

### a) Social Network -- Connected Groups

```
Facebook: 2 tỷ users, "bạn của bạn" = connected component

  User A kết bạn B: union(A, B)
  "A và C có kết nối?" : find(A) == find(C)
  "Nhóm bạn A có bao nhiêu người?" : size[find(A)]
```

Tại sao Union-Find thay vì BFS?
- 2 tỷ users, trung bình 500 friends = graph khổng lồ
- BFS per query: O(V+E) = quá chậm cho real-time
- Union-Find per query: O(α(n)) ≈ O(1) = instant

Khi A kết bạn B → `union(A, B)`. Khi hỏi "A và C connected?" → `find(A) == find(C)`. Nhanh gọn.

### b) Image Processing -- Connected Component Labeling

```
Binary image (ảnh đen trắng):
  Pixel = 0 (background) hoặc 1 (object)
  Adjacent 1-pixels (kề nhau) → union chúng lại
  Mỗi connected component = 1 vật thể

  ░░░█░░        ░░░1░░
  ░░██░░   →    ░░11░░    → union adjacent 1s
  ░░░░██        ░░░░22       → 2 objects detected!

  Object 1: {(0,3), (1,2), (1,3)}
  Object 2: {(2,4), (2,5)}
  num_components of 1-pixels = 2
```

Ứng dụng: đếm tế bào trong microscope image, detect object trong ảnh satellite, OCR character segmentation.

### c) Network Connectivity

```
Servers connect dần trong data center:
  Server A nối Server B: union(A, B)
  "A reachable from B?" : find(A) == find(B)
  Network partition? : num_components > 1

  ┌───┐    ┌───┐    ┌───┐
  │ A │────│ B │    │ C │     ← 2 components: {A,B} và {C}
  └───┘    └───┘    └───┘       num_components = 2 → PARTITION!

  Nối A-C: union(A, C) → num_components = 1 → fully connected!
```

### d) Percolation (Physics Simulation)

```
N×N grid, mỗi cell open/closed randomly
"Top connected to bottom?" = Union-Find!

  Trick: thêm virtual_top (nối tất cả cell hàng đầu)
         và virtual_bottom (nối tất cả cell hàng cuối)

  Check: find(virtual_top) == find(virtual_bottom)

  ██░██        ← hàng đầu (nối virtual_top)
  █░░░█
  ██░██
  █░░░█
  ██░██        ← hàng cuối (nối virtual_bottom)

  ░ = open, █ = closed
  Nước có chảy từ trên xuống dưới được không?

Used in: material science, epidemiology (disease spread),
         game theory (Hex game winner detection)
```

---

## Độ phức tạp

| Thao tác | Thời gian amortized |
|---|---|
| find | O(α(n)) ~ O(1) |
| union | O(α(n)) ~ O(1) |
| Bộ nhớ | O(n) |

So sánh các mức tối ưu:

| Không tối ưu | Chỉ path compression | Path compression + union by rank |
|---|---|---|
| O(n) mỗi find | O(log n) amortized | O(α(n)) amortized |

**Ý nghĩa thực tế:** 1 triệu thao tác union/find trên 1 triệu phần tử? Chỉ mất khoảng 4 triệu phép tính. Nhanh như duyệt mảng!

---

## Ví dụ

```rust
use rust_ds2a::graph::UnionFind;

let mut uf = UnionFind::new(6);

// Ban đầu, mỗi phần tử là 1 nhóm riêng
assert_ne!(uf.find(0), uf.find(1));

// Kết bạn
uf.union(0, 1);   // nhóm {0,1}
uf.union(2, 3);   // nhóm {2,3}
uf.union(4, 5);   // nhóm {4,5}

// Cùng nhóm → cùng gốc
assert_eq!(uf.find(0), uf.find(1));
assert_eq!(uf.find(2), uf.find(3));
assert_ne!(uf.find(0), uf.find(2)); // khác nhóm

// Gộp 2 nhóm
uf.union(1, 3);  // gộp {0,1} và {2,3} → {0,1,2,3}
assert_eq!(uf.find(0), uf.find(3)); // giờ cùng nhóm!

// union trả về false nếu đã cùng nhóm
assert!(!uf.union(0, 2)); // 0 và 2 đã cùng nhóm {0,1,2,3}

// Ứng dụng: phát hiện vòng lặp cho Kruskal
let mut uf2 = UnionFind::new(4);
uf2.union(0, 1);   // cạnh 0-1: an toàn
uf2.union(1, 2);   // cạnh 1-2: an toàn
let would_cycle = !uf2.union(0, 2); // cạnh 0-2: tạo vòng!
assert!(would_cycle);
```

### Connected components counting (mở rộng)

```rust
// Đếm nhóm: thêm biến count, mỗi union thành công giảm 1
let mut uf = UnionFind::new(5);
let mut count = 5;  // ban đầu 5 nhóm

if uf.union(0, 1) { count -= 1; }  // count = 4
if uf.union(2, 3) { count -= 1; }  // count = 3
if uf.union(0, 2) { count -= 1; }  // count = 2
if uf.union(0, 1) { count -= 1; }  // false! count vẫn = 2

assert_eq!(count, 2);  // 2 nhóm: {0,1,2,3} và {4}
```

---

## Những cái bẫy hay gặp

### a) Quên path compression

❌ Viết `find` chỉ return root mà không nén đường đi:
```rust
fn find(&self, mut x: usize) -> usize {
    while self.parent[x] != x { x = self.parent[x]; }
    x  // tìm được root, nhưng không nén!
}
```

✅ Luôn thêm path compression -- chỉ 1 dòng code:
```rust
fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]); // NÉN!
    }
    self.parent[x]
}
```

💡 Không có path compression → worst case O(n) mỗi find (cây suy biến thành danh sách). Với path compression → O(α(n)) ≈ O(1). **1 dòng code, performance tăng dramatically.**

### b) Union bằng x, y thay vì find(x), find(y)

❌ Gắn trực tiếp node vào node:
```rust
fn union(&mut self, x: usize, y: usize) {
    self.parent[x] = y;  // SAI! Phải union ROOTS!
}
```

✅ Luôn tìm root trước khi union:
```rust
fn union(&mut self, x: usize, y: usize) -> bool {
    let rx = self.find(x);  // root của x
    let ry = self.find(y);  // root của y
    if rx == ry { return false; }
    self.parent[rx] = ry;   // union ROOTS
    true
}
```

💡 Union node thay vì root → cây bị sai, các phần tử "mất kết nối" với nhóm cũ. **Luôn `find` trước, union roots.**

### c) Dùng Union-Find khi cần delete/disconnect

❌ Cố xóa kết nối giữa 2 phần tử trong Union-Find

✅ Union-Find chỉ support **union** (merge). KHÔNG support **split** (tách). Nếu cần disconnect → cần structure khác hoặc offline trick.

💡 **Offline trick**: nếu biết trước toàn bộ operations, process **ngược** (reverse order). Delete trở thành add (union). Ví dụ: "xóa edge theo thứ tự 1,2,3" → process "thêm edge theo thứ tự 3,2,1".

### d) `find()` cần `&mut self` vì path compression

❌ Viết `find(&self, ...)` (immutable) → không thể path compress

✅ Viết `find(&mut self, ...)` (mutable) → path compression modify `parent[]`

💡 Đây là **Rust-specific insight** quan trọng. Path compression thay đổi internal state, nên cần mutable reference. Design choice:
- `find(&self)` -- immutable, không compress → O(log n). An toàn nhưng chậm.
- `find(&mut self)` -- mutable, compress → O(α(n)). **Luôn chọn cái này.**

Nếu cần gọi `find` trong context immutable (ví dụ khi iterate) → clone hoặc refactor.

---

## Khi nào dùng Union-Find?

| Tình huống | Union-Find? | Thay bằng gì? | Tại sao? |
|---|---|---|---|
| "A và B connected?" (dynamic graph) | ✅ | -- | O(α(n)) per query |
| Kruskal MST (cycle check) | ✅ | -- | Core requirement |
| Count connected components (dynamic) | ✅ | -- | Track `num_components` |
| Group/cluster membership | ✅ | -- | `find()` = group ID |
| Need actual path A→B | ❌ | BFS/DFS | UF chỉ biết "connected", không biết path |
| Need disconnect/split | ❌ | Other DS | UF chỉ union, không split |
| Static graph, 1 lần check | ⚠️ | BFS/DFS | 1 BFS đủ, UF overhead không cần thiết |
| Weighted relationships | ⚠️ | Weighted UF | Nâng cao, nhưng possible |
| Bipartite check | ⚠️ | BFS/DFS hoặc weighted UF | Possible nhưng tricky |

**Tín hiệu nhận diện trong phỏng vấn:** Bài toán có "group", "connected", "merge", "same set", "union", "disjoint" → rất có thể là Union-Find.

---

## Luyện nhận diện Pattern

### a) Number of Provinces (LeetCode #547)

> N cities, `isConnected[i][j] = 1` nếu city i và j connected trực tiếp. Hỏi: có bao nhiêu provinces (connected components)?

**Gợi ý:**
- Tạo Union-Find với N elements
- Duyệt matrix: nếu `isConnected[i][j] == 1` → `union(i, j)`
- Answer = `num_components` sau khi union hết
- Có thể dùng DFS/BFS, nhưng Union-Find cleaner -- không cần visited array, không cần adjacency list

### b) Redundant Connection (LeetCode #684)

> Tree có N nodes, N-1 edges. Nhưng input cho N edges (thừa 1). Tìm edge nào có thể xóa để thành tree.

**Gợi ý:**
- Process edges theo thứ tự
- Mỗi edge: `union(u, v)`
- Edge nào mà `union` return `false` = **2 đỉnh đã cùng nhóm** = edge này tạo cycle = **redundant!**
- Return edge đó

Đây chính xác là cách Kruskal detect cycle!

### c) Accounts Merge (LeetCode #721)

> `accounts = [["John", "john@mail", "john@work"], ["John", "john@work", "john2@mail"], ...]`. Merge accounts cùng owner (có chung email).

**Gợi ý:**
- Map mỗi email → account index (đầu tiên gặp)
- Nếu email đã gặp ở account khác → `union(current_account, previous_account)`
- Group by `find(account_index)` → merged accounts
- Union-Find xử lý transitive: nếu A share email với B, B share email với C → A, B, C cùng nhóm

---

## Rust Ecosystem

### Trong crates phổ biến

- **`petgraph::unionfind::UnionFind`** -- built-in trong crate `petgraph`, được dùng bởi `min_spanning_tree`
- **`union-find` crate** -- standalone implementation với nhiều variant

### Rust ownership & Union-Find

```rust
// find(&mut self) vì path compression mutates
// Nếu cần concurrent access:
//   - Wrap trong Mutex: Mutex<UnionFind>
//   - Hoặc dùng lock-free Union-Find (phức tạp hơn nhiều)

use std::sync::Mutex;
let uf = Mutex::new(UnionFind::new(100));
// Thread-safe nhưng serialize tất cả operations
```

### KaCrab (message broker in Rust)

Union-Find có thể dùng trong message broker:

- **Consumer group membership**: union consumers subscribe cùng topic, `find()` = group coordinator
- **Partition connectivity**: check tất cả replicas của partition X reachable từ nhau (network partition detection)
- **Broker cluster health**: union connected brokers, `num_components > 1` = **network split detected!**

```
Cluster: [Broker0, Broker1, Broker2]

Heartbeat:
  Broker0 ↔ Broker1: union(0, 1)
  Broker1 ↔ Broker2: union(1, 2)
  num_components = 1 → healthy!

Network issue:
  Broker2 unreachable → num_components = 2
  → ALERT: network partition!
```

---

## Tổng kết

Union-Find kết thúc phần **graph data structures và algorithms**. Bạn đã master toàn bộ toolkit:

- **Representations** (adjacency list/matrix)
- **Traversals** (BFS/DFS)
- **Shortest path** (Dijkstra/Bellman-Ford/Floyd-Warshall)
- **MST** (Prim/Kruskal)
- **Topological sort** (Kahn's/DFS)
- **Union-Find** (dynamic connectivity)

Đây là foundation vững chắc cho mọi bài graph trong phỏng vấn. Hầu hết bài graph trên LeetCode medium/hard đều build trên những building blocks này.

Beauty của Union-Find: **1 mảng + 2 functions = near O(1) connectivity**. Đôi khi giải pháp đơn giản nhất lại mạnh mẽ nhất.

---

[← Topological Sort](./09-topological-sort.md) | [Recursion →](../06-algorithms/01-recursion.md)
