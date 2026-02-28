# Bubble Sort, Selection Sort & Insertion Sort

> 💡 **Đừng lo lắng:** Ba thuật toán này đều chạy O(n²) — nghe "chậm" đúng không? Nhưng đừng skip chúng. Chúng là **foundation**. Bạn cần hiểu tại sao chúng chậm, thì mới thấy Merge Sort, Quick Sort ở chương sau hay ở chỗ nào. Giống như học cộng trừ trước rồi mới học nhân chia — không ai nhảy thẳng được. Hơn nữa, Insertion Sort thực tế được dùng **rất nhiều** bên trong các thuật toán sort cao cấp (Timsort, pdqsort). Nên đây không phải kiến thức "học cho có" đâu.

## Đây là gì?

Bạn đã bao giờ sắp xếp bài khi chơi tiến lên chưa? Ba thuật toán sắp xếp cơ bản này chính là những cách tự nhiên nhất mà con người thường làm khi sắp xếp thứ gì đó bằng tay.

### Bubble Sort — sắp xếp nổi bọt

Tưởng tượng bạn có một hàng bong bóng trong nước. Bong bóng lớn (nhẹ hơn) sẽ từ từ **nổi lên trên mặt nước**. Tương tự, trong Bubble Sort, phần tử lớn nhất sẽ dần dần "nổi" lên cuối mảng sau mỗi lượt duyệt.

Cách hoạt động: đi qua mảng, so sánh từng cặp kề nhau. Nếu sai thứ tự thì đổi chỗ. Sau mỗi lượt, phần tử lớn nhất "nổi" về đúng vị trí.

### Selection Sort — sắp xếp chọn

Giống cách bạn sắp bài khi chơi tiến lên: **nhìn cả đống bài, tìm lá nhỏ nhất, rút ra đặt trước**. Rồi lại tìm lá nhỏ nhất trong số còn lại, đặt tiếp theo. Cứ thế cho đến hết.

### Insertion Sort — sắp xếp chèn

Giống cách bạn cầm bài trên tay: mỗi khi rút thêm một lá bài mới, bạn **chèn nó vào đúng vị trí** trong những lá đã sắp xếp sẵn trên tay.

| Thuật toán | Ý tưởng chính |
|-----------|--------------|
| **Bubble Sort** | Đổi chỗ cặp kề nhau, phần tử lớn "nổi" lên cuối |
| **Selection Sort** | Tìm phần tử nhỏ nhất, đặt vào đầu |
| **Insertion Sort** | Chèn phần tử vào đúng vị trí trong phần đã sắp xếp |

---

## Hoạt động như thế nào?

### Bubble Sort — từng bước

Mảng ban đầu: `[5, 3, 8, 1, 2]`

**Lượt 1:** So sánh từng cặp kề nhau, đổi chỗ nếu sai thứ tự.

```
[5, 3, 8, 1, 2]
 ^--^  5 > 3 ? Có! Đổi chỗ
[3, 5, 8, 1, 2]
    ^--^  5 > 8 ? Không. Giữ nguyên
[3, 5, 8, 1, 2]
       ^--^  8 > 1 ? Có! Đổi chỗ
[3, 5, 1, 8, 2]
          ^--^  8 > 2 ? Có! Đổi chỗ
[3, 5, 1, 2, 8]
                     ^^^ 8 đã "nổi" về cuối!
```

**Lượt 2:** Không cần xét phần tử cuối (đã đúng chỗ).

```
[3, 5, 1, 2, | 8]
 ^--^  Không đổi
    ^--^  5 > 1 ? Đổi chỗ
[3, 1, 5, 2, | 8]
       ^--^  5 > 2 ? Đổi chỗ
[3, 1, 2, 5, | 8]
              ^^^ 5 đã đúng chỗ!
```

**Lượt 3:**

```
[3, 1, 2, | 5, 8]
 ^--^  3 > 1 ? Đổi chỗ
[1, 3, 2, | 5, 8]
    ^--^  3 > 2 ? Đổi chỗ
[1, 2, 3, | 5, 8]
           ^^^ 3 đã đúng chỗ!
```

**Kết quả:** `[1, 2, 3, 5, 8]` -- Đã sắp xếp xong!

> **Mẹo tối ưu:** Nếu một lượt duyệt mà không có bất kỳ đổi chỗ nào, mảng đã được sắp xếp. Dừng luôn, không cần duyệt tiếp.

---

### Selection Sort — từng bước

Mảng ban đầu: `[5, 3, 8, 1, 2]`

```
Lượt 1: Tìm nhỏ nhất trong [5, 3, 8, 1, 2] -> 1 (vị trí 3)
        Đổi chỗ vị trí 0 và 3
        [1, 3, 8, 5, 2]
         ^              phần đã sắp xếp

Lượt 2: Tìm nhỏ nhất trong [3, 8, 5, 2] -> 2 (vị trí 4)
        Đổi chỗ vị trí 1 và 4
        [1, 2, 8, 5, 3]
         ^--^           phần đã sắp xếp

Lượt 3: Tìm nhỏ nhất trong [8, 5, 3] -> 3 (vị trí 4)
        Đổi chỗ vị trí 2 và 4
        [1, 2, 3, 5, 8]
         ^-----^        phần đã sắp xếp

Lượt 4: Tìm nhỏ nhất trong [5, 8] -> 5 (đã đúng chỗ)
        [1, 2, 3, 5, 8]
         ^--------^     phần đã sắp xếp

Kết quả: [1, 2, 3, 5, 8]
```

---

### Insertion Sort — từng bước

Mảng ban đầu: `[5, 3, 8, 1, 2]`

Tưởng tượng vạch `|` ngăn giữa phần đã sắp xếp (trái) và chưa sắp xếp (phải).

```
[5 | 3, 8, 1, 2]   Lấy 3, chèn vào phần trái
  3 < 5 ? Có! Dịch 5 sang phải, đặt 3 trước
[3, 5 | 8, 1, 2]   Lấy 8, chèn vào phần trái
  8 > 5 ? Đúng rồi! Giữ nguyên
[3, 5, 8 | 1, 2]   Lấy 1, chèn vào phần trái
  1 < 8 ? Dịch 8   -> [3, 5, _, 8]
  1 < 5 ? Dịch 5   -> [3, _, 5, 8]
  1 < 3 ? Dịch 3   -> [_, 3, 5, 8]
  Đặt 1 vào đầu    -> [1, 3, 5, 8]
[1, 3, 5, 8 | 2]   Lấy 2, chèn vào phần trái
  2 < 8 ? Dịch 8   -> [1, 3, 5, _, 8]
  2 < 5 ? Dịch 5   -> [1, 3, _, 5, 8]
  2 < 3 ? Dịch 3   -> [1, _, 3, 5, 8]
  2 > 1 ? Đúng!    -> [1, 2, 3, 5, 8]

Kết quả: [1, 2, 3, 5, 8]
```

---

## Stable vs Unstable Sort — tại sao quan trọng?

Trước khi xem code, cần hiểu một khái niệm quan trọng: **stability** (tính ổn định).

Tưởng tượng bạn có danh sách sinh viên, đã sắp xếp theo **tên** (A-Z). Giờ bạn muốn sắp xếp lại theo **điểm**:

```
Ban đầu (đã sort theo tên):
  An     - 8 điểm
  Bình   - 9 điểm
  Chi    - 8 điểm
  Dũng   - 9 điểm
```

**Stable sort** (sắp xếp ổn định): giữ nguyên thứ tự tương đối của các phần tử **bằng nhau**. An và Chi cùng 8 điểm — An vẫn đứng trước Chi vì An đứng trước trong danh sách gốc:

```
Stable sort theo điểm:
  An     - 8 điểm   ← An trước Chi (giữ thứ tự cũ)
  Chi    - 8 điểm
  Bình   - 9 điểm   ← Bình trước Dũng (giữ thứ tự cũ)
  Dũng   - 9 điểm
```

**Unstable sort**: không đảm bảo thứ tự cũ. Có thể Chi đứng trước An:

```
Unstable sort theo điểm:
  Chi    - 8 điểm   ← Chi trước An?! Thứ tự cũ bị phá
  An     - 8 điểm
  Dũng   - 9 điểm
  Bình   - 9 điểm
```

Vậy thuật toán nào stable, thuật toán nào không?

| Thuật toán | Stable? | Tại sao? |
|-----------|---------|----------|
| **Bubble Sort** | Stable | Chỉ đổi chỗ khi `>` (strict), phần tử bằng nhau không bị đổi |
| **Selection Sort** | **Unstable** | Đổi chỗ xa nhau, có thể phá thứ tự. Ví dụ: `[3a, 2, 3b]` → tìm min=2, swap với 3a → `[2, 3a, 3b]`? Không! Thực tế: `[2, 3a?, 3b?]` — phụ thuộc implementation |
| **Insertion Sort** | Stable | Dịch phần tử sang phải, chèn vào đúng chỗ. Phần tử bằng nhau giữ thứ tự cũ |

> **Tại sao cần quan tâm?** Khi bạn sort 2 lần (theo tên rồi theo điểm), stable sort giữ kết quả lần sort đầu. Unstable sort phá nó. Trong thực tế, database sorting, UI table sorting đều cần stable sort.

---

## Adaptive Sorting — ai thông minh hơn?

Một tính chất ít ai nói nhưng rất quan trọng: **adaptive** — thuật toán có nhanh hơn khi dữ liệu đã gần sorted không?

```
Mảng gần sorted: [1, 2, 4, 3, 5, 6, 7, 8, 9, 10]
                           ^--^ chỉ 2 phần tử sai chỗ
```

- **Insertion Sort**: ADAPTIVE. Mỗi phần tử chỉ cần dịch 1-2 bước → gần O(n). Đây là **bí mật** tại sao nó được dùng trong Timsort và pdqsort.
- **Bubble Sort**: Phiên bản có `swapped` flag thì dừng sớm được → có chút adaptive, nhưng vẫn chậm hơn Insertion Sort nhiều.
- **Selection Sort**: KHÔNG adaptive. Luôn quét hết mảng tìm min, dù mảng đã sorted hay chưa → luôn O(n²).

```
          Mảng đã sorted    Mảng ngẫu nhiên    Mảng ngược
          (best case)       (avg case)          (worst case)
          ─────────────     ──────────────      ──────────────
Insertion   O(n) ⚡           O(n²)              O(n²)
Bubble      O(n) ⚡           O(n²)              O(n²)
Selection   O(n²)  💀        O(n²)              O(n²)
```

---

## Code Rust

```rust
/// Bubble Sort — đổi chỗ cặp kề nhau, phần tử lớn "nổi" lên cuối.
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n.saturating_sub(i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // Nếu không có đổi chỗ nào -> mảng đã sắp xếp, dừng sớm
        if !swapped {
            break;
        }
    }
}

/// Selection Sort — tìm phần tử nhỏ nhất, đặt vào đầu phần chưa sắp xếp.
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

/// Insertion Sort — chèn phần tử vào đúng vị trí trong phần đã sắp xếp.
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
```

**Ghi chú về Rust:**

- `T: Ord` cho phép sắp xếp bất kỳ kiểu dữ liệu nào có thể so sánh được (số nguyên, chuỗi, ...).
- `arr.swap(i, j)` đổi chỗ 2 phần tử mà không gặp vấn đề với borrow checker. Nếu bạn viết `arr[i] = arr[j]` sẽ bị lỗi vì Rust không cho mượn mutable 2 lần.
- `saturating_sub` tránh lỗi tràn số khi trừ số không dấu (unsigned). Ví dụ: `0usize - 1` sẽ panic, nhưng `0usize.saturating_sub(1)` trả về `0`.

> **Tại sao Insertion Sort dùng `>` chứ không phải `>=`?** Vì dùng `>` (strict greater) thì phần tử bằng nhau KHÔNG bị đổi chỗ → giữ thứ tự cũ → **stable**. Nếu đổi thành `>=` thì mất stability.

---

## Độ phức tạp

### Bảng so sánh chi tiết

| | Bubble Sort | Selection Sort | Insertion Sort |
|---|---|---|---|
| **Best** | O(n) | O(n²) | O(n) |
| **Average** | O(n²) | O(n²) | O(n²) |
| **Worst** | O(n²) | O(n²) | O(n²) |
| **Bộ nhớ** | O(1) | O(1) | O(1) |
| **Stable?** | Có | Không | Có |
| **In-place?** | Có | Có | Có |
| **Adaptive?** | Có (swapped flag) | Không | Có |
| **So sánh** | O(n²) | O(n²) | O(n²) |
| **Swaps** | O(n²) | O(n) | O(n²) |

**Giải thích thực tế:**

- **O(n²)** nghĩa là: nếu mảng có 1.000 phần tử, cần khoảng 1.000.000 phép tính. Với 10.000 phần tử -> 100.000.000 phép tính. Quá chậm cho dữ liệu lớn!
- **O(n)** ở trường hợp tốt nhất: khi mảng gần như đã sắp xếp, Bubble Sort và Insertion Sort chỉ cần duyệt qua một lần.
- Selection Sort có **ít swap nhất** — O(n) swaps. Hữu ích khi swap tốn kém (ví dụ: di chuyển file lớn trên disk).

### Khi nào dùng cái nào?

| Tình huống | Chọn | Tại sao |
|-----------|------|---------|
| Mảng nhỏ (< 20-50 phần tử) | **Insertion Sort** | Overhead thấp, hằng số nhỏ, nhanh hơn cả Quick Sort cho mảng nhỏ |
| Mảng gần sorted | **Insertion Sort** | O(n) — nhanh gần như linear |
| Cần ít swap nhất | **Selection Sort** | Chỉ O(n) swaps |
| Cần stable + đơn giản | **Insertion Sort** | Stable + adaptive + nhanh trên mảng nhỏ |
| Mảng lớn (> 1000) | **Không dùng cả 3** | Dùng Merge Sort hoặc Quick Sort (chương sau) |
| Giảng dạy / demo | **Bubble Sort** | Dễ hiểu nhất, visual nhất |

---

## Insertion Sort — ngôi sao ẩn giấu

Insertion Sort xứng đáng được nói thêm vì nó **thực sự được dùng trong production**.

### Bí mật: O(n) trên mảng gần sorted

Khi mảng gần sorted, mỗi phần tử chỉ cần dịch 1-2 bước. Tổng số swap ít, nên chạy gần O(n):

```
Mảng gần sorted: [1, 2, 3, 5, 4, 6, 7, 8]
                              ^--^ chỉ cần swap 1 lần

Insertion Sort chỉ cần 1 pass + 1 swap → gần O(n)!
```

### Hybrid sorting algorithms dùng Insertion Sort

Các thuật toán sort "xịn" nhất hiện nay đều dùng Insertion Sort cho mảng nhỏ:

```
Timsort (Python, Java, Rust sort()):
  1. Chia mảng thành các "run" nhỏ (32-64 phần tử)
  2. Sort mỗi run bằng... INSERTION SORT! ← đây nè
  3. Merge các run lại (giống Merge Sort)

pdqsort (C++, Rust sort_unstable()):
  1. Quick Sort cho mảng lớn
  2. Khi partition nhỏ (< ~24 phần tử) → chuyển sang INSERTION SORT
  3. Khi detect mảng gần sorted → dùng Insertion Sort luôn
```

Tại sao không dùng Quick Sort/Merge Sort cho mảng nhỏ luôn? Vì overhead của recursion (tạo stack frame — nhớ chương Recursion không?) + cache miss khiến chúng chậm hơn Insertion Sort đơn giản trên mảng nhỏ.

---

## Pitfalls — sai lầm hay gặp

### Pitfall 1: Nghĩ O(n²) "vô dụng, không cần học"

❌ **Sai:** "O(n²) chậm vậy thì skip, học thẳng Quick Sort đi."

✅ **Đúng:** O(n²) sort là foundation. Insertion Sort được dùng bên trong Timsort và pdqsort — hai thuật toán sort phổ biến nhất thế giới.

💡 **Tại sao:** Hiểu O(n²) sort giúp bạn: (1) appreciate tại sao O(n log n) sort hay hơn, (2) biết khi nào O(n²) lại nhanh hơn (mảng nhỏ, mảng gần sorted), (3) trả lời phỏng vấn khi được hỏi "tại sao không dùng Insertion Sort cho mảng lớn?"

### Pitfall 2: Nhầm stable vs unstable

❌ **Sai:** "Stable/unstable chỉ là lý thuyết, code thế nào chẳng được."

✅ **Đúng:** Stable sort quan trọng khi bạn sort nhiều lần theo các tiêu chí khác nhau (sort theo tên rồi sort theo điểm).

💡 **Tại sao:** Trong Rust, `sort()` là stable (Timsort) và `sort_unstable()` là unstable (pdqsort). Nếu bạn dùng sai, kết quả sort có thể khác mong đợi. Xem phần Rust Ecosystem bên dưới.

### Pitfall 3: Nghĩ Bubble Sort và Insertion Sort "giống nhau vì đều O(n²)"

❌ **Sai:** "Cả 2 đều O(n²), chọn cái nào cũng vậy."

✅ **Đúng:** Insertion Sort nhanh hơn Bubble Sort trên hầu hết mọi input thực tế.

💡 **Tại sao:** Bubble Sort mỗi lần swap chỉ dịch phần tử 1 bước. Insertion Sort dịch phần tử đến đúng vị trí luôn. Số phép so sánh giống nhau, nhưng Insertion Sort ít swap hơn nhiều. Thêm nữa, Insertion Sort adaptive còn Bubble Sort thì chậm chạp.

---

## Rust Ecosystem — sort trong thực tế

Rust standard library cung cấp 2 method sort:

```rust
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

// sort() — Timsort, stable, O(n log n)
// Giữ thứ tự tương đối của phần tử bằng nhau
v.sort();

// sort_unstable() — pdqsort, unstable, O(n log n)
// Nhanh hơn sort() ~10-30% vì không cần allocate thêm memory
// Nhưng không đảm bảo thứ tự phần tử bằng nhau
v.sort_unstable();
```

### Khi nào dùng `sort()` vs `sort_unstable()`?

```
sort()           sort_unstable()
──────────────   ──────────────────
Timsort          pdqsort
Stable           Unstable
O(n) extra mem   O(1) extra mem (in-place)
Chậm hơn ~10%   Nhanh hơn ~10%

Dùng khi:        Dùng khi:
- Cần stable     - Không quan tâm thứ tự phần tử bằng nhau
- Sort struct    - Sort số (i32, f64...)
  theo nhiều     - Performance-critical code
  tiêu chí
```

**Cả hai đều dùng Insertion Sort cho mảng nhỏ bên trong.** Kiến thức bạn học ở chương này không phí đâu.

### sort_by và sort_by_key

```rust
#[derive(Debug)]
struct Student {
    name: String,
    score: u32,
}

let mut students = vec![
    Student { name: "An".into(), score: 8 },
    Student { name: "Bình".into(), score: 9 },
    Student { name: "Chi".into(), score: 8 },
];

// sort_by — stable, giữ thứ tự An trước Chi (cùng 8 điểm)
students.sort_by(|a, b| a.score.cmp(&b.score));

// sort_by_key — stable, ngắn gọn hơn
students.sort_by_key(|s| s.score);

// sort_unstable_by — unstable, nhanh hơn
students.sort_unstable_by(|a, b| a.score.cmp(&b.score));
```

### KaCrab integration

Trong crate `rust_ds2a`, bạn đã có sẵn 3 hàm sort cơ bản để thực hành:

```rust
use rust_ds2a::sorting::{bubble_sort, selection_sort, insertion_sort};

let mut v = vec![5, 3, 8, 1, 2];
insertion_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 5, 8]);
```

So sánh với standard library:

```rust
// Bạn tự viết — để học
insertion_sort(&mut v);

// Production code — dùng standard library
v.sort();           // stable (Timsort, bên trong có Insertion Sort)
v.sort_unstable();  // unstable nhưng nhanh hơn (pdqsort)
```

---

## Ví dụ

```rust
use rust_ds2a::sorting::{bubble_sort, selection_sort, insertion_sort};

// Bubble Sort với số nguyên
let mut v = vec![5, 3, 8, 1, 2];
bubble_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 5, 8]);

// Selection Sort với chuỗi
let mut v = vec!["cherry", "apple", "banana"];
selection_sort(&mut v);
assert_eq!(v, vec!["apple", "banana", "cherry"]);

// Insertion Sort
let mut v = vec![4, 1, 3, 2];
insertion_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 4]);
```

---

## Practice — luyện tập

### LeetCode 75. Sort Colors

> Cho mảng chỉ chứa 0, 1, 2. Sắp xếp in-place.

Bài này có thể giải bằng nhiều cách — thử dùng Insertion Sort hoặc Selection Sort trước, rồi tối ưu bằng counting sort hoặc Dutch National Flag algorithm (3-way partition — sẽ gặp lại ở Quick Sort).

```rust
// Cách 1: Dùng Insertion Sort — đúng nhưng O(n²)
fn sort_colors_insertion(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j - 1] > nums[j] {
            nums.swap(j - 1, j);
            j -= 1;
        }
    }
}

// Cách 2: Counting sort — O(n), tận dụng chỉ có 3 giá trị
fn sort_colors(nums: &mut Vec<i32>) {
    let (mut c0, mut c1, mut c2) = (0, 0, 0);
    for &x in nums.iter() {
        match x {
            0 => c0 += 1,
            1 => c1 += 1,
            _ => c2 += 1,
        }
    }
    let mut i = 0;
    for _ in 0..c0 { nums[i] = 0; i += 1; }
    for _ in 0..c1 { nums[i] = 1; i += 1; }
    for _ in 0..c2 { nums[i] = 2; i += 1; }
}
```

### LeetCode 147. Insertion Sort List

> Sắp xếp linked list bằng Insertion Sort.

Bài này giúp bạn hiểu Insertion Sort trên linked list (khác array). Trên array, bạn dịch phần tử sang phải. Trên linked list, bạn thay đổi pointer. Nhớ lại chương Singly Linked List — thao tác insert vào giữa list cần giữ tham chiếu đến node trước đó.

---

## Tổng kết

```
Ba thuật toán O(n²) — chậm nhưng quan trọng:

  Bubble Sort    → dễ hiểu, dễ dạy, ít khi dùng thực tế
  Selection Sort → ít swap, unstable, không adaptive
  Insertion Sort → NGÔI SAO: stable, adaptive, dùng trong Timsort/pdqsort

Key takeaways:
  ✅ Stable sort giữ thứ tự phần tử bằng nhau
  ✅ Adaptive sort nhanh hơn trên mảng gần sorted
  ✅ Insertion Sort = O(n) trên mảng gần sorted
  ✅ Rust: sort() = stable, sort_unstable() = nhanh hơn
  ✅ Cả 3 đều in-place (O(1) memory)
```

---

## Chương tiếp theo: Merge Sort

Ba thuật toán này đều O(n²) — quá chậm cho mảng lớn. Có cách nào nhanh hơn không?

Có! **Merge Sort** đạt O(n log n) bằng cách áp dụng **Divide and Conquer** (chia để trị) — chia mảng thành 2 nửa, sort từng nửa (bằng recursion — nhớ chương trước không?), rồi gộp lại. Thú vị hơn: Merge Sort cũng stable, và là nền tảng của Timsort mà bạn vừa nghe nói ở chương này.

Câu hỏi để suy nghĩ trước: *"Nếu chia mảng thành 2 nửa đã sorted, gộp chúng lại mất bao lâu?"*

---

---

[← Recursion](./01-recursion.md) | [Merge Sort →](./03-merge-sort.md)
