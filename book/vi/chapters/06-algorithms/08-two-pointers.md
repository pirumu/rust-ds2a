# Two Pointers

## Đây là gì?

Tưởng tượng hai người đứng ở hai đầu một con đường. Người bên trái bước sang phải, người bên phải bước sang trái. Họ tiến về giữa cho đến khi gặp nhau. Trên đường đi, họ có thể kiểm tra điều gì đó (ví dụ: tổng khoảng cách, so sánh độ cao, ...).

Đây là kỹ thuật **Two Pointers** (hai con trỏ) — dùng 2 chỉ số di chuyển trên mảng để giải bài toán trong O(n), thay vì O(n^2) nếu duyệt 2 vòng for lồng nhau.

Hai kiểu chính:

| Kiểu | Mô tả | Ví dụ |
|------|-------|-------|
| **Đối diện** (opposing) | Một đầu, một cuối, tiến về giữa | Two Sum, Container with Most Water |
| **Cùng hướng** (same direction) | Cả hai từ đầu, tốc độ khác nhau | Xóa phần tử trùng, Fast & Slow |

---

## Hoạt động như thế nào?

### Bài 1: Two Sum trên mảng đã sắp xếp

Cho mảng đã sắp xếp, tìm 2 phần tử có tổng bằng target.

```
arr = [1, 2, 3, 4, 6], target = 6

Bước 1:  [1, 2, 3, 4, 6]
          L              R     tổng = 1 + 6 = 7 > 6
                                     --> Tổng quá lớn! Giảm R

Bước 2:  [1, 2, 3, 4, 6]
          L           R        tổng = 1 + 4 = 5 < 6
                                     --> Tổng quá nhỏ! Tăng L

Bước 3:  [1, 2, 3, 4, 6]
             L        R        tổng = 2 + 4 = 6 == target
                                     --> Tìm thấy! Vị trí (1, 3)
```

**Tại sao cách này đúng?**

```
Vì mảng đã sắp xếp:
- Tổng quá LỚN  -> giảm R (vì phần tử bên phải lớn nhất)
- Tổng quá NHỎ  -> tăng L (vì phần tử bên trái nhỏ nhất)
- Mỗi bước loại được 1 khả năng -> không bỏ sót cặp nào!

Minh họa:
  L ---->              <---- R
  [1,  2,  3,  4,  6]
   ^               ^      7 > 6, di chuyển R sang trái
   ^           ^           5 < 6, di chuyển L sang phải
       ^       ^           6 == 6, tìm thấy!
```

### Bài 2: Container with Most Water

Cho mảng độ cao, tìm 2 thanh tạo container chứa nhiều nước nhất.

```
heights = [1, 8, 6, 2, 5, 4, 8, 3, 7]
            L                       R

Diện tích = min(cao_trái, cao_phải) * khoảng_cách

Bước 1: L=0, R=8
  Diện tích = min(1, 7) * 8 = 8
  Thanh trái thấp hơn -> di chuyển L sang phải (hy vọng tìm thanh cao hơn)

Bước 2: L=1, R=8
  Diện tích = min(8, 7) * 7 = 49
  Thanh phải thấp hơn -> di chuyển R sang trái

Bước 3: L=1, R=7
  Diện tích = min(8, 3) * 6 = 18
  Thanh phải thấp hơn -> di chuyển R

... tiếp tục cho đến L gặp R.

Max = 49
```

**Tại sao di chuyển thanh thấp hơn?** Vì diện tích = min(trái, phải) * khoảng cách. Khoảng cách sẽ giảm (L và R tiến lại gần nhau), nên chỉ có cách tăng độ cao mới cải thiện diện tích. Di chuyển thanh CAO không giúp gì (min vẫn là thanh thấp).

### Bài 3: Xóa phần tử trùng (cùng hướng)

```
arr = [1, 1, 2, 2, 3]
       s              s = slow (vị trí ghi)
       f              f = fast (vị trí đọc)

f=0: arr[0]=1, lần đầu gặp     -> ghi tại s=0, s=1
f=1: arr[1]=1, trùng            -> bỏ qua
f=2: arr[2]=2, phần tử mới     -> ghi tại s=1, s=2
f=3: arr[3]=2, trùng            -> bỏ qua
f=4: arr[4]=3, phần tử mới     -> ghi tại s=2, s=3

Kết quả (3 phần tử đầu): [1, 2, 3, ...]
```

---

## Code Rust

```rust
/// Two Sum trên mảng đã sắp xếp.
/// Trả về vị trí 2 phần tử có tổng bằng target.
pub fn two_sum_sorted(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    if arr.len() < 2 {
        return None;
    }
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;

    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return Some((left, right));    // Tìm thấy!
        } else if sum < target {
            left += 1;                      // Tổng quá nhỏ, tăng L
        } else {
            right -= 1;                     // Tổng quá lớn, giảm R
        }
    }
    None  // Không tìm thấy cặp nào
}
```

**Ghi chú về Rust:**

- `Option<(usize, usize)>` trả về `Some((i, j))` nếu tìm thấy, `None` nếu không.
- Không cần `Clone` hay `Copy` — chỉ đọc dữ liệu, không thay đổi mảng.

---

## Độ phức tạp

| Bài toán | Thời gian | Bộ nhớ |
|---------|----------|--------|
| Two Sum (sorted) | O(n) | O(1) |
| Container with Most Water | O(n) | O(1) |
| Xóa trùng | O(n) | O(1) |

**Giải thích thực tế:**

- Mỗi con trỏ di chuyển tối đa n lần. Tổng cộng vẫn chỉ là O(n).
- So với brute force (2 vòng for lồng nhau) là O(n^2), two pointers nhanh gấp n lần!
- **Điều kiện**: với opposing pointers, mảng thường phải **đã sắp xếp**. Với same-direction pointers, không cần.

---

## Ví dụ

```rust
use rust_ds2a::searching::two_sum_sorted;

let arr = [1, 2, 3, 4, 6];
assert_eq!(two_sum_sorted(&arr, 6), Some((1, 3)));  // 2 + 4 = 6

// Không tìm thấy
let arr = [1, 2, 3];
assert_eq!(two_sum_sorted(&arr, 100), None);

// Hoạt động với số âm (mảng phải đã sắp xếp)
let arr = [-3, -1, 0, 2, 4, 5];
assert_eq!(two_sum_sorted(&arr, 1), Some((1, 3)));  // -1 + 2 = 1
```
