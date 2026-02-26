# Monotonic Stack

Bạn đã học Stack ở Phần 2. Giờ mình sẽ dùng stack theo cách đặc biệt — giữ các phần tử luôn tăng (hoặc giảm) trong stack.

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

Hai loại:

| Loại | Trong stack giữ | Dùng khi |
|------|----------------|----------|
| **Monotonic Increasing** | Nhỏ -> lớn (đáy -> đỉnh) | Tìm next greater element |
| **Monotonic Decreasing** | Lớn -> nhỏ (đáy -> đỉnh) | Tìm next smaller element |

---

## Tại sao cần Monotonic Stack?

Nếu dùng 2 vòng for lồng nhau: O(n^2). Với mảng 1 triệu phần tử, quá chậm!

Monotonic Stack giải quyết trong **O(n)** vì mỗi phần tử chỉ được push và pop tối đa 1 lần.

```
Brute force:     O(n^2)     ← 2 vòng for
Monotonic Stack:  O(n)      ← mỗi phần tử push 1 lần, pop 1 lần
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

**Bài toán:** Cho giá cổ phiếu mỗi ngày, tìm **span** — số ngày liên tiếp (kể cả hôm nay) mà giá <= giá hôm nay.

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

## Bảng độ phức tạp

| Bài toán | Time | Space | Ghi chú |
|----------|------|-------|---------|
| Next Greater Element | O(n) | O(n) | Monotonic increasing stack |
| Next Smaller Element | O(n) | O(n) | Monotonic decreasing stack |
| Daily Temperatures | O(n) | O(n) | Biến thể Next Greater |
| Largest Rectangle in Histogram | O(n) | O(n) | Thêm sentinel h=0 cuối |
| Stock Span | O(n) | O(n) | Nhìn ngược về trái |

**Pattern chung:**
1. Duyệt mảng, push index vào stack.
2. Khi phần tử mới vi phạm tính đơn điệu -> pop và ghi nhận kết quả.
3. Mỗi phần tử push/pop tối đa 1 lần -> tổng O(n).

```rust
// cargo test --lib monotonic_stack
```
