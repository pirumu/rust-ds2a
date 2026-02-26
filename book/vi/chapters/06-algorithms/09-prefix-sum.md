# Prefix Sum

## Đây là gì?

Tưởng tượng bạn là thu ngân ở siêu thị. Trên kệ hàng có 100 sản phẩm xếp thành một hàng, mỗi sản phẩm có giá khác nhau. Khách hàng liên tục hỏi: "Tổng giá từ sản phẩm thứ 20 đến thứ 50 là bao nhiêu?"

Cách **chậm**: mỗi lần khách hỏi, bạn cộng từ sản phẩm 20 đến 50. Nếu 1000 khách hỏi, bạn cộng 1000 lần.

Cách **nhanh**: bạn chuẩn bị sẵn một bảng ghi **tổng tích lũy** — "từ đầu kệ đến vị trí i, tổng giá là bao nhiêu". Khi khách hỏi "từ 20 đến 50", bạn chỉ cần **lấy tổng đến 50 trừ tổng đến 19**. Một phép trừ, xong!

Đây là kỹ thuật **Prefix Sum** (tổng tiền tố). Bạn đã học arrays ở Phần 1. Giờ mình sẽ học cách tiền xử lý mảng để trả lời câu hỏi "tổng từ i đến j" trong O(1).

---

## Cách hoạt động

### Xây dựng mảng prefix sum

Cho mảng gốc `arr`:

```
arr =    [2, 4, 1, 3, 5]
index:    0  1  2  3  4
```

Mảng prefix sum `P` có **n+1 phần tử**, bắt đầu bằng 0:

```
P[0] = 0
P[1] = P[0] + arr[0] = 0 + 2 = 2
P[2] = P[1] + arr[1] = 2 + 4 = 6
P[3] = P[2] + arr[2] = 6 + 1 = 7
P[4] = P[3] + arr[3] = 7 + 3 = 10
P[5] = P[4] + arr[4] = 10 + 5 = 15

P = [0, 2, 6, 7, 10, 15]
```

Mỗi `P[i]` = tổng của `arr[0..i]` (từ đầu đến trước vị trí i).

### Truy vấn tổng đoạn (Range Sum Query)

Muốn tính tổng từ index `left` đến `right` (bao gồm cả hai)?

```
range_sum(left, right) = P[right + 1] - P[left]
```

Ví dụ: tổng từ index 1 đến 3 = `arr[1] + arr[2] + arr[3]` = 4 + 1 + 3 = **8**

```
arr = [2, 4, 1, 3, 5]
           ^  ^  ^
           1  2  3       <-- đoạn cần tính

P   = [0, 2, 6, 7, 10, 15]
           ^          ^
          P[1]       P[4]

range_sum(1, 3) = P[4] - P[1] = 10 - 2 = 8  ✓
```

**Tại sao đúng?** Vì `P[4]` = tổng từ đầu đến index 3, và `P[1]` = tổng từ đầu đến index 0. Trừ đi phần "thừa" bên trái, còn lại đúng đoạn cần tính.

```
P[4] = arr[0] + arr[1] + arr[2] + arr[3]
P[1] = arr[0]

P[4] - P[1] = arr[1] + arr[2] + arr[3] = 4 + 1 + 3 = 8
```

---

## 1D Prefix Sum — Code Rust

```rust
pub fn prefix_sum(arr: &[i64]) -> Vec<i64> {
    let mut prefix = Vec::with_capacity(arr.len() + 1);
    prefix.push(0);
    for &val in arr {
        let last = *prefix.last().unwrap();
        prefix.push(last + val);
    }
    prefix
}

pub fn range_sum(prefix: &[i64], left: usize, right: usize) -> i64 {
    prefix[right + 1] - prefix[left]
}
```

Dùng thử:

```rust
let arr = [2, 4, 1, 3, 5];
let p = prefix_sum(&arr);          // [0, 2, 6, 7, 10, 15]
let total = range_sum(&p, 1, 3);   // 8
```

- **Xây dựng:** O(n) time, O(n) space
- **Truy vấn:** O(1) time, O(1) space

---

## Subarray Sum = k (HashMap trick)

Bài toán: đếm có bao nhiêu subarray (mảng con liên tiếp) có tổng bằng `target`.

### Ý tưởng

Nếu duyệt brute force tất cả cặp (i, j), mất O(n^2). Nhưng nếu dùng prefix sum + HashMap, chỉ cần O(n)!

Gọi `current_sum` = prefix sum tại vị trí hiện tại. Nếu tồn tại prefix sum trước đó bằng `current_sum - target`, thì đoạn giữa hai prefix sum đó có tổng đúng bằng `target`.

### Ví dụ

```
arr = [1, 2, -1, 2, 1], target = 3

Bước qua từng phần tử, theo dõi current_sum và HashMap:

index | val | current_sum | current_sum - target | HashMap         | count
------|-----|-------------|---------------------|-----------------|------
 -    |  -  |     0       |         -           | {0: 1}          | 0
  0   |  1  |     1       |     1 - 3 = -2      | {0:1, 1:1}      | 0
  1   |  2  |     3       |     3 - 3 = 0       | {0:1, 1:1, 3:1} | 1
  2   | -1  |     2       |     2 - 3 = -1      | {..., 2:1}      | 1
  3   |  2  |     4       |     4 - 3 = 1       | {..., 4:1}      | 2
  4   |  1  |     5       |     5 - 3 = 2       | {..., 5:1}      | 3

Đáp án: 3 subarrays có tổng = 3
  [1, 2]        (index 0..1)
  [2, -1, 2]    (index 1..3)
  [-1, 2, 1]    (index 2..4) -- hoặc chờ đã, kiểm tra lại:
                 Ah đúng rồi: current_sum=5, tìm 5-3=2 trong map
                 2 xuất hiện tại index 2, nên subarray là index 3..4 = [2, 1]
```

### Code

```rust
use std::collections::HashMap;

pub fn subarray_sum_count(arr: &[i64], target: i64) -> usize {
    let mut count: usize = 0;
    let mut current_sum: i64 = 0;
    let mut map: HashMap<i64, usize> = HashMap::new();
    map.insert(0, 1); // empty prefix — quan trọng!

    for &val in arr {
        current_sum += val;
        if let Some(&c) = map.get(&(current_sum - target)) {
            count += c;
        }
        *map.entry(current_sum).or_insert(0) += 1;
    }
    count
}
```

**Tại sao khởi tạo `map.insert(0, 1)`?** Vì nếu `current_sum` đúng bằng `target` ngay tại vị trí nào đó, thì `current_sum - target = 0`, và ta cần biết rằng prefix sum = 0 đã tồn tại (là "trước khi bắt đầu mảng").

- **Time:** O(n)
- **Space:** O(n)

---

## 2D Prefix Sum

### Bài toán

Cho ma trận 2D, trả lời nhanh câu hỏi: "Tổng tất cả phần tử trong hình chữ nhật con từ (r1, c1) đến (r2, c2)?"

### Xây dựng

Giống 1D nhưng mở rộng ra 2 chiều. Mảng prefix 2D có kích thước `(rows+1) x (cols+1)`:

```
matrix:                   prefix (có viền 0):
  1  2  3                 0  0  0  0
  4  5  6                 0  1  3  6
  7  8  9                 0  5  12 21
                          0  12 27 45
```

Công thức xây dựng:

```
prefix[r][c] = matrix[r-1][c-1]
             + prefix[r-1][c]     (phía trên)
             + prefix[r][c-1]     (bên trái)
             - prefix[r-1][c-1]   (bị cộng 2 lần, trừ đi)
```

Minh họa vùng cộng:

```
Tính prefix[2][2] = matrix[1][1] + phía trên + bên trái - góc

  +-----+-----+
  |  A  | A+B |    A = prefix[1][1] = 1
  +-----+-----+    B = prefix[1][2] - prefix[1][1] = 3 - 1 = 2
  | A+C |  ?  |    C = prefix[2][1] - prefix[1][1] = 5 - 1 = 4
  +-----+-----+

  ? = matrix[1][1] + prefix[1][2] + prefix[2][1] - prefix[1][1]
    = 5 + 3 + 5 - 1 = 12

Kiểm tra: 1 + 2 + 4 + 5 = 12  ✓
```

### Truy vấn hình chữ nhật con

```
range_sum_2d(r1, c1, r2, c2) = prefix[r2+1][c2+1]
                              - prefix[r1][c2+1]     (phía trên)
                              - prefix[r2+1][c1]      (bên trái)
                              + prefix[r1][c1]         (trừ 2 lần, cộng lại)
```

Ví dụ: tổng hình chữ nhật từ (1,1) đến (2,2):

```
matrix:
  1  2  3
  4 [5  6]    <-- vùng cần tính
  7 [8  9]

range_sum_2d(1, 1, 2, 2) = prefix[3][3] - prefix[1][3] - prefix[3][1] + prefix[1][1]
                          = 45 - 6 - 12 + 1
                          = 28

Kiểm tra: 5 + 6 + 8 + 9 = 28  ✓
```

### Code

```rust
pub fn prefix_sum_2d(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    if matrix.is_empty() {
        return vec![vec![0]];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut prefix = vec![vec![0i64; cols + 1]; rows + 1];

    for r in 1..=rows {
        for c in 1..=cols {
            prefix[r][c] = matrix[r - 1][c - 1]
                + prefix[r - 1][c]
                + prefix[r][c - 1]
                - prefix[r - 1][c - 1];
        }
    }
    prefix
}

pub fn range_sum_2d(
    prefix: &[Vec<i64>],
    r1: usize, c1: usize,
    r2: usize, c2: usize,
) -> i64 {
    prefix[r2 + 1][c2 + 1]
        - prefix[r1][c2 + 1]
        - prefix[r2 + 1][c1]
        + prefix[r1][c1]
}
```

- **Xây dựng:** O(rows * cols) time, O(rows * cols) space
- **Truy vấn:** O(1)

---

## Bảng tổng hợp độ phức tạp

| Thao tác | Time | Space |
|----------|------|-------|
| Xây prefix sum 1D | O(n) | O(n) |
| Range sum 1D | O(1) | O(1) |
| Subarray sum = k (HashMap) | O(n) | O(n) |
| Xây prefix sum 2D | O(rows * cols) | O(rows * cols) |
| Range sum 2D | O(1) | O(1) |

---

## Khi nào dùng Prefix Sum?

- Khi có **nhiều truy vấn** "tổng từ i đến j" trên cùng một mảng
- Khi cần đếm subarray có tổng bằng k (kết hợp HashMap)
- Khi làm việc với ma trận 2D và cần tổng hình chữ nhật con
- Bất kỳ bài toán nào liên quan đến **tổng tích lũy** hoặc **hiệu prefix sum**

Prefix sum đơn giản nhưng cực kỳ mạnh. Nó biến truy vấn O(n) thành O(1) chỉ với một bước tiền xử lý!
