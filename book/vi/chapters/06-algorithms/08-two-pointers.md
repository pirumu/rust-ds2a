# Two Pointers

> **Anxiety check:** Nếu bạn hiểu được Binary Search ở chương trước, bạn đã biết Two Pointers rồi đấy -- chỉ là chưa nhận ra thôi. Binary Search dùng 2 biến `lo` và `hi` di chuyển về giữa. Two Pointers **tổng quát hóa** pattern đó: thay vì luôn chia đôi, 2 con trỏ di chuyển theo **logic riêng** tùy bài toán. Không có toán khó, không có đệ quy, không có cấu trúc dữ liệu phức tạp. Chỉ có 2 biến chạy trên mảng. Thở đi, chương này dễ.

---

## Đây là gì?

Tưởng tượng hai người đứng ở hai đầu một con đường. Người bên trái bước sang phải, người bên phải bước sang trái. Họ tiến về giữa cho đến khi gặp nhau. Trên đường đi, họ có thể kiểm tra điều gì đó (ví dụ: tổng khoảng cách, so sánh độ cao, ...).

Đây là kỹ thuật **Two Pointers** (hai con trỏ) — dùng 2 chỉ số di chuyển trên mảng để giải bài toán trong O(n), thay vì O(n²) nếu duyệt 2 vòng for lồng nhau.

### Cầu nối từ Binary Search

Nhớ Binary Search không? Bạn có `lo` và `hi`, mỗi bước thu hẹp khoảng tìm kiếm:

```
Binary Search:    lo -------> mid <------- hi     (chia đôi mỗi bước)
Two Pointers:     L -------->     <-------- R     (di chuyển theo logic bài toán)
```

Binary Search luôn nhảy đến giữa. Two Pointers linh hoạt hơn — di chuyển L hoặc R tùy điều kiện. Nhưng ý tưởng cốt lõi giống nhau: **thu hẹp không gian tìm kiếm mỗi bước**.

---

## 3 Pattern chính

| Pattern | Mô tả | Hình ảnh | Ví dụ |
|---------|--------|----------|-------|
| **Đối diện** (opposite direction) | Một đầu, một cuối, tiến về giữa | `L -->  <-- R` | Two Sum sorted, Container With Most Water |
| **Cùng hướng** (same direction / fast-slow) | Cả hai từ đầu, tốc độ khác nhau | `S -> F -->` | Xóa trùng, phát hiện cycle (linked list) |
| **Sliding Window variant** | Cả hai từ đầu, cửa sổ co giãn | `L ....... R -->` | Longest substring, minimum window |

> Pattern thứ 3 (sliding window) quan trọng đến mức nó có **chương riêng** — chương tiếp theo! Ở đây mình tập trung vào 2 pattern đầu.

---

## Pattern 1: Opposite Direction (đối diện)

### Bài 1: Two Sum trên mảng đã sắp xếp

Cho mảng đã sắp xếp, tìm 2 phần tử có tổng bằng target.

**Ẩn dụ:** Hai người bạn đứng ở 2 đầu thang cuốn (thang đứng yên). Mỗi người cầm một tấm bảng ghi số. Họ cộng 2 số lại:
- Tổng quá lớn? Người bên PHẢI (số lớn) lùi 1 bước sang trái.
- Tổng quá nhỏ? Người bên TRÁI (số nhỏ) tiến 1 bước sang phải.
- Tổng đúng? High five!

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

### Tại sao converge (hội tụ) đúng?

Đây là câu hỏi hay nhất — tại sao cách này **không bỏ sót** cặp nào?

```
Gọi đáp án đúng là (a, b) với a < b.

Giả sử tại một thời điểm, L đang ở vị trí a.

- Nếu R > b:  arr[L] + arr[R] > target (vì arr[R] >= arr[b])
              --> Giảm R. R tiến dần về b.
- Nếu R = b:  arr[L] + arr[R] = target --> Tìm thấy!

Tương tự nếu R đang ở vị trí b, L sẽ tiến dần về a.

Kết luận: L không bao giờ "nhảy qua" a, R không bao giờ "nhảy qua" b.
          Vì mỗi bước chỉ loại đúng 1 khả năng đã được chứng minh sai.
```

Minh họa trực quan hơn — mỗi bước loại 1 hàng hoặc 1 cột:

```
        arr[R] -->  6   4   3   2   1
arr[L]
  |      1          7   5   4   3   2
  v      2          8  [6]  5   4   3
         3          9   7  [6]  5   4
         4         10   8   7  [6]  5
         6         12  10   9   8   7

Bắt đầu ở góc trên-phải (1+6=7). Quá lớn → sang trái.
1+4=5. Quá nhỏ → xuống dưới. 2+4=6. Tìm thấy!

Two pointers đi từ góc trên-phải, zigzag đến đáp án.
Mỗi bước loại 1 hàng hoặc 1 cột → tối đa n+n = 2n bước.
```

### Bài 2: Container With Most Water (LeetCode #11)

Cho mảng độ cao, tìm 2 thanh tạo container chứa nhiều nước nhất.

**Ẩn dụ:** Bạn có hàng rào gỗ với các thanh cao thấp khác nhau. Bạn muốn chọn 2 thanh để **kẹp giữa** chứa nước mưa. Nước sẽ tràn ở thanh THẤP hơn, nên diện tích = min(trái, phải) × khoảng cách.

```
heights = [1, 8, 6, 2, 5, 4, 8, 3, 7]

Trực quan:

     8              8
     |  6           |
     |  |     5  4  |     7
     |  |  2  |  |  |  3  |
  1  |  |  |  |  |  |  |  |
  |  |  |  |  |  |  |  |  |
  0  1  2  3  4  5  6  7  8   <-- index

  L                          R

Diện tích = min(cao_trái, cao_phải) × khoảng_cách
```

Trace chi tiết:

```
Bước 1: L=0, R=8
  min(1, 7) × 8 = 8
  Thanh trái (1) thấp hơn → di chuyển L sang phải

Bước 2: L=1, R=8
  min(8, 7) × 7 = 49  ★ max hiện tại
  Thanh phải (7) thấp hơn → di chuyển R sang trái

Bước 3: L=1, R=7
  min(8, 3) × 6 = 18
  Thanh phải (3) thấp hơn → di chuyển R

Bước 4: L=1, R=6
  min(8, 8) × 5 = 40
  Bằng nhau → di chuyển bên nào cũng được, chọn R

Bước 5: L=1, R=5
  min(8, 4) × 4 = 16
  Di chuyển R

Bước 6: L=1, R=4
  min(8, 5) × 3 = 15
  Di chuyển R

Bước 7: L=1, R=3
  min(8, 2) × 2 = 4
  Di chuyển R

Bước 8: L=1, R=2
  min(8, 6) × 1 = 6
  Di chuyển R

L >= R → Dừng. Max = 49
```

**Tại sao di chuyển thanh thấp hơn?**

```
Diện tích = min(trái, phải) × khoảng_cách

Khoảng cách sẽ GIẢM (L và R tiến lại gần nhau).
Chỉ có cách tăng min(trái, phải) mới CÓ THỂ cải thiện diện tích.

- Di chuyển thanh CAO: min vẫn là thanh thấp → diện tích CHẮC CHẮN giảm
- Di chuyển thanh THẤP: min CÓ THỂ tăng → còn hy vọng cải thiện

→ Luôn di chuyển thanh thấp hơn!
```

---

## Pattern 2: Same Direction (cùng hướng / fast-slow)

### Bài 3: Xóa phần tử trùng (in-place)

**Ẩn dụ:** Bạn có hàng người xếp hàng, nhiều người trùng tên. Bạn muốn chỉ giữ lại mỗi tên 1 người. Fast chạy dọc hàng kiểm tra từng người. Slow đứng ở vị trí "ghi" — chỉ nhích lên khi Fast tìm được người mới.

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

Nếu bạn đã học Linked List ở Phần 2, bạn biết fast/slow pointer dùng để phát hiện cycle. Cùng 1 pattern — 2 con trỏ chạy cùng hướng, tốc độ khác nhau.

---

## 3Sum = Sort + Two Sum (Pattern Composition)

Bài **3Sum** (LeetCode #15): tìm tất cả bộ ba có tổng = 0.

**Ý tưởng cốt lõi:** Cố định 1 phần tử, biến bài toán thành **Two Sum** trên phần còn lại!

```
nums = [-1, 0, 1, 2, -1, -4]

Bước 0: Sort → [-4, -1, -1, 0, 1, 2]

Với i=0, nums[i]=-4:
  target = 0 - (-4) = 4
  Two Sum trên [-1, -1, 0, 1, 2] với target=4
  L=1, R=5: -1+2=1 < 4, L++
  L=2, R=5: -1+2=1 < 4, L++
  L=3, R=5: 0+2=2 < 4, L++
  L=4, R=5: 1+2=3 < 4, L++
  L=5 >= R → không tìm thấy

Với i=1, nums[i]=-1:
  target = 0 - (-1) = 1
  Two Sum trên [-1, 0, 1, 2] với target=1
  L=2, R=5: -1+2=1 == target → tìm thấy! [-1, -1, 2]
  Tiếp tục: L=3, R=4: 0+1=1 == target → [-1, 0, 1]

Với i=2, nums[i]=-1:
  TRÙNG với i=1 (cùng giá trị -1) → BỎ QUA (tránh duplicate)

Với i=3, nums[i]=0:
  target = 0
  L=4, R=5: 1+2=3 > 0, R--
  L >= R → không tìm thấy

Kết quả: [[-1, -1, 2], [-1, 0, 1]]
```

**Độ phức tạp:** O(n²) — sort O(n log n) + vòng ngoài O(n) × Two Sum O(n) = O(n²). Tốt hơn brute force O(n³)!

```rust
pub fn three_sum(nums: &mut [i32]) -> Vec<[i32; 3]> {
    nums.sort();
    let mut result = Vec::new();
    let n = nums.len();

    for i in 0..n {
        // Bỏ qua phần tử trùng ở vòng ngoài
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        // Tối ưu: nếu nums[i] > 0, tổng 3 số dương không thể = 0
        if nums[i] > 0 {
            break;
        }

        let target = -nums[i];
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[left] + nums[right];
            if sum == target {
                result.push([nums[i], nums[left], nums[right]]);
                // Bỏ qua duplicate bên trái
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                // Bỏ qua duplicate bên phải
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result
}
```

Đây là ví dụ đẹp của **pattern composition**: bạn đã biết Two Sum, bạn chỉ cần thêm 1 vòng for bên ngoài. Kỹ thuật phỏng vấn cực kỳ phổ biến.

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

/// Container With Most Water — tìm diện tích lớn nhất.
pub fn max_area(heights: &[i32]) -> i32 {
    let mut left = 0usize;
    let mut right = heights.len() - 1;
    let mut max = 0i32;

    while left < right {
        let width = (right - left) as i32;
        let height = heights[left].min(heights[right]);
        let area = width * height;
        max = max.max(area);

        if heights[left] < heights[right] {
            left += 1;     // Di chuyển thanh thấp hơn
        } else {
            right -= 1;
        }
    }
    max
}
```

**Ghi chú về Rust:**

- `Option<(usize, usize)>` trả về `Some((i, j))` nếu tìm thấy, `None` nếu không.
- Không cần `Clone` hay `Copy` — chỉ đọc dữ liệu, không thay đổi mảng.
- `.min()` và `.max()` là method trên `i32` — idiomatic hơn `std::cmp::min(a, b)`.
- `as i32` cast `usize` sang `i32` cho phép tính diện tích. Trong production code, bạn sẽ muốn dùng `i32::try_from()` để tránh overflow, nhưng trong leetcode thì `as` đủ rồi.

---

## Độ phức tạp

| Bài toán | Thời gian | Bộ nhớ |
|---------|----------|--------|
| Two Sum (sorted) | O(n) | O(1) |
| Container With Most Water | O(n) | O(1) |
| Xóa trùng | O(n) | O(1) |
| 3Sum | O(n²) | O(1)* |

*\*Không kể output. Sorting in-place nên không tốn thêm bộ nhớ.*

**Giải thích thực tế:**

- Mỗi con trỏ di chuyển tối đa n lần. Tổng cộng vẫn chỉ là O(n).
- So với brute force (2 vòng for lồng nhau) là O(n²), two pointers nhanh gấp n lần!
- **Điều kiện**: với opposing pointers, mảng thường phải **đã sắp xếp**. Với same-direction pointers, không cần.

---

## Pitfalls — Bẫy thường gặp

### Bẫy 1: Quên sort trước khi dùng opposite-direction pointers

❌ **Sai:**
```rust
let arr = [3, 1, 4, 2];
two_sum_sorted(&arr, 5);  // Kết quả SAI! Mảng chưa sort!
```

✅ **Đúng:**
```rust
let mut arr = [3, 1, 4, 2];
arr.sort();                   // [1, 2, 3, 4]
two_sum_sorted(&arr, 5);     // Some((1, 2)) = 2 + 3 = 5
```

💡 **Tại sao:** Opposite-direction pointers dựa vào tính chất "tăng L → tổng tăng, giảm R → tổng giảm". Nếu mảng chưa sort, tính chất này không đúng → bỏ sót đáp án.

### Bẫy 2: Duplicate handling trong 3Sum

❌ **Sai:**
```rust
// Không skip duplicate → kết quả có bộ ba trùng
for i in 0..n {
    // ... two sum ...
}
```

✅ **Đúng:**
```rust
for i in 0..n {
    if i > 0 && nums[i] == nums[i - 1] { continue; }  // Skip!
    // Và skip duplicate bên trong two sum nữa
}
```

💡 **Tại sao:** Mảng sorted có thể có nhiều phần tử giống nhau liên tiếp. Nếu không skip, `[-1, -1, -1, 0, 1]` sẽ cho ra `[-1, 0, 1]` nhiều lần.

### Bẫy 3: Boundary conditions — off-by-one

❌ **Sai:**
```rust
while left <= right {   // <= thay vì <
    // Khi left == right, đang so sánh phần tử với chính nó!
}
```

✅ **Đúng:**
```rust
while left < right {    // Strict less than
    // 2 con trỏ luôn trỏ đến 2 phần tử KHÁC NHAU
}
```

💡 **Tại sao:** Two pointers tìm **cặp** 2 phần tử khác nhau. Khi `left == right`, chỉ còn 1 phần tử — không tạo thành cặp.

### Bẫy 4: Integer overflow khi tính tổng

❌ **Sai (trong C++/Java):**
```
mid = (lo + hi) / 2;   // lo + hi có thể overflow!
```

✅ **Đúng:**
```rust
let mid = lo + (hi - lo) / 2;  // An toàn!
// Hoặc trong Rust: lo.checked_add(hi) nếu cần
```

💡 **Tại sao:** Rust sẽ panic khi overflow trong debug mode, nên bạn sẽ phát hiện sớm. Nhưng trong release mode nó wrap around — viết an toàn từ đầu là tốt nhất. Nhớ bài học từ Binary Search chương trước!

---

## Khi nào dùng Two Pointers?

| Dấu hiệu | Pattern | Ví dụ |
|-----------|---------|-------|
| Mảng **đã sort** + tìm cặp/bộ ba thỏa điều kiện | Opposite direction | Two Sum, 3Sum, 4Sum |
| Tìm cặp (i, j) tối ưu trên mảng | Opposite direction | Container With Most Water |
| **Palindrome** check | Opposite direction | Valid Palindrome |
| Xóa/lọc phần tử **in-place** | Same direction (fast/slow) | Remove duplicates, Move zeroes |
| Phát hiện **cycle** | Same direction (fast/slow) | Linked list cycle detection |
| Merge 2 mảng sorted | Same direction (2 pointers riêng) | Merge sorted arrays |
| Tìm đoạn con thỏa điều kiện | Sliding window (→ chương sau) | Longest substring |

**Quy tắc ngón tay cái:** Nếu brute force cần 2 vòng for lồng nhau O(n²), hãy nghĩ đến Two Pointers trước!

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

---

## Rust Ecosystem

**Iterators & Two Pointers:** Rust standard library dùng pattern tương tự two pointers ở nhiều nơi:

```rust
// slice::partition_point — binary search dưới dạng khác
let arr = [1, 2, 3, 4, 5];
let idx = arr.partition_point(|&x| x < 3);  // idx = 2

// Vec::dedup — chính là fast/slow pointer xóa trùng!
let mut v = vec![1, 1, 2, 2, 3];
v.dedup();  // [1, 2, 3]

// slice::sort + dedup = combo phổ biến
let mut v = vec![3, 1, 2, 1, 3];
v.sort();
v.dedup();  // [1, 2, 3]
```

**`Vec::retain`** — same-direction pointer ẩn bên trong:

```rust
let mut v = vec![1, 2, 3, 4, 5];
v.retain(|&x| x % 2 == 0);  // [2, 4]
// Bên trong retain, Rust dùng fast/slow pointer
// để giữ lại phần tử thỏa điều kiện, in-place!
```

**`Iterator::zip`** — chạy 2 iterator song song, giống 2 pointers cùng hướng cùng tốc độ:

```rust
let names = ["Alice", "Bob"];
let scores = [95, 87];
let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
// [("Alice", 95), ("Bob", 87)]
```

**Trong thực tế:** Two pointers pattern xuất hiện trong merge sort (merge 2 mảng sorted), trong network packet processing (đọc/ghi buffer), và trong database join algorithms (sort-merge join). Nếu bạn dùng `itertools::merge` hay `itertools::kmerge`, bên dưới là two pointers.

---

## Practice

Sắp xếp từ dễ đến khó:

| # | Bài | Difficulty | Pattern | Gợi ý |
|---|-----|-----------|---------|-------|
| 1 | [Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/) (#167) | Easy | Opposite | Y hệt code trong chương này |
| 2 | [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/) (#125) | Easy | Opposite | L từ đầu, R từ cuối, so sánh ký tự |
| 3 | [Move Zeroes](https://leetcode.com/problems/move-zeroes/) (#283) | Easy | Same direction | Fast tìm số != 0, slow ghi |
| 4 | [Container With Most Water](https://leetcode.com/problems/container-with-most-water/) (#11) | Medium | Opposite | Di chuyển thanh thấp hơn |
| 5 | [3Sum](https://leetcode.com/problems/3sum/) (#15) | Medium | Sort + Opposite | Sort → fix 1 → Two Sum. Cẩn thận duplicate! |
| 6 | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/) (#42) | Hard | Opposite | Tương tự Container nhưng cộng nước ở MỖI vị trí. Giữ `left_max` và `right_max` |

> Bài #42 (Trapping Rain Water) cũng giải được bằng **Prefix Sum** (chương sau) hoặc **Stack** (Phần 2). Nhiều cách tiếp cận — đó là vẻ đẹp của DSA!

---

## Tiếp theo: Prefix Sum

Two Pointers giúp bạn duyệt mảng thông minh với O(1) bộ nhớ. Nhưng nếu bạn cần trả lời **hàng nghìn câu hỏi** "tổng từ i đến j là bao nhiêu?" thì sao? Dùng vòng for mỗi lần sẽ chậm kinh khủng.

**Prefix Sum** giải quyết bằng cách tính trước tổng tích lũy — biến mỗi truy vấn từ O(n) thành O(1). Giống như thay vì đếm tiền mỗi lần khách hỏi, bạn chuẩn bị sẵn bảng "tổng đến vị trí i" một lần duy nhất.

---

[← Binary Search](./07-binary-search.md) | [Prefix Sum →](./09-prefix-sum.md)
