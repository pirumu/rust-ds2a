# Backtracking

> 💡 **Đừng lo lắng:** Backtracking nghe có vẻ "cao siêu" nhưng thật ra bạn đã làm nó mỗi ngày. Mở tủ quần áo chọn đồ mặc đi chơi -- thử áo xanh, không hợp, cởi ra, thử áo đỏ. Đó chính là backtracking. Nếu bạn đã đọc qua DFS (chương 5.3) và Recursion (chương 6.1), bạn đã có đủ 100% kiến thức nền. Chương này chỉ ghép chúng lại thôi.

---

## Đây là gì?

Tưởng tượng bạn đang đi trong một mê cung. Tại mỗi ngã rẽ, bạn chọn một đường đi. Nếu đi vào ngõ cụt, bạn **quay lại** ngã rẽ trước đó và thử đường khác. Cứ thế cho đến khi tìm được lối ra.

Đây chính là **Backtracking** (quay lui) -- một phương pháp có hệ thống để khám phá mọi lời giải có thể. Nó xây dựng ứng viên từng bước và **bỏ cuộc** (pruning -- cắt tỉa) ngay khi phát hiện ứng viên đó không thể dẫn đến lời giải hợp lệ.

### Bridge: DFS + Undo = Backtracking

Nhớ DFS ở chương Graph không? DFS đi sâu vào một nhánh, hết đường thì quay lại. Backtracking **chính là DFS**, nhưng trên một **cây quyết định ẩn** (implicit decision tree) -- cái cây này không tồn tại sẵn trong memory, nó được tạo ra khi bạn chạy.

```
DFS trên graph thật        Backtracking
─────────────────          ──────────────────
Duyệt các node có sẵn     Duyệt các "quyết định" (chọn số, đặt quân hậu...)
visited[] để không lặp     make() + undo() để thử rồi quay lại
Graph nằm sẵn trong RAM    Cây quyết định được sinh ra lúc chạy
```

Công thức đơn giản: **DFS (chương 5.3) + undo (hoàn tác) = Backtracking**.

Backtracking dùng cho:
- **Bài toán ràng buộc**: N-Queens, Sudoku
- **Liệt kê tổ hợp**: hoán vị, tập con, tổ hợp
- **Tối ưu**: khi kết hợp với giới hạn (bounding)

---

## Backtracking Template

Mọi bài backtracking đều theo **một** template duy nhất. Học thuộc cái này, giải được 90% bài:

```
fn backtrack(state, result):
    if is_complete(state):           ← 1. Điều kiện dừng
        result.add(state.clone())
        return
    for choice in choices(state):    ← 2. Duyệt các lựa chọn
        if is_valid(choice):         ← 3. Cắt tỉa (pruning)
            make(choice)             ← 4. Chọn
            backtrack(state, result) ← 5. Đệ quy
            undo(choice)             ← 6. Hoàn tác (QUAY LUI!)
```

6 bước. Nhớ: **make -> recurse -> undo**. Bộ ba này là trái tim của backtracking.

Bây giờ hãy xem template này áp dụng vào từng bài cụ thể.

---

## Hoạt động như thế nào?

### Cây quyết định và cắt tỉa

Ý tưởng chính: khám phá cây lựa chọn. Tại mỗi node, nếu lời giải một phần vi phạm ràng buộc, **cắt tỉa** -- không cần khám phá nhánh con nào của nó.

```
Không cắt tỉa:               Có cắt tỉa:
     *                             *
   / | \ \                       / | \
  *  *  *  *                   *  X  *
 /|\ ...                      /|\
* * * *                       * X *
                              ...

X = bị cắt tỉa (vi phạm ràng buộc, bỏ qua toàn bộ nhánh con)
```

Cắt tỉa giúp giảm ĐÁNG KỂ số node phải duyệt. Ví dụ: N-Queens với n=8, nếu không cắt tỉa phải duyệt 8^8 = 16 triệu node. Với cắt tỉa, chỉ cần vài ngàn.

### N-Queens từng bước (n=4)

Đặt 4 quân hậu lên bàn cờ 4x4 sao cho không có 2 quân nào tấn công nhau (cùng hàng, cùng cột, cùng đường chéo).

**Ẩn dụ thực tế:** Bạn phải xếp chỗ ngồi cho 4 người ở 4 bàn (mỗi hàng 1 bàn), sao cho không ai "nhìn thấy" ai -- không cùng cột, không cùng đường chéo.

```
Áp dụng template:
  - state     = placement[] (cột đã chọn cho mỗi hàng)
  - complete  = đã đặt đủ n hàng
  - choices   = cột 0..n cho hàng hiện tại
  - valid     = cột chưa bị chiếm, 2 đường chéo chưa bị chiếm
  - make      = đánh dấu cột + 2 đường chéo, push cột vào placement
  - undo      = bỏ đánh dấu, pop khỏi placement
```

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
      Hàng 3: thử cột 2 -- OK  --> TIM THAY LOI GIAI!

Lời giải 1:         Lời giải 2:
  . Q . .             . . Q .
  . . . Q             Q . . .
  Q . . .             . . . Q
  . . Q .             . Q . .
```

### Sudoku -- giải bằng Backtracking

Với mỗi ô trống, thử lần lượt số từ 1 đến 9. Nếu số nào không vi phạm (cùng hàng, cùng cột, cùng ô 3x3), điền vào và tiếp tục. Nếu bị kẹt (không có số nào hợp lệ), quay lui.

```
Áp dụng template:
  - state     = board 9x9
  - complete  = không còn ô trống
  - choices   = số 1..=9 cho ô trống tiếp theo
  - valid     = số chưa xuất hiện trong hàng, cột, ô 3x3
  - make      = điền số vào ô
  - undo      = xóa số (đặt lại 0)
```

```
  Ô trống -> thử 1: vi phạm hàng
              thử 2: vi phạm cột
              thử 3: OK! Điền 3, tiếp tục ô tiếp theo
                     ...
                     Bị kẹt! Quay lui, xóa 3
              thử 4: OK! Điền 4, tiếp tục...
```

---

## 3 Pattern kinh điển: Subset vs Permutation vs Combination

Đây là 3 biến thể backtracking phổ biến nhất. Chúng khác nhau ở **choices** và **cách tránh trùng lặp**.

### So sánh nhanh

```
Input: [1, 2, 3]

Subset (tập con):          Permutation (hoán vị):     Combination (tổ hợp):
{}, {1}, {2}, {3},         [1,2,3], [1,3,2],          Vd: chọn 2 từ [1,2,3]
{1,2}, {1,3}, {2,3},       [2,1,3], [2,3,1],          {1,2}, {1,3}, {2,3}
{1,2,3}                    [3,1,2], [3,2,1]
= 2^n kết quả              = n! kết quả               = C(n,k) kết quả
```

### Pattern 1: Subsets -- "Lấy hay không lấy?"

Ẩn dụ: Bạn đứng trước tủ đồ, mỗi món đồ bạn quyết định **lấy** hoặc **bỏ**.

```rust
// Template: duyệt từ index, mỗi phần tử có thể lấy hoặc không
fn subsets_helper(arr, idx, current, result):
    result.add(current.clone())        // Mọi trạng thái đều là kết quả
    for i in idx..arr.len():           // Chỉ duyệt từ idx TRỞ ĐI (tránh trùng)
        current.push(arr[i])
        subsets_helper(arr, i + 1, current, result)
        current.pop()                  // Undo!
```

```
Cây quyết định (Subsets [1,2,3]):

                    []
           /        |        \
        [1]        [2]      [3]
       /    \       |
    [1,2]  [1,3]  [2,3]
      |
  [1,2,3]
```

Điểm mấu chốt: vòng for bắt đầu từ `idx`, đảm bảo chỉ lấy phần tử **phía sau** -- nên {1,2} và {2,1} không bao giờ cùng xuất hiện.

### Pattern 2: Permutations -- "Xếp thứ tự"

Ẩn dụ: Bạn có 3 cuốn sách, xếp lên kệ. Thứ tự khác nhau = kết quả khác nhau.

```rust
// Template: swap-based, mỗi vị trí chọn 1 phần tử chưa dùng
fn permute_helper(arr, start, n, result):
    if start == n:
        result.add(arr.clone())
        return
    for i in start..n:
        arr.swap(start, i)            // Chọn phần tử i cho vị trí start
        permute_helper(arr, start+1, n, result)
        arr.swap(start, i)            // Undo swap!
```

```
Cây quyết định (Permutations [1,2,3]):

                    [1,2,3]
          /            |           \
     [1,_,_]       [2,_,_]      [3,_,_]
      /    \        /    \        /    \
  [1,2,_] [1,3,_] [2,1,_] [2,3,_] [3,1,_] [3,2,_]
    |        |       |       |       |        |
 [1,2,3] [1,3,2] [2,1,3] [2,3,1] [3,1,2] [3,2,1]
```

Điểm mấu chốt: Dùng **swap** thay vì `used[]` array -- tiết kiệm bộ nhớ, code ngắn hơn.

### Pattern 3: Combinations -- "Chọn k từ n"

Ẩn dụ: Bạn có 5 bạn bè, muốn rủ 3 người đi ăn. Không quan tâm thứ tự (rủ A trước B hay B trước A đều giống nhau).

```rust
// Template: giống subset nhưng chỉ thu kết quả khi current.len() == k
fn combine_helper(n, k, start, current, result):
    if current.len() == k:
        result.add(current.clone())
        return
    for i in start..=n:
        current.push(i)
        combine_helper(n, k, i + 1, current, result)
        current.pop()                  // Undo!
```

### Tóm tắt khác biệt

```
                 Subset          Permutation      Combination
─────────────────────────────────────────────────────────────
Bắt đầu vòng for idx (tăng)      start (swap)     start (tăng)
Điều kiện dừng  Không (add mọi)  start == n       len == k
Thứ tự          Không quan trọng Quan trọng       Không quan trọng
Số kết quả      2^n              n!               C(n,k)
```

---

## Chiến lược cắt tỉa (Pruning Strategies)

Cắt tỉa là thứ biến backtracking từ "chậm chết" thành "chấp nhận được". Có 2 chiến lược chính:

### 1. Early Termination -- Dừng sớm

Khi biết chắc nhánh hiện tại không thể dẫn đến lời giải, dừng ngay.

```
Ví dụ: Combination Sum, target = 7, candidates = [2, 3, 6, 7]

Nếu tổng hiện tại đã > 7 -> dừng ngay, không cần thử thêm.

current = [2, 2, 2, 2] -> tổng = 8 > 7 -> CẮT!
                                          (không thử thêm 2, 3, 6, 7)
```

```rust
// Khi candidates đã sort, nếu thêm candidate[i] vượt target -> break luôn
if current_sum + candidates[i] > target {
    break;  // Các candidate sau còn lớn hơn, không cần thử
}
```

### 2. Constraint Propagation -- Lan truyền ràng buộc

Khi chọn 1 giá trị, suy ra ngay những giá trị nào bị loại ở các bước sau.

```
Ví dụ: Sudoku -- điền số 5 vào ô (0,0)
  -> Loại số 5 khỏi tất cả ô trống trong hàng 0
  -> Loại số 5 khỏi tất cả ô trống trong cột 0
  -> Loại số 5 khỏi tất cả ô trống trong ô 3x3 góc trên trái

N-Queens -- đặt hậu ở cột 2, hàng 0
  -> Đánh dấu cột 2 không thể dùng
  -> Đánh dấu 2 đường chéo qua (0,2) không thể dùng
```

Trong code N-Queens, mảng `cols[]`, `diag1[]`, `diag2[]` chính là constraint propagation -- kiểm tra O(1) thay vì duyệt toàn bộ board.

---

## Code Rust

### N-Queens

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
            // Cắt tỉa: cột, đường chéo 1, đường chéo 2 đều trống?
            if !cols[col] && !diag1[d1] && !diag2[d2] {
                // --- make ---
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;
                placement.push(col);
                // --- recurse ---
                solve(row + 1, n, placement, cols, diag1, diag2, solutions);
                // --- undo ---
                placement.pop();
                cols[col] = false;
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
```

**Trace chi tiết (n=4, tìm lời giải đầu tiên):**

```
solve(row=0): thử col=0
  cols=[T,F,F,F]  placement=[0]
  solve(row=1): thử col=0 -> cols[0]=T, skip
                thử col=1 -> diag2 trùng, skip
                thử col=2 -> OK!
    cols=[T,F,T,F]  placement=[0,2]
    solve(row=2): col=0,1,2,3 -> tất cả bị chặn
    QUAY LUI: cols=[T,F,F,F]  placement=[0]
                thử col=3 -> OK!
    cols=[T,F,F,T]  placement=[0,3]
    solve(row=2): thử col=1 -> OK!
      cols=[T,T,F,T]  placement=[0,3,1]
      solve(row=3): col=0,1,2,3 -> tất cả bị chặn
      QUAY LUI: placement=[0,3]
    QUAY LUI: placement=[0]
  QUAY LUI: placement=[]

solve(row=0): thử col=1
  cols=[F,T,F,F]  placement=[1]
  solve(row=1): thử col=3 -> OK!
    cols=[F,T,F,T]  placement=[1,3]
    solve(row=2): thử col=0 -> OK!
      cols=[T,T,F,T]  placement=[1,3,0]
      solve(row=3): thử col=2 -> OK!
        placement=[1,3,0,2]  row=4=n  --> LOI GIAI!
        . Q . .
        . . . Q
        Q . . .
        . . Q .
```

### Sudoku Solver

```rust
/// Sudoku: điền số vào ô trống (giá trị 0) bằng backtracking.
pub fn solve_sudoku(board: &mut [[u8; 9]; 9]) -> bool {
    // Tìm ô trống tiếp theo
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == 0 {
                for num in 1..=9 {
                    if is_valid(board, r, c, num) {
                        board[r][c] = num;           // make
                        if solve_sudoku(board) {
                            return true;             // Giải được!
                        }
                        board[r][c] = 0;             // undo (quay lui)
                    }
                }
                return false;  // Không có số nào hợp lệ -> quay lui
            }
        }
    }
    true  // Không còn ô trống -> đã giải xong
}
```

### Permutations (swap-based)

```rust
/// Tất cả hoán vị bằng backtracking (dựa trên swap).
pub fn permutations<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = arr.to_vec();
    let n = current.len();

    fn permute_helper<T: Clone>(
        arr: &mut Vec<T>, start: usize, n: usize, result: &mut Vec<Vec<T>>
    ) {
        if start == n {
            result.push(arr.clone());  // clone ở đây vì arr bị mutate
            return;
        }
        for i in start..n {
            arr.swap(start, i);        // make: chọn arr[i] cho vị trí start
            permute_helper(arr, start + 1, n, result);
            arr.swap(start, i);        // undo: hoán đổi lại
        }
    }

    permute_helper(&mut current, 0, n, &mut result);
    result
}
```

### Subsets (tập lũy thừa)

```rust
/// Tập lũy thừa: bao gồm/loại trừ từng phần tử.
pub fn subsets<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let mut current: Vec<T> = Vec::new();

    fn subsets_helper<T: Clone>(
        arr: &[T], idx: usize, current: &mut Vec<T>, result: &mut Vec<Vec<T>>
    ) {
        result.push(current.clone()); // Mọi trạng thái đều là subset
        for i in idx..arr.len() {
            current.push(arr[i].clone());  // make
            subsets_helper(arr, i + 1, current, result);
            current.pop();                 // undo
        }
    }

    subsets_helper(arr, 0, &mut current, &mut result);
    result
}
```

**Ghi chú về Rust:**

- Dùng 3 mảng `cols`, `diag1`, `diag2` để kiểm tra nhanh (O(1)) thay vì duyệt toàn bộ bàn cờ. Đây là constraint propagation.
- `placement.push(col)` rồi `placement.pop()` là pattern make/undo kinh điển: thêm -> đệ quy -> bỏ.
- Hàm `solve` là hàm lồng (nested function) trong Rust -- không cần tạo struct riêng.
- `arr.swap(start, i)` trong permutations -- Rust yêu cầu `&mut` nên swap rất tự nhiên, không cần mảng `used[]` phụ.

---

## Pitfalls — Những cái bẫy hay gặp

### 1. Quên undo -> State bị corrupt

```
❌ Sai:
    current.push(choice);
    backtrack(state, result);
    // Quên current.pop() !!!

✅ Đúng:
    current.push(choice);
    backtrack(state, result);
    current.pop();             // LUÔN LUÔN undo

💡 Tại sao: Nếu quên undo, các nhánh sau sẽ thấy state "bẩn" từ nhánh trước.
   Kết quả sẽ sai hoàn toàn -- và rất khó debug vì lỗi "lan" sang mọi nhánh.
   Mẹo: Viết make và undo cạnh nhau TRƯỚC khi viết phần recurse ở giữa.
```

### 2. Clone overhead -> Memory Explosion

```
❌ Sai:
    // Clone toàn bộ board 9x9 mỗi lần gọi đệ quy
    let new_board = board.clone();
    backtrack(new_board, result);

✅ Đúng:
    // Mutate in-place + undo
    board[r][c] = num;
    backtrack(board, result);
    board[r][c] = 0;

💡 Tại sao: Clone tạo bản sao mới mỗi lần gọi. Với Sudoku, nếu có 50 ô trống
   và mỗi ô thử 9 số, clone 50 lần x 81 ô = hàng chục ngàn bản sao.
   Mutate in-place + undo = 0 allocation phụ.
   Chỉ clone khi cần LƯU kết quả (result.push(state.clone())).
```

### 3. Pruning không đủ -> TLE (Time Limit Exceeded)

```
❌ Sai:
    // Combination Sum: không sort, không cắt sớm
    for i in 0..candidates.len() {
        backtrack(...)  // Duyệt hết mọi thứ, kể cả khi tổng đã vượt target
    }

✅ Đúng:
    // Sort candidates trước, break sớm khi tổng vượt target
    candidates.sort();
    for i in start..candidates.len() {
        if current_sum + candidates[i] > target { break; }
        backtrack(...)
    }

💡 Tại sao: Không có pruning, backtracking là brute-force thuần túy -- O(n^n)
   hoặc tệ hơn. Pruning tốt có thể cắt 99% nhánh.
   Luôn tự hỏi: "Tại bước này, có cách nào biết sớm nhánh này sẽ thất bại?"
```

### 4. Kết quả trùng lặp

```
❌ Sai:
    // Subset với input [1, 2, 2]: kết quả có {1,2} xuất hiện 2 lần

✅ Đúng:
    // Sort input trước, skip duplicate ở cùng level
    if i > start && candidates[i] == candidates[i-1] { continue; }

💡 Tại sao: Khi input có phần tử trùng, cùng 1 level trong cây quyết định
   sẽ sinh ra nhánh giống hệt nhau. Sort + skip là pattern chuẩn để loại trùng.
```

---

## Khi nào dùng Backtracking?

| Dấu hiệu | Ví dụ | Dùng Backtracking? |
|-----------|-------|--------------------|
| "Tìm TẤT CẢ lời giải" | All permutations, all subsets | Có |
| "Tìm MỘT lời giải thỏa ràng buộc" | Sudoku, N-Queens | Có |
| "Tìm giá trị TỐI ƯU" (min/max) | Knapsack, coin change | DP thường tốt hơn |
| Input nhỏ (n <= 15-20) | Bitmask problems | Có (hoặc bitmask) |
| Input lớn (n > 25) | - | Thường TLE, cần DP hoặc Greedy |
| "Có bao nhiêu cách?" (chỉ đếm) | Counting subsets | DP nếu có overlapping |

**Quy tắc ngón tay cái:** Nếu bài yêu cầu **liệt kê** (enumerate) mọi kết quả, backtracking là lựa chọn tự nhiên. Nếu chỉ cần **đếm** hoặc **tối ưu**, xem xét DP trước (chương 6.13).

---

## Độ phức tạp

| Bài toán | Thời gian | Bộ nhớ |
|---------|----------|--------|
| N-Queens | O(n!) | O(n) |
| Sudoku | O(9^(ô trống)) | O(ô trống) |
| Hoán vị | O(n! * n) | O(n! * n) |
| Tập con | O(2^n * n) | O(2^n * n) |
| Tổ hợp C(n,k) | O(C(n,k) * k) | O(C(n,k) * k) |
| Combination Sum | O(2^t) (t = target/min) | O(target/min) |

**Giải thích thực tế:**

- Cắt tỉa giảm ĐÁNG KỂ số node phải duyệt so với worst-case. Ví dụ: N-Queens n=8 có 92 lời giải, nhưng chỉ duyệt vài ngàn node (thay vì hàng triệu).
- Backtracking vẫn là thuật toán **exponential** -- không phù hợp cho bài toán quá lớn. Nhưng với kích thước vừa phải (Sudoku 9x9, N-Queens n < 20), nó rất hiệu quả.
- So với DP (chương 6.13): DP tránh tính lại bài toán con trùng lặp. Backtracking thì duyệt hết mọi nhánh. Nếu bài có **overlapping sub-problems**, DP thắng. Nếu cần **liệt kê mọi lời giải**, backtracking phù hợp hơn.

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

---

## Practice — Luyện tập

Sắp xếp từ dễ đến khó. Mỗi bài map với 1 pattern ở trên:

| # | Bài | Pattern | Gợi ý |
|---|-----|---------|-------|
| 78 | [Subsets](https://leetcode.com/problems/subsets/) | Subset template | Template y hệt `subsets()` ở trên |
| 46 | [Permutations](https://leetcode.com/problems/permutations/) | Permutation template | Swap-based hoặc used[] đều được |
| 39 | [Combination Sum](https://leetcode.com/problems/combination-sum/) | Combination + pruning | Sort + break sớm khi tổng vượt target |
| 51 | [N-Queens](https://leetcode.com/problems/n-queens/) | Constraint backtracking | Dùng cols[], diag1[], diag2[] |
| 79 | [Word Search](https://leetcode.com/problems/word-search/) | Grid backtracking | DFS trên matrix, đánh dấu visited rồi undo |

**Gợi ý tiếp cận:** Với mỗi bài, hãy tự hỏi 6 câu theo template:
1. **state** là gì?
2. **complete** khi nào?
3. **choices** ở mỗi bước là gì?
4. **valid** -- điều kiện cắt tỉa?
5. **make** -- thay đổi state thế nào?
6. **undo** -- hoàn tác thế nào?

Trả lời được 6 câu này = giải được bài.

---

## Rust Ecosystem

### `itertools` crate

Trong Rust production code, bạn không cần tự viết permutations/combinations. Crate [`itertools`](https://docs.rs/itertools/) cung cấp sẵn:

```rust
use itertools::Itertools;

// Hoán vị
let perms: Vec<Vec<&i32>> = vec![1, 2, 3].iter().permutations(3).collect();
assert_eq!(perms.len(), 6);

// Tổ hợp
let combs: Vec<Vec<&i32>> = vec![1, 2, 3, 4].iter().combinations(2).collect();
assert_eq!(combs.len(), 6);  // C(4,2) = 6

// Tập lũy thừa (power set)
let pset: Vec<Vec<&i32>> = vec![1, 2, 3].iter().powerset().collect();
assert_eq!(pset.len(), 8);
```

`itertools` dùng **lazy iterators** -- chỉ sinh kết quả khi cần, tiết kiệm memory. Trong phỏng vấn thì tự viết, trong production thì dùng `itertools`.

### Ownership và Backtracking

Rust's borrow checker thực ra **giúp** bạn viết backtracking đúng:

- `&mut` bắt buộc bạn chỉ có 1 mutable reference -- không thể vô tình modify state từ 2 nơi.
- Nếu quên `undo`, compiler sẽ không báo lỗi (đây là logic error, không phải type error). Nhưng pattern `push/pop`, `swap/swap` rất rõ ràng trong Rust.
- `Vec::push()` + `Vec::pop()` là O(1) amortized -- hoàn hảo cho make/undo.

### KaCrab -- giải bài tập

Dùng KaCrab để luyện backtracking problems:

```bash
# Giải bài Subsets trên LeetCode
kacrab solve 78

# Chạy test cho bài N-Queens
kacrab test 51
```

---

## Tổng kết

```
Backtracking = DFS trên cây quyết định ẩn + undo

Template duy nhất:
  make -> recurse -> undo

3 pattern chính:
  Subset:      for i in idx..n, mọi trạng thái là kết quả
  Permutation: swap(start, i), kết quả khi start == n
  Combination: for i in start..n, kết quả khi len == k

Cắt tỉa = sống còn:
  Early termination + Constraint propagation

Nhớ:
  - Luôn undo
  - Clone chỉ khi lưu kết quả
  - Sort input để dễ pruning
```

---

## Tiếp theo

Chúc mừng! Bạn đã hoàn thành **Phần 6: Giải thuật** -- từ Recursion đến Backtracking. Đây là nền tảng vững chắc để giải phần lớn bài phỏng vấn.

Phần tiếp theo -- **Phần 7: Kỹ thuật giải bài** -- bắt đầu với [Bit Manipulation](../07-patterns/01-bit-manipulation.md). Bit manipulation là kỹ thuật dùng phép toán bit (AND, OR, XOR, shift) để giải bài cực nhanh. Nhiều bài backtracking (như subset) có thể giải bằng bitmask -- nhanh hơn, code ngắn hơn. Đó sẽ là cầu nối hoàn hảo từ chương này.

---

[← Dynamic Programming](./13-dynamic-programming.md) | [Bit Manipulation →](../07-patterns/01-bit-manipulation.md)
