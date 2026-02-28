# Graph Patterns

> 💡 **Đừng lo lắng:** Thở đi. Chương này KHÔNG dạy gì mới. Đây là compilation của những patterns bạn đã học trong Phần 5 — BFS, DFS, Dijkstra, Topological Sort, Union-Find. Mình chỉ tổng hợp lại và chỉ cho bạn cách nhận diện: "Bài này dùng cái gì?" Nếu bạn đã đọc Phần 5, bạn đã biết 80% rồi.

## Đây là gì?

Bạn đã học Graph, BFS, DFS, Union-Find ở Phần 5. Giờ mình áp dụng vào các bài toán phổ biến nhất trong phỏng vấn và thực tế.

Tưởng tượng bạn đã biết cách đi bộ (BFS/DFS) và cách nhóm bạn bè (Union-Find). Giờ mình dùng những kỹ năng đó để trả lời những câu hỏi thú vị hơn:

- **"Chia được 2 đội không?"** → Bipartite Check
- **"Có vòng lặp không?"** → Cycle Detection
- **"Có bao nhiêu nhóm?"** → Connected Components
- **"Học môn nào trước?"** → Course Schedule (Topological Sort)
- **"Thứ tự chữ cái là gì?"** → Alien Dictionary
- **"Copy đồ thị?"** → Clone Graph

> **Tại sao quan trọng?** Đây là những pattern giải được hàng chục bài LeetCode medium/hard. Nắm vững những cái này = giải được phần lớn bài graph trong phỏng vấn. Cuối chương, mình cũng giới thiệu thêm 0-1 BFS, Multi-source BFS và Tarjan SCC cho bạn nào muốn đi sâu hơn.

---

## Flowchart: Đọc đề xong, dùng gì?

Khi gặp bài graph, hỏi mấy câu này theo thứ tự:

```
Đề bài nói gì?
│
├─ "Đường ngắn nhất?"
│   ├─ Không trọng số ──────────→ BFS
│   ├─ Trọng số 0/1 ────────────→ 0-1 BFS (deque)
│   ├─ Trọng số ≥ 0 ────────────→ Dijkstra
│   └─ Có trọng số âm ──────────→ Bellman-Ford
│
├─ "Thứ tự / phụ thuộc / prerequisite?"
│   └─ ──────────────────────────→ Topological Sort (Kahn's)
│
├─ "Nhóm / đảo / connected component?"
│   ├─ Đếm nhóm ────────────────→ DFS/BFS đếm
│   └─ Gộp nhóm / hỏi cùng nhóm → Union-Find
│
├─ "Chia 2 đội / 2 màu?"
│   └─ ──────────────────────────→ Bipartite (BFS 2-color)
│
├─ "Có cycle không?"
│   ├─ Đồ thị có hướng ─────────→ DFS 3-color
│   └─ Đồ thị vô hướng ─────────→ Union-Find
│
└─ "Khoảng cách từ nhiều nguồn?"
    └─ ──────────────────────────→ Multi-source BFS
```

> **Mẹo:** In flowchart này ra giấy, dán cạnh màn hình. Khi luyện bài, thử match đề vào flowchart trước khi code. Làm vài chục bài sẽ thành phản xạ.

---

## Bipartite Check — chia đồ thị thành 2 nhóm

### Bài toán

Cho đồ thị vô hướng. Có thể tô 2 màu sao cho không có 2 đỉnh kề nhau cùng màu không?

**Ví dụ thực tế:** Chia học sinh thành 2 đội. Nếu 2 người ghét nhau thì phải khác đội. Có chia được không?

### Ý tưởng

Dùng BFS tô màu. Bắt đầu từ node bất kỳ, tô màu 1. Tất cả hàng xóm tô màu 2. Hàng xóm của hàng xóm tô lại màu 1. Nếu gặp xung đột (hàng xóm đã cùng màu) → không bipartite.

```
Ví dụ 1 — Bipartite (hình vuông):

  0 --- 1          Tô màu:
  |     |          0: ■ (màu 1)
  3 --- 2          1: □ (màu 2) — hàng xóm của 0
                   2: ■ (màu 1) — hàng xóm của 1
                   3: □ (màu 2) — hàng xóm của 0

  Kiểm tra: mọi cạnh nối ■ với □ → OK ✓

Ví dụ 2 — KHÔNG bipartite (tam giác):

  0 --- 1          Tô màu:
   \   /           0: ■ (màu 1)
    \ /            1: □ (màu 2)
     2             2: phải là ■ (vì kề 1□)
                      nhưng cũng kề 0■ → XUNG ĐỘT! ✗

Quy tắc: Đồ thị có chu trình lẻ → không bipartite.
```

### BFS 2-coloring

```
queue = [0]    color = [1, 0, 0, 0]    (0 = chưa tô)

Pop 0 (color=1):
  hàng xóm 1: chưa tô → tô color=2, push
  hàng xóm 3: chưa tô → tô color=2, push
  color = [1, 2, 0, 2]

Pop 1 (color=2):
  hàng xóm 0: đã tô 1 ≠ 2 → OK
  hàng xóm 2: chưa tô → tô color=1, push
  color = [1, 2, 1, 2]

Pop 3 (color=2):
  hàng xóm 0: đã tô 1 ≠ 2 → OK
  hàng xóm 2: đã tô 1 ≠ 2 → OK

Pop 2 (color=1):
  hàng xóm 1: đã tô 2 ≠ 1 → OK
  hàng xóm 3: đã tô 2 ≠ 1 → OK

Không có xung đột → Bipartite ✓
```

---

## Cycle Detection — Directed Graph (DFS 3 màu)

### Bài toán

Cho đồ thị **có hướng**. Có chu trình không?

**Ví dụ thực tế:** Trong hệ thống build (Makefile), A phụ thuộc B, B phụ thuộc C, C phụ thuộc A → deadlock! Không thể build được.

### Ý tưởng: 3 trạng thái

DFS thông thường chỉ có 2 trạng thái (visited/unvisited). Để phát hiện cycle trong đồ thị có hướng, cần **3 trạng thái**:

- **WHITE (trắng):** chưa thăm
- **GRAY (xám):** đang trong đường DFS hiện tại (trên recursion stack)
- **BLACK (đen):** đã xử lý xong hoàn toàn

Nếu DFS gặp node GRAY → có cycle (vì ta đang quay lại node trên chính đường đi hiện tại).

```
Đồ thị: 0 → 1 → 2 → 0   (có cycle)

DFS từ 0:
  0: WHITE → GRAY
    → đi tới 1
    1: WHITE → GRAY
      → đi tới 2
      2: WHITE → GRAY
        → đi tới 0
        0 đang GRAY! → CÓ CYCLE ✗

Đồ thị: 0 → 1 → 2   (không cycle)

DFS từ 0:
  0: WHITE → GRAY
    1: WHITE → GRAY
      2: WHITE → GRAY
        2: không có hàng xóm → GRAY → BLACK
      1: GRAY → BLACK
    0: GRAY → BLACK
  Không gặp GRAY nào → KHÔNG cycle ✓
```

> **Tại sao 2 màu không đủ?** Trong đồ thị có hướng, gặp lại node đã visited chưa chắc là cycle. Ví dụ: 0→1, 0→2, 1→2. Node 2 được visit 2 lần nhưng không có cycle. Chỉ khi gặp node **đang trên đường đi hiện tại** (GRAY) mới là cycle.

---

## Cycle Detection — Undirected Graph (Union-Find)

### Bài toán

Cho đồ thị **vô hướng**. Có chu trình không?

### Ý tưởng

Dùng Union-Find. Duyệt từng cạnh:
- Nếu 2 đỉnh **chưa cùng nhóm** → union chúng
- Nếu 2 đỉnh **đã cùng nhóm** → thêm cạnh này tạo cycle!

```
Edges: (0,1), (1,2), (2,0)

Bước 1: edge (0,1)
  find(0)=0, find(1)=1 → khác nhóm → union
  Nhóm: {0, 1}  {2}

Bước 2: edge (1,2)
  find(1)=0, find(2)=2 → khác nhóm → union
  Nhóm: {0, 1, 2}

Bước 3: edge (2,0)
  find(2)=0, find(0)=0 → CÙNG NHÓM! → CYCLE ✗
```

> **Tại sao Union-Find cho undirected?** Vì trong graph vô hướng, mỗi cạnh chỉ xét 1 lần. Nếu 2 đỉnh đã connected mà vẫn có cạnh nối → chắc chắn cycle. Đơn giản hơn DFS cho trường hợp vô hướng.

---

## Connected Components — đếm nhóm liên thông

### Bài toán

Cho đồ thị vô hướng, có bao nhiêu nhóm đỉnh liên thông?

**Ví dụ thực tế:** Mạng xã hội — có bao nhiêu "nhóm bạn" tách biệt? Hoặc: trên bản đồ có bao nhiêu hòn đảo?

### Ý tưởng

Đơn giản: duyệt tất cả đỉnh. Mỗi khi gặp đỉnh chưa thăm, bắt đầu DFS/BFS từ đó (thăm hết component), tăng bộ đếm.

```
Đồ thị 5 đỉnh:  0-1, 2-3-4

  0 --- 1     2 --- 3
                    |
                    4

DFS từ 0: thăm {0, 1}           → component 1
Đỉnh 2 chưa thăm:
DFS từ 2: thăm {2, 3, 4}        → component 2
Tất cả đã thăm.

Kết quả: 2 connected components
```

---

## Course Schedule — xếp lịch học (Topological Sort)

### Bài toán

Cho `n` môn học và danh sách prerequisite `[a, b]` nghĩa là "muốn học a phải học b trước". Hỏi: có thể hoàn thành tất cả môn không?

**Ví dụ thực tế:** Đại học — muốn học "Machine Learning" phải học "Linear Algebra" trước. Muốn học "Linear Algebra" phải học "Calculus" trước. Nếu "Calculus" lại yêu cầu "Machine Learning" → KHÔNG THỂ! (circular dependency)

### Ý tưởng: Kahn's Algorithm (BFS Topological Sort)

1. Tính **in-degree** (số prerequisite) cho mỗi môn
2. Đưa tất cả môn có in-degree = 0 vào queue (môn không cần prerequisite)
3. Lặp: pop 1 môn → giảm in-degree của các môn phụ thuộc. Nếu in-degree về 0 → push vào queue
4. Nếu đã xử lý đủ n môn → OK. Nếu không → có cycle.

```
4 môn: 0, 1, 2, 3
Prereqs: [1,0], [2,0], [3,1], [3,2]
(Học 0 trước 1 và 2; học 1,2 trước 3)

       0
      / \
     1   2
      \ /
       3

In-degree: [0, 1, 1, 2]
Queue ban đầu: [0]     (chỉ môn 0 có in-degree = 0)

Pop 0: giảm in-degree 1→0, 2→0    → push 1, 2
       In-degree: [0, 0, 0, 2]
       Queue: [1, 2]    processed = 1

Pop 1: giảm in-degree 3→1         → chưa push
       In-degree: [0, 0, 0, 1]
       Queue: [2]       processed = 2

Pop 2: giảm in-degree 3→0         → push 3
       In-degree: [0, 0, 0, 0]
       Queue: [3]       processed = 3

Pop 3: không giảm ai
       processed = 4

4 == n → CÓ THỂ hoàn thành ✓
Thứ tự: 0 → 1 → 2 → 3
```

> **Nếu có cycle?** Ví dụ: 0←1, 1←0. In-degree: [1, 1]. Queue ban đầu rỗng! processed = 0 ≠ 2 → KHÔNG THỂ hoàn thành.

---

## Alien Dictionary — Từ điển ngoài hành tinh

### Bài toán

Cho danh sách từ đã **sắp xếp** theo bảng chữ cái của người ngoài hành tinh. Tìm thứ tự các ký tự trong bảng chữ cái đó.

**Ví dụ thực tế:** Bạn nhặt được một cuốn từ điển của người ngoài hành tinh. Bạn không biết bảng chữ cái của họ, nhưng bạn biết cuốn từ điển đã sắp xếp. Từ thứ tự các từ, bạn suy ra thứ tự các chữ cái.

### Ý tưởng

1. So sánh từng cặp từ liên tiếp, tìm ký tự **khác nhau đầu tiên** → đó cho ta một quan hệ thứ tự (a trước b)
2. Xây **DAG** (Directed Acyclic Graph) từ các quan hệ này
3. Chạy **Topological Sort** trên DAG → ra thứ tự bảng chữ cái

```
Từ đã sắp xếp: ["wrt", "wrf", "er", "ett", "rftt"]

So sánh từng cặp:
  "wrt" vs "wrf" → t trước f    (ký tự khác đầu tiên: t vs f)
  "wrf" vs "er"  → w trước e    (ký tự khác đầu tiên: w vs e)
  "er"  vs "ett" → r trước t    (ký tự khác đầu tiên: r vs t)
  "ett" vs "rftt"→ e trước r    (ký tự khác đầu tiên: e vs r)

DAG:
  t → f
  w → e → r → t
            ↘ (f đã có)

Topological Sort: w → e → r → t → f

Bảng chữ cái ngoài hành tinh: w, e, r, t, f
```

### Cạm bẫy

- Nếu từ A là prefix của từ B mà A đứng sau B → input **không hợp lệ** (ví dụ: "abc" đứng sau "ab")
- Nếu DAG có cycle → không tồn tại thứ tự hợp lệ
- Có thể có nhiều thứ tự đúng (khi DAG không xác định hoàn toàn)

**Complexity:** O(C) trong đó C là tổng số ký tự trong tất cả các từ.

---

## Course Schedule II — Sắp xếp môn học (trả về thứ tự)

### Bài toán

Giống Course Schedule ở trên, nhưng bây giờ phải **trả về thứ tự học** cụ thể, không chỉ kiểm tra có thể hay không.

**Ví dụ thực tế:** Bạn đăng ký 4 môn ở đại học. Mỗi môn có prerequisite. Bạn cần biết: "Học môn nào trước, môn nào sau?"

### Ý tưởng: Kahn's Algorithm (lưu thứ tự)

Giống hệt Kahn's Algorithm ở trên, chỉ thêm bước **lưu lại thứ tự** các môn khi pop ra khỏi queue.

```
4 môn: 0, 1, 2, 3
Prereqs: [1,0], [2,0], [3,1], [3,2]

       0
      / \
     1   2
      \ /
       3

Kahn's Algorithm:
  In-degree: [0, 1, 1, 2]
  Queue: [0]
  Kết quả: []

  Pop 0 → Kết quả: [0]
    Giảm in-degree: 1→0, 2→0 → push cả hai
    Queue: [1, 2]

  Pop 1 → Kết quả: [0, 1]
    Giảm in-degree: 3→1
    Queue: [2]

  Pop 2 → Kết quả: [0, 1, 2]
    Giảm in-degree: 3→0 → push 3
    Queue: [3]

  Pop 3 → Kết quả: [0, 1, 2, 3]

Thứ tự học: 0 → 1 → 2 → 3
(Nếu pop 2 trước 1 thì ra: 0 → 2 → 1 → 3, cũng đúng!)
```

> **Khác gì Course Schedule I?** Course Schedule I chỉ hỏi "có thể không?" (true/false). Course Schedule II hỏi "thứ tự cụ thể là gì?" Thuật toán giống nhau, chỉ khác ở việc lưu kết quả.

---

## Clone Graph — Sao chép đồ thị

### Bài toán

Cho một đồ thị vô hướng (dạng adjacency list). Tạo **bản sao sâu** (deep copy) -- mỗi node trong bản sao là node mới, không dùng chung reference với bản gốc.

**Ví dụ thực tế:** Copy một mạng xã hội. Mỗi người (node) cần được tạo mới, nhưng vẫn giữ đúng quan hệ bạn bè (edges).

### Ý tưởng

Dùng BFS/DFS + HashMap để theo dõi node nào đã clone:

1. Tạo clone của node bắt đầu, lưu vào HashMap `{original → clone}`
2. BFS: với mỗi hàng xóm của node hiện tại:
   - Nếu chưa clone → tạo clone mới, lưu vào HashMap
   - Thêm clone của hàng xóm vào danh sách hàng xóm của clone hiện tại

```
Bản gốc:          Bản sao:
  1 --- 2           1' --- 2'
  |     |           |      |
  4 --- 3           4' --- 3'

HashMap: {1→1', 2→2', 3→3', 4→4'}
```

**Complexity:** O(V + E) time, O(V) space cho HashMap.

> **Mẹo phỏng vấn:** Bài này đơn giản nhưng hay xuất hiện. Điểm mấu chốt: dùng HashMap để tránh clone trùng khi gặp cycle trong graph.

---

## Nâng cao: 0-1 BFS, Multi-source BFS, Tarjan SCC

### 0-1 BFS — BFS cho graph có trọng số 0 và 1

BFS thông thường chỉ đúng với graph **không trọng số** (mọi cạnh bằng nhau). Nhưng nếu graph có cạnh trọng số **chỉ là 0 hoặc 1**, ta có thể dùng **0-1 BFS** thay vì Dijkstra.

Thay vì dùng priority queue (Dijkstra, O(E log V)), ta dùng **deque**:
- Cạnh trọng số 0 → push **đầu** deque (ưu tiên xử lý trước)
- Cạnh trọng số 1 → push **cuối** deque (xử lý sau)

Kết quả: tìm đường ngắn nhất trong O(V + E) thay vì O(E log V). Nhanh hơn Dijkstra!

```
Ví dụ: Mê cung có cửa. Đi qua cửa tốn 1, đi bình thường tốn 0.

  A --0-- B --1-- C
  |               |
  1               0
  |               |
  D -----0------ E

Deque: [A]  dist = [0, inf, inf, inf, inf]

Pop A: hàng xóm B(0), D(1)
  B: dist=0 → push đầu      Deque: [B, D]
  D: dist=1 → push cuối

Pop B: hàng xóm C(1)
  C: dist=1 → push cuối      Deque: [D, C]

Pop D: hàng xóm E(0)
  E: dist=1 → push đầu       Deque: [E, C]

Đường ngắn nhất: A→B=0, A→D=1, A→B→C=1, A→D→E=1
```

### Multi-source BFS — BFS từ nhiều điểm cùng lúc

BFS thường bắt đầu từ **1 đỉnh**. Multi-source BFS bắt đầu từ **nhiều đỉnh cùng lúc** -- đưa tất cả vào queue ngay từ đầu.

**Khi nào dùng?** Bài toán kiểu "tìm khoảng cách từ mỗi ô đến ô `0` gần nhất" (LeetCode 542), hoặc "lửa lan từ nhiều điểm, ai thoát được?"

```
Ví dụ: Tìm khoảng cách đến 0 gần nhất

Input:              Output (khoảng cách):
  0  1  1             0  1  2
  1  1  1             1  2  3
  1  1  0             2  1  0

Bước 1: Đưa TẤT CẢ ô = 0 vào queue
  Queue: [(0,0), (2,2)]   dist = 0

Bước 2: BFS lan ra từ cả hai nguồn cùng lúc
  Lớp 1: (0,1), (1,0), (2,1), (1,2)   dist = 1
  Lớp 2: (1,1), (0,2)                  dist = 2
  Lớp 3: (1,1) đã thăm, chỉ còn khoảng cách xa hơn
```

Mấu chốt: thay vì chạy BFS N lần (từ mỗi nguồn), ta chạy **1 lần** BFS với tất cả nguồn trong queue. Complexity vẫn O(V + E).

### Tarjan SCC — Tìm thành phần liên thông mạnh

**Strongly Connected Component** (SCC) -- trong đồ thị có hướng, một nhóm đỉnh mà từ bất kỳ đỉnh nào trong nhóm đều có thể đi đến mọi đỉnh khác trong nhóm.

**Ví dụ thực tế:** Trong mạng web, nhóm trang mà mỗi trang đều có thể click link đến mọi trang khác trong nhóm.

Thuật toán Tarjan dùng DFS + stack. Ý tưởng chính: mỗi node có 2 giá trị:
- `disc[v]`: thời điểm phát hiện node v
- `low[v]`: node nhỏ nhất (theo disc) mà v có thể reach được

Khi `disc[v] == low[v]`, v là "root" của một SCC. Pop tất cả node từ stack cho đến v → đó là 1 SCC.

```
Đồ thị: 0→1→2→0, 2→3→4→3

SCC 1: {0, 1, 2}  — vòng 0→1→2→0
SCC 2: {3, 4}     — vòng 3→4→3
```

**Complexity:** O(V + E) -- chỉ cần 1 lần DFS.

> **Khi nào dùng Tarjan?** Khi cần tìm các "cụm" trong đồ thị có hướng mà trong mỗi cụm, mọi node đều reach được nhau. Ứng dụng: tối ưu compiler, phân tích mạng, tìm deadlock phức tạp.

---

## Bảng độ phức tạp

| Pattern | Time | Space | Kỹ thuật chính |
|---|---|---|---|
| Bipartite Check | O(V + E) | O(V) | BFS 2-coloring |
| Cycle (Directed) | O(V + E) | O(V) | DFS 3-color (white/gray/black) |
| Cycle (Undirected) | O(E * alpha(V)) | O(V) | Union-Find |
| Connected Components | O(V + E) | O(V) | DFS/BFS đếm |
| Course Schedule | O(V + E) | O(V + E) | Kahn's BFS (Topological Sort) |
| Alien Dictionary | O(C) | O(1) | DAG + Topological Sort |
| Clone Graph | O(V + E) | O(V) | BFS/DFS + HashMap |
| 0-1 BFS | O(V + E) | O(V) | Deque (thay priority queue) |
| Multi-source BFS | O(V + E) | O(V) | BFS nhiều nguồn |
| Tarjan SCC | O(V + E) | O(V) | DFS + stack |

**Khi nào dùng gì:**
- Cần chia 2 nhóm / kiểm tra đồ thị 2 phía → **Bipartite**
- Cần phát hiện deadlock / circular dependency → **Cycle Detection**
- Đồ thị có hướng → DFS 3-color. Vô hướng → Union-Find
- Cần đếm "đảo" / nhóm riêng biệt → **Connected Components**
- Cần sắp xếp thứ tự phụ thuộc → **Topological Sort**
- Suy ra thứ tự từ danh sách đã sắp xếp → **Alien Dictionary**
- Deep copy đồ thị → **Clone Graph**
- Đường ngắn nhất với trọng số 0/1 → **0-1 BFS**
- Khoảng cách từ nhiều nguồn cùng lúc → **Multi-source BFS**
- Tìm nhóm liên thông mạnh (đồ thị có hướng) → **Tarjan SCC**

---

## Pitfalls — Những lỗi hay gặp

Mấy lỗi này mình thấy lặp đi lặp lại, kể cả người đã học xong Phần 5:

| Sai lầm | Hậu quả | Cách tránh |
|---|---|---|
| BFS mà quên `visited` check trước khi push | Cùng node push nhiều lần, TLE hoặc sai kết quả | Đánh dấu visited **ngay khi push**, không phải khi pop |
| DFS directed graph dùng 2 color thay vì 3 | Báo cycle sai (false positive) | Luôn dùng WHITE/GRAY/BLACK cho directed |
| Dijkstra với trọng số âm | Kết quả sai, Dijkstra KHÔNG xử lý được trọng số âm | Dùng Bellman-Ford nếu có edge âm |
| Topological Sort quên check cycle | Trả về kết quả sai nếu input có cycle | Đếm `processed`, so sánh với `n` |
| Union-Find quên path compression | Đúng kết quả nhưng chậm, TLE | Luôn dùng cả path compression + union by rank |
| BFS tìm shortest path trên weighted graph | Sai! BFS chỉ đúng cho unweighted | Weighted → Dijkstra hoặc 0-1 BFS |

---

## Practice — Top bài graph cho phỏng vấn

Sắp theo độ khó tăng dần. Làm từ trên xuống.

**Cơ bản (warm up):**
- Number of Islands (LC 200) — Connected Components, DFS/BFS
- Clone Graph (LC 133) — BFS + HashMap
- Max Area of Island (LC 695) — DFS đếm size

**Trung bình (phỏng vấn hay hỏi nhất):**
- Course Schedule (LC 207) — Cycle Detection / Topological Sort
- Course Schedule II (LC 210) — Topological Sort trả thứ tự
- Is Graph Bipartite? (LC 785) — BFS 2-coloring
- 01 Matrix (LC 542) — Multi-source BFS
- Rotting Oranges (LC 994) — Multi-source BFS

**Nâng cao (nếu muốn flex):**
- Alien Dictionary (LC 269) — DAG + Topological Sort
- Shortest Path in Binary Matrix (LC 1091) — BFS
- Network Delay Time (LC 743) — Dijkstra
- Redundant Connection (LC 684) — Union-Find cycle detection
- Accounts Merge (LC 721) — Union-Find

> **Lời khuyên thật lòng:** Đừng cố làm hết. Làm 2-3 bài mỗi pattern, hiểu kỹ, rồi mới qua pattern tiếp. Làm 5 bài mà hiểu tốt hơn làm 20 bài mà copy-paste solution.

---

## Code Rust

```rust,noplayground
{{#include ../../../../src/graph_patterns.rs}}
```

---

## Tiếp theo

Chương sau: **[Tree Patterns](09-tree-patterns.md)** — áp dụng DFS/BFS lên cây. Cây chỉ là graph đặc biệt (connected, no cycle), nên nhiều kỹ thuật ở đây sẽ dùng lại được. Nếu bạn đã quen DFS trên graph, DFS trên tree sẽ dễ thở hơn nhiều.

---

[← String Matching](./07-string-matching.md) | [Tree Patterns →](./09-tree-patterns.md)
