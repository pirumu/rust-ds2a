# Biểu diễn đồ thị (Graph Representations)

## Đây là gì?

Hãy tưởng tượng bạn đang nhìn **bản đồ đường phố** của thành phố. Mỗi ngã tư là một điểm, mỗi con đường nối hai ngã tư lại với nhau. Đó chính là **graph** (đồ thị).

Hoặc nghĩ về **mạng xã hội** -- mỗi người là một điểm, khi hai người kết bạn thì có một đường nối giữa họ.

**Graph** gồm hai thành phần:
- **Vertex** (đỉnh) -- như ngã tư trên bản đồ, hay một người trên mạng xã hội
- **Edge** (cạnh) -- như con đường nối hai ngã tư, hay quan hệ bạn bè giữa hai người

Khác với tree (cây), graph có thể có **cycle** (vòng lặp) -- tức là đi theo các cạnh rồi quay lại được điểm xuất phát. Và graph không cần có "gốc" như cây.

Câu hỏi quan trọng: lưu graph trong máy tính bằng cách nào? Có hai cách phổ biến: **adjacency list** (danh sách kề) và **adjacency matrix** (ma trận kề). Mỗi cách có ưu nhược khác nhau.

## Hoạt động như thế nào?

### Thuật ngữ cơ bản

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

### Adjacency list (danh sách kề)

Giống như **danh bạ điện thoại** -- mỗi người lưu danh sách bạn bè của mình.

Mỗi đỉnh giữ một danh sách các đỉnh kề (hàng xóm) và trọng số cạnh. Với đồ thị vô hướng ở trên:

```
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

### Adjacency matrix (ma trận kề)

Giống như **bảng tính Excel** -- hàng là đỉnh đi, cột là đỉnh đến. Ô có giá trị 1 nghĩa là có cạnh.

```
    0  1  2  3
0 [ 0  1  1  0 ]    ← 0 nối với 1 và 2
1 [ 1  0  1  1 ]    ← 1 nối với 0, 2, 3
2 [ 1  1  0  1 ]    ← 2 nối với 0, 1, 3
3 [ 0  1  1  0 ]    ← 3 nối với 1 và 2
```

**Ưu điểm:**
- Kiểm tra cạnh cực nhanh: O(1) -- chỉ cần nhìn ô trong bảng
- Đơn giản, dễ hiểu

**Nhược điểm:**
- Tốn bộ nhớ: O(V^2). Dù graph có ít cạnh vẫn phải tạo bảng to
- Duyệt hàng xóm chậm: O(V) -- phải quét cả hàng

### Khi nào dùng cái nào?

| | Adjacency List | Adjacency Matrix |
|---|---|---|
| Bộ nhớ | O(V + E) | O(V^2) |
| Kiểm tra cạnh | O(degree) | O(1) |
| Duyệt hàng xóm | O(degree) | O(V) |
| Thêm cạnh | O(1) | O(1) |
| Phù hợp với | Graph thưa (ít cạnh) | Graph dày (nhiều cạnh) |

**Thực tế:** Hầu hết graph ngoài đời đều **thưa** (sparse) -- số cạnh E nhỏ hơn nhiều so với V^2. Nên adjacency list là lựa chọn mặc định.

Ví dụ: Facebook có hàng tỷ người dùng (V), nhưng mỗi người chỉ có vài trăm bạn (degree nhỏ). Dùng matrix sẽ cần tỷ x tỷ ô -- lãng phí khủng khiếp!

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

## Độ phức tạp

| Thao tác | Adjacency List | Adjacency Matrix |
|---|---|---|
| Xây graph | O(V + E) | O(V^2) |
| Thêm cạnh | O(1) | O(1) |
| Xóa cạnh | O(degree) | O(1) |
| Kiểm tra cạnh (u, v) | O(degree(u)) | O(1) |
| Duyệt hàng xóm | O(degree(u)) | O(V) |
| Bộ nhớ | O(V + E) | O(V^2) |

**Ý nghĩa thực tế:** Với graph 1 triệu đỉnh, mỗi đỉnh có 10 hàng xóm:
- Adjacency list: lưu ~10 triệu cạnh. Gọn!
- Adjacency matrix: lưu 10^12 ô. Không vừa RAM!

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
