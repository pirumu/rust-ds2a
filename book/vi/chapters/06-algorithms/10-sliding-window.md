# Sliding Window

> 💡 **Đừng lo lắng:** Nếu bạn đã hiểu Two Pointers và Prefix Sum ở hai chương trước, bạn đã sẵn sàng 80% cho chương này rồi. Sliding Window chỉ là Two Pointers dạng "cùng hướng" + một cửa sổ chạy trên mảng. Không có gì mới hoàn toàn -- chỉ là cách kết hợp thông minh những gì bạn đã biết. Bạn hoàn toàn làm được.

---

## Từ Prefix Sum và Two Pointers đến Sliding Window

Ở [chương Prefix Sum](09-prefix-sum.md), bạn biết cách tính tổng đoạn bất kỳ trong O(1) bằng tiền xử lý. Ở [chương Two Pointers](08-two-pointers.md), bạn học cách dùng 2 con trỏ di chuyển trên mảng để giảm từ O(n^2) xuống O(n).

Sliding Window là **con đẻ** của cả hai:

```
Prefix Sum:     Tính tổng đoạn [i..j] nhanh
Two Pointers:   Hai con trỏ cùng hướng (L và R) chạy trên mảng
                     ↓
Sliding Window: L và R tạo thành "cửa sổ" [L..R]
                R mở rộng cửa sổ → thêm phần tử
                L thu hẹp cửa sổ → bỏ phần tử
                Cập nhật kết quả liên tục mà KHÔNG tính lại từ đầu
```

**Khác nhau chính:**
- **Two Pointers**: thường dùng 2 pointer đối diện (một đầu, một cuối)
- **Sliding Window**: luôn dùng 2 pointer cùng hướng, tạo thành một "cửa sổ" liên tục

Khi nào chuyển từ Prefix Sum sang Sliding Window? Khi bạn chỉ cần **một đáp án tối ưu** (max/min subarray) chứ không cần trả lời nhiều truy vấn. Sliding Window không cần O(n) bộ nhớ tiền xử lý như Prefix Sum.

---

## Đây là gì?

Tưởng tượng bạn ngồi trên tàu hỏa và nhìn qua cửa sổ. Khi tàu chạy, cảnh bên ngoài thay đổi -- phía trước có cảnh mới xuất hiện, phía sau có cảnh cũ biến mất. Nhưng **kích thước cửa sổ không đổi**.

Đây chính là kỹ thuật **Sliding Window** (cửa sổ trượt). Thay vì tính lại toàn bộ cho mỗi vị trí cửa sổ, ta chỉ cần **cập nhật**: thêm phần tử mới (bên phải) và bỏ phần tử cũ (bên trái).

Hai dạng chính:
- **Cửa sổ cố định** (fixed-size) -- kích thước cho trước (ví dụ: tổng lớn nhất của k phần tử)
- **Cửa sổ co giãn** (variable-size) -- cửa sổ mở rộng/thu hẹp theo điều kiện

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

## Template: Fixed vs Variable Window

Hai dạng sliding window có **khung code cố định**. Thuộc 2 template này, bạn giải được hầu hết bài sliding window.

### Template 1: Fixed-size Window

```
Khi nào dùng: đề bài cho trước kích thước window k
Ví dụ: "tổng lớn nhất của k phần tử liên tiếp"

┌─────────────────────────────────────────┐
│  1. Tính kết quả cho window đầu tiên    │
│     (index 0 đến k-1)                   │
│                                         │
│  2. Trượt window từ k đến n-1:          │
│     - Thêm arr[i] vào window            │
│     - Bỏ arr[i-k] khỏi window           │
│     - Cập nhật kết quả (max/min/...)     │
└─────────────────────────────────────────┘
```

```rust
// Template Fixed Window
fn fixed_window(arr: &[i32], k: usize) -> i32 {
    // Bước 1: window đầu tiên
    let mut window_state = /* tính từ arr[0..k] */;
    let mut result = window_state;

    // Bước 2: trượt
    for i in k..arr.len() {
        window_state += arr[i];       // thêm phần tử mới (bên phải)
        window_state -= arr[i - k];   // bỏ phần tử cũ (bên trái)
        result = result.max(window_state);
    }
    result
}
```

### Template 2: Variable-size Window

```
Khi nào dùng: tìm subarray/substring dài nhất hoặc ngắn nhất
              thỏa mãn một điều kiện nào đó
Ví dụ: "chuỗi con dài nhất không có ký tự lặp"

┌─────────────────────────────────────────┐
│  1. R chạy từ 0 đến n-1 (mở rộng)      │
│                                         │
│  2. Với mỗi R, KIỂM TRA điều kiện:     │
│     - Nếu VI PHẠM → shrink L            │
│       (while vi phạm: bỏ arr[L], L++)   │
│                                         │
│  3. Cập nhật kết quả (max/min length)   │
└─────────────────────────────────────────┘
```

```rust
// Template Variable Window
fn variable_window(arr: &[i32], condition: ...) -> usize {
    let mut left = 0;
    let mut result = 0;                  // hoặc usize::MAX nếu tìm min
    let mut window_state = /* ... */;

    for right in 0..arr.len() {
        // Mở rộng: thêm arr[right] vào window
        window_state.update(arr[right]);

        // Shrink: thu hẹp cho đến khi window hợp lệ
        while !is_valid(&window_state) {
            window_state.remove(arr[left]);
            left += 1;
        }

        // Cập nhật kết quả
        result = result.max(right - left + 1);
    }
    result
}
```

**Quy tắc vàng cho shrink:** Thu hẹp `left` khi window **vi phạm** điều kiện bài toán. Cụ thể:

| Bài toán | Khi nào shrink? |
|----------|----------------|
| Longest substring no repeat | Khi ký tự mới đã tồn tại trong window |
| Smallest subarray with sum >= target | Khi tổng window >= target (shrink để tìm min) |
| At most K distinct characters | Khi số ký tự khác nhau > K |
| Minimum window substring | Khi window chứa đủ tất cả ký tự cần thiết (shrink để tìm min) |

---

## Code Rust

### Bài 1: Fixed Window -- Tổng lớn nhất của k phần tử

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
```

### Bài 2: Variable Window + HashMap -- Longest Substring Without Repeating (#3)

Đây là bài kinh điển kết hợp **HashMap + Sliding Window**. HashMap nhớ vị trí cuối cùng gặp mỗi ký tự, giúp ta nhảy `left` thẳng đến vị trí đúng thay vì shrink từng bước.

```rust
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

**Trace chi tiet cho `"abcabcbb"`:**

```
HashMap: {}   start=0   max_len=0

i=0  b='a'  last_seen={} → chưa gặp
     last_seen = {'a':0}
     len = 0-0+1 = 1   max_len=1
     window: [a]

i=1  b='b'  last_seen={'a':0} → chưa gặp 'b'
     last_seen = {'a':0, 'b':1}
     len = 1-0+1 = 2   max_len=2
     window: [a,b]

i=2  b='c'  chưa gặp
     last_seen = {'a':0, 'b':1, 'c':2}
     len = 2-0+1 = 3   max_len=3
     window: [a,b,c]

i=3  b='a'  đã gặp tại 0, 0 >= start(0) → start = 0+1 = 1
     last_seen = {'a':3, 'b':1, 'c':2}
     len = 3-1+1 = 3   max_len=3
     window: [b,c,a]

i=4  b='b'  đã gặp tại 1, 1 >= start(1) → start = 1+1 = 2
     last_seen = {'a':3, 'b':4, 'c':2}
     len = 4-2+1 = 3   max_len=3
     window: [c,a,b]

i=5  b='c'  đã gặp tại 2, 2 >= start(2) → start = 2+1 = 3
     last_seen = {'a':3, 'b':4, 'c':5}
     len = 5-3+1 = 3   max_len=3
     window: [a,b,c]

i=6  b='b'  đã gặp tại 4, 4 >= start(3) → start = 4+1 = 5
     last_seen = {'a':3, 'b':6, 'c':5}
     len = 6-5+1 = 2   max_len=3
     window: [c,b]

i=7  b='b'  đã gặp tại 6, 6 >= start(5) → start = 6+1 = 7
     last_seen = {'a':3, 'b':7, 'c':5}
     len = 7-7+1 = 1   max_len=3
     window: [b]

Đáp án: 3
```

**Tại sao check `prev >= start`?** Vì HashMap không xóa ký tự cũ. Nếu `prev < start`, ký tự đó đã **nằm ngoài** window rồi -- không cần quan tâm.

### Bài 3: Variable Window -- Minimum Size Subarray Sum (#209)

Bài toán: tìm mảng con **ngắn nhất** có tổng >= target.

Ẩn dụ: bạn đứng ở buffet, cần chọn ít đĩa nhất sao cho tổng calo >= target. Bạn cứ lấy thêm đĩa (mở rộng R) cho đến khi đủ calo, rồi thử bỏ bớt đĩa cũ nhất (thu hẹp L) xem có vẫn đủ không.

```rust
/// Mảng con ngắn nhất có tổng >= target.
pub fn smallest_subarray_with_sum(arr: &[i32], target: i32) -> Option<usize> {
    let mut window_sum: i32 = 0;
    let mut min_len: usize = usize::MAX;
    let mut start: usize = 0;

    for end in 0..arr.len() {
        window_sum += arr[end];          // mở rộng
        while window_sum >= target {      // đã đủ → thu hẹp!
            let current_len = end - start + 1;
            if current_len < min_len {
                min_len = current_len;
            }
            window_sum -= arr[start];    // bỏ phần tử trái
            start += 1;
        }
    }

    if min_len == usize::MAX {
        None
    } else {
        Some(min_len)
    }
}
```

**Trace cho `[2, 1, 5, 2, 3, 2]`, target = 7:**

```
start=0, window_sum=0, min_len=MAX

end=0: sum=2                    < 7, tiếp
end=1: sum=3                    < 7, tiếp
end=2: sum=8                    >= 7! len=3, min_len=3
       shrink: sum=8-2=6, start=1    < 7, dừng shrink
end=3: sum=6+2=8                >= 7! len=3, min_len=3
       shrink: sum=8-1=7, start=2    >= 7! len=2, min_len=2  ← cập nhật!
       shrink: sum=7-5=2, start=3    < 7, dừng
end=4: sum=2+3=5                < 7, tiếp
end=5: sum=5+2=7                >= 7! len=3, min_len vẫn 2
       shrink: sum=7-2=5, start=4    < 7, dừng

Đáp án: 2 (subarray [5, 2])
```

### Bài 4: Sliding Window Maximum (#239) -- Deque + Sliding Window

Đây là bài **khó nhất** trong chương, kết hợp Sliding Window với Deque. Nếu bạn đã đọc [chương Deque](../02-linear-structures/05-deque.md), bạn đã thấy bài này ở đó rồi. Giờ mình nhìn lại từ góc nhìn Sliding Window.

**Bài toán:** Tìm giá trị lớn nhất trong mỗi cửa sổ kích thước k.

**Tại sao không dùng `max()` thường?** Vì mỗi lần gọi `max()` trên k phần tử tốn O(k). Tổng cộng O(nk). Với Deque, ta đạt O(n).

**Ý tưởng:** Duy trì một **monotonic deque** (deque giảm dần) chứa index. Front luôn là index của phần tử lớn nhất trong window.

```
Mảng: [1, 3, -1, -3, 5, 3, 6, 7],  k=3

Deque chứa INDEX, giữ giá trị GIẢM DẦN từ front đến back.

i=0  nums[0]=1
     Deque: [0]                     (giá trị: [1])
     Window chưa đủ k phần tử

i=1  nums[1]=3 > nums[0]=1 → pop back(0)
     Deque: [1]                     (giá trị: [3])
     Window chưa đủ

i=2  nums[2]=-1 < nums[1]=3 → giữ nguyên
     Deque: [1, 2]                  (giá trị: [3, -1])
     Window đủ! max = nums[1] = 3        ← output: 3

i=3  nums[3]=-3 < nums[2]=-1 → giữ nguyên
     Deque: [1, 2, 3]              (giá trị: [3, -1, -3])
     Index 1 vẫn trong window (1+3=4 > 3)
     max = nums[1] = 3                   ← output: 3

i=4  nums[4]=5 > nums[3]=-3 → pop
     nums[4]=5 > nums[2]=-1 → pop
     nums[4]=5 > nums[1]=3  → pop
     Deque: [4]                     (giá trị: [5])
     max = nums[4] = 5                   ← output: 5

i=5  nums[5]=3 < nums[4]=5 → giữ
     Deque: [4, 5]                  (giá trị: [5, 3])
     max = nums[4] = 5                   ← output: 5

i=6  nums[6]=6 > nums[5]=3 → pop
     nums[6]=6 > nums[4]=5 → pop
     Deque: [6]                     (giá trị: [6])
     max = nums[6] = 6                   ← output: 6

i=7  nums[7]=7 > nums[6]=6 → pop
     Deque: [7]                     (giá trị: [7])
     max = nums[7] = 7                   ← output: 7

Kết quả: [3, 3, 5, 5, 6, 7]
```

**Code Rust:**

```rust
use std::collections::VecDeque;

/// Max trong mỗi cửa sổ kích thước k. Dùng monotonic deque.
pub fn max_of_subarrays(arr: &[i32], k: usize) -> Vec<i32> {
    if k == 0 || arr.len() < k {
        return vec![];
    }
    let mut result = Vec::with_capacity(arr.len() - k + 1);
    let mut deque: VecDeque<usize> = VecDeque::new();  // chứa index

    for i in 0..arr.len() {
        // Xóa index đã ra khỏi window
        if let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            }
        }
        // Xóa các index có giá trị <= nums[i] (chúng không bao giờ là max nữa)
        while let Some(&back) = deque.back() {
            if arr[back] <= arr[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);

        // Window đủ k phần tử → ghi nhận max
        if i >= k - 1 {
            result.push(arr[deque[0]]);
        }
    }
    result
}
```

**Liên kết chương Deque:** `VecDeque` ở đây chính là cái ring buffer bạn đã học ở [chương Deque](../02-linear-structures/05-deque.md). `pop_front()` O(1) -- chỉ tiến head pointer. `pop_back()` O(1) -- chỉ lùi tail pointer. Không có phần tử nào bị dịch. Đó là lý do Deque cho phép ta đạt O(n) tổng cộng.

---

## Ghi chú về Rust

- `arr[..k].iter().sum()` tính tổng k phần tử đầu tiên một cách gọn gàng.
- `HashMap` dùng để nhớ vị trí cuối cùng gặp mỗi ký tự. Trong Rust, `if let Some(&prev)` là cách pattern match đẹp để kiểm tra và lấy giá trị cùng lúc.
- `as_bytes()` chuyển chuỗi thành mảng byte -- nhanh hơn duyệt theo `char` khi chỉ cần xử lý ASCII.
- `VecDeque<usize>` cho monotonic deque -- borrow checker thân thiện vì ta chứa `usize` index thay vì reference.

---

## Độ phức tạp

| Hàm | Thời gian | Bộ nhớ |
|-----|----------|--------|
| max_sum_subarray_of_size_k | O(n) | O(1) |
| longest_substring_no_repeat | O(n) | O(min(n, alphabet)) |
| smallest_subarray_with_sum | O(n) | O(1) |
| max_of_subarrays | O(n) | O(k) |

**Giải thích thực tế:**

- Tất cả đều đạt thời gian tuyến tính O(n) vì mỗi phần tử được xử lý tối đa 2 lần (một lần khi con trỏ phải đi qua, một lần khi con trỏ trái đi qua).
- Cửa sổ cố định (sum) chỉ cần vài biến -> O(1) bộ nhớ.
- Cửa sổ co giãn (substring) cần HashMap để theo dõi ký tự -> bộ nhớ phụ thuộc kích thước alphabet.
- Sliding Window Maximum cần Deque chứa tối đa k index -> O(k) bộ nhớ.

---

## Những cái bẫy hay gặp

---

❌ **Sai:** Quên shrink window -- chỉ mở rộng R mà không bao giờ thu hẹp L

✅ **Đúng:** Luôn có vòng `while` (hoặc `if`) để shrink khi vi phạm điều kiện

💡 **Tại sao:** Không shrink = brute force trá hình. Bạn xét tất cả subarray bắt đầu từ 0, không bao giờ bỏ qua phần tử thừa bên trái. Kết quả sẽ sai (window quá rộng) hoặc chậm (O(n^2)).

```rust
// ❌ Quên shrink -- left luôn = 0
for right in 0..n {
    window_sum += arr[right];
    result = result.max(right - left + 1);  // left không bao giờ tăng!
}

// ✅ Có shrink
for right in 0..n {
    window_sum += arr[right];
    while /* vi phạm điều kiện */ {
        window_sum -= arr[left];
        left += 1;                          // left tiến về phía right
    }
    result = result.max(right - left + 1);
}
```

---

❌ **Sai:** Off-by-one khi tính kích thước window

✅ **Đúng:** Kích thước window = `right - left + 1` (bao gồm cả 2 đầu)

💡 **Tại sao:** Lỗi kinh điển. Window từ index 2 đến index 4 có **3** phần tử, không phải 2. Công thức: `right - left + 1`. Nếu bạn dùng `right - left`, thiếu 1.

```
Index:  0  1  [2  3  4]  5  6
                L     R

Kích thước = R - L + 1 = 4 - 2 + 1 = 3  ✓
Nếu dùng R - L = 4 - 2 = 2              ✗ (thiếu 1!)
```

---

❌ **Sai:** Reset state sai khi shrink -- quên trừ/xóa phần tử rời window

✅ **Đúng:** Mỗi lần `left++`, phải cập nhật window state tương ứng

💡 **Tại sao:** Window state (tổng, HashMap, counter...) phải luôn phản ánh **đúng** các phần tử trong window hiện tại. Nếu bạn tăng `left` mà không trừ `arr[left]` khỏi `window_sum`, state sai, kết quả sai.

```rust
// ❌ Tăng left mà quên trừ
while window_sum >= target {
    min_len = min_len.min(right - left + 1);
    left += 1;  // arr[left-1] vẫn nằm trong window_sum!
}

// ✅ Trừ trước, tăng sau
while window_sum >= target {
    min_len = min_len.min(right - left + 1);
    window_sum -= arr[left];  // bỏ phần tử trái ra khỏi state
    left += 1;
}
```

---

❌ **Sai:** Fixed window mà dùng template variable (hoặc ngược lại)

✅ **Đúng:** Nhận diện đúng dạng bài trước khi code

💡 **Tại sao:** Fixed window không cần `while` shrink -- chỉ cần trượt `window_sum += arr[i] - arr[i-k]`. Variable window cần `while` shrink. Nếu dùng nhầm template, code phức tạp hóa không cần thiết hoặc sai logic.

---

## Khi nào dùng Sliding Window?

| Dấu hiệu trong đề bài | Dạng Window | Ví dụ |
|------------------------|-------------|-------|
| "subarray/substring có kích thước k" | Fixed | Max sum of k elements |
| "subarray/substring dài nhất thỏa..." | Variable (tìm max) | Longest substring no repeat |
| "subarray/substring ngắn nhất thỏa..." | Variable (tìm min) | Minimum size subarray sum |
| "tất cả window kích thước k" | Fixed + Deque | Sliding window maximum |
| Có keyword "contiguous", "consecutive" | Xem xét SW | Các dạng subarray |
| Yêu cầu O(n) trên subarray liên tiếp | Rất có thể SW | -- |

**Khi nào KHÔNG dùng?**

| Tình huống | Dùng gì thay thế? |
|------------|-------------------|
| Subarray không liên tiếp (subsequence) | DP, Two Pointers |
| Cần tổng đoạn bất kỳ, nhiều truy vấn | Prefix Sum |
| Mảng không có tính chất đơn điệu khi mở rộng/thu hẹp | Brute force hoặc DP |
| Cần sắp xếp hoặc so sánh toàn cục | Sorting, Heap |

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

---

## Luyện tập

### Bài 1: Minimum Size Subarray Sum (LeetCode #209)

Cho mảng số nguyên dương `nums` và số `target`, tìm mảng con liên tiếp ngắn nhất có tổng >= target.

**Gợi ý:** Dùng template Variable Window. Shrink khi tổng >= target (vì đã thỏa mãn, thử tìm ngắn hơn).

<details>
<summary>Hướng dẫn</summary>

Chính là hàm `smallest_subarray_with_sum` ở trên. Pattern:
1. Mở rộng R, cộng dồn `window_sum`
2. `while window_sum >= target`: ghi nhận min length, shrink L
3. Nếu cuối cùng `min_len` vẫn là MAX -> trả `None`

Key insight: vì tất cả phần tử dương, khi mở rộng window thì tổng luôn tăng, khi thu hẹp thì tổng luôn giảm. Tính đơn điệu này cho phép dùng Sliding Window.

</details>

---

### Bài 2: Longest Substring Without Repeating Characters (LeetCode #3)

Cho chuỗi `s`, tìm chuỗi con dài nhất không có ký tự lặp.

**Gợi ý:** HashMap + Variable Window. HashMap nhớ vị trí cuối cùng gặp mỗi ký tự.

<details>
<summary>Hướng dẫn</summary>

Chính là hàm `longest_substring_no_repeat` ở trên. Hai cách approach:

**Cách 1 (HashMap, nhảy left):** Khi gặp ký tự trùng tại vị trí `prev`, nhảy `start = prev + 1`. Không cần while loop vì HashMap cho phép nhảy trực tiếp.

**Cách 2 (HashSet, shrink từng bước):** Dùng HashSet theo dõi ký tự trong window. Khi gặp trùng, `while` remove `arr[left]` khỏi set và `left++` cho đến khi hết trùng.

Cách 1 nhanh hơn (ít bước shrink), cách 2 dễ hiểu hơn cho người mới.

</details>

---

### Bài 3: Sliding Window Maximum (LeetCode #239)

Cho mảng `nums` và kích thước window `k`, trả về mảng chứa max của mỗi window.

**Gợi ý:** Monotonic Deque giảm dần. Nhớ lại [chương Deque](../02-linear-structures/05-deque.md).

<details>
<summary>Hướng dẫn</summary>

Chính là hàm `max_of_subarrays` ở trên. 3 bước mỗi iteration:

1. **Xóa index hết hạn** ở front (nếu `front + k <= i`)
2. **Xóa giá trị nhỏ hơn** ở back (chúng không bao giờ là max khi `arr[i]` còn trong window)
3. **Push** index `i` vào back

Deque luôn giảm dần -> `deque[0]` luôn là index của max. Mỗi phần tử vào deque đúng 1 lần, ra đúng 1 lần -> O(n) tổng cộng.

</details>

---

## Sliding Window trong Rust Ecosystem

- **`std::collections::VecDeque`** -- bạn đã dùng cho monotonic deque trong Sliding Window Maximum. Ring buffer O(1) ở cả hai đầu.

- **`itertools::tuple_windows()`** -- tạo fixed-size sliding window trên iterator. Rất tiện cho prototype nhanh:

```rust
use itertools::Itertools;

// Tổng mỗi window 3 phần tử
let sums: Vec<i32> = vec![2, 1, 5, 1, 3, 2]
    .iter()
    .tuple_windows()
    .map(|(a, b, c)| a + b + c)
    .collect();
// sums = [8, 7, 9, 6]
```

- **`slice::windows(k)`** -- standard library, tạo sliding window trên slice:

```rust
let arr = [2, 1, 5, 1, 3, 2];
let max_sum = arr.windows(3)
    .map(|w| w.iter().sum::<i32>())
    .max();
assert_eq!(max_sum, Some(9));
```

Lưu ý: `windows()` và `tuple_windows()` tiện nhưng **tính lại sum mỗi window** (O(nk)). Chỉ dùng cho prototyping hoặc khi k nhỏ. Sliding window thủ công O(n) mới là cách tối ưu.

- **KaCrab / streaming systems** -- Kafka consumer xử lý message theo batch (window). Ví dụ: "tính trung bình latency trong 5 phút gần nhất" chính là fixed-size sliding window trên time series.

---

## Chương tiếp theo

Bạn đã nắm 3 kỹ thuật xử lý mảng hiệu quả: **Prefix Sum** (tiền xử lý + truy vấn O(1)), **Two Pointers** (2 con trỏ loại bỏ trường hợp thừa), và **Sliding Window** (cửa sổ trượt cập nhật liên tục). Cả 3 đều biến O(n^2) thành O(n) bằng cách **không tính lại từ đầu**.

Chương tiếp theo là [**Divide and Conquer**](11-divide-and-conquer.md) -- chia bài toán lớn thành bài toán nhỏ rồi gộp kết quả. Bạn đã gặp tư duy này trong Merge Sort và Binary Search. Giờ mình sẽ nhìn nó như một **framework** chung để giải nhiều dạng bài khác nhau. Từ "trượt cửa sổ" sang "chia để trị" -- hai cách tiếp cận hoàn toàn khác nhau cho cùng mục tiêu: giảm khối lượng tính toán.

---

---

[← Prefix Sum](./09-prefix-sum.md) | [Divide & Conquer →](./11-divide-and-conquer.md)
