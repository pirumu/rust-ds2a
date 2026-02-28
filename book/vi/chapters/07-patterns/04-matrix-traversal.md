# Matrix Traversal

> 💡 **Đừng lo lắng:** Chương này dài, nhưng bạn không cần nhớ hết ngay. Mỗi bài toán là một "công thức nấu ăn" riêng — bạn chỉ cần hiểu **ý tưởng** rồi khi cần thì mở ra xem lại. Đọc thoải mái, không ai thi bạn đâu.

---

## Đây là gì?

Bạn đã học BFS/DFS ở Phần 5 trên graph. Giờ mình áp dụng lên ma trận 2D — tưởng tượng ma trận như một **bản đồ**, mỗi ô là một thành phố, và các ô kề nhau (trên/dưới/trái/phải) là các con đường nối giữa chúng.

Ma trận xuất hiện **khắp nơi**: hình ảnh trên màn hình (mỗi pixel là 1 ô), bản đồ game, bảng tính Excel, hay bản đồ vệ tinh.

Trong chương này, mình sẽ đi qua 6 bài toán kinh điển trên ma trận:

1. **Spiral Order** — đọc ma trận theo hình xoắn ốc
2. **Diagonal Traversal** — đọc theo đường chéo
3. **Rotate 90°** — xoay ma trận
4. **Flood Fill** — tô màu vùng liên thông (như paint bucket)
5. **Đếm đảo** — DFS trên lưới
6. **Đường đi ngắn nhất** — BFS trên lưới

---

## Grid chính là Graph

Trước khi vào từng bài, mình cần hiểu một điều quan trọng: **mọi ma trận 2D đều là một graph** đang giả dạng.

Tưởng tượng bạn nhìn bản đồ thành phố từ trên cao. Mỗi ngã tư là một **node**. Mỗi con đường nối hai ngã tư là một **edge**. Ma trận cũng vậy:

```
Ma trận 3x3:             Graph ẩn bên trong:

 A  B  C                 A — B — C
 D  E  F                 |   |   |
 G  H  I                 D — E — F
                          |   |   |
                          G — H — I

Mỗi ô = 1 node.
Mỗi ô có tối đa 4 "hàng xóm" (trên/dưới/trái/phải) = 4 edges.
```

Vì grid = graph, nên bạn dùng **BFS/DFS** trên grid y hệt như trên graph. Chỉ khác là thay vì adjacency list, bạn dùng **tọa độ (row, col)** và kiểm tra 4 hướng.

### Direction Array Trick — Mẹo di chuyển 4 hướng

Khi duyệt grid, bạn luôn cần thử đi 4 hướng: lên, phải, xuống, trái. Thay vì viết 4 cục `if`, dùng **direction array**:

```rust
// Lên, Phải, Xuống, Trái
let dx: [i32; 4] = [-1, 0, 1, 0];
let dy: [i32; 4] = [0, 1, 0, -1];

// Duyệt 4 hàng xóm:
for d in 0..4 {
    let nx = x as i32 + dx[d];
    let ny = y as i32 + dy[d];
    if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
        // (nx, ny) là hàng xóm hợp lệ
    }
}
```

Muốn đi **8 hướng** (kể cả chéo)? Thêm 4 hướng chéo:

```rust
let dx: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
let dy: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
```

Mẹo nhỏ nhưng giúp code gọn hơn rất nhiều. Bạn sẽ thấy pattern này lặp đi lặp lại trong mọi bài grid.

### In-place Visited — Đánh dấu "đã thăm" không cần thêm bộ nhớ

Khi duyệt graph thường, bạn cần mảng `visited`. Nhưng trên grid, nhiều khi bạn có thể **sửa trực tiếp grid** thay vì tạo mảng visited riêng:

```
Đếm đảo: gặp '1' → đổi thành '0' (nhấn chìm)
Flood fill: gặp old_color → đổi thành new_color
```

Lợi ích: tiết kiệm O(m×n) bộ nhớ cho mảng visited. Nhưng **cẩn thận** — bạn đang thay đổi input. Nếu đề bài nói "không được sửa input" thì phải dùng visited array.

---

## 1. Spiral Order (Xoắn ốc)

### Ý tưởng

Tưởng tượng bạn đang đi quanh một sân vận động hình chữ nhật. Bạn bắt đầu từ góc trên-trái, đi sang phải dọc hàng trên, rẽ xuống dọc cột phải, rồi quay lại dọc hàng dưới, rồi lên dọc cột trái. Xong một vòng, bạn thu hẹp đường chạy vào trong và lặp lại.

```
Ma trận 3x3:

 1  2  3
 4  5  6
 7  8  9

Đường đi xoắn ốc:

 →  →  →
         ↓
 ←  ←  ←
 ↓
 →  →

Kết quả: [1, 2, 3, 6, 9, 8, 7, 4, 5]
```

### Thuật toán

Duy trì 4 biên: `top`, `bottom`, `left`, `right`. Sau mỗi lượt đi, thu hẹp biên tương ứng.

```
Bước 1: → đi từ left → right trên hàng top, rồi top += 1
Bước 2: ↓ đi từ top → bottom trên cột right, rồi right -= 1
Bước 3: ← đi từ right → left trên hàng bottom, rồi bottom -= 1
Bước 4: ↑ đi từ bottom → top trên cột left, rồi left += 1

Lặp lại cho đến khi top > bottom hoặc left > right.
```

### Code Rust

```rust
pub fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() { return vec![]; }
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut result = Vec::with_capacity(m * n);
    let (mut top, mut bot) = (0i32, m as i32 - 1);
    let (mut left, mut right) = (0i32, n as i32 - 1);

    while top <= bot && left <= right {
        for c in left..=right { result.push(matrix[top as usize][c as usize]); }
        top += 1;
        for r in top..=bot   { result.push(matrix[r as usize][right as usize]); }
        right -= 1;
        if top <= bot {
            for c in (left..=right).rev() { result.push(matrix[bot as usize][c as usize]); }
            bot -= 1;
        }
        if left <= right {
            for r in (top..=bot).rev() { result.push(matrix[r as usize][left as usize]); }
            left += 1;
        }
    }
    result
}
```

**Time:** O(m×n) — mỗi ô đọc đúng 1 lần. **Space:** O(m×n) cho kết quả.

---

## 2. Diagonal Traversal (Đường chéo)

Tất cả các ô `(r, c)` mà `r + c` bằng nhau nằm trên cùng một đường chéo.

```
Ma trận 3x3 — đánh số đường chéo (r+c):

 0  1  2
 1  2  3
 2  3  4

Nhóm theo đường chéo:
  d=0: [1]
  d=1: [2, 4]
  d=2: [3, 5, 7]
  d=3: [6, 8]
  d=4: [9]
```

Chỉ cần tạo mảng có `m + n - 1` nhóm, duyệt toàn bộ ma trận, bỏ phần tử vào nhóm `r + c`.

**Time:** O(m×n). **Space:** O(m×n).

---

## 3. Rotate 90° (Xoay 90 độ theo chiều kim đồng hồ)

### Ý tưởng

Tưởng tượng bạn cầm tờ giấy, xoay nó 90° sang phải. Hàng trên cùng trở thành cột phải nhất.

Có một mẹo đơn giản: **transpose** (đổi hàng thành cột) rồi **đảo ngược mỗi hàng**.

```
Bước 1 — Transpose (đối xứng qua đường chéo):

 1 2 3      1 4 7
 4 5 6  →   2 5 8
 7 8 9      3 6 9

Bước 2 — Reverse mỗi hàng:

 1 4 7      7 4 1
 2 5 8  →   8 5 2
 3 6 9      9 6 3
```

Kết quả: ma trận xoay 90° theo chiều kim đồng hồ. Tất cả **in-place**, không cần thêm bộ nhớ!

**Time:** O(n²). **Space:** O(1).

---

## 4. Flood Fill (Tô màu tràn)

### Ý tưởng

Bạn biết công cụ **Paint Bucket** trong Microsoft Paint không? Click vào một vùng màu xanh, cả vùng xanh liên thông chuyển sang màu đỏ. Đó chính là flood fill.

```
Trước (click vào ô giữa, giá trị 1, tô màu 2):

 1 1 1          2 2 2
 1 1 0    →     2 2 0
 1 0 1          2 0 1

Ô (2,2) = 1 nhưng không liên thông (bị chặn bởi 0) nên giữ nguyên.
```

### Thuật toán

Dùng DFS từ ô `(sr, sc)`:
1. Nếu ô hiện tại có màu khác `original_color` → dừng.
2. Tô màu mới.
3. Gọi đệ quy sang 4 ô kề (trên/dưới/trái/phải).

Trường hợp đặc biệt: nếu `new_color == original_color`, không cần làm gì (tránh vòng lặp vô hạn!).

**Time:** O(m×n). **Space:** O(m×n) cho call stack.

---

## 5. Đếm đảo (Number of Islands)

### Ý tưởng

Cho một bản đồ gồm đất (`'1'`) và nước (`'0'`). Đếm có bao nhiêu **hòn đảo** — tức nhóm đất liên thông (4 hướng).

```
 1 1 0 0 0
 1 1 0 0 0       → 3 hòn đảo
 0 0 1 0 0
 0 0 0 1 1
```

### Thuật toán

Duyệt toàn bộ lưới. Mỗi khi gặp `'1'`:
1. Tăng bộ đếm đảo.
2. DFS từ ô đó, "nhấn chìm" tất cả `'1'` liên thông thành `'0'` (đánh dấu đã thăm).

Khi duyệt xong, mỗi đảo đã bị nhấn chìm nên không đếm lại.

```
Bắt đầu tại (0,0) = '1':
  count = 1
  DFS nhấn chìm: (0,0), (0,1), (1,0), (1,1) → tất cả thành '0'

Tiếp tục duyệt... gặp (2,2) = '1':
  count = 2
  DFS nhấn chìm: (2,2)

Tiếp... gặp (3,3) = '1':
  count = 3
  DFS nhấn chìm: (3,3), (3,4)

Kết quả: 3
```

Chú ý: đây chính là in-place visited — mình sửa `'1'` thành `'0'` thay vì tạo mảng `visited` riêng.

**Time:** O(m×n). **Space:** O(m×n) worst case (call stack).

---

## 6. Đường đi ngắn nhất trong lưới (Shortest Path in Binary Grid)

### Ý tưởng

Tưởng tượng bạn đang ở góc trên-trái của một mê cung lưới. Bạn muốn tới góc dưới-phải. Các ô `0` đi được, ô `1` là tường. Bạn có thể đi **8 hướng** (kể cả chéo). Tìm đường đi ngắn nhất.

Đây là bài BFS kinh điển — vì BFS luôn tìm đường ngắn nhất trong đồ thị không trọng số.

```
 0 0 0
 1 1 0      Đường đi: (0,0)→(0,1)→(0,2)→(1,2)→(2,2)
 1 1 0      Kết quả: 5 ô → nhưng thực ra đi chéo ngắn hơn!

 0 0 0
 1 1 0      Đường tối ưu: (0,0)→(0,1)→(0,2)→(1,2)→(2,2) = 5? Không!
 1 1 0      (0,0)→(0,1)→(1,2)→(2,2) = 4 ✓ (đi chéo từ (0,1) tới (1,2))
```

### Thuật toán

1. Nếu ô bắt đầu hoặc ô đích là `1` → trả về `-1`.
2. BFS từ `(0, 0)` với khoảng cách ban đầu = 1.
3. Mỗi bước, thử 8 hướng. Nếu ô kế chưa thăm và là `0`, thêm vào queue.
4. Khi tới `(m-1, n-1)`, trả khoảng cách. Nếu queue hết mà chưa tới → `-1`.

**Time:** O(m×n). **Space:** O(m×n).

---

## Bảng độ phức tạp

| Bài toán | Time | Space | Ghi chú |
|---|---|---|---|
| Spiral Order | O(m×n) | O(m×n) | Kết quả chứa tất cả phần tử |
| Diagonal Traversal | O(m×n) | O(m×n) | Nhóm theo `r + c` |
| Rotate 90° | O(n²) | O(1) | In-place: transpose + reverse |
| Flood Fill | O(m×n) | O(m×n) | DFS, call stack |
| Number of Islands | O(m×n) | O(m×n) | DFS, nhấn chìm đảo |
| Shortest Path (BFS) | O(m×n) | O(m×n) | 8 hướng, BFS queue |

---

## Pitfalls — Những cái bẫy hay gặp

### 1. Quên kiểm tra out-of-bounds

❌ **Sai**: Truy cập `grid[nx][ny]` mà không check xem `nx`, `ny` có nằm trong grid không.

```rust
// Crash! nx có thể âm hoặc >= rows
let val = grid[nx as usize][ny as usize];
```

✅ **Đúng**: Luôn check biên TRƯỚC khi truy cập.

```rust
if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
    let val = grid[nx as usize][ny as usize];
}
```

💡 **Tại sao**: Ma trận không vô hạn. Ô ở góc chỉ có 2 hàng xóm, ô ở cạnh có 3. Nếu bạn dùng `usize` cho tọa độ mà trừ 1 khi đang ở hàng 0, Rust sẽ panic vì underflow.

### 2. Dùng 8 hướng khi chỉ cần 4 hướng (hoặc ngược lại)

❌ **Sai**: Bài Number of Islands nói "4-directional" mà bạn duyệt 8 hướng → đếm sai (hai đảo chéo nhau bị gộp thành 1).

```
 1 0          4 hướng: 2 đảo ✓
 0 1          8 hướng: 1 đảo ✗
```

✅ **Đúng**: Đọc kỹ đề. "Adjacent" thường là 4 hướng. "Connected" có thể là 8. Shortest Path in Binary Grid cho phép 8.

💡 **Tại sao**: Khác biệt nhỏ trong đề bài → kết quả sai hoàn toàn. Luôn xác nhận rõ: 4 hay 8 hướng.

### 3. Quên reset visited khi cần backtrack

❌ **Sai**: Trong bài Word Search, dùng in-place visited (`grid[r][c] = '#'`) nhưng quên đổi lại sau khi backtrack.

```rust
fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, word: &[u8], idx: usize) -> bool {
    grid[r][c] = '#'; // đánh dấu
    // ... thử 4 hướng ...
    // ❌ Quên: grid[r][c] = original_char;
    false
}
```

✅ **Đúng**: Backtrack — trả lại giá trị gốc sau khi thử xong.

```rust
let original = grid[r][c];
grid[r][c] = '#';
// ... thử 4 hướng ...
grid[r][c] = original; // ✅ Trả lại
false
```

💡 **Tại sao**: Flood Fill và Number of Islands không cần reset vì mỗi ô chỉ thăm 1 lần. Nhưng Word Search cần thử nhiều đường khác nhau — nếu không reset, các đường sau bị "chặn" bởi dấu `#` từ đường trước.

---

## Khi nào dùng gì?

| Tình huống | Dùng gì | Ví dụ |
|---|---|---|
| Duyệt ma trận theo thứ tự đặc biệt | Spiral / Diagonal | Spiral Matrix, Diagonal Traverse |
| Xoay hoặc biến đổi ma trận | Transpose + Reverse | Rotate Image |
| Tô màu / tìm vùng liên thông cùng giá trị | DFS (Flood Fill) | Flood Fill, Surrounded Regions |
| Đếm nhóm liên thông | DFS + in-place visited | Number of Islands, Max Area of Island |
| Tìm đường ngắn nhất (mọi bước = 1) | BFS | Shortest Path in Binary Grid, Rotting Oranges |
| Tìm đường có chướng ngại vật thay đổi | BFS + trạng thái mở rộng | Shortest Path with Obstacles Elimination |
| Tìm từ trong lưới (backtracking) | DFS + reset visited | Word Search |

> **Mẹo phỏng vấn**: Khi thấy bài toán trên ma trận 2D mà hỏi về "vùng liên thông" hay "đường đi" → nghĩ ngay tới DFS/BFS. Ma trận chính là graph, mỗi ô là node, 4 (hoặc 8) ô kề là edge.

---

## Practice — Luyện tập

Đây là 3 bài kinh điển bạn **nên** làm sau khi đọc chương này:

### 1. Number of Islands — LeetCode #200

Đề bài: cho grid gồm `'1'` (đất) và `'0'` (nước), đếm số đảo.

Đây chính là bài 5 trong chương. Bạn đã biết cách giải rồi — giờ tự code lại không nhìn bài.

**Gợi ý**: DFS + nhấn chìm. Direction array 4 hướng.

### 2. Rotting Oranges — LeetCode #994

Đề bài: grid có cam tươi (`1`), cam thối (`2`), và ô trống (`0`). Mỗi phút, cam thối làm thối tất cả cam tươi kề (4 hướng). Hỏi: bao nhiêu phút để tất cả cam thối?

**Tại sao hay**: Đây là BFS **multi-source** — bạn bắt đầu từ NHIỀU ô cùng lúc (tất cả cam thối ban đầu đều vào queue). Mỗi "vòng" BFS = 1 phút.

```
Phút 0:     Phút 1:     Phút 2:
 2 1 1       2 2 1       2 2 2
 1 1 0  →    2 1 0  →    2 2 0
 0 1 1       0 1 1       0 2 1

Phút 3:     Phút 4:
 2 2 2       2 2 2
 2 2 0  →    2 2 0       → 4 phút
 0 2 2       0 2 2
```

**Gợi ý**: Đếm cam tươi trước. BFS level-by-level. Nếu cuối cùng còn cam tươi → trả -1.

### 3. Word Search — LeetCode #79

Đề bài: cho grid chữ cái và một từ, hỏi có thể tìm từ đó bằng cách đi 4 hướng liên tục không (mỗi ô chỉ dùng 1 lần).

**Tại sao hay**: Đây là DFS + **backtracking** trên grid. Bạn phải **reset visited** sau khi thử xong mỗi đường — khác với Number of Islands.

**Gợi ý**: Từ mỗi ô match ký tự đầu, DFS thử tìm ký tự tiếp theo ở 4 hướng. Nhớ backtrack!

---

## Rust Ecosystem

Khi làm việc với ma trận trong Rust thực tế (không phải phỏng vấn), bạn có một số crate hữu ích:

- **[`ndarray`](https://crates.io/crates/ndarray)**: Mảng n-chiều kiểu NumPy. Hỗ trợ slicing, broadcasting, operations trên ma trận. Đây là lựa chọn phổ biến nhất cho scientific computing trong Rust.

- **[`nalgebra`](https://crates.io/crates/nalgebra)**: Thư viện linear algebra. Nếu bạn cần phép xoay, phép nhân ma trận, hay giải hệ phương trình — dùng crate này.

- **[`image`](https://crates.io/crates/image)**: Xử lý ảnh. Ảnh chính là ma trận pixel — flood fill, convolution, transform đều liên quan trực tiếp tới chương này.

- **[`petgraph`](https://crates.io/crates/petgraph)**: Thư viện graph. Nếu bài toán grid phức tạp quá, bạn có thể convert grid thành graph rồi dùng BFS/DFS/Dijkstra sẵn có.

Trong phỏng vấn thì bạn tự implement. Nhưng trong production, dùng crate sẵn sẽ an toàn và nhanh hơn nhiều.

---

## Tiếp theo

Chương sau: **[Linked List Tricks](./05-linked-list-tricks.md)** — những mẹo xử lý linked list mà bạn sẽ gặp đi gặp lại: fast/slow pointer (tìm giữa list, detect cycle), dummy head (tránh xử lý edge case), và reverse linked list in-place. Nếu bạn thấy linked list khó — chương sau sẽ giúp bạn "bẻ gãy" nỗi sợ đó.

---

[← Intervals](./03-intervals.md) | [Linked List Tricks →](./05-linked-list-tricks.md)
