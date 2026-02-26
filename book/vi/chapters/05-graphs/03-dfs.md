# Tìm kiếm theo chiều sâu (Depth-First Search - DFS)

## Đây là gì?

Hãy tưởng tượng bạn đang **đi trong mê cung**. Chiến thuật của bạn: cứ đi thẳng vào một ngõ, đi sâu nhất có thể. Nếu gặp ngõ cụt thì quay lại ngã rẽ gần nhất, rồi thử ngõ khác. Lặp lại cho đến khi khám phá hết mê cung.

Đó chính là **DFS** (Depth-First Search -- tìm kiếm theo chiều sâu). Khác với BFS (lan ra theo từng lớp), DFS **lao sâu vào một hướng** trước khi quay lại thử hướng khác.

Tại sao cần DFS?
- **Phát hiện chu trình** (cycle) trong graph
- **Topological sort** (sắp xếp topo) -- sắp thứ tự các công việc phụ thuộc nhau
- Tìm **thành phần liên thông** (connected components)
- Giải các bài toán **quay lui** (backtracking) như Sudoku, N-queens

DFS dùng **stack** (ngăn xếp) -- hoặc dùng **recursion** (đệ quy), vì bản chất recursion chính là stack.

## Hoạt động như thế nào?

### Từng bước một

Cho graph sau, bắt đầu từ đỉnh 0:

```
       0
      / \
     1   2
    / \   \
   3   4   5
```

**So sánh BFS và DFS:**
```
BFS (theo lớp):  0, 1, 2, 3, 4, 5    ← hết lớp này rồi sang lớp kia
DFS (theo sâu):  0, 1, 3, 4, 2, 5    ← lao sâu vào nhánh 1 trước
```

**Quá trình DFS (dùng stack):**

```
Bước 0: Push 0          Stack: [0]                Thứ tự: []
Bước 1: Pop 0, push 2,1 Stack: [2, 1]             Thứ tự: [0]
  ↳ Vào mê cung, gặp ngã ba → chọn đi vào ngõ 1

Bước 2: Pop 1, push 4,3 Stack: [2, 4, 3]          Thứ tự: [0, 1]
  ↳ Đi sâu tiếp, gặp ngã ba → chọn ngõ 3

Bước 3: Pop 3           Stack: [2, 4]              Thứ tự: [0, 1, 3]
  ↳ Ngõ cụt! Quay lại...

Bước 4: Pop 4           Stack: [2]                 Thứ tự: [0, 1, 3, 4]
  ↳ Ngõ cụt nữa! Quay lại...

Bước 5: Pop 2, push 5   Stack: [5]                 Thứ tự: [0, 1, 3, 4, 2]
  ↳ Giờ mới thử nhánh 2

Bước 6: Pop 5           Stack: []                  Thứ tự: [0, 1, 3, 4, 2, 5]
  ↳ Xong!
```

Chú ý: DFS khám phá **hết nhánh 1** (đỉnh 1 → 3 → 4) rồi mới sang nhánh 2 (đỉnh 2 → 5).

```
Mê cung tương đương:

  Vào → [0] → [1] → [3] ← cụt!
               ↓
              [4] ← cụt!
         ↓
        [2] → [5] ← cụt!

DFS đi: 0 → 1 → 3 (cụt, quay lại) → 4 (cụt, quay lại) → 2 → 5
```

### Ứng dụng của DFS

- **Phát hiện chu trình:** Nếu DFS gặp lại đỉnh tổ tiên (back edge), có chu trình
- **Topological sort:** Thứ tự ngược của DFS post-order
- **Thành phần liên thông:** Chạy DFS từ mỗi đỉnh chưa thăm
- **Tìm đường:** DFS liệt kê được tất cả đường đi giữa hai đỉnh

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`. Dùng stack tường minh để tránh tràn stack khi graph sâu.

```rust
pub fn dfs(&self, start: usize) -> Vec<usize> {
    let mut visited = vec![false; self.num_vertices];
    let mut order = Vec::new();
    let mut stack = vec![start];

    while let Some(v) = stack.pop() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        order.push(v);
        // Push hàng xóm theo thứ tự ngược
        // để đỉnh nhỏ nhất được pop (thăm) trước
        for &(neighbor, _) in self.adjacency_list[v].iter().rev() {
            if !visited[neighbor] {
                stack.push(neighbor);
            }
        }
    }
    order
}
```

Giải thích:

- Kiểm tra `visited` **khi pop**, không phải khi push. Tại sao? Vì cùng một đỉnh có thể bị push nhiều lần từ nhiều hàng xóm khác nhau
- Push hàng xóm **ngược** để đỉnh đầu tiên trong danh sách kề nằm trên đỉnh stack → được pop trước
- Dùng stack tường minh thay vì recursion để tránh **stack overflow** với graph lớn

### Phiên bản đệ quy

Dễ hiểu hơn nhưng có nguy cơ tràn stack:

```rust
fn dfs_recursive(graph: &Graph, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[v] = true;
    order.push(v);
    for &(neighbor, _) in graph.neighbors(v) {
        if !visited[neighbor] {
            dfs_recursive(graph, neighbor, visited, order);
        }
    }
}
```

Recursion chính là stack ẩn -- mỗi lần gọi hàm = push lên call stack, mỗi lần return = pop.

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V + E) |
| Bộ nhớ | O(V) cho mảng visited + stack |
| Tìm đường ngắn nhất? | Không! |

Giống BFS, DFS thăm mỗi đỉnh 1 lần và xét mỗi cạnh 1 lần. Nhưng DFS **không đảm bảo** đường ngắn nhất -- nó có thể đi đường vòng xa trước khi tìm thấy đường tắt.

**So sánh BFS vs DFS:**

| | BFS | DFS |
|---|---|---|
| Cấu trúc dữ liệu | Queue | Stack |
| Thứ tự khám phá | Theo lớp (gần → xa) | Theo sâu (lao hết 1 nhánh) |
| Đường ngắn nhất? | Có (không trọng số) | Không |
| Phát hiện chu trình? | Khó hơn | Dễ |
| Bộ nhớ | Có thể lớn (lưu cả lớp) | Thường nhỏ hơn |

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

let mut g = Graph::new(5, false);
g.add_unweighted_edge(0, 1);
g.add_unweighted_edge(0, 2);
g.add_unweighted_edge(1, 3);
g.add_unweighted_edge(1, 4);

let order = g.dfs(0);
assert_eq!(order[0], 0);      // luôn bắt đầu từ đỉnh xuất phát
assert_eq!(order.len(), 5);   // thăm hết 5 đỉnh

// DFS trên graph có hướng
let mut dg = Graph::new(4, true);
dg.add_unweighted_edge(0, 1);
dg.add_unweighted_edge(0, 2);
dg.add_unweighted_edge(2, 3);
let order = dg.dfs(0);
assert_eq!(order[0], 0);
assert_eq!(order.len(), 4);
```
