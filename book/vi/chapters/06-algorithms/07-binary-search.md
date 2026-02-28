# Binary Search

> 💡 **Đừng lo lắng:** Binary Search nghe thì đơn giản — "chia đôi rồi tìm". Nhưng khi code, bạn sẽ gặp `lo <= hi` hay `lo < hi`? Update `lo = mid` hay `lo = mid + 1`? Off-by-one error khắp nơi. Đây là chương mà **ai cũng** từng bug. Bạn không ngu — Binary Search thật sự tricky. Đọc chậm, trace từng bước, rồi sẽ quen.

---

## Đây là gì?

Bạn chơi trò đoán số: "Tôi đang nghĩ một số từ 1 đến 100. Bạn đoán đi!" Mỗi lần bạn đoán, tôi nói "lớn hơn" hoặc "nhỏ hơn". Cách thông minh nhất? **Luôn đoán số ở giữa!**

- Đoán 50. "Lớn hơn." -> Số nằm trong 51-100.
- Đoán 75. "Nhỏ hơn." -> Số nằm trong 51-74.
- Đoán 62. "Lớn hơn." -> Số nằm trong 63-74.
- ...

Mỗi lần đoán, bạn **loại một nửa** số ứng cử viên. Từ 100 số, chỉ cần khoảng **7 lần đoán** là tìm ra!

Đây chính là **Binary Search** (tìm kiếm nhị phân). Điều kiện: mảng phải **đã được sắp xếp**.

> Nếu bạn đã đọc chương [Radix Sort](../06-algorithms/06-radix-sort.md), bạn biết rằng sorting là O(n log n). Binary Search chạy O(log n) trên mảng đã sort — nghĩa là sort một lần, search bao nhiêu lần cũng nhanh.

---

## Hoạt động như thế nào?

### Tìm kiếm chính xác

Tìm `target = 7` trong `[1, 3, 5, 7, 9, 11, 13]`:

```
Bước 1:  [1, 3, 5, 7, 9, 11, 13]
          lo          ^          hi
          mid = 3, arr[3] = 7 == target  -->  Tìm thấy tại vị trí 3!
```

Tìm `target = 6` trong `[1, 3, 5, 7, 9, 11, 13]`:

```
Bước 1:  [1, 3, 5, 7, 9, 11, 13]
          lo       ^             hi
          mid = 3, arr[3] = 7 > 6  -->  Tìm bên trái

Bước 2:  [1, 3, 5]
          lo  ^  hi
          mid = 1, arr[1] = 3 < 6  -->  Tìm bên phải

Bước 3:  [5]
          lo=hi=2
          mid = 2, arr[2] = 5 < 6  -->  Tìm bên phải

Bước 4:  lo > hi  -->  Không tìm thấy!
```

### Minh họa: loại nửa mỗi bước

```
100 phần tử
  |-- Bước 1 --> 50 phần tử
  |-- Bước 2 --> 25 phần tử
  |-- Bước 3 --> 12 phần tử
  |-- Bước 4 --> 6 phần tử
  |-- Bước 5 --> 3 phần tử
  |-- Bước 6 --> 1 phần tử
  |-- Bước 7 --> Tìm thấy hoặc không có!

1.000.000 phần tử? Chỉ cần ~20 bước!
```

---

## Hai template Binary Search: `lo < hi` vs `lo <= hi`

Đây là nguồn gốc 90% bug Binary Search. Hãy hiểu rõ:

### Template 1: `while lo < hi` (nửa mở `[lo, hi)`)

```
lo = 0, hi = n       // hi = arr.len(), KHÔNG phải arr.len() - 1
while lo < hi:
    mid = lo + (hi - lo) / 2
    if điều_kiện(mid):
        hi = mid      // mid có thể là đáp án, giữ lại
    else:
        lo = mid + 1  // mid chắc chắn không phải, bỏ
return lo             // lo == hi, đó là đáp án
```

**Khi nào dùng?** Khi tìm **vị trí đầu tiên** thỏa điều kiện (lower bound, upper bound, `partition_point`). Luôn kết thúc với `lo == hi`.

### Template 2: `while lo <= hi` (đóng `[lo, hi]`)

```
lo = 0, hi = n - 1   // hi = arr.len() - 1
while lo <= hi:
    mid = lo + (hi - lo) / 2
    if arr[mid] == target:
        return mid        // Tìm thấy!
    elif arr[mid] < target:
        lo = mid + 1
    else:
        hi = mid - 1      // mid - 1, KHÔNG phải mid
return -1                 // Không tìm thấy
```

**Khi nào dùng?** Khi tìm **chính xác** một giá trị. Return ngay khi tìm thấy.

### So sánh nhanh

| | `while lo < hi` | `while lo <= hi` |
|--|--|--|
| Khoảng | `[lo, hi)` nửa mở | `[lo, hi]` đóng |
| Init `hi` | `arr.len()` | `arr.len() - 1` |
| Update `hi` | `hi = mid` | `hi = mid - 1` |
| Kết thúc | `lo == hi` | `lo > hi` |
| Dùng cho | Lower/upper bound, tìm vị trí | Tìm chính xác |

> **Lời khuyên:** Nếu bạn mới, hãy **chọn một template và dùng cho mọi bài**. Template 1 (`while lo < hi`) linh hoạt hơn — bạn có thể giải hầu hết bài chỉ với nó.

---

## Lower Bound — tìm vị trí đầu tiên >= target

Không chỉ tìm chính xác, đôi khi bạn cần: "Vị trí đầu tiên mà phần tử >= target là bao nhiêu?"

```
arr = [1, 2, 4, 4, 6], target = 4

Bước 1: lo=0, hi=5
         mid=2: arr[2]=4 >= 4  -> có thể là đáp án, hi=2

Bước 2: lo=0, hi=2
         mid=1: arr[1]=2 < 4   -> chắc chắn không, lo=2

Kết thúc: lo=2, hi=2  -> trả về 2

=> Vị trí đầu tiên >= 4 là index 2  ✓
```

**Ẩn dụ:** Tưởng tượng bạn đang tìm chỗ ngồi trong rạp phim. Bạn muốn ngồi hàng đầu tiên mà số ghế >= số trên vé. Bạn chia đôi rạp, kiểm tra hàng giữa, rồi thu hẹp dần.

### Upper Bound — tìm vị trí đầu tiên > target

Tìm phần tử đầu tiên **lớn hơn** target:

```
arr = [1, 2, 4, 4, 6], target = 4

Bước 1: lo=0, hi=5
         mid=2: arr[2]=4 <= 4  -> không phải (cần > 4), lo=3

Bước 2: lo=3, hi=5
         mid=4: arr[4]=6 > 4   -> có thể là đáp án, hi=4

Bước 3: lo=3, hi=4
         mid=3: arr[3]=4 <= 4  -> lo=4

Kết thúc: lo=4, hi=4  -> trả về 4

=> Phần tử đầu tiên > 4 là 6 tại index 4  ✓
```

### Ứng dụng: đếm số lần xuất hiện

```
Số lần xuất hiện của 4 = upper_bound(4) - lower_bound(4) = 4 - 2 = 2

arr = [1, 2, 4, 4, 6]
            ^--^
            2 phần tử có giá trị 4
```

---

## Binary Search on Answer — kỹ thuật phỏng vấn cực phổ biến

Binary Search không chỉ tìm phần tử trong mảng. Bạn có thể **Binary Search trên không gian đáp án** (answer space).

### Ẩn dụ: tìm tốc độ vừa đủ

Bạn cần giao hàng trong 8 tiếng. Xe chạy nhanh thì tốn xăng. Bạn muốn tìm **tốc độ chậm nhất** mà vẫn giao kịp.

- Tốc độ 10 km/h? Quá chậm, không kịp.
- Tốc độ 100 km/h? Kịp, nhưng chắc chưa cần nhanh vậy.
- Tốc độ 55 km/h? Kịp. Thử chậm hơn.
- Tốc độ 30 km/h? Không kịp. Thử nhanh hơn.
- ...

Bạn đang **Binary Search trên đáp án** (tốc độ), không phải trên mảng!

### Pattern chung

```
lo = giá_trị_nhỏ_nhất_có_thể
hi = giá_trị_lớn_nhất_có_thể

while lo < hi:
    mid = lo + (hi - lo) / 2
    if kiểm_tra(mid):     // mid có khả thi không?
        hi = mid          // Thử giảm xuống (tìm min)
    else:
        lo = mid + 1

return lo  // Giá trị nhỏ nhất thỏa mãn
```

### Ví dụ: Koko Eating Bananas (LeetCode #875)

Koko có `n` đống chuối. Cô ấy ăn với tốc độ `k` quả/giờ. Có `h` giờ. Tìm tốc độ `k` **nhỏ nhất** để ăn hết.

```
piles = [3, 6, 7, 11], h = 8

Thử k=1:  ceil(3/1) + ceil(6/1) + ceil(7/1) + ceil(11/1) = 3+6+7+11 = 27 giờ > 8  ❌
Thử k=11: ceil(3/11)+ ceil(6/11)+ ceil(7/11)+ ceil(11/11)= 1+1+1+1  = 4 giờ  <= 8 ✓
Thử k=6:  ceil(3/6) + ceil(6/6) + ceil(7/6) + ceil(11/6) = 1+1+2+2  = 6 giờ  <= 8 ✓
Thử k=3:  ceil(3/3) + ceil(6/3) + ceil(7/3) + ceil(11/3) = 1+2+3+4  = 10 giờ > 8  ❌
Thử k=4:  ceil(3/4) + ceil(6/4) + ceil(7/4) + ceil(11/4) = 1+2+2+3  = 8 giờ  <= 8 ✓
Thử k=3 đã fail → đáp án là k=4
```

```rust
fn min_eating_speed(piles: &[u64], h: u64) -> u64 {
    let mut lo: u64 = 1;
    let mut hi: u64 = *piles.iter().max().unwrap();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        // Tổng số giờ cần nếu ăn tốc độ mid
        let hours: u64 = piles.iter()
            .map(|&p| (p + mid - 1) / mid)  // ceil division
            .sum();
        if hours <= h {
            hi = mid;       // Kịp! Thử chậm hơn
        } else {
            lo = mid + 1;   // Không kịp, phải nhanh hơn
        }
    }
    lo
}
```

> **Nhận dạng BS on Answer:** Khi đề bài hỏi "tìm giá trị nhỏ nhất/lớn nhất mà thỏa điều kiện X", và bạn có thể **kiểm tra** một giá trị nhanh chóng — nghĩ đến Binary Search on Answer.

---

## Rotated Sorted Array (LeetCode #33)

Một bài variant cổ điển: mảng sorted bị xoay.

### Ẩn dụ

Tưởng tượng bạn có một dãy số đã sắp xếp viết trên một vòng tròn giấy. Ai đó cắt vòng tròn tại một điểm rồi dán lại thành đường thẳng. Kết quả: mảng sorted bị "xoay".

```
Trước: [1, 2, 3, 4, 5, 6, 7]
                    ↓ xoay tại vị trí 3
Sau:   [4, 5, 6, 7, 1, 2, 3]
        ─────────── ───────
        nửa sorted  nửa sorted
```

**Quan sát key:** Luôn có ít nhất **một nửa** là sorted. Kiểm tra nửa nào sorted, rồi quyết định tìm bên nào.

### Trace chi tiết

```
arr = [4, 5, 6, 7, 0, 1, 2], target = 0

Bước 1: lo=0, hi=6, mid=3
  arr[mid]=7, arr[lo]=4
  Nửa trái [4,5,6,7] sorted (arr[lo] <= arr[mid])
  target=0 không nằm trong [4..7] → tìm bên phải
  lo = 4

Bước 2: lo=4, hi=6, mid=5
  arr[mid]=1, arr[lo]=0
  Nửa trái [0,1] sorted (arr[lo] <= arr[mid])
  target=0 nằm trong [0..1] → tìm bên trái
  hi = 5

Bước 3: lo=4, hi=5, mid=4
  arr[mid]=0 == target → Tìm thấy tại index 4!
```

### Code

```rust
fn search_rotated(arr: &[i32], target: i32) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = arr.len().wrapping_sub(1); // tránh underflow khi arr rỗng
    if arr.is_empty() { return None; }

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if arr[mid] == target {
            return Some(mid);
        }

        // Nửa trái sorted?
        if arr[lo] <= arr[mid] {
            if arr[lo] <= target && target < arr[mid] {
                // Target nằm trong nửa trái sorted
                hi = mid.wrapping_sub(1);
            } else {
                lo = mid + 1;
            }
        }
        // Nửa phải sorted
        else {
            if arr[mid] < target && target <= arr[hi] {
                // Target nằm trong nửa phải sorted
                lo = mid + 1;
            } else {
                hi = mid.wrapping_sub(1);
            }
        }

        // Tránh infinite loop khi wrapping
        if hi == usize::MAX { break; }
    }
    None
}
```

---

## Code Rust

```rust
/// Tìm kiếm nhị phân — trả về vị trí của target, hoặc None.
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;  // Tránh tràn số!
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Lower bound — vị trí đầu tiên mà arr[i] >= target.
pub fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] < *target {
            lo = mid + 1;  // Phần tử này nhỏ quá, bỏ qua
        } else {
            hi = mid;      // Có thể là kết quả, thử thu nhỏ vùng tìm kiếm
        }
    }
    lo
}

/// Upper bound — vị trí đầu tiên mà arr[i] > target.
pub fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] <= *target {
            lo = mid + 1;  // Phần tử này <= target, bỏ qua
        } else {
            hi = mid;      // Có thể là kết quả
        }
    }
    lo
}
```

**Ghi chú về Rust:**

- `lo + (hi - lo) / 2` thay vì `(lo + hi) / 2` để tránh tràn số. Trong C/C++, `(lo + hi)` có thể vượt quá giới hạn integer. Rust sẽ panic khi overflow, nhưng vẫn nên viết an toàn.
- Khoảng nửa mở `[lo, hi)` là cách viết idiomatic trong Rust — giúp tránh lỗi off-by-one.
- `cmp` và `match` là cách viết Rust đẹp và an toàn — xử lý cả 3 trường hợp (bằng, nhỏ hơn, lớn hơn) một cách rõ ràng.

---

## Pitfalls — Những cái bẫy hay gặp

### 1. Integer overflow khi tính mid

❌ Sai:
```rust
let mid = (lo + hi) / 2;  // lo + hi có thể tràn!
```

✅ Đúng:
```rust
let mid = lo + (hi - lo) / 2;
```

💡 Tại sao: Với C/C++, nếu `lo = 2_000_000_000` và `hi = 2_000_000_000`, thì `lo + hi = 4 tỷ` — vượt quá `i32::MAX`. Rust sẽ panic ở debug mode, nhưng viết đúng từ đầu luôn tốt hơn.

### 2. Infinite loop khi update sai

❌ Sai:
```rust
// Template [lo, hi) nhưng update lo = mid (thiếu +1)
while lo < hi {
    let mid = lo + (hi - lo) / 2;
    if condition { hi = mid; }
    else { lo = mid; }   // BUG! Khi hi = lo + 1, mid = lo → lo không đổi → vòng lặp vô hạn
}
```

✅ Đúng:
```rust
else { lo = mid + 1; }   // Luôn tiến lên ít nhất 1
```

💡 Tại sao: Khi `lo = 5, hi = 6` thì `mid = 5`. Nếu `lo = mid = 5`, không gì thay đổi. Vòng lặp chạy mãi. `lo = mid + 1 = 6`, lúc này `lo == hi`, vòng lặp kết thúc.

### 3. Nhầm template — `lo < hi` vs `lo <= hi`

❌ Sai:
```rust
// Dùng while lo <= hi nhưng update hi = mid (không -1)
while lo <= hi {
    let mid = lo + (hi - lo) / 2;
    if arr[mid] > target { hi = mid; }  // BUG! Infinite loop
    // ...
}
```

✅ Đúng:
```rust
// while lo <= hi đi với hi = mid - 1
while lo <= hi {
    let mid = lo + (hi - lo) / 2;
    if arr[mid] > target { hi = mid - 1; }
    // ...
}

// HOẶC: while lo < hi đi với hi = mid
while lo < hi {
    let mid = lo + (hi - lo) / 2;
    if arr[mid] > target { hi = mid; }
    // ...
}
```

💡 Tại sao: Mỗi template có "bộ đôi" update riêng. Trộn lẫn = bug.

### 4. Quên check mảng rỗng

❌ Sai:
```rust
let hi = arr.len() - 1;  // Panic nếu arr rỗng! (0 - 1 = underflow)
```

✅ Đúng:
```rust
if arr.is_empty() { return None; }
let hi = arr.len() - 1;

// Hoặc dùng template [lo, hi) để tránh hoàn toàn:
let hi = arr.len();  // Khi arr rỗng: lo=0, hi=0, vòng lặp không chạy → OK
```

💡 Tại sao: `usize` (unsigned) trong Rust không có số âm. `0 - 1` sẽ panic ở debug mode hoặc wrap thành `usize::MAX` ở release mode. Cả hai đều sai.

---

## Khi nào dùng Binary Search?

| Tình huống | Dùng gì | Ví dụ |
|-----------|---------|-------|
| Tìm giá trị trong mảng sorted | BS cơ bản | LeetCode #704 |
| Tìm vị trí chèn | Lower bound | `partition_point` |
| Đếm số lần xuất hiện | Lower + Upper bound | `upper - lower` |
| Mảng sorted bị xoay | Rotated BS | LeetCode #33 |
| Tìm min/max thỏa điều kiện | BS on answer | LeetCode #875 |
| Tìm peak element | BS trên xu hướng tăng/giảm | LeetCode #162 |
| Ma trận sorted | BS trên ma trận phẳng | LeetCode #74 |

**Dấu hiệu nhận biết:**
- Đề bài có "sorted" hoặc bạn có thể tạo sorted order
- Đề bài hỏi "minimum/maximum value that satisfies..."
- Brute force là O(n), bạn cần O(log n)
- Có thể verify đáp án nhanh (O(n) hoặc O(1)) nhưng tìm đáp án chậm

---

## Độ phức tạp

| Phép toán | Thời gian | Bộ nhớ |
|----------|----------|--------|
| binary_search | O(log n) | O(1) |
| lower_bound | O(log n) | O(1) |
| upper_bound | O(log n) | O(1) |
| BS on answer | O(log(range) * cost_check) | O(1) |

**Giải thích thực tế:**

- **O(log n)** cực kỳ nhanh. 1 tỷ phần tử? Chỉ cần ~30 bước.
- **O(1) bộ nhớ**: chỉ cần vài biến, không cần mảng phụ.
- **Điều kiện**: mảng PHẢI đã sắp xếp. Nếu chưa sắp xếp, kết quả sẽ sai. Chi phí sắp xếp là O(n log n), nhưng chỉ cần làm một lần.
- **BS on answer**: `range` là khoảng đáp án (`hi - lo`), `cost_check` là chi phí verify mỗi lần. Ví dụ Koko: O(log(max_pile) * n).

---

## Ví dụ

```rust
use rust_ds2a::searching::{binary_search, lower_bound, upper_bound};

// Tìm kiếm chính xác
let arr = [1, 3, 5, 7, 9, 11, 13];
assert_eq!(binary_search(&arr, &7), Some(3));
assert_eq!(binary_search(&arr, &6), None);    // Không tìm thấy

// Lower bound và Upper bound
let arr = [1, 2, 4, 4, 6];
assert_eq!(lower_bound(&arr, &4), 2);  // Vị trí đầu tiên >= 4
assert_eq!(upper_bound(&arr, &4), 4);  // Vị trí đầu tiên > 4

// Đếm số lần xuất hiện của 4
let count = upper_bound(&arr, &4) - lower_bound(&arr, &4);
assert_eq!(count, 2);  // Có 2 số 4 trong mảng
```

---

## Rust Ecosystem — Binary Search trong thực tế

Rust standard library có sẵn Binary Search. Bạn không cần tự viết trong production code:

### `slice::binary_search()`

```rust
let arr = [1, 3, 5, 7, 9];

// Trả về Result<usize, usize>
// Ok(index) nếu tìm thấy, Err(index) là vị trí nên chèn
match arr.binary_search(&5) {
    Ok(idx)  => println!("Tìm thấy tại {idx}"),   // Ok(2)
    Err(idx) => println!("Không thấy, chèn tại {idx}"),
}

// Không tìm thấy 4: Err(2) — nên chèn tại index 2
assert_eq!(arr.binary_search(&4), Err(2));
```

> Lưu ý: khi có nhiều phần tử bằng nhau, `binary_search()` **không đảm bảo** trả về cái đầu tiên. Nếu cần vị trí đầu tiên, dùng `partition_point`.

### `slice::partition_point()`

Đây chính là lower bound / upper bound của Rust:

```rust
let arr = [1, 2, 4, 4, 6];

// Lower bound: vị trí đầu tiên >= 4
// partition_point tìm vị trí đầu tiên mà predicate trả về FALSE
let lower = arr.partition_point(|&x| x < 4);   // 2
assert_eq!(lower, 2);

// Upper bound: vị trí đầu tiên > 4
let upper = arr.partition_point(|&x| x <= 4);  // 4
assert_eq!(upper, 4);

// Đếm số 4
assert_eq!(upper - lower, 2);
```

> **Tên hơi lạ** — `partition_point` lấy từ ý tưởng: mảng được "chia" thành 2 phần, bên trái thỏa predicate, bên phải không. Hàm trả về điểm chia. Giống hệt `lower_bound` trong C++.

### `BTreeMap` và `BTreeSet`

Nhớ chương [Binary Search Tree](../03-trees-and-heaps/02-bst.md)? `BTreeMap` trong Rust dùng B-Tree — cũng dựa trên ý tưởng Binary Search nhưng trên cấu trúc cây. Hỗ trợ `range()` để tìm kiếm khoảng:

```rust
use std::collections::BTreeSet;

let set: BTreeSet<i32> = [1, 3, 5, 7, 9].into();

// Tìm tất cả phần tử trong khoảng [3, 7]
let in_range: Vec<_> = set.range(3..=7).collect();
assert_eq!(in_range, [&3, &5, &7]);
```

---

## Practice — Luyện tập

| Bài | Difficulty | Pattern | Gợi ý |
|-----|-----------|---------|-------|
| [#704 Binary Search](https://leetcode.com/problems/binary-search/) | Easy | BS cơ bản | Template 1 hoặc 2 đều được. Luyện tay. |
| [#35 Search Insert Position](https://leetcode.com/problems/search-insert-position/) | Easy | Lower bound | Chính xác là `lower_bound` / `partition_point` |
| [#33 Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/) | Medium | Rotated BS | Xác định nửa nào sorted, rồi kiểm tra target nằm bên nào |
| [#875 Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/) | Medium | BS on answer | BS trên tốc độ `k`. Check function: tính tổng giờ |
| [#153 Find Minimum in Rotated](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/) | Medium | Rotated BS | So sánh `arr[mid]` với `arr[hi]` |
| [#1011 Capacity To Ship Packages](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/) | Medium | BS on answer | Tương tự #875, BS trên capacity |

> **Thứ tự gợi ý:** #704 -> #35 -> #33 -> #875. Làm lần lượt, mỗi bài xây trên bài trước.

---

## Tiếp theo

Chương tiếp theo: **[Two Pointers](../06-algorithms/08-two-pointers.md)** — một kỹ thuật thường đi cùng Binary Search. Trong Two Pointers, hai chỉ số di chuyển trên mảng sorted để giải bài toán trong O(n). Thực tế, nhiều bài phỏng vấn kết hợp cả hai: Binary Search để thu hẹp không gian, Two Pointers để kiểm tra điều kiện.

> Binary Search cho bạn O(log n). Two Pointers cho bạn O(n) thay vì O(n^2). Cả hai đều dựa trên một ý tưởng đơn giản: **mảng sorted chứa nhiều thông tin hơn bạn nghĩ — hãy tận dụng nó.**

---

[← Radix Sort](./06-radix-sort.md) | [Two Pointers →](./08-two-pointers.md)
