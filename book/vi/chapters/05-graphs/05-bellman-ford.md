# Thuật toán Bellman-Ford

## Đây là gì?

Dijkstra như Google Maps bình thường -- chỉ xử lý được đường có chi phí dương (khoảng cách, thời gian). Nhưng đời thực có lúc **đi một đoạn lại được tiền**.

Hãy tưởng tượng bạn giao hàng giữa các thành phố. Hầu hết chặng đường tốn xăng (chi phí dương). Nhưng có chặng bạn **chở hàng thuê dọc đường, tiền công nhận được nhiều hơn tiền xăng** -- tổng chi phí chặng đó thành **âm**. Đó chính là **trọng số âm** (negative weight). Dijkstra không xử lý được, nhưng **Bellman-Ford** thì có thể.

**Bellman-Ford** giải cùng bài toán với Dijkstra (đường ngắn nhất từ 1 nguồn), nhưng thêm 2 khả năng:
1. Xử lý **trọng số âm**
2. Phát hiện **negative cycle** (chu trình âm)

**Negative cycle** là gì? Là một vòng lặp mà đi quanh 1 vòng lại **giảm chi phí**. Giống như có 3 thành phố mà cứ chạy vòng A->B->C->A, mỗi vòng bạn lại lãi thêm tiền -- chạy mãi, lãi mãi, không bao giờ dừng. Khi có negative cycle, **không tồn tại đường ngắn nhất**.

Cái giá phải trả: Bellman-Ford chậm hơn Dijkstra. O(V * E) vs O((V + E) log V).

## Hoạt động như thế nào?

### Ý tưởng cốt lõi

Rất đơn giản: **thử cải thiện mọi cạnh**, lặp lại V-1 lần.

Tại sao V-1 lần? Vì đường đi ngắn nhất đơn giản (không lặp đỉnh) có tối đa V-1 cạnh. Mỗi vòng lặp "nới lỏng" (relax) thêm ít nhất 1 cạnh trên đường ngắn nhất.

### Từng bước trên graph nhỏ

```
Graph (có hướng, có trọng số âm):
  0 --(1)--> 1 --(-3)--> 2 --(2)--> 3
  0 --(4)--> 2

Các cạnh: (0,1,1), (1,2,-3), (0,2,4), (2,3,2)
             ↑              ↑ giảm giá!
```

```
Khởi tạo:  dist = [0, INF, INF, INF]

Vòng 1 (xét tất cả cạnh):
  Cạnh (0→1, w=1):  dist[1] = min(INF, 0+1)  = 1
  Cạnh (1→2, w=-3): dist[2] = min(INF, 1-3)  = -2   ← giảm giá!
  Cạnh (0→2, w=4):  dist[2] = min(-2, 0+4)   = -2   (không đổi)
  Cạnh (2→3, w=2):  dist[3] = min(INF, -2+2) = 0
  dist = [0, 1, -2, 0]

Vòng 2 (xét lại tất cả cạnh):
  Không có gì thay đổi → đã hội tụ!

Vòng 3:
  Không đổi.
```

Kết quả: `dist = [0, 1, -2, 0]`

Đường 0→1→2 có chi phí 1+(-3) = -2. Rẻ hơn đường 0→2 trực tiếp (chi phí 4)!

### Phát hiện negative cycle

Sau V-1 vòng, chạy **thêm 1 vòng nữa**. Nếu vẫn cải thiện được khoảng cách nào đó, có nghĩa là tồn tại negative cycle.

```
Ví dụ negative cycle:
  0 --(1)--> 1 --(-1)--> 2 --(-1)--> 0
                                       ↑
  Trọng số vòng: 1 + (-1) + (-1) = -1 < 0

  Đi 1 vòng: chi phí giảm 1
  Đi 2 vòng: chi phí giảm 2
  Đi 1000 vòng: chi phí giảm 1000
  ...vô tận!

  → Sau V-1 vòng, vòng thứ V vẫn cải thiện
  → Phát hiện negative cycle → trả về None
```

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub fn bellman_ford(&self, start: usize) -> Option<Vec<i64>> {
    let n = self.num_vertices;
    let mut dist = vec![i64::MAX; n];
    dist[start] = 0;

    // Thu thập tất cả cạnh
    let edges: Vec<(usize, usize, i64)> = (0..n)
        .flat_map(|u| self.adjacency_list[u].iter().map(move |&(v, w)| (u, v, w)))
        .collect();

    // Relax V-1 lần
    for _ in 0..n.saturating_sub(1) {
        for &(u, v, w) in &edges {
            if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }

    // Kiểm tra negative cycle
    for &(u, v, w) in &edges {
        if dist[u] != i64::MAX && dist[u] + w < dist[v] {
            return None; // có negative cycle!
        }
    }

    Some(dist)
}
```

Giải thích:

- `dist[u] != i64::MAX` -- kiểm tra trước khi relax để tránh tràn số. Nếu u chưa đến được, không relax cạnh từ u
- V-1 vòng relax đảm bảo tìm đường ngắn nhất qua tối đa V-1 cạnh = mọi đường đi đơn giản
- Vòng kiểm tra cuối cùng: nếu vẫn relax được → negative cycle → trả về `None`

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V * E) |
| Bộ nhớ | O(V + E) |
| Xử lý trọng số âm? | Có |
| Phát hiện negative cycle? | Có |

### So sánh Dijkstra vs Bellman-Ford

| Tính năng | Dijkstra | Bellman-Ford |
|---|---|---|
| Thời gian | O((V + E) log V) | O(V * E) |
| Trọng số âm | Không | Có |
| Negative cycle | Không phát hiện | Phát hiện được |
| Tốc độ thực tế | Nhanh hơn | Chậm hơn |

**Quy tắc:** Trọng số không âm → dùng Dijkstra. Có trọng số âm → dùng Bellman-Ford.

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// Graph có trọng số âm nhưng không có negative cycle
let mut g = Graph::new(4, true);
g.add_edge(0, 1, 1);
g.add_edge(1, 2, -3);   // "giảm giá" 3 đơn vị
g.add_edge(0, 2, 4);
g.add_edge(2, 3, 2);

let dist = g.bellman_ford(0).expect("không có negative cycle");
assert_eq!(dist, vec![0, 1, -2, 0]);
// 0→1→2 = 1+(-3) = -2, rẻ hơn 0→2 trực tiếp (=4)

// Graph có negative cycle
let mut g2 = Graph::new(3, true);
g2.add_edge(0, 1, 1);
g2.add_edge(1, 2, -1);
g2.add_edge(2, 0, -1);  // vòng lặp: 1+(-1)+(-1) = -1 < 0
assert!(g2.bellman_ford(0).is_none()); // không có lời giải!
```
