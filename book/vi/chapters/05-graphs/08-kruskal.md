# Thuật toán Kruskal (Cây khung nhỏ nhất)

## Đây là gì?

Cùng bài toán nối điện cho làng, nhưng **cách làm khác** so với Prim.

**Prim** bắt đầu từ 1 nhà, mở rộng dần (giống vết dầu loang).
**Kruskal** thì nhìn tất cả đoạn dây, **chọn đoạn ngắn nhất** trước, nối nếu không tạo vòng lặp.

Hãy tưởng tượng bạn có một **đống dây điện** với chiều dài khác nhau. Bạn sắp xếp từ ngắn đến dài. Rồi lần lượt lấy từng sợi: nếu nối 2 nhà chưa cùng nhóm → nối. Nếu nối 2 nhà đã cùng nhóm → bỏ (vì sẽ tạo vòng lặp, lãng phí dây).

Kruskal dùng **Union-Find** để kiểm tra nhanh: "2 nhà này đã cùng nhóm chưa?"

## Hoạt động như thế nào?

### Từng bước trên graph nhỏ

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

### Tại sao cần Union-Find?

Câu hỏi then chốt: "2 đỉnh này đã kết nối chưa?"

Cách đơn giản: duyệt cây mỗi lần → O(V). Chậm!
Cách thông minh: dùng **Union-Find** → gần như O(1) mỗi lần. Xem chi tiết ở chương Union-Find.

```
Union-Find quản lý các nhóm:

Ban đầu:     {0}  {1}  {2}  {3}     ← mỗi nhà 1 nhóm
Sau nối 0-1: {0,1}  {2}  {3}
Sau nối 1-3: {0,1,3}  {2}
Sau nối 0-2: {0,1,2,3}              ← tất cả cùng nhóm

Hỏi: "0 và 3 cùng nhóm?" → Union-Find trả lời O(1): "CÓ!"
```

### So sánh Prim vs Kruskal

| Tính năng | Prim | Kruskal |
|---|---|---|
| Cách tiếp cận | Mở rộng 1 cây | Gộp nhiều rừng |
| Cấu trúc dữ liệu | Priority queue | Union-Find |
| Thời gian | O(E log V) | O(E log E) |
| Phù hợp với | Graph dày | Graph thưa |
| Bắt đầu từ | 1 đỉnh | Cạnh ngắn nhất toàn cục |

Thực tế, cả hai chạy tốc độ tương tự. Kruskal thường được ưa thích khi cạnh đã được sắp sẵn hoặc graph cho dưới dạng danh sách cạnh.

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

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Sắp xếp cạnh | O(E log E) = O(E log V) |
| Thao tác Union-Find | O(E * alpha(V)) ~ O(E) |
| Tổng thời gian | O(E log E) |
| Bộ nhớ | O(V + E) |

Chi phí chủ yếu là sắp xếp cạnh. Union-Find gần như miễn phí nhờ path compression và union by rank.

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
