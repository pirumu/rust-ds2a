# Sắp xếp topo (Topological Sort)

## Đây là gì?

Hãy nghĩ về **thứ tự học môn ở đại học**. Muốn học Giải tích thì phải học Toán cao cấp trước. Muốn học Machine Learning thì phải học Xác suất trước. Muốn học Xác suất thì phải học Toán trước.

```
Toán → Xác suất → Machine Learning
Toán → Giải tích
```

**Topological sort** (sắp xếp topo) sắp xếp các môn học sao cho mọi điều kiện tiên quyết đều được thỏa mãn -- môn nào cần học trước thì nằm trước trong danh sách.

Chính xác hơn: với mọi cạnh `u → v` trong graph, đỉnh `u` luôn đứng trước `v` trong thứ tự sắp xếp.

**Điều kiện quan trọng:** Topological sort chỉ hoạt động trên **DAG** (Directed Acyclic Graph -- đồ thị có hướng không có chu trình). Tại sao? Nếu có chu trình (A phải học trước B, B phải học trước A), thì không ai học được gì cả!

**Ứng dụng thực tế:**
- Hệ thống build: compile file nào trước?
- Lập lịch công việc: việc nào làm trước?
- Công thức Excel: tính ô nào trước?
- Package manager: cài thư viện nào trước?

## Hoạt động như thế nào?

### Ví dụ DAG

```
  0 --> 1 --> 3
  |           ^
  v           |
  2 ----------+

Cạnh: 0→1, 0→2, 1→3, 2→3

Đọc: "0 phải xong trước 1 và 2. 1 và 2 phải xong trước 3."
```

Các thứ tự topo hợp lệ:
- `[0, 1, 2, 3]` -- làm 1 trước 2
- `[0, 2, 1, 3]` -- làm 2 trước 1

Cả hai đều đúng vì mọi cạnh đều "chỉ về phía trước" trong danh sách.

### Thuật toán Kahn (dựa trên BFS)

Ý tưởng: Tìm công việc **không có điều kiện tiên quyết** (in-degree = 0). Làm xong, xóa khỏi danh sách. Lặp lại.

```
In-degree (số cạnh vào): [0:0, 1:1, 2:1, 3:2]
                           ↑
                    Đỉnh 0 không cần điều kiện tiên quyết

Bước 1: Đỉnh 0 có in-degree 0 → xuất 0
  Xóa cạnh 0→1 và 0→2
  In-degree: [1:0, 2:0, 3:2]

Bước 2: Đỉnh 1 có in-degree 0 → xuất 1
  Xóa cạnh 1→3
  In-degree: [2:0, 3:1]

Bước 3: Đỉnh 2 có in-degree 0 → xuất 2
  Xóa cạnh 2→3
  In-degree: [3:0]

Bước 4: Đỉnh 3 có in-degree 0 → xuất 3

Kết quả: [0, 1, 2, 3] ✓
```

### Phát hiện chu trình

Nếu graph có chu trình, một số đỉnh **không bao giờ** có in-degree = 0. Queue sẽ hết phần tử trước khi xuất hết tất cả đỉnh.

```
Graph có chu trình:
  0 → 1 → 2 → 0     (vòng tròn!)

In-degree: [0:1, 1:1, 2:1]    ← không ai có in-degree 0!
Queue rỗng ngay từ đầu → phát hiện chu trình!
```

### Cách tiếp cận DFS

Chạy DFS, ghi lại thứ tự **post-order** (khi DFS xong hết con cháu). Đảo ngược kết quả → topological order.

```
DFS post-order: 3, 1, 2, 0
Đảo ngược:      0, 2, 1, 3  ← thứ tự topo hợp lệ!
```

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`. Dùng thuật toán Kahn.

```rust
pub fn topological_sort(&self) -> Option<Vec<usize>> {
    let n = self.num_vertices;
    let mut in_degree = vec![0usize; n];

    // Tính in-degree cho mỗi đỉnh
    for u in 0..n {
        for &(v, _) in &self.adjacency_list[u] {
            in_degree[v] += 1;
        }
    }

    // Đưa các đỉnh có in-degree 0 vào queue
    let mut queue: VecDeque<usize> =
        (0..n).filter(|&v| in_degree[v] == 0).collect();
    let mut order = Vec::with_capacity(n);

    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &(v, _) in &self.adjacency_list[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if order.len() == n {
        Some(order)
    } else {
        None // có chu trình!
    }
}
```

Giải thích:

- **In-degree** = số cạnh đi vào 1 đỉnh = số điều kiện tiên quyết
- Đỉnh có in-degree 0 = không cần chờ ai → làm ngay
- Khi "xong" 1 đỉnh, giảm in-degree của hàng xóm. Nếu hàng xóm về 0 → đưa vào queue
- Nếu cuối cùng `order.len() < n` → có đỉnh chưa xuất → có chu trình → trả về `None`

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V + E) |
| Bộ nhớ | O(V) cho mảng in-degree + queue |
| Phát hiện chu trình? | Có (trả về None) |

Mỗi đỉnh vào/ra queue đúng 1 lần. Mỗi cạnh xét 1 lần khi tính in-degree và 1 lần khi giảm in-degree. Tổng: O(V + E).

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// DAG đơn giản: 0→1→3, 0→2→3
let mut g = Graph::new(4, true);
g.add_unweighted_edge(0, 1);
g.add_unweighted_edge(0, 2);
g.add_unweighted_edge(1, 3);
g.add_unweighted_edge(2, 3);

let order = g.topological_sort().expect("DAG có thứ tự topo");
// Kiểm tra: mọi cạnh u→v, u phải đứng trước v
let pos = |v: usize| order.iter().position(|&x| x == v).unwrap();
assert!(pos(0) < pos(1));  // 0 trước 1
assert!(pos(0) < pos(2));  // 0 trước 2
assert!(pos(1) < pos(3));  // 1 trước 3
assert!(pos(2) < pos(3));  // 2 trước 3

// Graph có chu trình → không có thứ tự topo
let mut cyclic = Graph::new(3, true);
cyclic.add_unweighted_edge(0, 1);
cyclic.add_unweighted_edge(1, 2);
cyclic.add_unweighted_edge(2, 0);  // tạo vòng!
assert!(cyclic.topological_sort().is_none());

// 1 đỉnh duy nhất (DAG tầm thường)
let single = Graph::new(1, true);
assert_eq!(single.topological_sort(), Some(vec![0]));
```
