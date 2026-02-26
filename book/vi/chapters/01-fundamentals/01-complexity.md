# Phân tích Độ phức tạp (Big-O)

## Đây là gì?

Bạn có 1000 cuốn sách chưa sắp xếp, cần tìm 1 cuốn. Bạn sẽ tìm thế nào?

**Cách 1:** Lật từng cuốn một, từ đầu đến cuối. Xui nhất thì bạn lật cả 1000 cuốn mới thấy. Đó là tìm kiếm tuyến tính -- **O(n)**.

**Cách 2:** Nếu sách đã xếp theo alphabet, bạn mở giữa kệ, xem tên sách. Nếu cuốn cần tìm nằm trước thì bỏ nửa sau. Lặp lại. Chỉ cần khoảng 10 lần là tìm thấy trong 1000 cuốn. Đó là tìm kiếm nhị phân -- **O(log n)**.

**Cách 3:** Bạn nhớ chính xác cuốn sách nằm ở vị trí số 42. Đi thẳng tới, lấy luôn. Chỉ 1 bước. Đó là **O(1)**.

Vậy Big-O là gì? Nó là cách chúng ta **đo tốc độ** của một thuật toán khi dữ liệu lớn dần. Thay vì đếm giây (vì máy nhanh chậm khác nhau), ta đếm **số bước** cần thực hiện.

Big-O -- viết là O(...) -- mô tả trường hợp **xấu nhất**. Khi nói một hàm là O(n), nghĩa là: khi dữ liệu tăng, thời gian chạy tăng **tỷ lệ thuận** với n.

Còn hai ký hiệu anh em:
- **Omega** -- trường hợp tốt nhất (ít khi quan tâm)
- **Theta** -- trường hợp chính xác (tight bound)

Nhưng trong thực tế, 99% thời gian người ta chỉ nói Big-O.

**Tại sao phải học cái này?** Vì nó giúp bạn **dự đoán** xem code chạy nhanh hay chậm. Nếu không hiểu Big-O, bạn có thể viết code mà với 100 phần tử chạy tốt, nhưng với 1 triệu phần tử thì chạy cả ngày không xong.

## Hoạt động như thế nào?

### Các lớp độ phức tạp phổ biến

Từ nhanh nhất đến chậm nhất:

```
O(1) < O(log n) < O(n) < O(n log n) < O(n^2) < O(2^n) < O(n!)
```

Hãy tưởng tượng bạn có n = 1000. Mỗi "bước" mất 1 micro giây (1 phần triệu giây):

| Lớp | n = 10 | n = 100 | n = 1,000 | n = 1,000,000 | Ví dụ thực tế |
|-----|--------|---------|-----------|---------------|---------------|
| **O(1)** | 1 | 1 | 1 | 1 | Mở tủ locker đúng số |
| **O(log n)** | 3 | 7 | 10 | 20 | Tìm tên trong danh bạ đã sắp xếp |
| **O(n)** | 10 | 100 | 1,000 | 1,000,000 | Đếm số người trong hàng trà sữa |
| **O(n log n)** | 33 | 664 | 9,966 | 19,931,568 | Sắp xếp danh sách lớp |
| **O(n^2)** | 100 | 10,000 | 1,000,000 | 10^12 | So sánh từng cặp học sinh |
| **O(2^n)** | 1,024 | ~10^30 | --- | --- | Liệt kê mọi tổ hợp |
| **O(n!)** | 3,628,800 | --- | --- | --- | Thử mọi cách sắp xếp |

Nhìn dòng O(n^2) với n = 1,000,000: cần 10^12 bước. Nếu mỗi bước 1 micro giây thì mất... **11 ngày**. Trong khi O(n log n) chỉ mất **20 giây**.

Đó là lý do Big-O quan trọng.

### Cách phân tích vòng lặp

**Một vòng lặp đơn** -- duyệt qua n phần tử là O(n):

```rust
fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for &val in arr {       // runs n times
        total += val;       // O(1) work per iteration
    }
    total
}
// Total: O(n)
```

Giống như bạn đếm tiền trong ví -- phải lật từng tờ một. Có bao nhiêu tờ thì lật bấy nhiêu lần.

**Vòng lặp lồng nhau** -- vòng lặp trong vòng lặp thì **nhân** lại:

```rust
fn print_pairs(arr: &[i32]) {
    for i in 0..arr.len() {         // n iterations
        for j in 0..arr.len() {     // n iterations each
            println!("({}, {})", arr[i], arr[j]);
        }
    }
}
// Total: O(n * n) = O(n^2)
```

Tưởng tượng lớp có 30 bạn. Thầy giáo bảo mỗi bạn phải bắt tay tất cả bạn khác. Bạn thứ 1 bắt tay 30 người, bạn thứ 2 bắt tay 30 người... Tổng = 30 x 30 = 900 cái bắt tay. Đó là n^2.

**Vòng lặp chia đôi** -- mỗi bước chia đôi vấn đề là O(log n):

```rust
fn count_halves(mut n: usize) -> usize {
    let mut steps = 0;
    while n > 1 {
        n /= 2;            // problem size halves
        steps += 1;
    }
    steps
}
// Total: O(log n)
```

Giống trò chơi đoán số: "Mình nghĩ 1 số từ 1 đến 1000. Bạn đoán 500. Mình nói: lớn hơn. Bạn đoán 750..." Mỗi lần đoán, bạn loại bỏ **một nửa** khả năng. Chỉ cần ~10 lần đoán cho 1000 số.

**Chia để trị (Divide and Conquer)** -- chia đôi VÀ làm O(n) việc ở mỗi tầng thì ra O(n log n). Merge sort là ví dụ kinh điển.

### Cách phân tích đệ quy (recursion)

Với hàm đệ quy, ta viết **công thức truy hồi** rồi giải.

**Ví dụ -- Fibonacci (cách chậm):**

```rust
fn fib(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    fib(n - 1) + fib(n - 2)
}
// Recurrence: T(n) = T(n-1) + T(n-2) + O(1)
// Solution:   T(n) = O(2^n)  (exponential!)
```

Mỗi lần gọi sinh ra 2 lần gọi nữa, tạo thành cây nhị phân sâu n tầng. Tổng số lần gọi xấp xỉ 2^n. Với n = 40, đó là hơn 1 tỷ lần gọi. Chậm kinh khủng!

**Ví dụ -- Binary search (đệ quy):**

```rust
fn binary_search_rec(arr: &[i32], target: i32, lo: usize, hi: usize) -> Option<usize> {
    if lo >= hi {
        return None;
    }
    let mid = lo + (hi - lo) / 2;
    match arr[mid].cmp(&target) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Less => binary_search_rec(arr, target, mid + 1, hi),
        std::cmp::Ordering::Greater => binary_search_rec(arr, target, lo, mid),
    }
}
// Recurrence: T(n) = T(n/2) + O(1)
// Solution:   T(n) = O(log n)
```

Mỗi bước chỉ đi vào **một nhánh** (trái hoặc phải), giảm phạm vi tìm kiếm đi một nửa. Nên chỉ cần O(log n).

### Độ phức tạp không gian (Space Complexity)

Ngoài thời gian, ta cũng quan tâm thuật toán dùng bao nhiêu **bộ nhớ thêm** (ngoài dữ liệu đầu vào).

| Trường hợp | Space |
|------------|-------|
| Vài biến cố định | O(1) |
| Copy toàn bộ input | O(n) |
| Ma trận 2 chiều | O(n^2) |
| Đệ quy sâu d tầng (call stack) | O(d) |

Ví dụ: hàm `sum` ở trên chỉ dùng 1 biến `total` -- O(1) space. Nhưng nếu bạn tạo một mảng mới copy toàn bộ dữ liệu -- O(n) space.

## Độ phức tạp

Bảng tóm tắt -- giữ lại đây để tra cứu suốt cuốn sách:

| Dạng code | Time | Space |
|-----------|------|-------|
| Một vòng lặp duyệt n phần tử | O(n) | O(1) |
| Vòng lặp lồng 2 tầng | O(n^2) | O(1) |
| Chia đôi mỗi bước | O(log n) | O(1) |
| Chia để trị + gộp | O(n log n) | O(n) |
| Liệt kê mọi tập con | O(2^n) | O(n) |
| Liệt kê mọi hoán vị | O(n!) | O(n) |

## Ví dụ

### Nhận biết Big-O trong code thực tế

**O(1) -- Tra cứu HashMap:**

```rust
use std::collections::HashMap;

fn lookup(map: &HashMap<String, i32>, key: &str) -> Option<i32> {
    map.get(key).copied()   // average-case O(1)
}
```

Giống như tìm số điện thoại khi bạn **biết chính xác tên người**. HashMap dùng hash function để nhảy thẳng tới đúng chỗ.

**O(n) -- Tìm giá trị lớn nhất:**

```rust
fn find_max(arr: &[i32]) -> Option<i32> {
    arr.iter().copied().max()  // one pass -> O(n)
}
```

Phải nhìn qua **tất cả** mới biết ai cao nhất lớp. Không có đường tắt.

**O(n log n) -- Sắp xếp rồi loại trùng:**

```rust
fn unique_sorted(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();           // O(n log n)
    v.dedup();          // O(n)
    v                   // dominant term: O(n log n)
}
```

Ở đây `sort()` chiếm O(n log n), `dedup()` chỉ O(n). Khi cộng lại, ta lấy **phần lớn nhất**: O(n log n). Giống như bạn nấu cơm 30 phút rồi rửa bát 5 phút -- tổng thời gian phụ thuộc vào việc nấu cơm.

**O(n^2) -- Kiểm tra mọi cặp:**

```rust
fn has_pair_with_sum(arr: &[i32], target: i32) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == target {
                return true;
            }
        }
    }
    false
}
```

Bạn có danh sách giá tiền, muốn tìm 2 món cộng lại đúng 100k. Phải thử từng cặp một. Nếu có 1000 món, cần thử gần 500,000 cặp.

### Quy tắc nhớ nhanh

1. **Bỏ hằng số.** O(2n) vẫn là O(n). Vì khi n đủ lớn, nhân 2 chẳng thay đổi bản chất.
2. **Bỏ số hạng nhỏ.** O(n^2 + n) vẫn là O(n^2). Khi n = 1 triệu, n^2 = 10^12 còn n chỉ = 10^6 -- nhỏ xíu.
3. **Các bước liên tiếp thì cộng.** O(n) + O(m) = O(n + m).
4. **Các bước lồng nhau thì nhân.** Vòng lặp trong vòng lặp: O(n * m).
5. **Tập trung vào trường hợp xấu nhất.** Trừ khi đề nói khác.
