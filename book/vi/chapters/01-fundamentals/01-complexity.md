# Phân tích Độ phức tạp (Big-O)

> 💡 **Đừng lo lắng:** Big-O nghe như toán cao cấp nhưng thực ra chỉ là đếm bước thôi. Bạn không cần giỏi toán -- chỉ cần biết trả lời câu hỏi: "nếu dữ liệu tăng gấp 10, code chạy lâu hơn bao nhiêu?" Đọc xong chương này bạn sẽ thấy nó dễ hơn bạn tưởng rất nhiều.

## Đây là gì?

> **Nếu bạn đang đọc đến đây và cảm thấy "Big-O nghe đáng sợ quá"** -- bạn không cô đơn đâu. Rất nhiều lập trình viên đã đi làm 2-3 năm vẫn nhầm lẫn khi phân tích Big-O. Đây là một kỹ năng cần **luyện tập**, không phải thứ ai cũng hiểu ngay từ lần đầu. Bài này sẽ đi từng bước nhỏ, có ví dụ cụ thể cho mỗi khái niệm. Nếu đọc xong một phần mà thấy chưa hiểu, đọc lại lần nữa -- hoàn toàn bình thường.

### Mental model: Câu hỏi duy nhất bạn cần nhớ

Trước khi vào bất kỳ ký hiệu nào, hãy ghi nhớ **một câu hỏi duy nhất** -- đây là kim chỉ nam xuyên suốt bài:

> **"Nếu dữ liệu tăng gấp 10 lần, số bước tăng bao nhiêu lần?"**

Ví dụ:
- Tìm sách bằng số locker → dữ liệu tăng 10 lần, vẫn chỉ 1 bước → **không đổi**
- Lật từng cuốn sách → 10 lần sách = 10 lần bước → **tăng tỷ lệ thuận**
- So sánh từng cặp học sinh → 10 lần học sinh = 100 lần cặp → **tăng bình phương**

Mỗi khi bạn gặp một đoạn code và không biết Big-O là gì, hãy tự hỏi câu hỏi trên. Nó sẽ giúp bạn đoán đúng 80% các trường hợp mà không cần nhớ công thức.

---

Bạn có 1000 cuốn sách chưa sắp xếp, cần tìm 1 cuốn. Bạn sẽ tìm thế nào?

**Cách 1:** Lật từng cuốn một, từ đầu đến cuối. Xui nhất thì bạn lật cả 1000 cuốn mới thấy. Đó là tìm kiếm tuyến tính -- **O(n)**.

→ Dữ liệu tăng 10 lần? Số bước cũng tăng 10 lần. ✓

**Cách 2:** Nếu sách đã xếp theo alphabet, bạn mở giữa kệ, xem tên sách. Nếu cuốn cần tìm nằm trước thì bỏ nửa sau. Lặp lại. Chỉ cần khoảng 10 lần là tìm thấy trong 1000 cuốn. Đó là tìm kiếm nhị phân -- **O(log n)**.

→ Dữ liệu tăng 10 lần? Số bước chỉ tăng thêm ~3 bước. ✓

**Cách 3:** Bạn nhớ chính xác cuốn sách nằm ở vị trí số 42. Đi thẳng tới, lấy luôn. Chỉ 1 bước. Đó là **O(1)**.

→ Dữ liệu tăng 10 lần? Vẫn 1 bước. ✓

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

#### O(n log n) -- Tại sao lại "n nhân log n"?

Phần này hay bị bỏ qua, nhưng O(n log n) xuất hiện **cực kỳ thường xuyên** (mọi thuật toán sort tốt đều là O(n log n)), nên hãy hiểu kỹ.

Tưởng tượng bạn là giáo viên, cần xếp 16 bài kiểm tra theo điểm từ thấp đến cao.

**Cách brute force (O(n^2)):** So sánh mọi bài với mọi bài khác. 16 × 16 = 256 lần so sánh.

**Cách thông minh hơn (merge sort -- O(n log n)):**
1. Chia 16 bài thành 2 nhóm, mỗi nhóm 8 bài
2. Chia tiếp mỗi nhóm 8 thành 2 nhóm 4
3. Chia tiếp mỗi nhóm 4 thành 2 nhóm 2
4. Chia tiếp mỗi nhóm 2 thành 2 nhóm 1 (đã sort sẵn!)
5. Bây giờ **gộp lại**: gộp 2 nhóm 1 thành nhóm 2 đã sort, gộp 2 nhóm 2 thành nhóm 4... cho đến khi gộp thành 1 nhóm 16 đã sort.

```
Tầng 1:  [16 bài]                          ← gộp: so sánh ~16 lần
Tầng 2:  [8 bài] [8 bài]                   ← gộp: so sánh ~16 lần
Tầng 3:  [4] [4] [4] [4]                   ← gộp: so sánh ~16 lần
Tầng 4:  [2][2][2][2][2][2][2][2]          ← gộp: so sánh ~16 lần
         [1][1][1][1][1][1][1][1][1][1]... ← đã sort sẵn
```

Có bao nhiêu tầng? Chia đôi 16 → 8 → 4 → 2 → 1, tức **4 tầng = log₂(16)**. Mỗi tầng làm ~n = 16 phép so sánh. Tổng = **n × log n = 16 × 4 = 64** lần so sánh.

So với cách brute force 256 lần, nhanh hơn 4 lần. Với n = 1,000,000 thì khác biệt là **11 ngày vs 20 giây**. Đó là sức mạnh của O(n log n).

Cách nhớ: **O(n^2)** giống "so sánh mọi người với mọi người". **O(n log n)** giống "chia thành nhóm nhỏ, sort từng nhóm, rồi gộp lại thông minh".

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

**Chia để trị (Divide and Conquer)** -- chia đôi VÀ làm O(n) việc ở mỗi tầng thì ra O(n log n). Merge sort là ví dụ kinh điển (xem phần giải thích O(n log n) ở trên).

### Cách phân tích đệ quy (recursion)

> **Chưa biết đệ quy là gì?** Đệ quy là khi một hàm **gọi lại chính nó**. Giống như bạn đứng giữa hai tấm gương đối diện -- hình ảnh phản chiếu lặp lại vô tận. Trong lập trình, đệ quy luôn cần một "điều kiện dừng" (base case) để không lặp mãi. Nếu bạn chưa quen, đọc qua [chương Đệ quy](../06-algorithms/01-recursion.md) trước rồi quay lại đây.

Với hàm đệ quy, ta viết **công thức truy hồi** (recurrence relation) rồi giải.

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

Tại sao lại O(2^n)? Hãy vẽ ra cây gọi hàm để thấy:

```
                    fib(5)
                   /      \
              fib(4)        fib(3)
             /     \        /     \
         fib(3)   fib(2)  fib(2)  fib(1)
         /   \    /   \    /   \
     fib(2) fib(1) fib(1) fib(0) fib(1) fib(0)
     /   \
 fib(1) fib(0)
```

Mỗi lần gọi sinh ra **2** lần gọi nữa. Tầng 1 có 1 node, tầng 2 có 2, tầng 3 có 4, tầng 4 có 8... Tổng xấp xỉ 2^n. Với n = 40, đó là hơn 1 tỷ lần gọi. Chậm kinh khủng!

Và để ý: fib(3) được tính **2 lần**, fib(2) được tính **3 lần**. Rất nhiều công tính lặp lại vô ích. (Đây là lý do memoization/dynamic programming ra đời -- xem chương DP.)

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

#### Master Theorem -- công cụ giải recurrence nhanh

Khi bạn thấy recurrence dạng **T(n) = a · T(n/b) + O(n^d)**, Master Theorem cho bạn đáp án ngay:

| So sánh | Kết quả | Ví dụ |
|---------|---------|-------|
| a < b^d | **O(n^d)** | Công việc ở mỗi tầng giảm nhanh, tầng gốc dominate |
| a = b^d | **O(n^d · log n)** | Mỗi tầng làm việc ngang nhau |
| a > b^d | **O(n^(log_b a))** | Lá của cây dominate |

Giải thích bằng tiếng người:
- **a** = số bài toán con (chia thành bao nhiêu phần)
- **b** = kích thước giảm bao nhiêu lần (chia đôi thì b = 2)
- **d** = lượng công việc "ngoài đệ quy" ở mỗi tầng

Áp dụng:
- **Binary search:** T(n) = 1·T(n/2) + O(1) → a=1, b=2, d=0 → a = b^d = 1 → O(n^0 · log n) = **O(log n)** ✓
- **Merge sort:** T(n) = 2·T(n/2) + O(n) → a=2, b=2, d=1 → a = b^d = 2 → O(n^1 · log n) = **O(n log n)** ✓

Không cần nhớ công thức chính xác -- chỉ cần biết nó tồn tại để khi gặp recurrence dạng này, bạn tra cứu được.

### Độ phức tạp không gian (Space Complexity)

Ngoài thời gian, ta cũng quan tâm thuật toán dùng bao nhiêu **bộ nhớ thêm** (ngoài dữ liệu đầu vào).

Tại sao quan trọng? Vì máy tính có **giới hạn RAM**. Nếu thuật toán của bạn cần O(n^2) space với n = 100,000, đó là 10 tỷ ô nhớ -- có thể **crash** chương trình.

#### O(1) space -- chỉ dùng vài biến

```rust
fn find_max(arr: &[i32]) -> Option<i32> {
    let mut max = arr.first()?;
    for &val in &arr[1..] {
        if val > *max {
            max = &val;
        }
    }
    Some(*max)
}
// Chỉ dùng 1 biến `max` -- O(1) space
// Dù mảng có 1 triệu phần tử, vẫn chỉ cần 1 biến
```

#### O(n) space -- tạo cấu trúc mới tỷ lệ với input

```rust
fn reverse_copy(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr.len());
    for &val in arr.iter().rev() {
        result.push(val);
    }
    result
}
// Tạo Vec mới chứa n phần tử -- O(n) space
```

#### O(n^2) space -- ma trận 2 chiều

```rust
fn create_distance_matrix(n: usize) -> Vec<Vec<i32>> {
    vec![vec![0; n]; n]
    // Tạo bảng n x n -- O(n^2) space
    // Với n = 10,000: cần 100 triệu ô nhớ (~400MB cho i32)
}
```

#### Call stack của đệ quy cũng chiếm space!

Đây là điều **rất nhiều người không biết**. Mỗi lần gọi hàm đệ quy, máy tính phải lưu lại "đang ở đâu" trên call stack. Đệ quy sâu n tầng = O(n) space:

```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 { return 1; }
    n * factorial(n - 1)
    // Mỗi lần gọi đệ quy, 1 frame được push lên call stack
    // factorial(5) → factorial(4) → factorial(3) → factorial(2) → factorial(1)
    // Call stack sâu 5 tầng → O(n) space
}
```

```
Call stack khi tính factorial(5):
┌─────────────┐
│ factorial(1) │  ← đang chạy
├─────────────┤
│ factorial(2) │  ← đang đợi
├─────────────┤
│ factorial(3) │  ← đang đợi
├─────────────┤
│ factorial(4) │  ← đang đợi
├─────────────┤
│ factorial(5) │  ← đang đợi
└─────────────┘
```

Nếu n = 1,000,000, call stack sẽ có 1 triệu frame → có thể bị **stack overflow**!

#### Trade-off: Đánh đổi space để giảm time

Đôi khi ta **chấp nhận dùng thêm bộ nhớ** để code chạy nhanh hơn nhiều. Ví dụ kinh điển: bài toán Two Sum.

**Cách chậm -- O(n^2) time, O(1) space:**

```rust
fn two_sum_brute(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}
```

**Cách nhanh -- O(n) time, O(n) space:**

```rust
use std::collections::HashMap;

fn two_sum_fast(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &val) in arr.iter().enumerate() {
        let complement = target - val;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.insert(val, i);
    }
    None
}
// Dùng thêm O(n) space cho HashMap
// Đổi lại: time giảm từ O(n^2) xuống O(n)
```

Với n = 1,000,000: cách chậm cần ~500 tỷ phép so sánh (~6 ngày). Cách nhanh chỉ cần 1 triệu bước (~1 giây), đổi lại dùng thêm ~8MB RAM. Hoàn toàn đáng.

| Trường hợp | Space |
|------------|-------|
| Vài biến cố định | O(1) |
| Copy toàn bộ input | O(n) |
| Ma trận 2 chiều | O(n^2) |
| Đệ quy sâu d tầng (call stack) | O(d) |

## Sai lầm thường gặp

### Sai lầm 1: Nghĩ O(n) + O(n) = O(n^2)

**Cách nghĩ sai:** "Có 2 cái O(n), nhân lại thành O(n^2)."

```rust
fn process(arr: &[i32]) -> i32 {
    // Bước 1: tìm max -- O(n)
    let max = arr.iter().max().unwrap_or(&0);

    // Bước 2: tính tổng -- O(n)
    let sum: i32 = arr.iter().sum();

    max + sum
}
// ĐÚNG: O(n) + O(n) = O(2n) = O(n)
// SAI:  O(n) × O(n) = O(n^2)
```

**Cách nghĩ đúng:** Hai vòng lặp **nối tiếp nhau** thì **cộng**. Chỉ khi vòng lặp **lồng bên trong** nhau thì mới **nhân**. Giống như bạn đi chợ mất 30 phút, rồi nấu cơm mất 30 phút -- tổng là 60 phút (cộng), không phải 900 phút (nhân).

### Sai lầm 2: Bỏ quên space complexity của đệ quy

**Cách nghĩ sai:** "Hàm này không tạo mảng mới, nên space là O(1)."

```rust
fn sum_recursive(arr: &[i32]) -> i32 {
    if arr.is_empty() { return 0; }
    arr[0] + sum_recursive(&arr[1..])
    // Không tạo mảng mới... nhưng call stack sâu n tầng!
    // Space: O(n) chứ không phải O(1)
}
```

**Cách nghĩ đúng:** Mỗi lần gọi đệ quy, máy phải lưu trạng thái lên call stack. Đệ quy sâu n tầng = O(n) space. Luôn hỏi: "Đệ quy này sâu tối đa bao nhiêu tầng?"

### Sai lầm 3: Nghĩ HashMap luôn là O(1)

**Cách nghĩ sai:** "HashMap.get() là O(1), luôn luôn."

```rust
use std::collections::HashMap;

fn count_words(words: &[String]) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in words {
        *map.entry(word.as_str()).or_insert(0) += 1;
    }
    map
}
// Average-case: O(1) per operation → tổng O(n)
// Worst-case:   O(n) per operation → tổng O(n^2)
// (khi tất cả keys hash vào cùng 1 bucket)
```

**Cách nghĩ đúng:** HashMap là O(1) **trung bình** (average-case). Trong trường hợp xấu nhất (hash collision), nó có thể là O(n). Trong thực tế gần như luôn là O(1), nhưng khi phỏng vấn hoặc viết code production, nên biết điều này.

### Sai lầm 4: Nghĩ O(n log n) và O(n^2) "gần nhau"

**Cách nghĩ sai:** "log n nhỏ thôi, nên n log n và n^2 chênh lệch không nhiều."

Hãy nhìn bảng này:

| n | log₂ n | n log n | n^2 | Chênh lệch |
|---|--------|---------|-----|------------|
| 100 | 7 | 700 | 10,000 | 14x |
| 10,000 | 13 | 130,000 | 100,000,000 | 769x |
| 1,000,000 | 20 | 20,000,000 | 1,000,000,000,000 | **50,000x** |

**Cách nghĩ đúng:** log n tăng **cực kỳ chậm**. log₂(1 triệu) chỉ = 20. Nên n log n gần với n hơn là gần với n^2. Sự khác biệt giữa O(n log n) và O(n^2) có thể là khác biệt giữa **chạy xong trong 20 giây** và **chạy mất 11 ngày**.

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

## Luyện nhận diện

Hãy thử phân tích Big-O của mỗi đoạn code dưới đây **trước khi** xem đáp án.

**Bài 1:**

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}
```

<details>
<summary>Xem đáp án</summary>

**O(1)** -- Chỉ thực hiện 1 phép chia lấy dư và 1 phép so sánh. Không phụ thuộc vào độ lớn của n.
</details>

**Bài 2:**

```rust
fn contains(arr: &[i32], target: i32) -> bool {
    for &val in arr {
        if val == target {
            return true;
        }
    }
    false
}
```

<details>
<summary>Xem đáp án</summary>

**O(n)** -- Trường hợp xấu nhất: target không có trong mảng, phải duyệt hết n phần tử. Trường hợp tốt nhất là O(1) (tìm thấy ngay phần tử đầu), nhưng Big-O đo worst-case.
</details>

**Bài 3:**

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut lo, mut hi) = (0, arr.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}
```

<details>
<summary>Xem đáp án</summary>

**O(log n)** -- Mỗi bước chia đôi phạm vi tìm kiếm. n = 1,000,000 → chỉ cần ~20 bước. Pattern nhận diện: thấy `lo`, `hi`, `mid`, và phạm vi bị chia đôi mỗi vòng lặp → nghĩ ngay O(log n).
</details>

**Bài 4:**

```rust
fn all_pairs(arr: &[i32]) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            pairs.push((arr[i], arr[j]));
        }
    }
    pairs
}
```

<details>
<summary>Xem đáp án</summary>

**O(n^2)** -- Vòng lặp lồng nhau. Dù vòng trong bắt đầu từ `i + 1` (không phải 0), tổng số cặp vẫn là n*(n-1)/2 ≈ n^2/2. Bỏ hằng số → O(n^2). Chú ý space cũng là O(n^2) vì `pairs` chứa ~n^2/2 phần tử.
</details>

**Bài 5:**

```rust
fn sneaky_function(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &val in arr {
        if !result.contains(&val) {   // ← chú ý dòng này
            result.push(val);
        }
    }
    result
}
```

<details>
<summary>Xem đáp án</summary>

**O(n^2)** -- Đây là bài "bẫy"! Thoạt nhìn chỉ có 1 vòng lặp for → tưởng O(n). Nhưng `result.contains(&val)` bên trong **cũng duyệt qua toàn bộ result** -- đó là một vòng lặp ẩn O(n). Tổng: O(n) × O(n) = O(n^2).

Cách fix: dùng `HashSet` thay vì `Vec` để kiểm tra trùng lặp → O(n) tổng.
</details>

**Bài 6:**

```rust
fn build_string(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s = format!("{}{}", s, i);  // ← chú ý dòng này
    }
    s
}
```

<details>
<summary>Xem đáp án</summary>

**O(n^2)** -- Một bẫy kinh điển! `format!("{}{}", s, i)` tạo **string mới** mỗi lần, copy toàn bộ string cũ. Lần 1 copy 1 ký tự, lần 2 copy 2, lần 3 copy 3... Tổng = 1 + 2 + 3 + ... + n = n*(n+1)/2 = O(n^2).

Cách fix: dùng `s.push_str(&i.to_string())` hoặc `write!` -- append vào cuối thay vì tạo mới → O(n) tổng.
</details>

## Quy tắc nhớ nhanh

1. **Bỏ hằng số.** O(2n) vẫn là O(n). Vì khi n đủ lớn, nhân 2 chẳng thay đổi bản chất.
2. **Bỏ số hạng nhỏ.** O(n^2 + n) vẫn là O(n^2). Khi n = 1 triệu, n^2 = 10^12 còn n chỉ = 10^6 -- nhỏ xíu.
3. **Các bước liên tiếp thì cộng.** O(n) + O(m) = O(n + m).
4. **Các bước lồng nhau thì nhân.** Vòng lặp trong vòng lặp: O(n * m).
5. **Tập trung vào trường hợp xấu nhất.** Trừ khi đề nói khác.

## Quick Reference Card

Bảng tra cứu nhanh -- bookmark trang này!

### Pattern nhận diện

| Thấy pattern này trong code | → Nghĩ ngay |
|----------------------------|------------|
| Truy cập index, phép tính đơn | O(1) |
| `while n > 1 { n /= 2 }` hoặc binary search | O(log n) |
| Một vòng for qua n phần tử | O(n) |
| Sort, hoặc chia đôi + merge | O(n log n) |
| Vòng lặp lồng 2 tầng trên cùng tập dữ liệu | O(n^2) |
| Đệ quy chia 2 nhánh mỗi tầng (Fibonacci) | O(2^n) |
| Hoán vị, permutation | O(n!) |
| **Bẫy:** hàm ẩn O(n) bên trong vòng lặp | Kiểm tra `.contains()`, string concat |

### Quy tắc rút gọn

```
O(2n)         → O(n)         bỏ hằng số
O(n^2 + n)    → O(n^2)       bỏ số hạng nhỏ
O(n) + O(n)   → O(n)         cộng, rồi bỏ hằng số
O(n) × O(n)   → O(n^2)       nhân khi lồng nhau
```

### Complexity của operations phổ biến

| Cấu trúc / Thuật toán | Operation | Average | Worst |
|----------------------|-----------|---------|-------|
| **Array/Vec** | access by index | O(1) | O(1) |
| | search (unsorted) | O(n) | O(n) |
| | push_back | O(1)* | O(n)* |
| **HashMap** | get/insert/remove | O(1) | O(n) |
| **BTreeMap** | get/insert/remove | O(log n) | O(log n) |
| **Sort** (Rust's sort) | | O(n log n) | O(n log n) |
| **Binary search** | | O(log n) | O(log n) |
| **BFS/DFS** | graph traversal | O(V + E) | O(V + E) |

*\* Vec push_back là O(1) amortized -- thỉnh thoảng phải mở rộng mảng thì mất O(n), nhưng trung bình vẫn O(1).*

### Checklist khi phân tích code mới

- [ ] Có vòng lặp không? Lồng bao nhiêu tầng?
- [ ] Có đệ quy không? Mỗi lần gọi sinh bao nhiêu nhánh?
- [ ] Có hàm ẩn bên trong vòng lặp không? (`.contains()`, `.find()`, string concat)
- [ ] Space: có tạo cấu trúc dữ liệu mới không? Đệ quy sâu bao nhiêu?
- [ ] Dùng mental model: "Dữ liệu tăng 10 lần, số bước tăng bao nhiêu lần?"

---

---

[← Giới thiệu](../../introduction.md) | [Arrays & Slices →](./02-arrays.md)
