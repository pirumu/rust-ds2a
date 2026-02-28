# Monotonic Stack

> 💡 **Đừng lo lắng:** Bạn đã biết Stack rồi (Phần 2, chương 3). Monotonic Stack chỉ là Stack + 1 rule duy nhất: **pop khi vi phạm thứ tự**. Nếu bạn hiểu push/pop, bạn đã hiểu 90% rồi. Thở đi, không khó đâu.

---

## Từ Stack thường sang Monotonic Stack

Nhớ lại Stack ở Phần 2? Push lên, pop ra, LIFO. Đơn giản.

Monotonic Stack vẫn là push/pop y hệt. Khác duy nhất: ta thêm một **invariant** (bất biến) -- các phần tử trong stack phải luôn tăng dần hoặc giảm dần. Khi phần tử mới vi phạm thứ tự này, ta pop cho đến khi thứ tự được khôi phục.

```
Stack thường:       push bất kỳ, pop đỉnh
                    [3, 7, 2, 9, 1] ← OK, không quan tâm thứ tự

Monotonic Stack:    push + giữ thứ tự, pop khi vi phạm
                    [1, 3, 7, 9]    ← luôn tăng dần (increasing)
                    hoặc
                    [9, 7, 3, 1]    ← luôn giảm dần (decreasing)
```

Vậy thôi. Không có phép thuật gì cả. Cùng push/pop, thêm invariant tăng/giảm.

---

## Đây là gì?

Tưởng tượng bạn đứng xếp hàng, nhìn về phía trước, muốn tìm **người cao hơn mình gần nhất** phía trước. Nếu người ngay trước bạn thấp hơn, bạn "nhìn xuyên qua" họ. Bạn chỉ dừng lại khi thấy ai đó cao hơn.

```
Hàng người (chiều cao):  160  155  170  165  180

Bạn là 160 -> nhìn qua 155 -> gặp 170. Done!
Bạn là 155 -> gặp 170 ngay. Done!
Bạn là 170 -> nhìn qua 165 -> gặp 180. Done!
Bạn là 165 -> gặp 180 ngay. Done!
Bạn là 180 -> không ai cao hơn. None!
```

**Monotonic Stack** (stack đơn điệu) là stack mà các phần tử bên trong luôn giữ thứ tự tăng dần hoặc giảm dần. Khi thêm phần tử mới, ta pop hết những phần tử vi phạm thứ tự. Lúc pop chính là lúc ta tìm được câu trả lời cho phần tử bị pop.

---

## Template: Increasing vs Decreasing

Hai loại, hai mục đích khác nhau:

| Loại | Trong stack giữ | Dùng khi | Pop khi |
|------|----------------|----------|---------|
| **Monotonic Increasing** | Nhỏ -> lớn (đáy -> đỉnh) | Tìm next **greater** element | Phần tử mới **lớn hơn** đỉnh |
| **Monotonic Decreasing** | Lớn -> nhỏ (đáy -> đỉnh) | Tìm next **smaller** element | Phần tử mới **nhỏ hơn** đỉnh |

### Template chung (Rust)

```rust
// Monotonic Increasing Stack — tìm next greater
let mut stack: Vec<usize> = Vec::new();  // luôn lưu INDEX, không lưu value

for i in 0..n {
    while let Some(&top) = stack.last() {
        if nums[top] < nums[i] {      // vi phạm thứ tự tăng
            // top đã tìm được next greater = nums[i]
            result[top] = nums[i];
            stack.pop();
        } else {
            break;
        }
    }
    stack.push(i);                     // push INDEX, không push value
}
```

```rust
// Monotonic Decreasing Stack — tìm next smaller
// Y hệt, chỉ đổi dấu < thành >
while let Some(&top) = stack.last() {
    if nums[top] > nums[i] {          // vi phạm thứ tự giảm
        result[top] = nums[i];
        stack.pop();
    } else {
        break;
    }
}
```

> **Luôn push index, không push value.** Index cho bạn cả vị trí lẫn giá trị (`nums[index]`). Value chỉ cho giá trị, mất vị trí. Đây là sai lầm phổ biến nhất -- xem phần Pitfalls bên dưới.

---

## Tại sao cần Monotonic Stack?

Nếu dùng 2 vòng for lồng nhau: O(n^2). Với mảng 1 triệu phần tử, quá chậm!

Monotonic Stack giải quyết trong **O(n)** vì mỗi phần tử chỉ được push và pop tối đa 1 lần.

```
Brute force:     O(n^2)     <- 2 vòng for
Monotonic Stack:  O(n)      <- mỗi phần tử push 1 lần, pop 1 lần
```

---

## Next Greater Element

**Bài toán:** Cho mỗi phần tử, tìm phần tử lớn hơn gần nhất bên phải.

### Ý tưởng

Duyệt từ trái sang phải. Dùng stack lưu **index** của các phần tử chưa tìm được câu trả lời. Khi gặp phần tử lớn hơn đỉnh stack -> đỉnh stack đã có câu trả lời, pop nó ra.

### ASCII walkthrough

```
nums = [2, 1, 2, 4, 3]
         0  1  2  3  4    <- index

Stack lưu index. Ta hiển thị giá trị trong ngoặc.

i=0: nums[0]=2
     Stack rỗng -> push 0
     Stack: [0(2)]

i=1: nums[1]=1
     1 < 2 (đỉnh stack) -> không pop
     Push 1
     Stack: [0(2), 1(1)]

i=2: nums[2]=2
     2 > 1 (đỉnh stack) -> pop 1, result[1] = 2
     2 >= 2 (đỉnh stack)? Không, 2 không > 2 -> dừng
     Push 2
     Stack: [0(2), 2(2)]

i=3: nums[3]=4
     4 > 2 (đỉnh stack) -> pop 2, result[2] = 4
     4 > 2 (đỉnh stack) -> pop 0, result[0] = 4
     Stack rỗng -> dừng
     Push 3
     Stack: [3(4)]

i=4: nums[4]=3
     3 < 4 (đỉnh stack) -> không pop
     Push 4
     Stack: [3(4), 4(3)]

Kết thúc: index 3 và 4 vẫn trong stack -> result = None

Kết quả: [Some(4), Some(2), Some(4), None, None]
```

### Code

```rust
pub fn next_greater_element(nums: &[i32]) -> Vec<Option<i32>> {
    let n = nums.len();
    let mut result = vec![None; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if nums[top] < nums[i] {
                result[top] = Some(nums[i]);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}
```

**Time:** O(n). **Space:** O(n).

> Next Smaller Element hoạt động y hệt, chỉ đổi dấu `<` thành `>`.

---

## Daily Temperatures

**Bài toán:** Cho nhiệt độ mỗi ngày, tìm số ngày phải chờ đến ngày ấm hơn.

Ý tưởng giống Next Greater Element, nhưng thay vì lưu giá trị, ta lưu **khoảng cách index**.

### ASCII walkthrough

```
temps = [73, 74, 75, 71, 69, 72, 76, 73]
          0   1   2   3   4   5   6   7

i=0: push 0.               Stack: [0(73)]
i=1: 74>73 -> pop 0,
     result[0] = 1-0 = 1.  Stack: [1(74)]
i=2: 75>74 -> pop 1,
     result[1] = 2-1 = 1.  Stack: [2(75)]
i=3: 71<75 -> push.        Stack: [2(75), 3(71)]
i=4: 69<71 -> push.        Stack: [2(75), 3(71), 4(69)]
i=5: 72>69 -> pop 4,
     result[4] = 5-4 = 1.
     72>71 -> pop 3,
     result[3] = 5-3 = 2.
     72<75 -> dừng, push.   Stack: [2(75), 5(72)]
i=6: 76>72 -> pop 5,
     result[5] = 6-5 = 1.
     76>75 -> pop 2,
     result[2] = 6-2 = 4.  Stack: [6(76)]
i=7: 73<76 -> push.        Stack: [6(76), 7(73)]

Còn lại trong stack -> result = 0.

Kết quả: [1, 1, 4, 2, 1, 1, 0, 0]
```

### Code

```rust
pub fn daily_temperatures(temps: &[i32]) -> Vec<i32> {
    let n = temps.len();
    let mut result = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if temps[top] < temps[i] {
                result[top] = (i - top) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}
```

**Time:** O(n). **Space:** O(n).

---

## Largest Rectangle in Histogram

**Bài toán:** Cho mảng `heights` biểu diễn histogram, tìm hình chữ nhật lớn nhất nằm gọn bên trong.

Đây là bài khó nhất trong chương này. Ý tưởng: với mỗi thanh, tìm thanh ngắn hơn gần nhất bên trái và bên phải. Khoảng cách giữa hai thanh đó chính là chiều rộng tối đa mà thanh hiện tại có thể "mở rộng".

### ASCII walkthrough

```
heights = [2, 1, 5, 6, 2, 3]
            0  1  2  3  4  5

Stack giữ monotonic increasing (tăng dần).

i=0: push 0.               Stack: [0(2)]
i=1: h=1 < 2
     Pop 0: h=2, w=1 (stack rỗng -> w=i=1).
     Area = 2*1 = 2. max=2.
     Push 1.                Stack: [1(1)]
i=2: h=5 > 1 -> push.      Stack: [1(1), 2(5)]
i=3: h=6 > 5 -> push.      Stack: [1(1), 2(5), 3(6)]
i=4: h=2 < 6
     Pop 3: h=6, left=2, w=4-2-1=1.
     Area = 6*1 = 6. max=6.
     Pop 2: h=5, left=1, w=4-1-1=2.
     Area = 5*2 = 10. max=10.
     2 >= 2? Không (không strictly >=). Dừng.

     Chú ý: trong code ta dùng >= để xử lý trường hợp bằng.
     Pop vì heights[1]=1 < 2 -> dừng.
     Push 4.                Stack: [1(1), 4(2)]
i=5: h=3 > 2 -> push.      Stack: [1(1), 4(2), 5(3)]

Dọn dẹp (thêm thanh ảo h=0 ở cuối):
i=6: h=0
     Pop 5: h=3, left=4, w=6-4-1=1. Area=3.
     Pop 4: h=2, left=1, w=6-1-1=4. Area=8.
     Pop 1: h=1, stack rỗng, w=6. Area=6.

max = 10.
```

### Code

```rust
pub fn largest_rectangle_histogram(heights: &[i32]) -> i64 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area: i64 = 0;

    for i in 0..=n {
        let cur_h = if i < n { heights[i] } else { 0 };

        while let Some(&top) = stack.last() {
            if heights[top] >= cur_h {
                stack.pop();
                let h = heights[top] as i64;
                let w = match stack.last() {
                    Some(&left) => (i - left - 1) as i64,
                    None => i as i64,
                };
                max_area = max_area.max(h * w);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
}
```

**Time:** O(n). **Space:** O(n).

> Trick: thêm thanh ảo có chiều cao 0 ở cuối (`i = n`) để đảm bảo mọi thanh đều được pop ra và tính diện tích.

---

## Stock Span

**Bài toán:** Cho giá cổ phiếu mỗi ngày, tìm **span** -- số ngày liên tiếp (kể cả hôm nay) mà giá <= giá hôm nay.

Tưởng tượng bạn nhìn lại phía sau: bao nhiêu ngày liên tiếp giá thấp hơn hoặc bằng hôm nay?

### ASCII walkthrough

```
prices = [100, 80, 60, 70, 60, 75, 85]
            0   1   2   3   4   5   6

i=0: Stack rỗng -> span = 0+1 = 1.
     Push 0.                Stack: [0(100)]

i=1: 80 <= 100? Không -> dừng.
     span = 1-0 = 1.
     Push 1.                Stack: [0(100), 1(80)]

i=2: 60 <= 80? Không -> dừng.
     span = 2-1 = 1.
     Push 2.                Stack: [0(100), 1(80), 2(60)]

i=3: 70 > 60 -> pop 2.
     70 <= 80? Không -> dừng.
     span = 3-1 = 2.
     Push 3.                Stack: [0(100), 1(80), 3(70)]

i=4: 60 <= 70? Không -> dừng.
     span = 4-3 = 1.
     Push 4.                Stack: [0(100), 1(80), 3(70), 4(60)]

i=5: 75 > 60 -> pop 4.
     75 > 70 -> pop 3.
     75 <= 80? Không -> dừng.
     span = 5-1 = 4.
     Push 5.                Stack: [0(100), 1(80), 5(75)]

i=6: 85 > 75 -> pop 5.
     85 > 80 -> pop 1.
     85 <= 100? Không -> dừng.
     span = 6-0 = 6.
     Push 6.                Stack: [0(100), 6(85)]

Kết quả: [1, 1, 1, 2, 1, 4, 6]
```

### Code

```rust
pub fn stock_span(prices: &[i32]) -> Vec<i32> {
    let n = prices.len();
    let mut result = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if prices[top] <= prices[i] {
                stack.pop();
            } else {
                break;
            }
        }
        result[i] = match stack.last() {
            Some(&prev) => (i - prev) as i32,
            None => (i + 1) as i32,
        };
        stack.push(i);
    }

    result
}
```

**Time:** O(n). **Space:** O(n).

---

## Trapping Rain Water

**Bài toán (LeetCode #42):** Cho mảng `height` biểu diễn độ cao các thanh. Tính lượng nước mưa có thể chứa giữa các thanh.

### Hình ảnh thực tế

Tưởng tượng bạn xây hàng rào bằng gạch có chiều cao khác nhau. Trời mưa xong, nước đọng lại giữa các thanh cao. Bạn cần tính tổng lượng nước đọng.

```
height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]

Nhìn từ bên cạnh:

              #
      # ~ ~ ~ # #
  # ~ # # ~ # # # # ~ #
──────────────────────────
  0 1 0 2 1 0 1 3 2 1 2 1

~ = nước đọng
# = thanh gạch
```

### Ý tưởng dùng Monotonic Stack

Dùng monotonic **decreasing** stack (giảm dần). Khi gặp thanh cao hơn đỉnh stack, nước bị "kẹp" giữa thanh hiện tại và thanh dưới đỉnh stack. Ta tính lượng nước theo từng "lớp ngang".

```
Nước được tính theo lớp ngang, không theo cột dọc:

      #                    #
  # ~ # ←── lớp trên   # ███ #
  # ~ # ←── lớp dưới   # ███ #
──────────            ──────────
  1 0 2               tính từng lớp khi pop
```

### ASCII walkthrough

```
height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
           0  1  2  3  4  5  6  7  8  9 10 11

Stack lưu index, giữ monotonic decreasing.

i=0: h=0, push.                    Stack: [0(0)]
i=1: h=1 > 0
     Pop 0 (h=0). Stack rỗng -> không có bờ trái -> nước = 0.
     Push 1.                       Stack: [1(1)]
i=2: h=0 < 1, push.               Stack: [1(1), 2(0)]
i=3: h=2 > 0
     Pop 2 (đáy h=0). Bờ trái = index 1 (h=1).
       w = 3-1-1 = 1
       water_h = min(1, 2) - 0 = 1
       water += 1*1 = 1.           Stack: [1(1)]
     h=2 > 1
     Pop 1 (đáy h=1). Stack rỗng -> không có bờ trái -> nước = 0.
     Push 3.                       Stack: [3(2)]
i=4: h=1 < 2, push.               Stack: [3(2), 4(1)]
i=5: h=0 < 1, push.               Stack: [3(2), 4(1), 5(0)]
i=6: h=1 > 0
     Pop 5 (đáy h=0). Bờ trái = index 4 (h=1).
       w = 6-4-1 = 1
       water_h = min(1, 1) - 0 = 1
       water += 1.  total=2.       Stack: [3(2), 4(1)]
     h=1 >= 1? Không strictly > -> dừng.
     Push 6.                       Stack: [3(2), 4(1), 6(1)]
i=7: h=3 > 1
     Pop 6 (đáy h=1). Bờ trái = index 4 (h=1).
       w = 7-4-1 = 2
       water_h = min(1, 3) - 1 = 0
       water += 0.                 Stack: [3(2), 4(1)]
     Pop 4 (đáy h=1). Bờ trái = index 3 (h=2).
       w = 7-3-1 = 3
       water_h = min(2, 3) - 1 = 1
       water += 3.  total=5.       Stack: [3(2)]
     Pop 3 (đáy h=2). Stack rỗng -> nước = 0.
     Push 7.                       Stack: [7(3)]
... (tiếp tục tương tự)

Tổng nước = 6.
```

### Code

```rust
pub fn trap_rain_water(height: &[i32]) -> i64 {
    let mut stack: Vec<usize> = Vec::new();
    let mut water: i64 = 0;

    for i in 0..height.len() {
        while let Some(&top) = stack.last() {
            if height[top] < height[i] {
                stack.pop();
                // Cần bờ trái để kẹp nước
                if let Some(&left) = stack.last() {
                    let w = (i - left - 1) as i64;
                    let h = (height[left].min(height[i]) - height[top]) as i64;
                    water += w * h;
                }
            } else {
                break;
            }
        }
        stack.push(i);
    }

    water
}
```

**Time:** O(n). **Space:** O(n).

> Bài này cũng giải được bằng Two Pointers (O(1) space) hoặc Prefix Max (Phần 6, chương 9). Monotonic Stack là cách "tự nhiên nhất" nếu bạn đã quen pattern -- tính nước theo lớp ngang khi pop.

---

## Pitfalls — Những cái bẫy hay gặp

❌ **Nhầm Increasing vs Decreasing**
```rust
// Muốn tìm next GREATER element
// Nhầm: dùng decreasing stack -> tìm next SMALLER
while nums[top] < nums[i] { ... }  // increasing -> next greater  ✓
while nums[top] > nums[i] { ... }  // decreasing -> next smaller
```
✅ **Nhớ rule:**
```
Tìm next GREATER  -> Monotonic INCREASING stack (nhỏ->lớn)
                     Pop khi phần tử mới LỚN HƠN đỉnh
Tìm next SMALLER  -> Monotonic DECREASING stack (lớn->nhỏ)
                     Pop khi phần tử mới NHỎ HƠN đỉnh
```
💡 Nghe ngược đời? Nghĩ thế này: stack increasing giữ các phần tử nhỏ. Phần tử lớn đến, "đè" các phần tử nhỏ -> pop -> tìm được greater.

---

❌ **Push value thay vì index**
```rust
stack.push(nums[i]);          // chỉ có giá trị, mất vị trí!
// Sau này muốn tính khoảng cách? Không được.
// result[???] = i - ???;     // không biết index của phần tử bị pop
```
✅ **Luôn push index**
```rust
stack.push(i);                // có index -> có cả vị trí lẫn giá trị
// nums[top] cho giá trị
// top cho vị trí
// i - top cho khoảng cách
```
💡 Index cho bạn mọi thứ: `nums[index]` = giá trị, `i - index` = khoảng cách. Value chỉ cho giá trị thôi. Đây là lý do mọi bài Monotonic Stack đều dùng `Vec<usize>` cho stack.

---

❌ **Quên check stack rỗng trước khi tính khoảng cách**
```rust
stack.pop();
let left = stack.last().unwrap();  // PANIC nếu stack rỗng!
let width = i - left - 1;
```
✅ **Luôn xử lý trường hợp stack rỗng**
```rust
stack.pop();
let width = match stack.last() {
    Some(&left) => i - left - 1,  // có bờ trái
    None => i,                     // không có bờ trái -> width = i
};
```
💡 Stack rỗng nghĩa là không có "bờ trái" nào chặn. Phần tử vừa pop có thể mở rộng tới tận đầu mảng. Bài Largest Rectangle in Histogram hay gặp bug này nhất.

---

❌ **Quên dọn stack sau vòng lặp**
```rust
for i in 0..n {
    // ... push/pop ...
}
// Quên xử lý các phần tử còn lại trong stack!
```
✅ **Dùng sentinel hoặc xử lý stack còn lại**
```rust
// Cách 1: Thêm sentinel (Largest Rectangle)
for i in 0..=n {  // <= n, thêm phần tử ảo h=0 ở cuối
    let cur = if i < n { heights[i] } else { 0 };
    // ...
}

// Cách 2: Phần tử còn lại = không có next greater (Next Greater Element)
// result đã được init là None/0 -> tự động đúng
```
💡 Sentinel (lính gác) là trick kinh điển: thêm phần tử ảo ở cuối để ép mọi phần tử trong stack phải pop ra.

---

## Khi nào dùng Monotonic Stack?

| Dấu hiệu trong đề bài | Monotonic Stack? | Ví dụ |
|------------------------|------------------|-------|
| "Next **greater/smaller** element" | Chắc chắn | LeetCode #496, #503 |
| "Bao nhiêu ngày **chờ** đến khi..." | Chắc chắn | Daily Temperatures #739 |
| "**Span** / bao nhiêu phần tử liên tiếp..." | Chắc chắn | Stock Span #901 |
| "**Largest rectangle** / diện tích lớn nhất" | Chắc chắn | Histogram #84, Maximal Rectangle #85 |
| "**Trapping** rain water / nước bị kẹp" | Rất phù hợp | #42 (cũng giải bằng two pointers) |
| "Previous greater/smaller" | Có (duyệt ngược hoặc nhìn stack) | |
| Cần so sánh phần tử với **hàng xóm** theo 1 chiều | Có thể | |
| Cần tìm min/max trong **sliding window** | Dùng Monotonic **Deque** (Phần 2, chương 5) | #239 |

**Quy tắc ngón tay cái:** Nếu bài toán yêu cầu tìm phần tử "gần nhất" thỏa điều kiện lớn hơn/nhỏ hơn theo 1 hướng, nghĩ đến Monotonic Stack.

---

## Bảng độ phức tạp

| Bài toán | Time | Space | Ghi chú |
|----------|------|-------|---------|
| Next Greater Element | O(n) | O(n) | Monotonic increasing stack |
| Next Smaller Element | O(n) | O(n) | Monotonic decreasing stack |
| Daily Temperatures | O(n) | O(n) | Biến thể Next Greater |
| Largest Rectangle in Histogram | O(n) | O(n) | Thêm sentinel h=0 cuối |
| Stock Span | O(n) | O(n) | Nhìn ngược về trái |
| Trapping Rain Water | O(n) | O(n) | Tính nước theo lớp ngang khi pop |

**Pattern chung:**
1. Duyệt mảng, push **index** vào stack.
2. Khi phần tử mới vi phạm tính đơn điệu -> pop và ghi nhận kết quả.
3. Mỗi phần tử push/pop tối đa 1 lần -> tổng O(n).

---

## Practice — Luyện tập

| # | Bài | Độ khó | Gợi ý |
|---|-----|--------|-------|
| 496 | [Next Greater Element I](https://leetcode.com/problems/next-greater-element-i/) | Easy | Dùng HashMap + Monotonic Stack trên nums2 |
| 739 | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/) | Medium | Next Greater Element, lưu khoảng cách |
| 901 | [Online Stock Span](https://leetcode.com/problems/online-stock-span/) | Medium | Monotonic decreasing, tính span khi push |
| 84 | [Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/) | Hard | Sentinel trick, tính area khi pop |
| 42 | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/) | Hard | Stack hoặc Two Pointers, tính nước lớp ngang |
| 85 | [Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/) | Hard | Mỗi row = histogram, gọi #84 |
| 503 | [Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii/) | Medium | Mảng vòng: duyệt 2*n, dùng i % n |

> Gợi ý thứ tự: 496 -> 739 -> 901 -> 84 -> 42 -> 503 -> 85.

---

## Rust Ecosystem

Trong Rust standard library, `Vec<T>` đã đủ để implement monotonic stack. Không cần crate ngoài. Pattern `while let Some(&top) = stack.last()` + `stack.pop()` là idiomatic Rust -- an toàn, rõ ràng, không panic.

Nếu bạn dùng crate `monotonic_stack` trên crates.io, hãy cẩn thận -- hầu hết các crate này chỉ wrap lại `Vec` mà thôi. Tự viết sẽ giúp bạn hiểu sâu hơn và linh hoạt hơn khi cần customize (ví dụ: strictly increasing vs non-strictly increasing).

```rust
// cargo test --lib monotonic_stack
```

---

---

[← Bit Manipulation](./01-bit-manipulation.md) | [Intervals →](./03-intervals.md)
