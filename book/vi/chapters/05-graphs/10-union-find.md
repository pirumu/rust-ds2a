# Union-Find (Tập hợp rời rạc)

## Đây là gì?

Hãy tưởng tượng trường học có nhiều **nhóm bạn**. Ban đầu ai cũng riêng lẻ. Khi 2 người kết bạn, nhóm của họ **hợp nhất** thành 1.

Bạn cần trả lời nhanh 2 câu hỏi:
1. **"An và Bình có cùng nhóm không?"** → thao tác **Find** (tìm)
2. **"Gộp nhóm của An và nhóm của Bình lại"** → thao tác **Union** (hợp)

Đó chính là **Union-Find** (còn gọi là **Disjoint Set Union** -- DSU -- cấu trúc tập hợp rời rạc).

Tại sao quan trọng? Union-Find là xương sống của thuật toán Kruskal (tìm MST). Nó trả lời "2 đỉnh có cùng thành phần liên thông không?" trong gần O(1) -- nhanh không tưởng.

Với 2 tối ưu hóa (**path compression** + **union by rank**), mỗi thao tác mất O(alpha(n)) -- alpha là hàm ngược Ackermann, luôn <= 4 với mọi n thực tế. Gần như O(1).

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

### Path compression (nén đường đi)

Vấn đề: cây có thể dài, `find` phải đi nhiều bước. Giải pháp: khi `find(3)`, **nối thẳng** mọi đỉnh trên đường đi đến gốc.

```
Trước find(3):              Sau find(3):
      0                          0
     / \                       / | \
    1   2                     1  2  3
        |
        3                   ← 3 bây giờ nối thẳng đến gốc!

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
  O(n) mỗi find     O(log n) amortized       O(alpha(n)) ~ O(1)

alpha(n) là hàm ngược Ackermann:
  alpha(10^80) = 4    ← số nguyên tử trong vũ trụ → vẫn chỉ là 4!
  Gần như hằng số.
```

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

- `parent`: Mảng trỏ đến cha. Ban đầu `parent[i] = i` (tự trỏ đến mình)
- `find`: Đệ quy tìm gốc. **Path compression** = gán `parent[x]` thẳng đến gốc, nén toàn bộ đường đi
- `union` trả về `false` nếu đã cùng nhóm -- rất hữu ích cho Kruskal (phát hiện vòng lặp)
- **Union by rank**: cây có rank nhỏ hơn gắn dưới cây có rank lớn hơn. Nếu bằng nhau, chọn 1 làm gốc và tăng rank

## Độ phức tạp

| Thao tác | Thời gian amortized |
|---|---|
| find | O(alpha(n)) ~ O(1) |
| union | O(alpha(n)) ~ O(1) |
| Bộ nhớ | O(n) |

So sánh các mức tối ưu:

| Không tối ưu | Chỉ path compression | Path compression + union by rank |
|---|---|---|
| O(n) mỗi find | O(log n) amortized | O(alpha(n)) amortized |

**Ý nghĩa thực tế:** 1 triệu thao tác union/find trên 1 triệu phần tử? Chỉ mất khoảng 4 triệu phép tính. Nhanh như duyệt mảng!

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
