# Thuật toán Bellman-Ford

> 💡 **Đừng lo lắng:** Bellman-Ford nghe phức tạp nhưng thực ra là thuật toán **đơn giản nhất** trong tất cả shortest path algorithms. Core logic chỉ có: *"duyệt tất cả cạnh, thử cải thiện distance, lặp lại V-1 lần."* Không cần Priority Queue, không cần visited array, không cần BTreeMap. Chỉ cần **2 vòng for lồng nhau**. Nếu bạn hiểu Dijkstra (chương trước), Bellman-Ford còn đơn giản hơn -- đánh đổi tốc độ lấy sự đơn giản. Bellman-Ford ít xuất hiện trong coding interview hơn Dijkstra, nhưng hay gặp trong system design (currency exchange, arbitrage detection) và là nền tảng cho distance-vector routing (giao thức RIP).

## Đây là gì?

Dijkstra như Google Maps bình thường -- chỉ xử lý được đường có chi phí dương (khoảng cách, thời gian). Nhưng đời thực có lúc **đi một đoạn lại được tiền**.

Hãy tưởng tượng bạn giao hàng giữa các thành phố. Hầu hết chặng đường tốn xăng (chi phí dương). Nhưng có chặng bạn **chở hàng thuê dọc đường, tiền công nhận được nhiều hơn tiền xăng** -- tổng chi phí chặng đó thành **âm**. Đó chính là **trọng số âm** (negative weight). Dijkstra không xử lý được, nhưng **Bellman-Ford** thì có thể.

**Bellman-Ford** giải cùng bài toán với Dijkstra (đường ngắn nhất từ 1 nguồn), nhưng thêm 2 khả năng:
1. Xử lý **trọng số âm**
2. Phát hiện **negative cycle** (chu trình âm)

**Negative cycle** là gì? Là một vòng lặp mà đi quanh 1 vòng lại **giảm chi phí**. Giống như có 3 thành phố mà cứ chạy vòng A->B->C->A, mỗi vòng bạn lại lãi thêm tiền -- chạy mãi, lãi mãi, không bao giờ dừng. Khi có negative cycle, **không tồn tại đường ngắn nhất**.

Cái giá phải trả: Bellman-Ford chậm hơn Dijkstra. O(V * E) vs O((V + E) log V).

---

## Dijkstra → Bellman-Ford: tại sao cần thuật toán mới?

Đặt 2 thuật toán cạnh nhau:

```
Dijkstra:                          Bellman-Ford:
  Greedy: xử lý node gần nhất       Brute force: thử cải thiện MỌI cạnh
  Cần Priority Queue (complex)       Chỉ cần 2 vòng for (simple!)
  O((V+E) log V) -- nhanh           O(V × E) -- chậm
  Negative weights: ❌                Negative weights: ✅
  Negative cycle detect: ❌           Negative cycle detect: ✅
```

```
Dijkstra:                          Bellman-Ford:
  "Thông minh, nhanh,               "Đơn giản, chậm,
   nhưng kén graph"                   nhưng xử lý mọi graph"
```

Quay lại ẩn dụ giao hàng: Dijkstra như tài xế thông minh, mỗi lần chọn đường ngắn nhất phía trước và **không bao giờ quay lại**. Bellman-Ford như tài xế kiên nhẫn, cứ **thử đi lại tất cả** các chặng nhiều lần -- chậm hơn, nhưng chắc chắn tìm được đường rẻ nhất dù có chặng "giảm giá".

### Tại sao Dijkstra fail với negative weight?

```
Graph: A --1--> B, A --5--> C --(-10)--> B

Dijkstra: xử lý B(dist=1) trước C(dist=5)
  → B "xong" với dist=1, đánh dấu "đã tối ưu"
  → Sau đó tìm C→B: 5+(-10) = -5 < 1
  → Nhưng B đã bị "lock"! Bỏ lỡ đường tốt hơn!

  Dijkstra trả về: dist[B] = 1  ← SAI!
  Đáp án đúng:     dist[B] = -5  (A→C→B = 5+(-10) = -5)

Bellman-Ford: thử TẤT CẢ cạnh V-1 lần, không "lock" gì cả
  → Vòng 1: relax A→B: dist[B]=1, relax A→C: dist[C]=5
  → Vòng 2: relax C→B: dist[B] = min(1, 5+(-10)) = -5 ✓
  → Tìm được đường tốt hơn nhờ "kiên nhẫn thử lại"
```

Dijkstra giả sử: *"node đã xử lý = đã tối ưu"*. Negative weight phá vỡ giả sử này. Bellman-Ford không có giả sử đó -- cứ thử lại, thử lại, cho đến khi chắc chắn.

---

## Hoạt động như thế nào?

### Ý tưởng cốt lõi

Rất đơn giản: **thử cải thiện mọi cạnh**, lặp lại V-1 lần.

Quay lại ẩn dụ giao hàng: bạn có danh sách TẤT CẢ các chặng đường giữa các thành phố (cạnh). Mỗi "vòng", bạn xem lại từng chặng: *"nếu đi qua chặng này, chi phí đến đích có giảm không?"* Nếu giảm → cập nhật. Lặp lại V-1 lần là đủ.

### Tại sao V-1 lần? -- Trực quan

```
Shortest path dài nhất có thể: V-1 cạnh
(V nodes, mỗi node đi qua 1 lần = tối đa V-1 edges)

Mỗi vòng relax "mở rộng" shortest path thêm 1 cạnh:

Graph: 0 → 1 → 2 → 3 → 4  (worst case: path dài V-1=4 cạnh)
       w=1  w=2  w=3  w=4

Khởi tạo: dist = [0, INF, INF, INF, INF]

Vòng 1: relax edge 0→1 → dist[1] = 0+1 = 1       (path 1 cạnh ✓)
         các cạnh khác: dist[u]=INF → skip

Vòng 2: relax edge 1→2 → dist[2] = 1+2 = 3       (path 2 cạnh ✓)

Vòng 3: relax edge 2→3 → dist[3] = 3+3 = 6       (path 3 cạnh ✓)

Vòng 4: relax edge 3→4 → dist[4] = 6+4 = 10      (path 4 cạnh = V-1 ✓)

Sau V-1 vòng, mọi shortest path (dù dài V-1 cạnh) đều đã được tìm.
```

**Tại sao vòng thứ V phát hiện negative cycle?**

```
Nếu vòng thứ V vẫn cải thiện distance → path dài hơn V-1 cạnh
→ phải đi qua node TRÙNG LẶP → đi vòng → có CYCLE
→ mà cycle này lại giảm cost → NEGATIVE CYCLE!
```

> **Lưu ý:** Trong trường hợp tốt nhất (thứ tự cạnh thuận lợi), Bellman-Ford có thể converge sớm hơn V-1 vòng. Nhưng worst case cần đủ V-1 vòng.

### Từng bước trên graph nhỏ

```
Graph (có hướng, có trọng số âm):
  0 --(1)--> 1 --(-3)--> 2 --(2)--> 3
  0 --(4)--> 2
                   ↑ giảm giá!

Các cạnh: (0,1,1), (1,2,-3), (0,2,4), (2,3,2)
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

Đường 0→1→2 có chi phí 1+(-3) = -2. Rẻ hơn đường 0→2 trực tiếp (chi phí 4)! Như kiểu: đi đường vòng qua thành phố 1, nhưng nhờ chở hàng thuê (giảm giá -3), tổng chi phí lại rẻ hơn đi thẳng.

---

## Phát hiện negative cycle -- trace chi tiết

Sau V-1 vòng, chạy **thêm 1 vòng nữa**. Nếu vẫn cải thiện được khoảng cách nào đó, có nghĩa là tồn tại negative cycle.

```
Graph có negative cycle:
  0 --(2)--> 1 --(-3)--> 2 --(-1)--> 0
                                       ↑ quay lại!
Cycle: 0→1→2→0, total weight = 2+(-3)+(-1) = -2

Giống như: giao hàng vòng 3 thành phố, mỗi vòng LÃI 2 đồng!
```

```
Khởi tạo: dist = [0, INF, INF]

Vòng 1:
  relax 0→1: dist[1] = 0+2 = 2
  relax 1→2: dist[2] = 2+(-3) = -1
  relax 2→0: dist[0] = min(0, -1+(-1)) = -2     ← dist[0] GIẢM!
  dist = [-2, 2, -1]

Vòng 2 (V-1=2):
  relax 0→1: dist[1] = min(2, -2+2) = 0          ← cải thiện!
  relax 1→2: dist[2] = min(-1, 0+(-3)) = -3      ← cải thiện!
  relax 2→0: dist[0] = min(-2, -3+(-1)) = -4     ← cải thiện!
  dist = [-4, 0, -3]

Vòng 3 (kiểm tra -- vòng thứ V):
  relax 0→1: dist[1] = min(0, -4+2) = -2         ← VẪN CẢI THIỆN!
  → NEGATIVE CYCLE DETECTED! ⚠️

Mỗi vòng, dist giảm thêm 2 (= |cycle weight|).
Chạy mãi → dist → -∞. Không tồn tại shortest path.
```

Quay lại ẩn dụ: bạn phát hiện tuyến giao hàng 3 thành phố mà mỗi vòng lãi 2 đồng. Lý thuyết thì bạn chạy vô hạn vòng để lãi vô hạn -- nên không có "chi phí thấp nhất". Bellman-Ford phát hiện điều này và trả về `None`.

---

## SPFA -- Bellman-Ford tăng tốc

Bellman-Ford gốc mỗi vòng relax **TẤT CẢ** E cạnh -- kể cả cạnh vô ích (dist[u] không đổi từ vòng trước). SPFA (Shortest Path Faster Algorithm) tối ưu: **chỉ relax cạnh từ node vừa được update**.

```
SPFA = Bellman-Ford + Queue

Bellman-Ford: mỗi vòng duyệt TẤT CẢ E cạnh       → lãng phí
SPFA: chỉ duyệt cạnh từ node có dist vừa thay đổi  → thông minh hơn
```

```
Pseudocode:
  queue = [start]
  in_queue = {start}

  while queue not empty:
    u = queue.pop_front()
    in_queue.remove(u)
    for each (v, w) in neighbors(u):
      if dist[u] + w < dist[v]:
        dist[v] = dist[u] + w
        if v not in in_queue:
          queue.push_back(v)
          in_queue.insert(v)

Average case: O(E) -- gần như linear!
Worst case: vẫn O(VE) -- nhưng hiếm gặp
```

Ẩn dụ: Bellman-Ford gốc như kiểm tra lại **mọi** chặng giao hàng mỗi ngày. SPFA chỉ kiểm tra lại những chặng **liên quan đến thành phố vừa có thay đổi chi phí** -- hiệu quả hơn nhiều!

> **Lưu ý:** SPFA rất phổ biến trong competitive programming. Nhưng cẩn thận -- worst case vẫn O(VE), và đã có test cases cố tình làm SPFA chậm. Trong contest, nếu bị TLE với SPFA, hãy thử Bellman-Ford gốc hoặc Dijkstra (nếu không có negative weight).

---

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

So sánh với Dijkstra -- code ngắn hơn nhiều:

```
Dijkstra cần:                    Bellman-Ford cần:
  BinaryHeap                       Vec<i64> (dist)
  Reverse wrapper                  Vec<(usize, usize, i64)> (edges)
  Stale entry check                2 vòng for lồng nhau
  Pop + push logic                 1 vòng kiểm tra cuối
  ~15 dòng logic                   ~10 dòng logic
```

---

## Bellman-Ford trong thực tế

### Currency Arbitrage Detection

Đây là ứng dụng hay nhất và elegant nhất của Bellman-Ford.

```
Currencies: USD, EUR, JPY
Exchange rates:
  1 USD → 0.85 EUR
  1 EUR → 130 JPY
  1 JPY → 0.0077 USD

Chuyển thành graph (dùng -log để biến nhân thành cộng):
  USD→EUR: -ln(0.85) = 0.163
  EUR→JPY: -ln(130)  = -4.868
  JPY→USD: -ln(0.0077) = 4.867

Cycle: USD→EUR→JPY→USD
  Weight: 0.163 + (-4.868) + 4.867 = 0.162 > 0
  → Không có arbitrage (đổi 1 vòng bị lỗ)

Nhưng nếu tỷ giá thay đổi và cycle weight < 0:
  → ARBITRAGE OPPORTUNITY!
  Ví dụ: $1000 → €850 → ¥110,500 → $1,003
  Lãi $3 mỗi vòng. Chạy nhanh trước khi rate thay đổi!

Bellman-Ford detect negative cycle trên currency graph
= detect arbitrage opportunity
```

Các quỹ hedge fund và trading firm chạy Bellman-Ford real-time trên ma trận tỷ giá để tìm arbitrage. Tốc độ là tất cả -- ai detect trước, người đó lãi.

### Distance-Vector Routing Protocol (RIP)

```
Mỗi router = node, link = edge
Mỗi router chạy Bellman-Ford locally:
  dist[neighbor] = min(current, link_cost + neighbor's dist)
Chia sẻ bảng dist[] với neighbors → converge to shortest paths

Đây là nền tảng của RIP (Routing Information Protocol).

Trade-off vs Link-State (OSPF -- dùng Dijkstra):
  Distance-Vector (BF): đơn giản, ít memory, nhưng converge chậm
  Link-State (Dijkstra): phức tạp, nhiều memory, nhưng converge nhanh
```

### Negative weights trong thực tế

```
Network flow: reverse edges có negative weight
  → Bellman-Ford dùng trong Ford-Fulkerson min-cost flow

Game AI: paths qua "bonus zones" giảm cost
  → Tìm đường đi qua nhiều bonus nhất

Financial: transactions giảm total cost (rebates, cashback)
  → Model discount chain as negative weight edges
```

---

## Shortest Path Family -- Toàn cảnh

Bạn đã học đủ 3/4 shortest path algorithms. Đặt chúng cạnh nhau:

| | BFS | Dijkstra | **Bellman-Ford** | Floyd-Warshall |
|---|-----|---------|----------------|---------------|
| Weight | Unweighted | Non-negative | **Any** | Any |
| Negative cycle | N/A | ❌ | **✅ Detect** | ✅ Detect |
| Source | Single | Single | **Single** | All pairs |
| Time | O(V+E) | O((V+E)logV) | **O(VE)** | O(V³) |
| Code complexity | Simple | Medium | **Simplest** | Simple |
| Data structure | Queue | Priority Queue | **Just arrays** | Matrix |

```
Decision tree -- chọn thuật toán nào?

  Unweighted graph?
    → BFS (nhanh nhất, O(V+E))

  Weighted, non-negative?
    → Dijkstra (nhanh, O((V+E)logV))

  Weighted, có thể có negative weight?
    → Bellman-Ford ✅ (chậm nhưng chắc chắn)

  Cần shortest path giữa MỌI cặp đỉnh?
    → Floyd-Warshall (chương tiếp theo)

  Cần detect negative cycle?
    → Bellman-Ford ✅ (unique capability)
```

---

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
| Code complexity | Cần PQ, Reverse | Chỉ cần arrays |
| Dense graph (E≈V²) | O(V² log V) | O(V³) -- rất chậm |

**Quy tắc:** Trọng số không âm → dùng Dijkstra. Có trọng số âm → dùng Bellman-Ford.

---

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

---

## Những cái bẫy hay gặp

### ❌ Dùng Bellman-Ford khi Dijkstra đủ

```
Graph: không có negative weight
Dùng Bellman-Ford → O(VE)
Dùng Dijkstra → O((V+E)logV)

Ví dụ: V=1000, E=5000
  BF:       1000 × 5000 = 5,000,000 operations
  Dijkstra: 6000 × ~10 = 60,000 operations
  → Dijkstra nhanh hơn ~80 lần!
```

✅ Luôn check trước: có negative weight không? Không → dùng Dijkstra.

💡 Bellman-Ford là "búa tạ" -- dùng được cho mọi thứ nhưng overkill cho graph non-negative. Dùng đúng tool cho đúng việc.

---

### ❌ Quên check `dist[u] != MAX` trước khi relax

```rust
// SAI: overflow!
if dist[u] + w < dist[v] {  // dist[u] = i64::MAX → MAX + w → tràn số!
    dist[v] = dist[u] + w;
}
```

✅ Luôn check:

```rust
if dist[u] != i64::MAX && dist[u] + w < dist[v] {
    dist[v] = dist[u] + w;
}
```

💡 Relax từ node chưa reach được → `i64::MAX + w` overflow → kết quả sai âm thầm. Trong competitive programming, nhiều người dùng sentinel value (`1e18` as i64) thay vì actual `i64::MAX` để tránh overflow khi cộng.

---

### ❌ Nhầm "negative weight" với "negative cycle"

```
Graph: A --(-5)--> B --3--> C
  Có negative weight (-5): ✅ OK!
  Không có cycle → shortest path TỒN TẠI
  Bellman-Ford tìm được: dist = [0, -5, -2]

Graph: A --(-5)--> B --3--> C --(-2)--> A
  Có negative cycle: A→B→C→A = -5+3+(-2) = -4 < 0
  Shortest path KHÔNG TỒN TẠI → Bellman-Ford trả về None
```

✅ Negative weight = OK, Bellman-Ford xử lý tốt. Chỉ negative **cycle** mới là vấn đề.

💡 Nhiều graph thực tế có negative weight (giảm giá, rebate) nhưng không có negative cycle. Bellman-Ford giải bình thường.

---

### ❌ Relax sai thứ tự edges

```
Bellman-Ford đúng với BẤT KỲ thứ tự relax edges nào.
Sẽ converge sau V-1 vòng dù edges relax theo thứ tự nào.
```

✅ Thứ tự không ảnh hưởng correctness, chỉ ảnh hưởng **convergence speed**.

💡 SPFA tối ưu bằng cách chỉ relax edges từ recently-updated nodes. Nhưng standard Bellman-Ford thì thứ tự không quan trọng.

---

## Khi nào dùng Bellman-Ford?

| Tình huống | Bellman-Ford? | Thay bằng gì? | Tại sao? |
|------------|-------------|---------------|----------|
| Graph có negative weights | ✅ | -- | Dijkstra fail với negative |
| Detect negative cycle | ✅ | -- | Unique capability |
| Currency arbitrage | ✅ | -- | Detect profitable cycles |
| Distance-vector routing | ✅ | -- | Distributed, simple |
| Non-negative weights | ❌ | Dijkstra | Dijkstra nhanh hơn nhiều |
| Dense graph (E≈V²) | ⚠️ | Johnson's | Bellman-Ford + Dijkstra combo |
| All-pairs shortest path | ❌ | Floyd-Warshall | BF chỉ single-source |
| Unweighted graph | ❌ | BFS | BFS O(V+E) << BF O(VE) |

---

## Luyện nhận diện Pattern

### Bài 1: Cheapest Flights Within K Stops (LeetCode #787)

> N cities, flights có price, tìm cheapest flight src→dst với tối đa K stops.

**Gợi ý:** Modified Bellman-Ford, chỉ chạy **K+1 vòng** thay vì V-1. Mỗi vòng = 1 hop. Trick quan trọng: **copy dist[] trước mỗi vòng** để tránh "chain relaxation" trong cùng 1 vòng.

```
Ví dụ: K=1 (tối đa 1 stop)
  → Chỉ chạy 2 vòng relax (K+1=2)
  → Vòng 1: direct flights
  → Vòng 2: 1-stop flights
  → KHÔNG copy dist → chain relax có thể tìm 2-stop flight trong 1 vòng!
```

### Bài 2: Negative Weight Edges (GeeksforGeeks)

> Cho directed weighted graph, tìm shortest path từ 0 đến tất cả nodes. Nếu node nằm trên negative cycle → đánh dấu -∞.

**Gợi ý:** Bellman-Ford V-1 vòng, rồi thêm 1 vòng. Mọi node vẫn update ở vòng V = reachable từ negative cycle → đánh dấu `-∞`. Chạy thêm V-1 vòng nữa để propagate `-∞` đến mọi node reachable từ cycle.

### Bài 3: Currency Exchange Arbitrage

> Cho bảng tỷ giá hối đoái giữa N đồng tiền, detect xem có arbitrage opportunity không.

**Gợi ý:** Chuyển exchange rate thành `-log(rate)`. Negative cycle = arbitrage opportunity. Bellman-Ford detect negative cycle = detect arbitrage.

```
rate = 0.85 → weight = -ln(0.85) = 0.163
rate = 130  → weight = -ln(130) = -4.868

Tại sao -log? Vì:
  product of rates = e^(-sum of weights)
  sum of weights < 0 → product > 1 → PROFIT!
```

---

## Bellman-Ford trong Rust ecosystem

```rust
// Bellman-Ford đơn giản nên hay được implement from scratch
// Nhưng nếu cần crate:

// petgraph -- built-in
use petgraph::algo::bellman_ford;

// pathfinding -- alternative
use pathfinding::directed::bellman_ford;
```

Bellman-Ford là thuật toán mà bạn nên implement from scratch (không cần crate) vì code chỉ có ~15 dòng và logic rất rõ ràng.

---

## Preview chương tiếp theo

Bellman-Ford hoàn thiện bộ **single-source** shortest path: BFS (unweighted), Dijkstra (non-negative), Bellman-Ford (any weights). Nhưng nếu bạn cần shortest path giữa **MỌI cặp** đỉnh thì sao? Chạy Bellman-Ford V lần → O(V²E) -- quá chậm. Chương tiếp theo: **Floyd-Warshall** -- thuật toán all-pairs shortest path với code ngắn đến kinh ngạc (3 vòng for lồng nhau), chạy O(V³), và cũng detect được negative cycle.

---

[← Dijkstra](./04-dijkstra.md) | [Floyd-Warshall →](./06-floyd-warshall.md)
