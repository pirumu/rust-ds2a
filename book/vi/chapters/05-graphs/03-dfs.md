# Tìm kiếm theo chiều sâu (Depth-First Search - DFS)

> 💡 **Đừng lo lắng:** Nếu bạn hiểu BFS (chương trước), DFS chỉ khác **1 dòng code**: thay Queue bằng Stack. Thế thôi. Hoặc nếu dùng recursion thì còn ngắn hơn -- recursion BẢN THÂN NÓ chính là stack (call stack). DFS dùng Stack mà bạn đã học từ chương 1. Phần khó hơn BFS là các **ứng dụng**: cycle detection, topological sort, backtracking. Nhưng mỗi ứng dụng đều xây trên cùng 1 DFS core. DFS xuất hiện cực nhiều trong phỏng vấn -- đặc biệt backtracking (permutation, combination, Sudoku, N-queens).

## Đây là gì?

Hãy tưởng tượng bạn đang **đi trong mê cung**. Chiến thuật của bạn: cứ đi thẳng vào một ngõ, đi sâu nhất có thể. Nếu gặp ngõ cụt thì quay lại ngã rẽ gần nhất, rồi thử ngõ khác. Lặp lại cho đến khi khám phá hết mê cung.

Đó chính là **DFS** (Depth-First Search -- tìm kiếm theo chiều sâu). Khác với BFS (lan ra theo từng lớp), DFS **lao sâu vào một hướng** trước khi quay lại thử hướng khác.

Tại sao cần DFS?
- **Phát hiện chu trình** (cycle) trong graph
- **Topological sort** (sắp xếp topo) -- sắp thứ tự các công việc phụ thuộc nhau
- Tìm **thành phần liên thông** (connected components)
- Giải các bài toán **quay lui** (backtracking) như Sudoku, N-queens

DFS dùng **stack** (ngăn xếp) -- hoặc dùng **recursion** (đệ quy), vì bản chất recursion chính là stack.

---

## Stack → DFS: BFS chỉ đổi Queue thành Stack

Nhớ chương trước? BFS dùng Queue. DFS dùng Stack. Chỉ đổi 2 dòng:

```
BFS (chương trước):             DFS (chương này):
  queue = VecDeque::new()         stack = Vec::new()     ← CHỈ ĐỔI CÁI NÀY
  queue.push_back(start)          stack.push(start)
  while let Some(v) =             while let Some(v) =
    queue.pop_front() {             stack.pop() {        ← VÀ CÁI NÀY
    ...                             ...
  }                               }

Queue (FIFO) → thăm RỘNG trước (gần → xa)
Stack (LIFO) → thăm SÂU trước (lao hết 1 nhánh)
```

**Trong mê cung:** BFS = lan ra từng vòng tròn, kiểm tra mọi ngõ cùng khoảng cách trước. DFS = lao vào 1 ngõ, đi tới cùng, gặp cụt mới quay lại.

### Recursion IS DFS

Nhớ chương Stack nói "recursion chính là call stack"?

```
DFS recursive = mỗi recursive call = push lên call stack
return = pop khỏi call stack
→ Recursion IS DFS!

Mọi tree traversal bạn đã viết (preorder, inorder, postorder)
ĐỀU LÀ DFS trên tree. Giờ mở rộng sang graph + visited.
```

Vậy nên DFS có 2 cách viết:
- **Iterative**: tự quản lý stack bằng `Vec` -- an toàn cho graph lớn
- **Recursive**: ngắn gọn hơn, nhưng graph sâu quá → stack overflow

---

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

---

## Hoạt động trên graph có cycle

Graph ở trên giống tree -- không có cycle. Nhưng graph thực tế thường có cycle. Hãy trace DFS trên directed graph có cycle:

```
Directed graph có cycle:

  0 → 1 → 3
  ↓   ↓
  2   4 → 1   (4→1 tạo cycle: 1→4→1)
```

**DFS từ 0 (recursive):**

```
dfs(0): visit 0, marked visited
  dfs(1): visit 1, marked visited
    dfs(3): visit 3 (không có neighbor chưa visited → return)
    dfs(4): visit 4, marked visited
      neighbor 1: ĐÃ VISITED!
      → Đây là BACK EDGE → CÓ CYCLE! (1 → 4 → 1)
  dfs(2): visit 2 (không có neighbor chưa visited → return)

Thứ tự visit: [0, 1, 3, 4, 2]
```

**Back edge** là gì? Cạnh đi đến node đang trong call stack (tổ tiên). Nếu gặp back edge = **chứng cứ có cycle**.

Trong mê cung: bạn đi 0 → 1 → 4, rồi từ 4 thấy đường dẫn ngược về 1 -- bạn đã đi vòng!

Với `visited` array, DFS **không bị lặp vô tận** -- gặp node đã visited thì skip, nhưng vẫn phát hiện được cycle.

---

## Cycle Detection chi tiết

Phát hiện chu trình là **ứng dụng #1** của DFS. Code trong `src/graph.rs` implement cả 2 trường hợp.

### Undirected graph -- check parent

Undirected graph: mỗi cạnh đi 2 chiều. Nếu DFS từ 0 đến 1, thì neighbor của 1 bao gồm 0. Đó **không phải** cycle -- chỉ là cạnh 2 chiều. Cycle thật = gặp visited node mà **không phải parent**.

```
Graph: 0 -- 1 -- 2 -- 0  (cycle)

dfs(0, parent=None):
  visit 0
  dfs(1, parent=0):
    visit 1
    dfs(2, parent=1):
      visit 2
      neighbor 0: visited, 0 ≠ parent(1) → CYCLE!
```

```rust
// Undirected: nếu gặp visited node mà KHÔNG PHẢI parent → cycle
fn has_cycle_undirected(graph: &Graph) -> bool {
    let mut visited = vec![false; graph.num_vertices()];
    for v in 0..graph.num_vertices() {
        if !visited[v] {
            if dfs_cycle(graph, v, usize::MAX, &mut visited) {
                return true;
            }
        }
    }
    false
}

fn dfs_cycle(graph: &Graph, v: usize, parent: usize,
             visited: &mut Vec<bool>) -> bool {
    visited[v] = true;
    for &(neighbor, _) in graph.neighbors(v) {
        if !visited[neighbor] {
            if dfs_cycle(graph, neighbor, v, visited) {
                return true;
            }
        } else if neighbor != parent {
            return true;  // visited + not parent = CYCLE!
        }
    }
    false
}
```

### Directed graph -- 3 trạng thái (White/Gray/Black)

Directed graph phức tạp hơn. Không có khái niệm "parent" rõ ràng. Thay vào đó dùng **3 màu**:

```
White (0) = chưa visit
Gray  (1) = đang visit (đang trong call stack -- đang đi sâu)
Black (2) = đã visit xong (return rồi -- đã quay lại)

Quy tắc: gặp Gray node → back edge → CYCLE!
```

**Trace 3-color trên graph CÓ cycle:**

```
Graph: 0 → 1 → 2 → 0 (cycle)

dfs(0): color[0] = Gray     ← đang đi vào ngõ 0
  dfs(1): color[1] = Gray   ← đi sâu vào ngõ 1
    dfs(2): color[2] = Gray ← đi sâu vào ngõ 2
      neighbor 0: color[0] = Gray → BACK EDGE → CYCLE!
      (0 vẫn Gray = vẫn đang trong call stack = tổ tiên)
```

**Trace 3-color trên graph KHÔNG có cycle:**

```
Graph: 0 → 1 → 2 (no cycle)

dfs(0): color[0] = Gray
  dfs(1): color[1] = Gray
    dfs(2): color[2] = Gray
    color[2] = Black (return -- ngõ cụt, quay lại)
  color[1] = Black (return)
color[0] = Black (return)

Gặp Black node = "đã xong rồi" = cross/forward edge, KHÔNG phải cycle
```

```rust
// Directed: dùng 3 trạng thái (White/Gray/Black)
fn has_cycle_directed(graph: &Graph) -> bool {
    // 0 = white, 1 = gray, 2 = black
    let mut color = vec![0u8; graph.num_vertices()];
    for v in 0..graph.num_vertices() {
        if color[v] == 0 {
            if dfs_cycle_directed(graph, v, &mut color) {
                return true;
            }
        }
    }
    false
}

fn dfs_cycle_directed(graph: &Graph, v: usize, color: &mut [u8]) -> bool {
    color[v] = 1; // Gray -- đang đi vào
    for &(neighbor, _) in graph.neighbors(v) {
        if color[neighbor] == 1 {
            return true; // back edge → cycle!
        }
        if color[neighbor] == 0 && dfs_cycle_directed(graph, neighbor, color) {
            return true;
        }
    }
    color[v] = 2; // Black -- xong rồi, quay lại
    false
}
```

> Mẹo nhớ: **Gray = đang đi** (trong mê cung, bạn còn ở ngõ đó). Gặp lại ngõ Gray = bạn đi vòng = cycle. **Black = đã quay lại** (bạn đã rời ngõ đó). Gặp ngõ Black = bình thường, không phải cycle.

---

## Topological Sort

**Ứng dụng #2** của DFS, rất hay hỏi trong phỏng vấn.

### Bài toán

Bạn có nhiều package cần compile, mỗi package phụ thuộc vào package khác. Hỏi: compile theo thứ tự nào?

```
Package dependencies (A phụ thuộc B = phải compile B trước A):
  A → B
  A → C
  B → D
  C → D

    A
   / \
  B   C
   \ /
    D

Topological order: [D, B, C, A] hoặc [D, C, B, A]
  D compile trước (không depend ai)
  B, C compile sau D
  A compile cuối (depend B, C)
```

Đây là **mê cung 1 chiều**: bạn phải đi hết ngõ sâu nhất trước, rồi quay lại. Node nào DFS **rời đi cuối cùng** = node nào phụ thuộc nhiều nhất = compile cuối cùng.

### DFS-based topological sort

Ý tưởng: chạy DFS, khi **return** (xong hết mọi neighbor) thì push node vào stack. Cuối cùng reverse stack = topological order.

```rust
fn topological_sort(graph: &Graph) -> Vec<usize> {
    let mut visited = vec![false; graph.num_vertices()];
    let mut stack = Vec::new();  // result stack

    for v in 0..graph.num_vertices() {
        if !visited[v] {
            topo_dfs(graph, v, &mut visited, &mut stack);
        }
    }
    stack.reverse();  // reverse post-order = topological order
    stack
}

fn topo_dfs(graph: &Graph, v: usize, visited: &mut Vec<bool>,
            stack: &mut Vec<usize>) {
    visited[v] = true;
    for &(neighbor, _) in graph.neighbors(v) {
        if !visited[neighbor] {
            topo_dfs(graph, neighbor, visited, stack);
        }
    }
    stack.push(v);  // push SAU KHI visit hết descendants
}
```

**Trace:**

```
Graph: A→B, A→C, B→D, C→D  (A=0, B=1, C=2, D=3)

dfs(0=A):
  dfs(1=B):
    dfs(3=D): không có neighbor chưa visited → push D
  push B
  dfs(2=C):
    3=D đã visited → skip
  push C
push A

Stack (push order): [D, B, C, A]
Reverse: [A, C, B, D]

Verify: A trước B ✓, A trước C ✓, B trước D ✓, C trước D ✓
```

> Topological sort chỉ có trên **DAG** (Directed Acyclic Graph). Nếu có cycle → không thể sort → phải detect cycle trước!

> Code trong `src/graph.rs` dùng **Kahn's algorithm** (BFS-based) cho topological sort -- cũng cho kết quả tương tự nhưng bằng cách đếm in-degree. DFS-based topological sort ở trên là cách tiếp cận khác, dùng post-order reverse.

---

## Connected Components

Đếm có bao nhiêu "nhóm" rời nhau trong undirected graph. Mỗi nhóm = 1 connected component.

**Ý tưởng:** DFS từ 1 đỉnh = khám phá hết 1 component (1 vùng mê cung). Đếm bao nhiêu lần phải bắt đầu DFS mới = bao nhiêu component.

```
Graph: 0--1  2--3  4   (3 cụm rời nhau)

DFS từ 0: thăm {0, 1} → component 1
DFS từ 2: thăm {2, 3} → component 2
DFS từ 4: thăm {4}    → component 3

Tổng: 3 connected components
```

```rust
fn count_components(graph: &Graph) -> usize {
    let mut visited = vec![false; graph.num_vertices()];
    let mut count = 0;

    for v in 0..graph.num_vertices() {
        if !visited[v] {
            dfs_mark(graph, v, &mut visited);  // DFS thăm toàn bộ component
            count += 1;
        }
    }
    count
}

fn dfs_mark(graph: &Graph, v: usize, visited: &mut Vec<bool>) {
    visited[v] = true;
    for &(neighbor, _) in graph.neighbors(v) {
        if !visited[neighbor] {
            dfs_mark(graph, neighbor, visited);
        }
    }
}
```

Trong mê cung: tưởng tượng có 3 mê cung riêng biệt, không nối với nhau. Bạn phải "dịch chuyển" sang mê cung mới mỗi khi khám phá hết mê cung hiện tại.

---

## Backtracking Pattern

**DFS + undo = backtracking.** Đây là pattern phỏng vấn cực phổ biến.

### Ý tưởng

Backtracking = DFS trên **cây quyết định ẩn** (implicit decision tree). Bạn không xây tree thật -- DFS + undo tạo ra tree "trong quá trình chạy".

Trong mê cung: mỗi ngã rẽ = 1 quyết định. Bạn chọn 1 ngõ (make choice), đi sâu vào (DFS), gặp cụt thì quay lại (undo choice) và thử ngõ khác.

### Template

```
fn backtrack(state, result):
    if is_solution(state):
        result.add(state.clone())
        return

    for choice in get_choices(state):
        state.make_choice(choice)     // DO
        backtrack(state, result)       // RECURSE (DFS đi sâu)
        state.undo_choice(choice)     // UNDO (quay lại ngã rẽ)
```

3 bước: **Chọn → Đi sâu → Quay lại**. Lặp cho mọi lựa chọn.

### Ví dụ: generate all permutations of [1, 2, 3]

```
                    []
           /        |        \
         [1]       [2]       [3]
        /   \     /   \     /   \
     [1,2] [1,3] [2,1] [2,3] [3,1] [3,2]
      |      |     |     |     |     |
   [1,2,3][1,3,2][2,1,3][2,3,1][3,1,2][3,2,1]

DFS traversal trên cây quyết định (decision tree)
Mỗi level = chọn 1 phần tử chưa dùng
Backtrack = pop phần tử vừa chọn, thử phần tử khác
```

```rust
fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    backtrack(nums, &mut current, &mut used, &mut result);
    result
}

fn backtrack(nums: &[i32], current: &mut Vec<i32>,
             used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        result.push(current.clone());  // found solution!
        return;
    }
    for i in 0..nums.len() {
        if !used[i] {
            used[i] = true;              // DO
            current.push(nums[i]);
            backtrack(nums, current, used, result);  // DFS
            current.pop();               // UNDO
            used[i] = false;
        }
    }
}
```

---

## DFS Timestamps -- Discovery & Finish

Concept nâng cao nhưng quan trọng cho nhiều algorithm. Mỗi node có 2 timestamp:
- **Discovery time**: khi DFS **bắt đầu** thăm node (đi vào ngõ)
- **Finish time**: khi DFS **xong** node (quay lại khỏi ngõ)

```
DFS từ 0 trên graph: 0→1→3, 0→2

Timestamp trace:
  time=1: discover 0    → disc[0]=1
    time=2: discover 1  → disc[1]=2
      time=3: discover 3 → disc[3]=3
      time=4: finish 3  → fin[3]=4
    time=5: finish 1    → fin[1]=5
    time=6: discover 2  → disc[2]=6
    time=7: finish 2    → fin[2]=7
  time=8: finish 0      → fin[0]=8

Node:     0       1       2       3
Disc:     1       2       6       3
Fin:      8       5       7       4
```

**Ứng dụng:**
- **Topological sort**: sort by finish time giảm dần = topological order
- **Back edge (cycle)**: `disc[u] < disc[v] < fin[v] < fin[u]` → v là descendant của u. Nếu v có cạnh ngược về u → cycle
- **Cross edge**: `fin[v] < disc[u]` → v đã xong trước khi u bắt đầu

Trong mê cung: discovery = bước vào ngõ, finish = bước ra khỏi ngõ. Ngõ nào bước ra cuối cùng = ngõ "bao" nhiều ngõ con nhất.

---

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`. Dùng stack tường minh để tránh tràn stack khi graph sâu.

### Phiên bản iterative (stack tường minh)

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

---

## DFS trong thực tế

### a) Compiler -- dependency resolution

```
cargo build: phải compile dependencies trước main package
  = topological sort trên dependency DAG

Cargo.toml:
  [dependencies]
  serde = "1.0"        ← compile serde trước
  tokio = "1.0"        ← compile tokio trước
  my-app               ← compile cuối cùng

Circular dependency (cycle) → cargo báo lỗi = cycle detection!
```

### b) Maze solving / Puzzle solving

```
Sudoku, N-queens, crossword:
  Thử 1 choice → DFS đi sâu → conflict → backtrack → thử khác

Sudoku: điền 1 vào ô trống → kiểm tra → OK → ô tiếp → ...
  → conflict → quay lại → thử 2 → ...
  = DFS trên cây quyết định
```

### c) File system traversal

```
ls -R (recursive listing):
  Đi vào folder con → đi sâu nhất → quay lại → folder con tiếp
  = DFS trên directory tree

src/
├── main.rs         ← thăm
├── lib.rs          ← thăm
└── utils/          ← đi vào (DFS đi sâu)
    ├── helpers.rs  ← thăm
    └── config.rs   ← thăm, quay lại utils, quay lại src
```

### d) Garbage collection

```
Mark-and-sweep GC (Java, Go, ...):
  DFS từ root objects → mark tất cả reachable objects
  Objects không marked = garbage → sweep (giải phóng bộ nhớ)
```

---

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
| Phát hiện chu trình? | Khó hơn | Dễ (back edge) |
| Bộ nhớ | Có thể lớn (lưu cả lớp) | Thường nhỏ hơn |
| Topological sort? | Kahn's algorithm (đếm in-degree) | Post-order reverse |
| Backtracking? | Không | DFS + undo |

---

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// DFS cơ bản
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

// Cycle detection
let mut cyclic = Graph::new(3, true);
cyclic.add_unweighted_edge(0, 1);
cyclic.add_unweighted_edge(1, 2);
cyclic.add_unweighted_edge(2, 0);  // tạo cycle
assert!(cyclic.has_cycle());

let mut acyclic = Graph::new(3, true);
acyclic.add_unweighted_edge(0, 1);
acyclic.add_unweighted_edge(1, 2);
assert!(!acyclic.has_cycle());

// Topological sort
let mut dag = Graph::new(4, true);
dag.add_unweighted_edge(0, 1);  // A → B
dag.add_unweighted_edge(0, 2);  // A → C
dag.add_unweighted_edge(1, 3);  // B → D
dag.add_unweighted_edge(2, 3);  // C → D
let topo = dag.topological_sort().unwrap();
// D phải đứng trước B và C, A đứng cuối
```

---

## Những cái bẫy hay gặp

### a) Dùng DFS tìm shortest path

❌ DFS tìm path từ A đến B, kết luận đó là shortest path

✅ DFS có thể tìm **A** path nhưng **KHÔNG đảm bảo shortest**. Bài "minimum steps" → luôn dùng **BFS**

💡 DFS đi sâu trước -- nó có thể tìm đường vòng xa trước đường ngắn. BFS lan theo lớp nên lớp đầu tiên chạm đích = shortest

### b) Stack overflow với recursive DFS

❌ Dùng recursive DFS trên graph 100K nodes chuỗi (0→1→2→...→99999)

✅ 100K recursive calls = stack overflow. Dùng **iterative DFS** (explicit stack) cho graph lớn

💡 Rust default stack size = **8MB** ≈ ~100K recursive calls. Dùng `stacker` crate hoặc chuyển sang iterative

### c) Quên đánh dấu visited

❌ DFS trên graph có cycle mà không dùng `visited`

✅ Không visited = vòng lặp **vô tận**. Luôn mark visited!

💡 Graph ≠ tree. Tree không có cycle nên không cần visited. Graph có thể có cycle → **bắt buộc** visited

### d) Nhầm visited timing giữa iterative và recursive

❌ Iterative DFS: mark visited khi push (giống BFS)

✅ Recursive: mark khi **enter** function. Iterative: mark khi **pop** (vì node có thể push nhiều lần)

💡 Hai cách mark khác nhau → thứ tự visit có thể khác nhau. Cả 2 đều đúng nhưng cho kết quả khác nhau

### e) Cycle detection undirected: quên check parent

❌ Undirected graph, DFS từ 0 đến 1, neighbor của 1 có 0 → kết luận "có cycle"

✅ Cạnh 0-1 là cạnh 2 chiều, **không phải cycle**. Phải skip parent!

💡 Undirected: chỉ kết luận cycle khi gặp visited node mà **không phải parent**. Directed: dùng 3-color (Gray = cycle)

---

## Khi nào DFS vs BFS?

| Tình huống | DFS? | BFS? | Tại sao? |
|------------|------|------|----------|
| Cycle detection | ✅ | ⚠️ | DFS back edge = cycle, trực tiếp |
| Topological sort | ✅ | ⚠️ (Kahn's) | DFS post-order = topo sort |
| Connected components | ✅ | ✅ | Cả hai đều OK |
| Backtracking (Sudoku, N-queens) | ✅ | ❌ | DFS + undo = backtracking |
| Path existence | ✅ | ✅ | Cả hai đều OK |
| All paths giữa 2 nodes | ✅ | ❌ | DFS + backtrack liệt kê path |
| Shortest path (unweighted) | ❌ | ✅ | BFS đảm bảo shortest |
| Level-order / distance | ❌ | ✅ | BFS biết level |
| Bipartite check | ⚠️ | ✅ | BFS + 2-coloring dễ hơn |
| Maze solving (any path) | ✅ | ✅ | DFS memory ít hơn |

**Tóm lại:** Cần shortest path → BFS. Cần cycle/topo/backtrack → DFS. Không chắc → thường DFS đơn giản hơn.

---

## Luyện nhận diện Pattern

3 bài kinh điển, mỗi bài dùng 1 ứng dụng DFS khác nhau:

### a) Number of Islands (LeetCode #200)

Cho 2D grid gồm '1' (đất) và '0' (nước), đếm số đảo.

```
Grid:
  1 1 0 0 0
  1 1 0 0 0
  0 0 1 0 0
  0 0 0 1 1

→ 3 đảo
```

**Gợi ý:** Mỗi ô '1' chưa visited → DFS flood fill (thăm tất cả ô '1' kề nhau) → 1 island. Đếm bao nhiêu lần bắt đầu DFS = **connected components** trên grid.

### b) Course Schedule (LeetCode #207)

Cho n courses và prerequisites, có thể hoàn thành tất cả courses không?

```
n=4, prerequisites: [[1,0],[2,0],[3,1],[3,2]]
  Course 1 cần Course 0, Course 2 cần Course 0, ...

0 → 1 → 3
0 → 2 → 3   → OK, không có cycle

Nhưng nếu thêm 3→0 → CYCLE → impossible!
```

**Gợi ý:** Build directed graph, check cycle. Có cycle → impossible. Dùng **DFS 3-color cycle detection**.

### c) Generate Parentheses (LeetCode #22)

Sinh tất cả tổ hợp n cặp ngoặc hợp lệ.

```
n=3 → ["((()))","(()())","(())()","()(())","()()()"]
```

**Gợi ý:** **Backtracking**. State = current string + open count + close count. Choice: thêm `(` nếu `open < n`, thêm `)` nếu `close < open`. DFS trên cây quyết định "thêm ( hay )?"

---

## DFS trong Rust ecosystem

### Rust-specific

- **Recursion limit**: Rust default stack size = 8MB. Deep recursion → stack overflow. Dùng `stacker` crate để mở rộng stack, hoặc chuyển sang iterative DFS
- **Ownership**: recursive DFS cần `&mut visited` -- Rust borrow checker đảm bảo không có data race

### petgraph crate

```rust
// petgraph -- thư viện graph phổ biến nhất của Rust
use petgraph::algo;

algo::has_path_connecting(&graph, a, b, None)  // DFS check path
algo::toposort(&graph, None)                    // topological sort
algo::kosaraju_scc(&graph)                      // strongly connected components
```

### Ứng dụng: Dependency resolution

```
cargo build:
  1. Đọc Cargo.toml → build dependency graph
  2. Topological sort → thứ tự compile
  3. Circular dependency? → DFS cycle detection → báo lỗi

Tương tự: npm, pip, apt-get -- tất cả đều dùng DFS/topological sort
```

---

## Tiếp theo: Dijkstra's Algorithm

BFS tìm shortest path trong graph **không trọng số**. Nhưng bản đồ thực tế mỗi đường có **khoảng cách khác nhau**. Đường 1km và đường 10km không nên coi bằng nhau.

Chương tiếp theo giới thiệu **Dijkstra's Algorithm** -- BFS "nâng cấp" cho weighted graph, dùng **Priority Queue** thay Queue. Nếu bạn hiểu BFS + Priority Queue (chương trước), bạn đã sẵn sàng cho Dijkstra.

---

---

[← BFS](./02-bfs.md) | [Dijkstra →](./04-dijkstra.md)
