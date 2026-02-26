# Thuật toán Floyd-Warshall

## Đây là gì?

Bạn đã từng thấy **bảng khoảng cách giữa các thành phố** trên bản đồ chưa? Kiểu như:

```
         Hà Nội   Đà Nẵng   Sài Gòn
Hà Nội      0       764       1726
Đà Nẵng    764       0        962
Sài Gòn   1726      962        0
```

Bảng này cho ta khoảng cách ngắn nhất giữa **TẤT CẢ các cặp** thành phố cùng lúc. Đó chính là bài toán mà **Floyd-Warshall** giải quyết.

Dijkstra và Bellman-Ford trả lời: "Đường ngắn nhất **từ 1 điểm** đến mọi nơi?"
Floyd-Warshall trả lời: "Đường ngắn nhất giữa **MỌI cặp** điểm?"

Ý tưởng rất đẹp -- dùng **dynamic programming** (quy hoạch động): Với mỗi đỉnh trung gian `k`, thử xem đi qua `k` có rút ngắn đường từ `i` đến `j` không?

## Hoạt động như thế nào?

### Công thức DP

```
dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])

Nghĩa là: đi thẳng từ i→j hay đi vòng qua k (i→k→j), cái nào ngắn hơn?
```

Ta thử với **mọi** đỉnh trung gian k, từ 0 đến V-1. Sau khi xét hết, `dist[i][j]` chính là khoảng cách ngắn nhất thật sự.

### Từng bước trên graph nhỏ

```
Graph:
  0 --(3)--> 1
  0 --(6)--> 2
  1 --(2)--> 2
  2 --(1)--> 3
  1 --(8)--> 3
```

**Ma trận khoảng cách ban đầu** (INF = không có cạnh trực tiếp):

```
     0    1    2    3
0 [  0    3    6  INF ]    ← 0 đến 1 = 3, đến 2 = 6
1 [INF    0    2    8 ]    ← 1 đến 2 = 2, đến 3 = 8
2 [INF  INF    0    1 ]    ← 2 đến 3 = 1
3 [INF  INF  INF    0 ]    ← 3 không đi đâu được
```

**Sau k=0** (thử đi qua đỉnh 0):
Không cải thiện gì. Vì không ai đi qua 0 được rút ngắn (0 không có cạnh vào từ 1, 2, 3).

**Sau k=1** (thử đi qua đỉnh 1):
```
dist[0][2] = min(6, 3+2) = 5   ← Đi 0→1→2 ngắn hơn 0→2 trực tiếp!
dist[0][3] = min(INF, 3+8) = 11

     0    1    2    3
0 [  0    3    5   11 ]
1 [INF    0    2    8 ]
2 [INF  INF    0    1 ]
3 [INF  INF  INF    0 ]
```

**Sau k=2** (thử đi qua đỉnh 2):
```
dist[0][3] = min(11, 5+1) = 6   ← Đi 0→1→2→3 ngắn hơn!
dist[1][3] = min(8, 2+1) = 3    ← Đi 1→2→3 ngắn hơn 1→3 trực tiếp!

     0    1    2    3
0 [  0    3    5    6 ]
1 [INF    0    2    3 ]
2 [INF  INF    0    1 ]
3 [INF  INF  INF    0 ]
```

**Sau k=3**: Không cải thiện.

Kết quả cuối cùng cho ta **bảng khoảng cách** hoàn chỉnh giữa tất cả các cặp đỉnh.

### Tại sao 3 vòng lặp lồng nhau?

```
for k in 0..V {          ← Vòng ngoài: thử mỗi đỉnh trung gian
    for i in 0..V {      ← Vòng giữa: mỗi đỉnh xuất phát
        for j in 0..V {  ← Vòng trong: mỗi đỉnh đích
            // Đi qua k có rút ngắn i→j không?
        }
    }
}
```

Đơn giản nhưng mạnh mẽ. O(V^3) -- chấp nhận được khi V không quá lớn.

## Code Rust

Code đầy đủ nằm trong `src/graph.rs`.

```rust
pub fn floyd_warshall(&self) -> Vec<Vec<i64>> {
    let n = self.num_vertices;
    let mut dist = vec![vec![i64::MAX; n]; n];

    // Khoảng cách đến chính mình = 0
    for i in 0..n {
        dist[i][i] = 0;
    }
    // Khởi tạo từ các cạnh trực tiếp
    for u in 0..n {
        for &(v, w) in &self.adjacency_list[u] {
            if w < dist[u][v] {
                dist[u][v] = w;
            }
        }
    }

    // 3 vòng lặp thần thánh
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                    let through_k = dist[i][k] + dist[k][j];
                    if through_k < dist[i][j] {
                        dist[i][j] = through_k;
                    }
                }
            }
        }
    }
    dist
}
```

Giải thích:

- Khởi tạo đường chéo = 0 (khoảng cách đến chính mình), cạnh trực tiếp = trọng số, còn lại = INF
- Kiểm tra `!= i64::MAX` tránh tràn số khi cộng với "vô cực"
- Nếu có cạnh song song giữa 2 đỉnh, lấy cạnh có trọng số nhỏ nhất

## Độ phức tạp

| Khía cạnh | Độ phức tạp |
|---|---|
| Thời gian | O(V^3) |
| Bộ nhớ | O(V^2) cho ma trận khoảng cách |
| Xử lý trọng số âm? | Có |
| Phát hiện negative cycle? | Có (kiểm tra đường chéo < 0) |

### Khi nào dùng thuật toán nào?

| Bài toán | Thuật toán tốt nhất |
|---|---|
| 1 nguồn, không trọng số âm | Dijkstra |
| 1 nguồn, có trọng số âm | Bellman-Ford |
| Tất cả cặp, graph dày | Floyd-Warshall |
| Tất cả cặp, graph thưa | Chạy Dijkstra từ mỗi đỉnh |

Floyd-Warshall tốt nhất khi cần khoảng cách tất cả cặp và V không quá lớn (vài nghìn đỉnh). Code đơn giản, dễ viết, ít lỗi.

## Ví dụ

```rust
use rust_ds2a::graph::Graph;

let mut g = Graph::new(4, true);
g.add_edge(0, 1, 3);
g.add_edge(0, 2, 6);
g.add_edge(1, 2, 2);
g.add_edge(2, 3, 1);
g.add_edge(1, 3, 8);

let dist = g.floyd_warshall();
assert_eq!(dist[0][1], 3);        // trực tiếp
assert_eq!(dist[0][2], 5);        // 0→1→2
assert_eq!(dist[0][3], 6);        // 0→1→2→3
assert_eq!(dist[1][3], 3);        // 1→2→3
assert_eq!(dist[3][0], i64::MAX); // không có đường từ 3 về 0
```
