# Tìm kiếm theo chiều rộng (Breadth-First Search - BFS)

## Đây là gì?

Hãy tưởng tượng bạn **lan truyền một tin đồn**. Bạn kể cho 3 người bạn thân. Rồi mỗi người bạn đó kể cho bạn bè của họ. Rồi bạn bè của bạn bè lại kể tiếp. Tin đồn lan ra theo **từng vòng** -- vòng 1 là bạn bè trực tiếp, vòng 2 là bạn của bạn, vòng 3 là bạn của bạn của bạn...

Đó chính là cách **BFS** (Breadth-First Search -- tìm kiếm theo chiều rộng) hoạt động. Nó khám phá graph **theo từng lớp**, từ gần đến xa.

Tại sao cần BFS? Vì BFS tự nhiên tìm được **đường đi ngắn nhất** (ít cạnh nhất) trong graph không trọng số. Giống như tin đồn đến tai ai đầu tiên = con đường ngắn nhất từ bạn đến người đó.

BFS dùng **queue** (hàng đợi) -- ai đến trước phục vụ trước, giống xếp hàng mua vé.

## Hoạt động như thế nào?

### Từng bước một

Cho graph sau, bắt đầu từ đỉnh 0:

```
       0          ← Vòng 0: bắt đầu
      / \
     1   2        ← Vòng 1: bạn bè của 0
    / \   \
   3   4   5      ← Vòng 2: bạn của bạn
```

**Quá trình thực hiện (theo dõi queue):**

```
Bước 0: Thăm 0       Queue: [1, 2]           Thứ tự: [0]
  ↳ 0 kể cho 1 và 2

Bước 1: Thăm 1       Queue: [2, 3, 4]        Thứ tự: [0, 1]
  ↳ 1 kể cho 3 và 4

Bước 2: Thăm 2       Queue: [3, 4, 5]        Thứ tự: [0, 1, 2]
  ↳ 2 kể cho 5

Bước 3: Thăm 3       Queue: [4, 5]           Thứ tự: [0, 1, 2, 3]
  ↳ 3 không có ai mới để kể

Bước 4: Thăm 4       Queue: [5]              Thứ tự: [0, 1, 2, 3, 4]

Bước 5: Thăm 5       Queue: []               Thứ tự: [0, 1, 2, 3, 4, 5]
```

Chú ý: Tất cả đỉnh ở vòng 1 (đỉnh 1, 2) được thăm **trước** mọi đỉnh ở vòng 2 (đỉnh 3, 4, 5). Đó là bản chất của BFS -- **hết lớp này rồi mới sang lớp tiếp**.

### Thuật toán

1. Đánh dấu đỉnh bắt đầu là "đã thăm", đưa vào queue
2. Lặp cho đến khi queue rỗng:
   a. Lấy đỉnh `v` ra khỏi queue (đầu hàng đợi)
   b. Với mỗi hàng xóm `u` chưa thăm của `v`:
      - Đánh dấu `u` là "đã thăm"
      - Đưa `u` vào queue (cuối hàng đợi)

```
Queue hoạt động giống xếp hàng mua vé:
  ┌───┬───┬───┬───┐
  │ 1 │ 2 │ 3 │ 4 │  → Vào cuối, ra đầu (FIFO)
  └───┴───┴───┴───┘
  ↑ ra                ↑ vào
```

### Tại sao BFS tìm đường ngắn nhất?

BFS thăm đỉnh **theo thứ tự khoảng cách** từ gần đến xa. Lần đầu tiên BFS gặp một đỉnh, đó chắc chắn là con đường ngắn nhất (tính theo số cạnh). DFS không có tính chất này.

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
use std::collections::VecDeque;

pub fn bfs(&self, start: usize) -> Vec<usize> {
    let mut visited = vec![false; self.num_vertices];
    let mut order = Vec::new();
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &(neighbor, _) in &self.adjacency_list[v] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    order
}
```

Giải thích:

- `visited`: Mảng đánh dấu ai đã nghe tin đồn rồi. Tránh kể lại cho người đã biết
- Đánh dấu "đã thăm" **khi đưa vào queue**, không phải khi lấy ra. Tại sao? Để tránh thêm cùng một đỉnh vào queue nhiều lần
- `_` bỏ qua trọng số cạnh -- BFS coi mọi cạnh như nhau
- `VecDeque` cho phép thêm cuối / lấy đầu đều O(1)

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V + E) |
| Bộ nhớ | O(V) cho mảng visited + queue |
| Tìm đường ngắn nhất (không trọng số)? | Có |

Mỗi đỉnh vào queue đúng 1 lần, ra đúng 1 lần. Mỗi cạnh được xét đúng 1 lần (2 lần nếu vô hướng). Tổng cộng O(V + E).

**Ý nghĩa thực tế:** Với graph 1 triệu đỉnh và 5 triệu cạnh, BFS chạy khoảng 6 triệu thao tác. Rất nhanh!

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// Xây graph đơn giản
//   0 -- 1 -- 3
//   |    |
//   2    4
let mut g = Graph::new(5, false);
g.add_unweighted_edge(0, 1);
g.add_unweighted_edge(0, 2);
g.add_unweighted_edge(1, 3);
g.add_unweighted_edge(1, 4);

let order = g.bfs(0);
// 0 đầu tiên, rồi hàng xóm của 0 (1, 2), rồi lớp tiếp theo (3, 4)
assert_eq!(order[0], 0);
assert_eq!(order.len(), 5);

// BFS từ đỉnh khác
let order2 = g.bfs(3);
assert_eq!(order2[0], 3);  // luôn bắt đầu từ đỉnh xuất phát

// Graph không liên thông -- BFS chỉ thăm được phần kết nối
// Giống hai hòn đảo riêng biệt, không có cầu nối
let mut g2 = Graph::new(4, false);
g2.add_unweighted_edge(0, 1);  // đảo 1
g2.add_unweighted_edge(2, 3);  // đảo 2
let order3 = g2.bfs(0);
assert_eq!(order3, vec![0, 1]); // chỉ thăm được đảo 1
```
