# Binary Search

## Đây là gì?

Bạn chơi trò đoán số: "Tôi đang nghĩ một số từ 1 đến 100. Bạn đoán đi!" Mỗi lần bạn đoán, tôi nói "lớn hơn" hoặc "nhỏ hơn". Cách thông minh nhất? **Luôn đoán số ở giữa!**

- Đoán 50. "Lớn hơn." -> Số nằm trong 51-100.
- Đoán 75. "Nhỏ hơn." -> Số nằm trong 51-74.
- Đoán 62. "Lớn hơn." -> Số nằm trong 63-74.
- ...

Mỗi lần đoán, bạn **loại một nửa** số ứng cử viên. Từ 100 số, chỉ cần khoảng **7 lần đoán** là tìm ra!

Đây chính là **Binary Search** (tìm kiếm nhị phân). Điều kiện: mảng phải **đã được sắp xếp**.

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

### Lower Bound — tìm vị trí đầu tiên >= target

Không chỉ tìm chính xác, đôi khi bạn cần: "Vị trí đầu tiên mà phần tử >= target là bao nhiêu?"

```
arr = [1, 2, 4, 4, 6], target = 4

lo=0, hi=5, mid=2: arr[2]=4 >= 4  -> hi=2
lo=0, hi=2, mid=1: arr[1]=2 < 4   -> lo=2
lo=2, hi=2  -> trả về 2

=> Vị trí đầu tiên của 4 là index 2
```

### Upper Bound — tìm vị trí đầu tiên > target

Tìm phần tử đầu tiên **lớn hơn** target:

```
arr = [1, 2, 4, 4, 6], target = 4

lo=0, hi=5, mid=2: arr[2]=4 <= 4  -> lo=3
lo=3, hi=5, mid=4: arr[4]=6 > 4   -> hi=4
lo=3, hi=4, mid=3: arr[3]=4 <= 4  -> lo=4
lo=4, hi=4  -> trả về 4

=> Phần tử đầu tiên > 4 là 6 tại index 4
```

### Ứng dụng: đếm số lần xuất hiện

```
Số lần xuất hiện của 4 = upper_bound(4) - lower_bound(4) = 4 - 2 = 2

arr = [1, 2, 4, 4, 6]
            ^--^
            2 phần tử có giá trị 4
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

## Độ phức tạp

| Phép toán | Thời gian | Bộ nhớ |
|----------|----------|--------|
| binary_search | O(log n) | O(1) |
| lower_bound | O(log n) | O(1) |
| upper_bound | O(log n) | O(1) |

**Giải thích thực tế:**

- **O(log n)** cực kỳ nhanh. 1 tỷ phần tử? Chỉ cần ~30 bước.
- **O(1) bộ nhớ**: chỉ cần vài biến, không cần mảng phụ.
- **Điều kiện**: mảng PHẢI đã sắp xếp. Nếu chưa sắp xếp, kết quả sẽ sai. Chi phí sắp xếp là O(n log n), nhưng chỉ cần làm một lần.

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
