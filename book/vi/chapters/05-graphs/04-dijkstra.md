# Thuật toán Dijkstra

## Đây là gì?

Hãy tưởng tượng bạn mở **Google Maps** để tìm đường từ nhà đến trường. Có nhiều đường đi khác nhau -- đường nào ngắn nhất? Đường nào nhanh nhất?

Đó chính là bài toán mà **thuật toán Dijkstra** giải quyết. Nó tìm **đường đi ngắn nhất** từ một điểm xuất phát đến tất cả các điểm khác trong graph có **trọng số không âm** (non-negative weights).

Tại sao BFS không đủ? BFS tìm đường ít cạnh nhất, nhưng không quan tâm trọng số. Ví dụ: đường đi qua 2 cạnh (trọng số 10 + 10 = 20) có thể dài hơn đường đi qua 5 cạnh (trọng số 1 + 1 + 1 + 1 + 1 = 5).

Ý tưởng cốt lõi của Dijkstra: **tham lam** (greedy). Luôn xử lý đỉnh gần nhất chưa xử lý. Đỉnh đó chắc chắn đã có khoảng cách ngắn nhất rồi, vì mọi đường khác phải đi qua đỉnh xa hơn (trọng số không âm → chỉ có thể dài thêm).

## Hoạt động như thế nào?

### Từng bước trên graph nhỏ

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

### Priority queue (hàng đợi ưu tiên)

Để tìm nhanh "đỉnh gần nhất chưa xử lý", ta dùng **min-heap** (đống nhỏ nhất). Thay vì quét tất cả đỉnh (O(V)), min-heap cho ta đỉnh nhỏ nhất trong O(log V).

```
Min-heap hoạt động như phòng khám ưu tiên:
  Bệnh nhân nặng nhất (khoảng cách nhỏ nhất) được khám trước
  ┌─────────┐
  │ (2, v2) │  ← đỉnh v2, khoảng cách 2 → xử lý trước!
  │ (4, v1) │
  │ (5, v3) │
  └─────────┘
```

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
            continue; // entry cũ, bỏ qua
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

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian (binary heap) | O((V + E) log V) |
| Thời gian (Fibonacci heap) | O(V log V + E) |
| Bộ nhớ | O(V + E) |
| Xử lý trọng số âm? | Không! |

Mỗi đỉnh xử lý 1 lần, mỗi cạnh gây tối đa 1 lần push vào heap. Mỗi thao tác heap mất O(log V). Tổng: O((V + E) log V).

**Ý nghĩa thực tế:** Google Maps dùng biến thể của Dijkstra để tìm đường cho hàng triệu truy vấn mỗi ngày.

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

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
```
