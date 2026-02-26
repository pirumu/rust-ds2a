# Thuật toán Prim (Cây khung nhỏ nhất)

## Đây là gì?

Hãy tưởng tượng bạn là **kỹ sư điện** cần nối điện cho tất cả các nhà trong một ngôi làng. Mỗi nhà cần được kết nối vào mạng lưới điện. Kéo dây giữa hai nhà tốn chi phí khác nhau (tùy khoảng cách, địa hình). Bạn muốn **nối tất cả các nhà** với **chi phí thấp nhất**.

Đó chính là bài toán **Minimum Spanning Tree** (MST -- cây khung nhỏ nhất).

**Thuật toán Prim** giải bài này bằng cách: bắt đầu từ 1 nhà, nối đến **nhà gần nhất** (rẻ nhất), rồi từ mạng lưới hiện tại, nối tiếp đến nhà gần nhất chưa nối, cứ mở rộng dần cho đến khi tất cả nhà đều có điện.

**MST** là gì?
- **Spanning** (khung): nối tất cả các đỉnh
- **Tree** (cây): không có vòng lặp, đúng V-1 cạnh
- **Minimum** (nhỏ nhất): tổng trọng số các cạnh nhỏ nhất có thể

## Hoạt động như thế nào?

### Từng bước trên graph nhỏ

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

### Tại sao Prim đúng?

Nhờ **tính chất lát cắt** (cut property): Chia các đỉnh thành 2 nhóm -- "đã trong MST" và "chưa trong MST". Cạnh rẻ nhất nối 2 nhóm này **chắc chắn** thuộc MST nào đó. Prim luôn chọn cạnh rẻ nhất nối 2 nhóm, nên luôn đúng.

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

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian (binary heap) | O(E log V) |
| Thời gian (Fibonacci heap) | O(E + V log V) |
| Bộ nhớ | O(V + E) |

Mỗi cạnh được push vào heap tối đa 1 lần. Mỗi thao tác heap mất O(log V). Tổng: O(E log V).

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
```
