# Backtracking

## Đây là gì?

Tưởng tượng bạn đang đi trong một mê cung. Tại mỗi ngã rẽ, bạn chọn một đường đi. Nếu đi vào ngõ cụt, bạn **quay lại** ngã rẽ trước đó và thử đường khác. Cứ thế cho đến khi tìm được lối ra.

Đây chính là **Backtracking** (quay lui) — một phương pháp có hệ thống để khám phá mọi lời giải có thể. Nó xây dựng ứng viên từng bước và **bỏ cuộc** (pruning — cắt tỉa) ngay khi phát hiện ứng viên đó không thể dẫn đến lời giải hợp lệ.

Có thể hình dung như duyệt theo chiều sâu (DFS) trên **cây quyết định**.

Backtracking dùng cho:
- **Bài toán ràng buộc**: N-Queens, Sudoku
- **Liệt kê tổ hợp**: hoán vị, tập con
- **Tối ưu**: khi kết hợp với giới hạn (bounding)

---

## Hoạt động như thế nào?

### Cây quyết định và cắt tỉa

Ý tưởng chính: khám phá cây lựa chọn. Tại mỗi node, nếu lời giải một phần vi phạm ràng buộc, **cắt tỉa** — không cần khám phá nhánh con nào của nó.

```
Mẫu tổng quát:

  backtrack(trạng thái):
      nếu trạng thái là lời giải hoàn chỉnh:
          ghi nhận lời giải
          return
      với mỗi lựa chọn có thể:
          nếu lựa chọn hợp lệ (kiểm tra cắt tỉa):
              thực hiện lựa chọn
              backtrack(trạng thái mới)
              hoàn tác lựa chọn              <-- quay lui!
```

### N-Queens từng bước (n=4)

Đặt 4 quân hậu lên bàn cờ 4x4 sao cho không có 2 quân nào tấn công nhau (cùng hàng, cùng cột, cùng đường chéo).

```
Cây quyết định (chọn cột cho mỗi hàng):

Hàng 0: thử cột 0
  Hàng 1: thử cột 0 -- X (cùng cột)
           thử cột 1 -- X (đường chéo)
           thử cột 2 -- OK
    Hàng 2: thử cột 0 -- X (đường chéo từ hàng 1)
             thử cột 1 -- X (đường chéo từ hàng 0)
             thử cột 2 -- X (cùng cột với hàng 1)
             thử cột 3 -- X (đường chéo từ hàng 1)
    QUAY LUI về hàng 1
           thử cột 3 -- OK
    Hàng 2: thử cột 0 -- X (đường chéo)
             thử cột 1 -- OK
      Hàng 3: thử cột 0 -- X
               thử cột 1 -- X
               thử cột 2 -- X
               thử cột 3 -- X
      QUAY LUI...

Hàng 0: thử cột 1
  Hàng 1: thử cột 3 -- OK
    Hàng 2: thử cột 0 -- OK
      Hàng 3: thử cột 2 -- OK  --> TÌM THẤY LỜI GIẢI!

Lời giải 1:         Lời giải 2:
  . Q . .             . . Q .
  . . . Q             Q . . .
  Q . . .             . . . Q
  . . Q .             . Q . .
```

### Minh họa cắt tỉa

```
Không cắt tỉa:           Có cắt tỉa:
     *                         *
   / | \ \                   / | \
  *  *  *  *               *  X  *
 /|\ ...                  /|\
* * * *                   * X *
                         ...

X = bị cắt tỉa (vi phạm ràng buộc, bỏ qua toàn bộ nhánh con)
```

Cắt tỉa giúp giảm ĐÁNG KỂ số node phải duyệt. Ví dụ: N-Queens với n=8, nếu không cắt tỉa phải duyệt 8^8 = 16 triệu node. Với cắt tỉa, chỉ cần vài ngàn.

### Sudoku — giải bằng Backtracking

Với mỗi ô trống, thử lần lượt số từ 1 đến 9. Nếu số nào không vi phạm (cùng hàng, cùng cột, cùng ô 3x3), điền vào và tiếp tục. Nếu bị kẹt (không có số nào hợp lệ), quay lui.

```
  Ô trống -> thử 1: vi phạm hàng
              thử 2: vi phạm cột
              thử 3: OK! Điền 3, tiếp tục ô tiếp theo
                     ...
                     Bị kẹt! Quay lui, xóa 3
              thử 4: OK! Điền 4, tiếp tục...
```

---

## Code Rust

```rust
/// N-Queens: tìm tất cả cách đặt n quân hậu trên bàn cờ n x n.
pub fn n_queens(n: usize) -> Vec<Vec<Vec<bool>>> {
    let mut solutions = Vec::new();
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n];  // row + col
    let mut diag2 = vec![false; 2 * n];  // row - col + n
    let mut placement: Vec<usize> = Vec::new();

    fn solve(
        row: usize, n: usize, placement: &mut Vec<usize>,
        cols: &mut Vec<bool>, diag1: &mut Vec<bool>, diag2: &mut Vec<bool>,
        solutions: &mut Vec<Vec<Vec<bool>>>,
    ) {
        if row == n {
            // Đã đặt hết n hàng -> tìm thấy lời giải
            let mut board = vec![vec![false; n]; n];
            for (r, &c) in placement.iter().enumerate() {
                board[r][c] = true;
            }
            solutions.push(board);
            return;
        }
        for col in 0..n {
            let (d1, d2) = (row + col, row + n - col);
            // Kiểm tra: cột, đường chéo 1, đường chéo 2 đều trống?
            if !cols[col] && !diag1[d1] && !diag2[d2] {
                cols[col] = true;       // Đánh dấu
                diag1[d1] = true;
                diag2[d2] = true;
                placement.push(col);
                solve(row + 1, n, placement, cols, diag1, diag2, solutions);
                placement.pop();        // Quay lui!
                cols[col] = false;      // Bỏ đánh dấu
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }
    }
    if n > 0 {
        solve(0, n, &mut placement, &mut cols, &mut diag1, &mut diag2, &mut solutions);
    }
    solutions
}

/// Sudoku: điền số vào ô trống (giá trị 0) bằng backtracking.
pub fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    // Tìm ô trống tiếp theo
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == 0 {
                for num in 1..=9 {
                    if is_valid(board, r, c, num) {
                        board[r][c] = num;           // Thử điền
                        if solve_sudoku(board) {
                            return true;             // Giải được!
                        }
                        board[r][c] = 0;             // Quay lui
                    }
                }
                return false;  // Không có số nào hợp lệ
            }
        }
    }
    true  // Không còn ô trống -> đã giải xong
}

/// Tất cả hoán vị bằng backtracking (dựa trên swap).
pub fn permutations<T: Clone>(arr: &[T]) -> Vec<Vec<T>> { /* ... */ }

/// Tập lũy thừa: bao gồm/loại trừ từng phần tử.
pub fn subsets<T: Clone>(arr: &[T]) -> Vec<Vec<T>> { /* ... */ }
```

**Ghi chú về Rust:**

- Dùng 3 mảng `cols`, `diag1`, `diag2` để kiểm tra nhanh (O(1)) thay vì duyệt toàn bộ bàn cờ. Đây là kỹ thuật tối ưu quan trọng.
- `placement.push(col)` rồi `placement.pop()` là pattern quay lui kinh điển: thêm -> đệ quy -> bỏ.
- Hàm `solve` là hàm lồng (nested function) trong Rust — không cần tạo struct riêng.

---

## Độ phức tạp

| Bài toán | Thời gian | Bộ nhớ |
|---------|----------|--------|
| N-Queens | O(n!) | O(n^2) |
| Sudoku | O(9^(ô trống)) | O(ô trống) |
| Hoán vị | O(n! * n) | O(n! * n) |
| Tập con | O(2^n * n) | O(2^n * n) |

**Giải thích thực tế:**

- Cắt tỉa giảm ĐÁNG KỂ số node phải duyệt so với worst-case. Ví dụ: N-Queens n=8 có 92 lời giải, nhưng chỉ duyệt vài ngàn node (thay vì hàng triệu).
- Backtracking vẫn là thuật toán **exponential** — không phù hợp cho bài toán quá lớn. Nhưng với kích thước vừa phải (Sudoku 9x9, N-Queens n < 20), nó rất hiệu quả.

---

## Ví dụ

```rust
use rust_ds2a::backtracking::*;

// N-Queens
let solutions = n_queens(4);
assert_eq!(solutions.len(), 2);     // 4x4 có 2 lời giải

let solutions = n_queens(8);
assert_eq!(solutions.len(), 92);    // 8x8 có 92 lời giải

// Hoán vị
let perms = permutations(&[1, 2, 3]);
assert_eq!(perms.len(), 6);         // 3! = 6

// Tập con (tập lũy thừa)
let sets = subsets(&[1, 2, 3]);
assert_eq!(sets.len(), 8);          // 2^3 = 8
```
