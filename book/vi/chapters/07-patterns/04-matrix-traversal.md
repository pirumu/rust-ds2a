# Matrix Traversal

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

## Khi nào dùng?

- **Spiral / Diagonal**: khi đề bài yêu cầu duyệt ma trận theo thứ tự đặc biệt.
- **Rotate**: xử lý hình ảnh, game (xoay bản đồ).
- **Flood Fill**: paint bucket, tìm vùng liên thông cùng giá trị.
- **Đếm đảo**: đếm connected component trên lưới — rất phổ biến trong phỏng vấn.
- **Shortest Path**: tìm đường ngắn nhất trên lưới — BFS là lựa chọn mặc định khi mọi bước có chi phí bằng nhau.

> **Mẹo phỏng vấn**: Khi thấy bài toán trên ma trận 2D mà hỏi về "vùng liên thông" hay "đường đi" → nghĩ ngay tới DFS/BFS. Ma trận chính là graph, mỗi ô là node, 4 (hoặc 8) ô kề là edge.
