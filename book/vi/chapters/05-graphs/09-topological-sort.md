# Sắp xếp topo (Topological Sort)

## Đây là gì?

> **Anxiety check:** "Topological sort" nghe rất toán học, nhưng bạn đã làm việc này mỗi ngày mà không biết -- sắp xếp thứ tự việc cần làm sao cho không bị "thiếu điều kiện". Đã biết BFS? Kahn's algorithm = **BFS + mảng in-degree**. Đã biết DFS? DFS topo sort = **DFS + push vào stack khi return**. Không có algorithm mới -- chỉ thêm **1 trick** lên BFS hoặc DFS bạn đã biết. Và topo sort xuất hiện **cực nhiều** trong phỏng vấn (Course Schedule là top 10 bài hay gặp nhất trên LeetCode) cũng như trong production (build systems, package managers, spreadsheets).

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

---

## Cầu nối: BFS → Kahn, DFS → Topo

Bạn đã biết BFS và DFS từ 2 chương trước. Topological sort chỉ là **thêm 1 trick nhỏ** lên algorithm bạn đã biết. Hãy so sánh:

```
BFS (chương trước):                    Kahn's Topo Sort:
  queue.push(start)                      queue.push(all in-degree=0 nodes)
  while queue:                           while queue:
    v = pop()                              v = pop()
    for neighbor:                          order.push(v)
      if !visited:                         for neighbor:
        visited = true                       in_degree[neighbor] -= 1
        queue.push(neighbor)                 if in_degree == 0:
                                               queue.push(neighbor)
```

**Khác biệt:**
- BFS: start từ **1 node**, dùng `visited[]`
- Kahn: start từ **TẤT CẢ in-degree=0 nodes**, dùng `in_degree[]`
- BFS: explore graph tự do
- Kahn: "peel off" layer by layer (nodes ready → process → expose new ready nodes)

```
DFS (chương trước):                    DFS Topo Sort:
  dfs(v):                                dfs(v):
    visited[v] = true                      visited[v] = true
    for neighbor:                          for neighbor:
      if !visited:                           if !visited:
        dfs(neighbor)                          dfs(neighbor)
                                         stack.push(v)  ← CHỈ THÊM DÒNG NÀY

                                         result = stack.reverse()
```

**Khác biệt:** DFS topo sort chỉ thêm **1 dòng** `stack.push(v)` sau khi xử lý xong tất cả con cháu. Đảo ngược stack → topological order.

Tóm lại:
- **Kahn = BFS + in-degree counting** (bóc lớp từ ngoài vào)
- **DFS topo = DFS + reverse post-order** (đi sâu nhất trước, ghi ngược lại)

---

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

Giống như đăng ký môn ở đại học: đầu tiên bạn chỉ được đăng ký những môn **không cần prerequisite**. Học xong, "mở khóa" các môn tiếp theo. Lặp lại cho đến khi tốt nghiệp.

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

---

## DFS Topo Sort -- trace chi tiết

Doc ở trên chỉ dùng Kahn's. Bây giờ xem cách DFS giải cùng bài toán.

### Ý tưởng

Chạy DFS, ghi lại thứ tự **post-order** (khi DFS xong hết con cháu của 1 node, push node đó vào stack). Đảo ngược kết quả → topological order.

### Trace từng bước

```
DAG:  0→1, 0→2, 1→3, 2→3, 1→4

DFS Topo Sort (recursive):

dfs(0):                           ← bắt đầu từ 0
  dfs(1):                         ← 0 có neighbor 1, đi sâu
    dfs(3):                       ← 1 có neighbor 3, đi sâu
      no unvisited neighbors
      → push 3 vào stack          stack: [3]
    dfs(4):                       ← 1 có neighbor 4, đi sâu
      no unvisited neighbors
      → push 4 vào stack          stack: [3, 4]
    → push 1 vào stack            stack: [3, 4, 1]
  dfs(2):                         ← 0 có neighbor 2, đi sâu
    3 already visited → skip
    → push 2 vào stack            stack: [3, 4, 1, 2]
  → push 0 vào stack              stack: [3, 4, 1, 2, 0]

Post-order stack: [3, 4, 1, 2, 0]
Reverse:          [0, 2, 1, 4, 3]  ← topological order!
```

### Kiểm tra kết quả

```
Verify order [0, 2, 1, 4, 3]:
  0→1: pos(0)=0 < pos(1)=2 ✓
  0→2: pos(0)=0 < pos(2)=1 ✓
  1→3: pos(1)=2 < pos(3)=4 ✓
  1→4: pos(1)=2 < pos(4)=3 ✓
  2→3: pos(2)=1 < pos(3)=4 ✓
  All edges "point forward" ✓
```

### Tại sao reverse post-order = topological order?

```
Post-order: node được push SAU KHI tất cả con cháu đã xử lý xong.
  → Node push cuối cùng = node KHÔNG depend ai (root)
  → Node push đầu tiên = node sâu nhất (leaf, depend nhiều nhất)

Reverse:
  → Root đứng đầu, leaf đứng cuối
  → = Topological order!

Intuition: DFS đi sâu nhất trước. Node sâu nhất (leaf,
  không có dependency nào phía sau) được push đầu tiên vào stack.
  Reverse → leaf đứng cuối, root đứng đầu.

Nói cách khác: khi dfs(u) return, tất cả nodes mà u depend
  đã được push vào stack rồi. Push u sau → reverse → u đứng trước
  tất cả descendants → đúng topological order.
```

---

## Kahn vs DFS Topo Sort

| | Kahn's (BFS-based) | DFS-based |
|---|---|---|
| Data structure | Queue + in-degree[] | Recursion/Stack + visited[] |
| Cycle detection | ✅ Tự nhiên (order.len < V) | ⚠️ Cần thêm 3-color |
| Output order | Deterministic (với sorted queue) | Phụ thuộc DFS traversal order |
| Parallelism | ✅ Natural (tất cả in-degree=0 process cùng lúc) | ❌ Sequential |
| Code complexity | Hơi nhiều hơn (tính in-degree) | Hơi ít hơn |
| Dùng trong thực tế | Build systems (make, Bazel) | Compiler analysis |

**Kahn's thường được ưu tiên** vì: (1) detect cycle dễ -- chỉ check `order.len == V`, (2) natural parallelism -- tất cả nodes in-degree=0 có thể process song song.

Nhưng DFS-based hữu ích khi bạn đã có DFS infrastructure sẵn (ví dụ compiler) và cần topo sort như "thêm 1 dòng code".

---

## Parallel Scheduling (Level-by-Level)

Kahn's có 1 variant cực hay: thay vì process 1 node/step, **process TẤT CẢ in-degree=0 nodes cùng lúc** = 1 "wave" / "level".

### Ví dụ: Build system

```
DAG: 0→2, 1→2, 2→3, 2→4, 3→5, 4→5

Level 0: {0, 1}     (in-degree=0, compile song song!)
Level 1: {2}        (chờ 0,1 xong)
Level 2: {3, 4}     (chờ 2 xong, compile song song!)
Level 3: {5}        (chờ 3,4 xong)

Total time (parallel): 4 levels
Total time (sequential): 6 steps
Speedup: 6/4 = 1.5x
```

Nghĩ như xếp lịch học đại học: **mỗi level = 1 học kỳ**. Tất cả môn trong cùng level học song song. Số levels = **số học kỳ tối thiểu** cần để tốt nghiệp.

### Ứng dụng

- **Build system:** `make -j4` compile independent files song song
- **Task scheduler:** run independent tasks simultaneously
- **CI/CD pipeline:** stage N+1 start khi tất cả tasks ở stage N xong
- **University:** minimum semesters to graduate = number of levels

---

## Phát hiện chu trình

### Kahn's approach (đơn giản nhất)

Nếu graph có chu trình, một số đỉnh **không bao giờ** có in-degree = 0. Queue sẽ hết phần tử trước khi xuất hết tất cả đỉnh.

```
Graph có chu trình:
  0 → 1 → 2 → 0     (vòng tròn!)

In-degree: [0:1, 1:1, 2:1]    ← không ai có in-degree 0!
Queue rỗng ngay từ đầu → phát hiện chu trình!
```

Detection: `if order.len() < num_vertices → có cycle!`

### DFS 3-color approach

DFS dùng **3 màu** để detect cycle:

```
WHITE (0) = chưa thăm
GRAY  (1) = đang trong call stack (đang explore con cháu)
BLACK (2) = đã xử lý xong

Rule: Nếu gặp node GRAY → back edge → CYCLE!

Ví dụ: 0 → 1 → 2 → 0

dfs(0): color[0] = GRAY
  dfs(1): color[1] = GRAY
    dfs(2): color[2] = GRAY
      neighbor 0: color[0] == GRAY → CYCLE DETECTED!
```

Tại sao GRAY = cycle? Vì GRAY nghĩa là node đang **trong đường đi hiện tại** từ root → node. Gặp lại nó = quay vòng = cycle.

Trong Rust code (xem `has_cycle_directed` trong `src/graph.rs`):

```rust
// 0 = white, 1 = gray, 2 = black
fn dfs_cycle_directed(&self, u: usize, color: &mut [u8]) -> bool {
    color[u] = 1; // GRAY
    for &(v, _) in &self.adjacency_list[u] {
        if color[v] == 1 {
            return true; // back edge → cycle!
        }
        if color[v] == 0 && self.dfs_cycle_directed(v, color) {
            return true;
        }
    }
    color[u] = 2; // BLACK
    false
}
```

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

### Kahn's Algorithm (BFS-based)

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

### DFS-based Topo Sort (reverse post-order)

```rust
pub fn topological_sort_dfs(&self) -> Option<Vec<usize>> {
    let n = self.num_vertices;
    // 0 = white (unvisited), 1 = gray (in current path), 2 = black (done)
    let mut color = vec![0u8; n];
    let mut stack = Vec::with_capacity(n);

    for v in 0..n {
        if color[v] == 0 {
            if !self.topo_dfs_visit(v, &mut color, &mut stack) {
                return None; // cycle detected
            }
        }
    }

    stack.reverse();
    Some(stack)
}

fn topo_dfs_visit(&self, u: usize, color: &mut [u8], stack: &mut Vec<usize>) -> bool {
    color[u] = 1; // gray
    for &(v, _) in &self.adjacency_list[u] {
        if color[v] == 1 {
            return false; // back edge → cycle
        }
        if color[v] == 0 && !self.topo_dfs_visit(v, color, stack) {
            return false;
        }
    }
    color[u] = 2; // black
    stack.push(u); // post-order: push AFTER all descendants done
    true
}
```

Giải thích:

- Dùng **3-color** (white/gray/black) vừa topo sort vừa detect cycle
- `stack.push(u)` chỉ khi **tất cả con cháu đã xử lý xong** (post-order)
- Cuối cùng `stack.reverse()` → topological order
- Gặp node GRAY (đang trong path hiện tại) → có back edge → cycle → return `None`

### Kahn's with Level Tracking (Parallel Scheduling)

```rust
pub fn topo_levels(&self) -> Option<Vec<Vec<usize>>> {
    let n = self.num_vertices;
    let mut in_degree = vec![0usize; n];

    for u in 0..n {
        for &(v, _) in &self.adjacency_list[u] {
            in_degree[v] += 1;
        }
    }

    let mut queue: VecDeque<usize> =
        (0..n).filter(|&v| in_degree[v] == 0).collect();
    let mut levels = Vec::new();
    let mut count = 0;

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::with_capacity(level_size);
        for _ in 0..level_size {
            let u = queue.pop_front().unwrap();
            level.push(u);
            count += 1;
            for &(v, _) in &self.adjacency_list[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        levels.push(level);
    }

    if count == n {
        Some(levels)
    } else {
        None // cycle detected
    }
}
```

Giải thích:

- Giống Kahn's bình thường, nhưng mỗi vòng while process **tất cả** nodes in-degree=0 hiện tại
- `level_size = queue.len()` ghi nhớ bao nhiêu nodes "sẵn sàng" ở đầu mỗi wave
- Mỗi `level` = 1 nhóm tasks có thể chạy song song
- Số levels = **thời gian tối thiểu** khi chạy parallel

---

## Topological Sort trong thực tế

### a) Build Systems (Make, Cargo, Bazel)

```
Makefile:
  main.o: main.c utils.h       ← main.o depend on main.c + utils.h
  utils.o: utils.c utils.h     ← utils.o depend on utils.c + utils.h
  app: main.o utils.o          ← app depend on main.o + utils.o

DAG: utils.h → main.o, utils.h → utils.o, main.o → app, utils.o → app

Topo sort: [utils.h, utils.c, main.c, utils.o, main.o, app]
make -j4: process nodes cùng level song song!
```

### b) Package Manager (cargo, npm, pip)

```
cargo build:
  my-app depends on serde, tokio
  tokio depends on mio, bytes

  DAG: mio → tokio, bytes → tokio, serde → my-app, tokio → my-app
  Compile order: [mio, bytes, serde, tokio, my-app]

  Circular dependency → cargo báo lỗi!
```

### c) Spreadsheet Cell Evaluation

```
Excel: A1 = 5, B1 = A1+3, C1 = B1*2

  DAG: A1 → B1 → C1
  Eval order: A1(=5) → B1(=8) → C1(=16)

  Circular: A1 = B1+1, B1 = A1+1 → ERROR! (cycle)
```

### d) Course Prerequisites (quay lại ẩn dụ đại học)

```
University degree plan:
  Calculus I → Calculus II → Differential Equations
  Linear Algebra → Machine Learning
  Statistics → Machine Learning

  Topo sort = semester plan
  Level-by-level = how many semesters minimum?

  Level 0: {Calc I, Linear Algebra, Statistics}  (no prereqs)
  Level 1: {Calc II, Machine Learning}           (prereqs done)
  Level 2: {Differential Equations}              (need Calc II)

  Minimum 3 semesters to graduate!
```

---

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V + E) |
| Bộ nhớ | O(V) cho mảng in-degree + queue |
| Phát hiện chu trình? | Có (trả về None) |

Mỗi đỉnh vào/ra queue đúng 1 lần. Mỗi cạnh xét 1 lần khi tính in-degree và 1 lần khi giảm in-degree. Tổng: O(V + E).

Cả 3 approach (Kahn, DFS, Level-by-level) đều O(V + E).

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// === Kahn's Algorithm ===

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

// === DFS-based Topo Sort ===

let dfs_order = g.topological_sort_dfs().expect("DAG có thứ tự topo");
let dfs_pos = |v: usize| dfs_order.iter().position(|&x| x == v).unwrap();
assert!(dfs_pos(0) < dfs_pos(1));
assert!(dfs_pos(0) < dfs_pos(2));
assert!(dfs_pos(1) < dfs_pos(3));
assert!(dfs_pos(2) < dfs_pos(3));
// Note: Kahn's và DFS có thể cho ra order khác nhau -- cả 2 đều valid!

// === Level-by-Level (Parallel Scheduling) ===

let mut dag = Graph::new(6, true);
dag.add_unweighted_edge(0, 2);
dag.add_unweighted_edge(1, 2);
dag.add_unweighted_edge(2, 3);
dag.add_unweighted_edge(2, 4);
dag.add_unweighted_edge(3, 5);
dag.add_unweighted_edge(4, 5);

let levels = dag.topo_levels().expect("DAG");
// Level 0: {0, 1}  - process song song
// Level 1: {2}
// Level 2: {3, 4}  - process song song
// Level 3: {5}
assert_eq!(levels.len(), 4);
assert!(levels[0].contains(&0) && levels[0].contains(&1));

// === Graph có chu trình → không có thứ tự topo ===

let mut cyclic = Graph::new(3, true);
cyclic.add_unweighted_edge(0, 1);
cyclic.add_unweighted_edge(1, 2);
cyclic.add_unweighted_edge(2, 0);  // tạo vòng!
assert!(cyclic.topological_sort().is_none());
assert!(cyclic.topological_sort_dfs().is_none());
assert!(cyclic.topo_levels().is_none());

// === 1 đỉnh duy nhất (DAG tầm thường) ===

let single = Graph::new(1, true);
assert_eq!(single.topological_sort(), Some(vec![0]));
```

---

## Những cái bẫy hay gặp

### a) Dùng topo sort trên undirected graph

❌ Áp dụng topological sort cho graph vô hướng

✅ Topo sort chỉ cho **directed** graph. Undirected edge A-B = A→B + B→A = cycle! Luôn check: graph có hướng không?

💡 Nếu đề bài cho undirected graph mà hỏi ordering → **đây không phải bài topo sort**. Nghĩ hướng khác.

### b) Quên check cycle

❌ Dùng DFS topo sort mà không check cycle → output "valid" order cho graph có cycle (kết quả sai!)

✅ **Luôn check cycle.** Kahn's detect cycle tự nhiên (`order.len < V`). DFS cần 3-color (white/gray/black).

💡 Kahn's là approach "an toàn" hơn vì cycle detection "miễn phí". DFS topo sort **phải** dùng 3-color, không được chỉ dùng `visited[]` boolean.

### c) Nghĩ topo sort là unique

❌ Assume DAG chỉ có **1** valid topological order

✅ DAG thường có **nhiều** valid topo orders. `[0,1,2,3]` và `[0,2,1,3]` đều có thể đúng.

💡 Nếu đề bài yêu cầu **lexicographically smallest** order → dùng **min-heap** (BinaryHeap + Reverse) thay queue trong Kahn's.

### d) Quên xử lý disconnected DAG

❌ Chỉ start topo sort từ 1 node, bỏ qua phần còn lại

✅ DAG có thể có nhiều connected components. Kahn's tự xử lý (enqueue **tất cả** in-degree=0 nodes). DFS cần **loop qua tất cả** unvisited nodes.

💡 Cả code Kahn's và DFS trong `src/graph.rs` đều xử lý disconnected graph đúng: Kahn's collect tất cả in-degree=0 vào queue ban đầu, DFS loop `for v in 0..n`.

---

## Khi nào dùng Topological Sort?

| Tình huống | Topo Sort? | Tại sao? |
|---|---|---|
| Task scheduling with dependencies | ✅ | Xếp thứ tự sao cho dependency trước |
| Build order (compile, install) | ✅ | File/package dependency |
| Course planning | ✅ | Prerequisites |
| Spreadsheet eval order | ✅ | Cell dependency |
| Detect circular dependency | ✅ | Kahn's: order.len < V |
| Parallel scheduling | ✅ | Level-by-level Kahn's |
| Undirected graph ordering | ❌ | Topo sort chỉ cho directed |
| Graph có cycle | ❌ | Không tồn tại topo order |
| Shortest path | ❌ | Dùng BFS/Dijkstra |
| Find all possible orderings | ⚠️ | Backtracking, exponential |

**Tín hiệu nhận diện trong phỏng vấn:** Bài toán có "phải làm X trước Y", "dependency", "prerequisite", "ordering" → rất có thể là topo sort.

---

## Luyện nhận diện Pattern

### a) Course Schedule (LeetCode #207)

> N courses, mỗi course có list prerequisites. Hỏi: có thể finish tất cả courses không?

**Gợi ý:**
- Build directed graph từ prerequisites
- Chạy topo sort (Kahn's hoặc DFS)
- Kahn's: `if order.len() == N → YES` (không có cycle)
- Nếu có cycle → có môn "kẹt" nhau → NO

### b) Course Schedule II (LeetCode #210)

> Giống #207, nhưng trả về **valid order** (nếu có).

**Gợi ý:**
- Kahn's algorithm, trả về `order` nếu `order.len() == N`
- Trả về empty vector nếu có cycle
- Đây chính xác là `topological_sort()` trong code!

### c) Alien Dictionary (LeetCode #269)

> Cho list sorted words trong alien language. Infer character ordering.

**Gợi ý:**
- Compare adjacent words → extract edges. Ví dụ: "abc" < "abd" → 'c' < 'd' → edge c→d
- Build graph of characters
- Topo sort trên character graph → alien alphabet order
- Cycle → invalid dictionary

---

## Rust Ecosystem

- **`petgraph::algo::toposort`** -- built-in topo sort (DFS-based) trong crate `petgraph`
- **`petgraph::algo::is_cyclic_directed`** -- cycle detection
- Kahn's algorithm dễ implement from scratch (như code ở trên)

**Ứng dụng thực tế trong Rust:**
- `cargo build` dùng topo sort để compile dependencies đúng thứ tự
- Build hệ thống pipeline: nếu tasks có dependency → topo sort cho execution order
- Config validation: detect circular dependencies trong configuration files

---

[← Thuật toán Kruskal (Cây khung nhỏ nhất)](./08-kruskal.md) | [Union-Find (Tập hợp rời rạc) → ./10-union-find.md)

## Chương tiếp theo

Topological sort dùng in-degree và BFS/DFS để order DAG. Kruskal's MST (chương trước) dùng **Union-Find** để check "2 node cùng nhóm chưa?". Chương tiếp theo deep dive vào **Union-Find (Disjoint Set Union)** -- data structure cho phép find root và union sets trong gần O(1). **Path compression + union by rank** là 2 kỹ thuật nhỏ tạo performance improvement lớn. Union-Find giải quyết mọi bài toán "connectivity" và "grouping" -- từ network connectivity đến image segmentation.

---

[← Kruskal](./08-kruskal.md) | [Union-Find →](./10-union-find.md)
