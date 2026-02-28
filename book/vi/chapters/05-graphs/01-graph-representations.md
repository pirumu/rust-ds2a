# Biểu diễn đồ thị (Graph Representations)

> 💡 **Đừng lo lắng:** Graph nghe trừu tượng nhưng bạn đã biết graph rồi -- **tree chính là graph** (graph không cycle, có root). Mọi tree mà bạn học suốt series (BST, AVL, Red-Black, B-Tree) đều là graph dạng đặc biệt. Giờ ta chỉ mở rộng: cho phép cycle, cho phép nhiều đường nối, bỏ khái niệm "root". Phần biểu diễn (chương này) đơn giản: chỉ là **`Vec<Vec<...>>`** (adjacency list) hoặc **`Vec<Vec<bool>>`** (matrix). Phần thuật toán (BFS/DFS, chương sau) mới là nơi thú vị thật sự. Graph là chủ đề **nặng nhất** trong phỏng vấn -- LeetCode có 200+ bài graph. Nắm vững biểu diễn = bước đầu tiên để chinh phục tất cả.

## Đây là gì?

Hãy tưởng tượng bạn đang nhìn **bản đồ đường phố** của thành phố. Mỗi ngã tư là một điểm, mỗi con đường nối hai ngã tư lại với nhau. Đó chính là **graph** (đồ thị).

Hoặc nghĩ về **mạng xã hội** -- mỗi người là một điểm, khi hai người kết bạn thì có một đường nối giữa họ.

**Graph** gồm hai thành phần:
- **Vertex** (đỉnh) -- như ngã tư trên bản đồ, hay một người trên mạng xã hội
- **Edge** (cạnh) -- như con đường nối hai ngã tư, hay quan hệ bạn bè giữa hai người

Khác với tree (cây), graph có thể có **cycle** (vòng lặp) -- tức là đi theo các cạnh rồi quay lại được điểm xuất phát. Và graph không cần có "gốc" như cây.

Câu hỏi quan trọng: lưu graph trong máy tính bằng cách nào? Có ba cách phổ biến: **edge list** (danh sách cạnh), **adjacency list** (danh sách kề), và **adjacency matrix** (ma trận kề). Mỗi cách có ưu nhược khác nhau.

## Tree → Graph: Bạn đã biết graph rồi!

Mọi tree là graph, nhưng **không phải mọi graph là tree**:

```
Tree (chương trước):             Graph (chương này):
  - Có root                       - Không cần root
  - Không cycle                   - Có thể có cycle
  - Parent→children (1→many)      - Any→any (many→many)
  - Unique path giữa 2 node       - Có thể nhiều path
  - N node → N-1 edge             - N node → 0 đến N(N-1)/2 edge

BST:          Graph:
    5            0 --- 1
   / \           |   / |
  3   7          |  /  |
                 2 --- 3

BST là graph connected, acyclic, rooted.
Graph có thể có cycle (0→1→3→2→0), nhiều path (0→1 hoặc 0→2→3→1).
```

Mọi thứ bạn đã học -- thực ra **đều là graph** dạng đặc biệt:

| Bạn đã biết | Thực ra là graph | Loại |
|---|---|---|
| Binary Tree, BST, AVL | Directed acyclic graph (DAG) | Tree |
| Linked List | Path graph | Tree (degenerate) |
| File system | DAG (hoặc tree) | Directed |
| Bản đồ thành phố | Weighted undirected graph | Undirected |
| Mạng xã hội (Facebook) | Undirected graph | Undirected |
| Twitter/Instagram follow | Directed graph | Directed |
| Trang web + hyperlinks | Directed graph | Directed (WWW) |
| Package dependencies (cargo) | DAG | Directed acyclic |
| Kafka topic dependencies | DAG | Directed |

Graph mở ra thế giới mới: từ quan hệ 1-nhiều (tree: parent→children) sang **quan hệ nhiều-nhiều** (graph: bất kỳ node nào có thể nối với bất kỳ node nào).

## Thuật ngữ cơ bản

```
Đồ thị vô hướng                Đồ thị có hướng (digraph)
(đường 2 chiều)                 (đường 1 chiều)

  0 --- 1                         0 ---> 1
  |   / |                         |      |
  |  /  |                         v      v
  | /   |                         2 ---> 3
  2 --- 3
```

- **Vertex** (đỉnh): Một điểm trong graph (0, 1, 2, 3 ở trên)
- **Edge** (cạnh): Đường nối giữa hai đỉnh. Đồ thị *có hướng* thì cạnh có chiều (mũi tên). Đồ thị *vô hướng* thì đi được cả hai chiều
- **Weight** (trọng số): Số gắn trên cạnh, ví dụ khoảng cách hay chi phí. Giống như độ dài đoạn đường trên bản đồ
- **Degree** (bậc): Số cạnh chạm vào một đỉnh. Đồ thị có hướng phân biệt *in-degree* (số cạnh đi vào) và *out-degree* (số cạnh đi ra)
- **Path** (đường đi): Chuỗi các đỉnh nối nhau bằng cạnh
- **Cycle** (chu trình): Đường đi bắt đầu và kết thúc tại cùng một đỉnh

### Dense vs Sparse

Đây là khái niệm **quan trọng nhất** khi chọn cách biểu diễn:

```
Dense (dày đặc): E ≈ V²
  Mỗi node nối gần hết node khác.
  Ví dụ: complete graph, flight routes giữa 50 thành phố lớn
  50 thành phố → tối đa 50×49/2 = 1,225 đường bay. Gần đầy!
  → Adjacency matrix OK

Sparse (thưa): E << V²
  Mỗi node ít neighbor.
  Ví dụ: social network (1B users, mỗi người ~500 bạn)
  E = 250B << (1B)² = 10^18
  → Adjacency list BẮT BUỘC (matrix cần 10^18 ô = ~1 exabyte!)
```

**Thực tế:** Hầu hết graph ngoài đời đều **sparse**. Social network, bản đồ, web -- tất cả đều sparse. Nên adjacency list là lựa chọn mặc định.

### Connected Components

Graph có thể bị chia thành nhiều phần không nối nhau:

```
  0 -- 1    3 -- 4    6
  |         |
  2         5

  3 connected components: {0,1,2}, {3,4,5}, {6}
```

BFS/DFS từ mỗi unvisited node → đếm được components. Ứng dụng: tìm "nhóm bạn" riêng biệt trong mạng xã hội, tìm mạng con tách biệt trong network.

## Undirected vs Directed vs Weighted -- khi nào cái nào?

**Undirected** -- quan hệ 2 chiều:
- Facebook friendship (A kết bạn B → B cũng kết bạn A)
- Đường 2 chiều trên bản đồ
- Handshake (bắt tay → cả hai đều tham gia)

**Directed** -- quan hệ 1 chiều:
- Twitter follow (A follow B ≠ B follow A)
- Đường 1 chiều
- Email gửi, dependency (A phụ thuộc B ≠ B phụ thuộc A)

**Weighted** -- cạnh có "chi phí":
- Khoảng cách (km giữa 2 ngã tư)
- Thời gian (phút di chuyển)
- Bandwidth, cost, latency

**Unweighted** -- chỉ "có nối hay không":
- Bạn bè (có/không, không có "mức độ bạn bè")
- Reachability, connected component

**DAG** (Directed Acyclic Graph) -- có hướng, KHÔNG có cycle:
- Build dependency (`cargo build` phải compile dependency trước)
- Class hierarchy, task scheduling
- Đặc biệt vì: cho phép **topological sort** (sắp xếp thứ tự)

```
Ví dụ DAG -- build order:

  serde ──→ my_lib ──→ my_app
              ↑
  tokio ──────┘

Build order (topological sort): serde → tokio → my_lib → my_app
```

## 3 cách biểu diễn graph

### 1. Edge List -- đơn giản nhất

Chỉ cần một danh sách tất cả các cạnh. Mỗi cạnh = (from, to, weight).

```rust
// Danh sách cạnh: mỗi cạnh = (from, to, weight)
let edges: Vec<(usize, usize, i64)> = vec![
    (0, 1, 5),
    (1, 2, 3),
    (2, 3, 7),
];
```

```
Graph:
  0 --(5)--> 1 --(3)--> 2 --(7)--> 3

Edge list:
  [(0,1,5), (1,2,3), (2,3,7)]
```

**Ưu điểm:** Đơn giản, memory O(E), thêm cạnh O(1).

**Nhược điểm:** "Lấy hàng xóm của node X" phải scan toàn bộ danh sách O(E). Hầu hết graph algorithms cần thao tác này, nên edge list ít dùng trong practice.

**Khi nào dùng:** Kruskal MST (cần sort edges by weight), lưu trữ đơn giản, serialization.

### 2. Adjacency List (danh sách kề) -- phổ biến nhất

Giống như **danh bạ điện thoại** -- mỗi người lưu danh sách bạn bè của mình.

Mỗi đỉnh giữ một danh sách các đỉnh kề (hàng xóm) và trọng số cạnh:

```
Graph vô hướng:
  0 --- 1
  |   / |
  |  /  |
  2 --- 3

đỉnh 0: [(1, 1), (2, 1)]          ← 0 nối với 1 và 2
đỉnh 1: [(0, 1), (2, 1), (3, 1)]  ← 1 nối với 0, 2, 3
đỉnh 2: [(0, 1), (1, 1), (3, 1)]  ← 2 nối với 0, 1, 3
đỉnh 3: [(1, 1), (2, 1)]          ← 3 nối với 1 và 2
```

**Ưu điểm:**
- Tiết kiệm bộ nhớ: O(V + E). Chỉ lưu những cạnh thực sự có
- Duyệt hàng xóm nhanh: O(degree) -- chỉ xem danh sách bạn bè
- Thêm cạnh: O(1)

**Nhược điểm:**
- Kiểm tra "đỉnh A có nối với đỉnh B không?" phải duyệt danh sách: O(degree)

### 3. Adjacency Matrix (ma trận kề)

Giống như **bảng tính Excel** -- hàng là đỉnh đi, cột là đỉnh đến. Ô có giá trị 1 nghĩa là có cạnh.

```
    0  1  2  3
0 [ 0  1  1  0 ]    ← 0 nối với 1 và 2
1 [ 1  0  1  1 ]    ← 1 nối với 0, 2, 3
2 [ 1  1  0  1 ]    ← 2 nối với 0, 1, 3
3 [ 0  1  1  0 ]    ← 3 nối với 1 và 2
```

**Ưu điểm:**
- Kiểm tra cạnh cực nhanh: O(1) -- chỉ cần nhìn ô `matrix[i][j]`
- Đơn giản, dễ hiểu
- Phù hợp cho các thuật toán matrix-based (Floyd-Warshall)

**Nhược điểm:**
- Tốn bộ nhớ: O(V²). Dù graph có ít cạnh vẫn phải tạo bảng to
- Duyệt hàng xóm chậm: O(V) -- phải quét cả hàng

### So sánh 3 cách

| | Edge List | Adj List | Adj Matrix |
|---|---|---|---|
| **Memory** | O(E) | O(V+E) | O(V²) |
| **Check edge (u,v)** | O(E) | O(degree) | **O(1)** |
| **Neighbors of u** | O(E) | **O(degree)** | O(V) |
| **Add edge** | O(1) | O(1) | O(1) |
| **Remove edge** | O(E) | O(degree) | O(1) |
| **Best for** | Kruskal MST, simple storage | BFS/DFS, most algos | Dense graph, matrix ops |

Edge list đơn giản nhất nhưng ít dùng vì hầu hết algorithms cần "lấy hàng xóm của node X" -- edge list phải scan O(E) cho thao tác này. Adjacency list O(degree) cho cùng thao tác.

## HashMap-based Adjacency List

Code trong series này dùng `Vec<Vec<(usize, i64)>>` -- node là số 0..n-1. Nhưng thực tế, node thường **không phải integer**: tên người, URL, địa chỉ IP...

```rust
use std::collections::HashMap;

// === Index-based (series này) -- node là số 0..n-1 ===
let n = 5;
let adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];

// === HashMap-based -- node là bất kỳ type nào ===
let mut adj: HashMap<String, Vec<(String, i64)>> = HashMap::new();

// Ví dụ: social network
adj.entry("Alice".to_string()).or_default()
   .push(("Bob".to_string(), 1));
adj.entry("Bob".to_string()).or_default()
   .push(("Alice".to_string(), 1));
// Alice kết bạn Bob → thêm cả hai chiều (undirected)
```

**Khi nào dùng HashMap:**
- Node có tên (string), UUID, hoặc custom struct
- Node không liên tiếp (ID: 1000, 5000, 999999)
- Node thêm/xóa dynamic (user join/leave)
- Real-world applications

**Khi nào dùng Vec:**
- Node là 0..n-1, biết trước số node
- Competitive programming (nhanh hơn HashMap)
- Performance-critical code

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

Struct chính -- dùng adjacency list:

```rust
pub struct Graph {
    adjacency_list: Vec<Vec<(usize, i64)>>,  // (hàng xóm, trọng số)
    directed: bool,
    num_vertices: usize,
}
```

Tại sao chọn adjacency list? Vì hầu hết bài toán thực tế dùng graph thưa. `Vec<Vec<(usize, i64)>>` lưu cho mỗi đỉnh một danh sách các cặp (đỉnh kề, trọng số).

**Tạo graph và thêm cạnh:**

```rust
impl Graph {
    pub fn new(num_vertices: usize, directed: bool) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); num_vertices],
            directed,
            num_vertices,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i64) {
        self.adjacency_list[from].push((to, weight));
        if !self.directed {
            // Vô hướng → thêm cả chiều ngược
            self.adjacency_list[to].push((from, weight));
        }
    }

    pub fn add_unweighted_edge(&mut self, from: usize, to: usize) {
        self.add_edge(from, to, 1);
    }
}
```

Với graph vô hướng, `add_edge` thêm cạnh vào **cả hai** danh sách kề. Với graph có hướng, chỉ thêm vào danh sách của đỉnh nguồn.

## Graph trong thực tế

### Google Maps / Navigation

```
Graph: giao lộ = vertex, đường = edge (weighted by khoảng cách/thời gian)
Query: tìm đường ngắn nhất A→B = Dijkstra / A*
Graph type: weighted, directed (đường 1 chiều), rất lớn (~1B edges)
Representation: adjacency list (sparse -- mỗi ngã tư chỉ nối 3-4 đường)
```

Khi bạn mở Google Maps, bạn đang query trên graph khổng lồ. Mỗi ngã tư là vertex, mỗi đoạn đường là weighted edge (weight = thời gian di chuyển). "Tìm đường" = shortest path algorithm.

### Social Network (Facebook/LinkedIn)

```
Graph: person = vertex, friendship = edge
Query: "người bạn có thể biết" = BFS depth 2-3
       "bao nhiêu bước từ A đến B" = shortest path (unweighted)
Graph type: undirected, unweighted, sparse (mỗi người ~500 bạn)
Representation: adjacency list (HashMap-based, node là user ID)
```

Facebook có ~3 tỷ users. Dùng adjacency matrix = 3B × 3B = 9 × 10^18 ô. Không vừa bất kỳ RAM nào. Adjacency list: mỗi user chỉ lưu ~500 bạn = 1.5 × 10^12 entries. Khả thi.

### Package Manager (npm, cargo)

```
Graph: package = vertex, dependency = directed edge
Query: build order = topological sort
       circular dependency = cycle detection
Graph type: DAG (directed acyclic -- nếu có cycle → lỗi!)
```

`cargo build` phải compile dependencies **TRƯỚC** package chính. Thứ tự compile = **topological sort** của dependency graph. Nếu A phụ thuộc B phụ thuộc C phụ thuộc A → cycle → `cargo` báo lỗi!

### Web Crawler (Google)

```
Graph: webpage = vertex, hyperlink = directed edge
Query: crawl = BFS/DFS từ seed URLs
       PageRank = importance score dựa trên incoming edges
Graph type: directed, extremely large (~100B+ pages)
```

Google crawler dùng BFS: bắt đầu từ vài URL, theo mỗi link trên trang, thêm vào queue... Đó chính là BFS trên graph of the web.

### Network Routing

```
Graph: router = vertex, connection = edge (weighted by latency)
Query: optimal route = shortest path
Graph type: weighted, directed, dynamic (edges thay đổi theo traffic)
```

Mỗi packet bạn gửi đi trên internet = shortest path trên router graph. OSPF protocol dùng Dijkstra!

## Độ phức tạp

| Thao tác | Edge List | Adjacency List | Adjacency Matrix |
|---|---|---|---|
| Xây graph | O(E) | O(V + E) | O(V²) |
| Thêm cạnh | O(1) | O(1) | O(1) |
| Xóa cạnh | O(E) | O(degree) | O(1) |
| Kiểm tra cạnh (u, v) | O(E) | O(degree(u)) | O(1) |
| Duyệt hàng xóm | O(E) | O(degree(u)) | O(V) |
| Bộ nhớ | O(E) | O(V + E) | O(V²) |

**Ý nghĩa thực tế:** Với graph 1 triệu đỉnh, mỗi đỉnh có 10 hàng xóm:
- Edge list: lưu ~10 triệu cạnh ≈ 240 MB (24 bytes/edge)
- Adjacency list: lưu ~10 triệu entries ≈ 160 MB. Gọn!
- Adjacency matrix: lưu 10^12 ô ≈ 1 TB. Không vừa RAM!

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

// Graph vô hướng, có trọng số
// Giống bản đồ: 4 ngã tư, nối bằng đường có chiều dài
let mut g = Graph::new(4, false);
g.add_edge(0, 1, 5);   // đường từ 0 đến 1, dài 5km
g.add_edge(1, 2, 3);   // đường từ 1 đến 2, dài 3km
g.add_edge(2, 3, 7);   // đường từ 2 đến 3, dài 7km

assert_eq!(g.num_vertices(), 4);
assert_eq!(g.neighbors(0), &[(1, 5)]);
assert_eq!(g.neighbors(1), &[(0, 5), (2, 3)]);  // vô hướng → 1 nối ngược về 0

// Graph có hướng, không trọng số
// Giống mạng xã hội follow: A follow B không có nghĩa B follow A
let mut dg = Graph::new(3, true);
dg.add_unweighted_edge(0, 1);  // 0 follow 1
dg.add_unweighted_edge(1, 2);  // 1 follow 2
assert_eq!(dg.neighbors(0), &[(1, 1)]);
assert!(dg.neighbors(2).is_empty()); // 2 không follow ai
```

**Trace qua xây graph vô hướng:**

```
Bước 1: Graph::new(4, false)
  adj = [[], [], [], []]    ← 4 đỉnh, chưa có cạnh

Bước 2: add_edge(0, 1, 5)
  adj[0].push((1, 5))       ← 0 → 1, weight 5
  adj[1].push((0, 5))       ← 1 → 0, weight 5 (vô hướng!)
  adj = [[(1,5)], [(0,5)], [], []]

Bước 3: add_edge(1, 2, 3)
  adj[1].push((2, 3))
  adj[2].push((1, 3))
  adj = [[(1,5)], [(0,5),(2,3)], [(1,3)], []]

Bước 4: add_edge(2, 3, 7)
  adj[2].push((3, 7))
  adj[3].push((2, 7))
  adj = [[(1,5)], [(0,5),(2,3)], [(1,3),(3,7)], [(2,7)]]

Graph:
  0 --(5)-- 1
             |
            (3)
             |
  3 --(7)-- 2
```

## Những cái bẫy hay gặp

### 1. Quên thêm cạnh ngược cho undirected graph

❌ **Sai:**
```rust
// Undirected graph nhưng chỉ thêm 1 chiều
adj[0].push(1);
// Thiếu: adj[1].push(0);
```

✅ **Đúng:**
```rust
// Undirected → LUÔN thêm cả hai chiều
adj[0].push(1);
adj[1].push(0);
// Hoặc dùng hàm add_edge đã xử lý tự động
```

💡 Đây là nguồn bug **phổ biến nhất** khi code graph. BFS/DFS sẽ cho kết quả sai vì không duyệt được ngược. Code trong series đã xử lý tự động trong `add_edge`.

### 2. Dùng adjacency matrix cho sparse graph

❌ **Sai:**
```rust
// 10K node × 10K node matrix = 100M ô ≈ 100MB (boolean)
let matrix = vec![vec![false; 10_000]; 10_000];
// Nhưng mỗi node chỉ có 10 neighbor!
```

✅ **Đúng:**
```rust
// Adjacency list: chỉ lưu edges thực sự có
// 10K × 10 = 100K entries ≈ 2MB. Tiết kiệm 50x!
let adj: Vec<Vec<usize>> = vec![vec![]; 10_000];
```

💡 Rule of thumb: nếu E < V²/10, dùng adjacency list. Hầu hết graph ngoài đời đều sparse.

### 3. Off-by-one với 0-indexed vs 1-indexed

❌ **Sai:**
```rust
// Đề bài: "node 1 to 5", nhưng Vec index từ 0
let adj: Vec<Vec<usize>> = vec![vec![]; 5];
adj[5].push(1); // PANIC! index out of bounds
```

✅ **Đúng:**
```rust
// Cách 1: allocate n+1 (bỏ index 0)
let adj: Vec<Vec<usize>> = vec![vec![]; 6]; // index 0-5
adj[5].push(1); // OK

// Cách 2: convert sang 0-indexed
let adj: Vec<Vec<usize>> = vec![vec![]; 5]; // index 0-4
adj[5 - 1].push(1 - 1); // adj[4].push(0)
```

💡 Nguồn bug #1 trong competitive programming! Đề bài thường đánh số node từ 1. Vec index từ 0. Phải +1 size hoặc -1 index.

### 4. Quên handle self-loop và multi-edge

❌ **Sai:** Assume graph luôn "simple" (không self-loop, không multi-edge).

✅ **Đúng:** Đọc kỹ đề bài. "Simple graph" = không self-loop, không multi-edge. Nếu đề không nói → phải handle cả hai.

💡 Adjacency list cho phép cả self-loop lẫn multi-edge. Adjacency matrix chỉ lưu 1 cạnh giữa 2 node (trừ khi dùng `i32` thay `bool` để đếm).

### 5. Dùng Vec khi node không phải integer

❌ **Sai:**
```rust
// Node là String, nhưng dùng Vec → phải convert thủ công
let adj: Vec<Vec<usize>> = vec![vec![]; n];
// "Alice" = 0, "Bob" = 1, ... → error-prone
```

✅ **Đúng:**
```rust
// Dùng HashMap khi node không phải integer
let mut adj: HashMap<String, Vec<String>> = HashMap::new();
adj.entry("Alice".into()).or_default().push("Bob".into());
```

💡 Hoặc tạo mapping `HashMap<String, usize>` trước, rồi dùng Vec index. Phổ biến trong competitive programming khi cần performance.

## Khi nào dùng cách biểu diễn nào?

| Tình huống | Representation | Tại sao? |
|---|---|---|
| BFS/DFS traversal | Adj List | Cần iterate neighbors nhanh |
| Dijkstra shortest path | Adj List | Iterate neighbors + weights |
| Floyd-Warshall (all pairs) | Adj Matrix | Matrix operations |
| Kruskal MST | Edge List | Sort edges by weight |
| Social network (sparse) | Adj List (HashMap) | Nodes là string, sparse |
| Flight connections (dense, ~50 cities) | Adj Matrix OK | 50×50 matrix nhỏ |
| Dynamic graph (add/remove nodes) | HashMap Adj List | Vec resize khó |
| Competitive programming | Vec Adj List | Nhanh nhất, index-based |
| Simple storage / serialization | Edge List | Đơn giản nhất |

**Quy tắc chung:**
- Mặc định → **Adjacency List** (phù hợp hầu hết trường hợp)
- Graph nhỏ + dense → xét Adjacency Matrix
- Chỉ cần edges (sort, filter) → Edge List

## Luyện nhận diện Pattern

### Bài 1: Model as Graph

Cho 5 scenario, xác định vertex, edge, directed hay undirected, weighted hay không:

| Scenario | Vertex | Edge | Directed? | Weighted? |
|---|---|---|---|---|
| Hệ thống flight booking | Sân bay | Chuyến bay | Directed (HN→SG ≠ SG→HN về giá) | Weighted (giá vé, thời gian) |
| Compiler dependency resolution | Source file | `#include` / `use` | Directed (A dùng B ≠ B dùng A) | Unweighted |
| Social media "mutual friends" | Người dùng | Quan hệ bạn bè | Undirected (mutual) | Unweighted |
| Network packet routing | Router | Connection | Directed | Weighted (latency, bandwidth) |
| Course prerequisites | Môn học | Prerequisite | Directed (Calculus → Physics) | Unweighted |

### Bài 2: Build adjacency list từ edge list

Edge list (directed): `[(0,1), (1,2), (2,0), (2,3)]`

```
Graph:
  0 → 1
  ↑   ↓
  2 ← ┘
  ↓
  3

Adjacency list:
  0: [1]
  1: [2]
  2: [0, 3]
  3: []

Adjacency matrix:
      0  1  2  3
  0 [ 0  1  0  0 ]
  1 [ 0  0  1  0 ]
  2 [ 1  0  0  1 ]
  3 [ 0  0  0  0 ]

Verify:
  - 4 edges trong edge list → 4 số 1 trong matrix ✓
  - out-degree: 0→1, 1→1, 2→2, 3→0
  - in-degree: 0→1, 1→1, 2→1, 3→1
  - Sum out-degree = sum in-degree = E = 4 ✓
```

### Bài 3: Choose representation

**Scenario:** Dataset 1M users, mỗi user trung bình 200 followers. Cần: (1) liệt kê followers nhanh, (2) check "A follow B?" nhanh.

<details>
<summary>Lời giải</summary>

**Option 1: Adjacency List**
- Memory: V + E = 1M + 200M entries ≈ 1.6 GB (8 bytes/entry)
- List followers: O(degree) = O(200) ✓
- Check "A follow B?": O(degree) = O(200) -- phải scan ✗

**Option 2: Adjacency Matrix**
- Memory: V² = (10^6)² = 10^12 ô ≈ 125 GB (1 bit/ô). Không vừa RAM!
- Check edge: O(1) nhưng... memory quá lớn ✗

**Option 3: Hybrid** (tốt nhất!)
- Adjacency List cho list followers: O(degree)
- Thêm `HashSet` per node cho check edge: O(1)
- Memory: 1.6 GB (list) + ~6.4 GB (HashSet overhead) ≈ 8 GB
- Hoặc: dùng `HashMap<UserId, HashSet<UserId>>` -- cả hai thao tác O(1) average

**Kết luận:** Adjacency List + HashSet per node. Tốn thêm memory nhưng cả hai query đều nhanh.

</details>

## Graph trong Rust ecosystem

### `petgraph` crate

Graph library phổ biến nhất trong Rust ecosystem:

```rust
use petgraph::graph::{DiGraph, UnGraph};

// Directed graph
let mut dg = DiGraph::new();
let a = dg.add_node("A");
let b = dg.add_node("B");
dg.add_edge(a, b, 5);

// Undirected graph
let mut ug = UnGraph::new_undirected();
```

- `DiGraph` (directed), `UnGraph` (undirected)
- `StableGraph` -- stable node indices khi remove (node index không thay đổi)
- Built-in: BFS, DFS, Dijkstra, topological sort, connected components
- **Production code → dùng petgraph. Learning/competitive programming → tự implement (như series này)**

### KaCrab connections

Nếu bạn đang build distributed system (như KaCrab -- Kafka clone bằng Rust):

- **Broker cluster topology** = graph (broker = vertex, connection = edge)
- **Consumer group rebalance** = bipartite graph matching (consumer ↔ partition)
- **Topic dependency** (topic A produces to topic B) = DAG
- Cluster monitoring = connected components (detect network partition)

Graph không chỉ là bài tập -- nó là foundation của distributed systems.

## Tiếp theo: BFS & DFS

Biểu diễn graph = **lưu trữ**. Nhưng sức mạnh thật sự nằm ở **thuật toán trên graph**.

Chương tiếp theo sẽ giới thiệu **BFS** (tìm kiếm theo chiều rộng) và **DFS** (tìm kiếm theo chiều sâu) -- 2 thuật toán nền tảng mà hầu hết graph algorithms khác đều xây trên. BFS tìm đường ngắn nhất (unweighted), DFS detect cycle, topological sort, connected components. Nếu bạn hiểu BFS + DFS, bạn giải được **80% bài graph trong phỏng vấn**.

---

---

[← Consistent Hashing](../04-hashing/04-consistent-hashing.md) | [BFS →](./02-bfs.md)
