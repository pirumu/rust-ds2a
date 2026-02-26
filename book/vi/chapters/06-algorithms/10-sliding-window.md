# Sliding Window

## Đây là gì?

Tưởng tượng bạn ngồi trên tàu hỏa và nhìn qua cửa sổ. Khi tàu chạy, cảnh bên ngoài thay đổi — phía trước có cảnh mới xuất hiện, phía sau có cảnh cũ biến mất. Nhưng **kích thước cửa sổ không đổi**.

Đây chính là kỹ thuật **Sliding Window** (cửa sổ trượt). Thay vì tính lại toàn bộ cho mỗi vị trí cửa sổ, ta chỉ cần **cập nhật**: thêm phần tử mới (bên phải) và bỏ phần tử cũ (bên trái).

Hai dạng chính:
- **Cửa sổ cố định** (fixed-size) — kích thước cho trước (ví dụ: tổng lớn nhất của k phần tử)
- **Cửa sổ co giãn** (variable-size) — cửa sổ mở rộng/thu hẹp theo điều kiện

---

## Hoạt động như thế nào?

### Cửa sổ cố định (Fixed-size)

Tìm tổng lớn nhất của 3 phần tử liên tiếp trong `[2, 1, 5, 1, 3, 2]`.

```
Kích thước cửa sổ k = 3

Bước 1: [2, 1, 5, 1, 3, 2]    cửa sổ = [2, 1, 5]    tổng = 8
         --------

Bước 2: [2, 1, 5, 1, 3, 2]    cửa sổ = [1, 5, 1]    tổng = 8-2+1 = 7
            --------
                                Bỏ 2 (bên trái), thêm 1 (bên phải)

Bước 3: [2, 1, 5, 1, 3, 2]    cửa sổ = [5, 1, 3]    tổng = 7-1+3 = 9  <-- max
               --------

Bước 4: [2, 1, 5, 1, 3, 2]    cửa sổ = [1, 3, 2]    tổng = 9-5+2 = 6
                  --------

Đáp án: 9
```

**Tại sao nhanh?** Thay vì cộng lại 3 số mỗi lần (tốn O(k)), ta chỉ trừ 1 số cũ và cộng 1 số mới (tốn O(1)). Tổng cộng: O(n) thay vì O(n*k).

### Cửa sổ co giãn (Variable-size)

Tìm chuỗi con dài nhất không có ký tự lặp trong `"abcabcbb"`.

```
"a b c a b c b b"
 L
 R

R=0: 'a' chưa gặp -> mở rộng, cửa sổ="a", dài=1
R=1: 'b' chưa gặp -> mở rộng, cửa sổ="ab", dài=2
R=2: 'c' chưa gặp -> mở rộng, cửa sổ="abc", dài=3   <-- tốt nhất
R=3: 'a' đã gặp tại 0 -> thu L về 1, cửa sổ="bca", dài=3
R=4: 'b' đã gặp tại 1 -> thu L về 2, cửa sổ="cab", dài=3
R=5: 'c' đã gặp tại 2 -> thu L về 3, cửa sổ="abc", dài=3
R=6: 'b' đã gặp tại 4 -> thu L về 5, cửa sổ="cb", dài=2
R=7: 'b' đã gặp tại 6 -> thu L về 7, cửa sổ="b", dài=1

Đáp án: 3
```

### Minh họa mở rộng / thu hẹp cửa sổ

```
Dạng cửa sổ co giãn:

  mở rộng R --->
  [==========]
   L         R

  Nếu vi phạm điều kiện:
  thu hẹp L --->
       [======]
        L     R
```

---

## Code Rust

```rust
/// Cửa sổ cố định: tổng lớn nhất của mảng con có kích thước k.
pub fn max_sum_subarray_of_size_k(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || arr.len() < k {
        return None;
    }
    // Tính tổng cửa sổ đầu tiên
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;

    // Trượt cửa sổ: bỏ phần tử trái, thêm phần tử phải
    for i in k..arr.len() {
        window_sum += arr[i] - arr[i - k];
        if window_sum > max_sum {
            max_sum = window_sum;
        }
    }
    Some(max_sum)
}

/// Cửa sổ co giãn: chuỗi con dài nhất không có ký tự lặp.
pub fn longest_substring_no_repeat(s: &str) -> usize {
    use std::collections::HashMap;
    let bytes = s.as_bytes();
    let mut last_seen: HashMap<u8, usize> = HashMap::new();
    let mut max_len: usize = 0;
    let mut start: usize = 0;

    for (i, &b) in bytes.iter().enumerate() {
        // Nếu ký tự đã gặp và nằm trong cửa sổ hiện tại
        if let Some(&prev) = last_seen.get(&b) {
            if prev >= start {
                start = prev + 1;  // Thu hẹp cửa sổ
            }
        }
        last_seen.insert(b, i);
        let current_len = i - start + 1;
        if current_len > max_len {
            max_len = current_len;
        }
    }
    max_len
}
```

**Ghi chú về Rust:**

- `arr[..k].iter().sum()` tính tổng k phần tử đầu tiên một cách gọn gàng.
- `HashMap` dùng để nhớ vị trí cuối cùng gặp mỗi ký tự. Trong Rust, `if let Some(&prev)` là cách pattern match đẹp để kiểm tra và lấy giá trị cùng lúc.
- `as_bytes()` chuyển chuỗi thành mảng byte — nhanh hơn duyệt theo `char` khi chỉ cần xử lý ASCII.

---

## Độ phức tạp

| Hàm | Thời gian | Bộ nhớ |
|-----|----------|--------|
| max_sum_subarray_of_size_k | O(n) | O(1) |
| longest_substring_no_repeat | O(n) | O(min(n, alphabet)) |

**Giải thích thực tế:**

- Cả hai đều đạt thời gian tuyến tính O(n) vì mỗi phần tử được xử lý tối đa 2 lần (một lần khi con trỏ phải đi qua, một lần khi con trỏ trái đi qua).
- Cửa sổ cố định chỉ cần vài biến -> O(1) bộ nhớ.
- Cửa sổ co giãn cần HashMap để theo dõi ký tự -> bộ nhớ phụ thuộc kích thước alphabet.

**Khi nào nên dùng?**

- Khi bài toán yêu cầu xét **mảng con liên tiếp** (subarray) hoặc **chuỗi con liên tiếp** (substring).
- Khi có thể cập nhật kết quả tăng dần thay vì tính lại từ đầu.

---

## Ví dụ

```rust
use rust_ds2a::sliding_window::{max_sum_subarray_of_size_k, longest_substring_no_repeat,
                                smallest_subarray_with_sum, max_of_subarrays};

// Cửa sổ cố định
let arr = [2, 1, 5, 1, 3, 2];
assert_eq!(max_sum_subarray_of_size_k(&arr, 3), Some(9));

// Cửa sổ co giãn
assert_eq!(longest_substring_no_repeat("abcabcbb"), 3);
assert_eq!(longest_substring_no_repeat("bbbbb"), 1);
assert_eq!(longest_substring_no_repeat("pwwkew"), 3);  // "wke"

// Mảng con nhỏ nhất có tổng >= 7
assert_eq!(smallest_subarray_with_sum(&[2, 1, 5, 2, 3, 2], 7), Some(2));

// Max trong mỗi cửa sổ kích thước 3
assert_eq!(max_of_subarrays(&[1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
```
