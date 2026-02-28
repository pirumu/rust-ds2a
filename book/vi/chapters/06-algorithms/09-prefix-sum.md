# Prefix Sum

> 💡 **Đừng lo lắng:** Nếu bạn hiểu phép cộng và phép trừ, bạn hiểu được prefix sum. Nghiêm túc đấy. Không cần toán cao siêu. Không cần thuật toán phức tạp. Chỉ cần: cộng dồn từ trái sang phải, rồi khi cần "tổng đoạn nào", lấy hai số trừ nhau. Vậy thôi. Chương này dài vì mình trình bày kỹ, không phải vì nó khó. Đọc từ từ, chạy từng ví dụ trong đầu, bạn sẽ thấy "ủa, có vậy thôi hả?"

---

## Bridge: Từ Array đến Prefix Sum

Ở [Chương 1 - Arrays](../01-fundamentals/02-arrays.md), bạn đã biết array cho phép truy cập O(1) theo index. Nhưng nếu ai đó hỏi "tổng từ index 3 đến index 7", bạn phải cộng 5 phần tử -- O(n).

Nếu hỏi 1 lần thì không sao. Nhưng nếu hỏi 10.000 lần trên cùng mảng? 10.000 x O(n) = đau.

**Prefix Sum** giải quyết vấn đề này: **precompute** (tính trước) một mảng tổng tích lũy, sau đó mỗi truy vấn chỉ mất O(1).

```
                Array thuần         Array + Prefix Sum
Xây dựng:      không cần            O(n) một lần
Mỗi truy vấn:  O(n)                 O(1)
Q truy vấn:     O(n * Q)             O(n + Q)
```

Đây là pattern cực kỳ phổ biến trong thuật toán: **đánh đổi thời gian xây dựng** để **tăng tốc truy vấn**. Bạn sẽ gặp pattern này lại ở Segment Tree, Sparse Table, và nhiều chỗ khác.

---

## Đây là gì?

Tưởng tượng bạn là thu ngân ở siêu thị. Trên kệ hàng có 100 sản phẩm xếp thành một hàng, mỗi sản phẩm có giá khác nhau. Khách hàng liên tục hỏi: "Tổng giá từ sản phẩm thứ 20 đến thứ 50 là bao nhiêu?"

Cách **chậm**: mỗi lần khách hỏi, bạn cộng từ sản phẩm 20 đến 50. Nếu 1000 khách hỏi, bạn cộng 1000 lần.

Cách **nhanh**: bạn chuẩn bị sẵn một bảng ghi **tổng tích lũy** -- "từ đầu kệ đến vị trí i, tổng giá là bao nhiêu". Khi khách hỏi "từ 20 đến 50", bạn chỉ cần **lấy tổng đến 50 trừ tổng đến 19**. Một phép trừ, xong!

Đây là kỹ thuật **Prefix Sum** (tổng tiền tố). Bạn đã học arrays ở [Phần 1](../01-fundamentals/02-arrays.md). Giờ mình sẽ học cách tiền xử lý mảng để trả lời câu hỏi "tổng từ i đến j" trong O(1).

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

## Pitfalls — Những cái bẫy hay gặp

### Pitfall 1: Off-by-one trong công thức

❌ **Sai:**

```rust
// Nhầm: dùng prefix[right] - prefix[left]
fn range_sum_wrong(prefix: &[i64], left: usize, right: usize) -> i64 {
    prefix[right] - prefix[left]   // THIẾU arr[right]!
}
```

✅ **Đúng:**

```rust
fn range_sum(prefix: &[i64], left: usize, right: usize) -> i64 {
    prefix[right + 1] - prefix[left]
}
```

💡 **Tại sao:** `prefix[right]` chỉ tính đến `arr[right - 1]`, chưa bao gồm `arr[right]`. Cần `prefix[right + 1]` mới bao gồm phần tử cuối.

**Mẹo nhớ:** Prefix sum có `n+1` phần tử. `P[i]` = tổng của `i` phần tử đầu tiên. Muốn bao gồm `arr[right]` thì cần `P[right + 1]` vì đó là tổng của `right + 1` phần tử đầu tiên.

### Pitfall 2: Quên P\[0\] = 0

❌ **Sai:**

```rust
// Bắt đầu từ arr[0] thay vì 0
let mut prefix = vec![arr[0]];
for i in 1..arr.len() {
    prefix.push(prefix[i - 1] + arr[i]);
}
// prefix = [2, 6, 7, 10, 15]  <-- thiếu số 0 ở đầu!
// range_sum(0, 2) = prefix[3] - prefix[0] = 10 - 2 = 8  <-- SAI (đáp án đúng: 7)
```

✅ **Đúng:**

```rust
let mut prefix = vec![0];  // BẮT ĐẦU BẰNG 0
for &val in arr {
    prefix.push(prefix.last().unwrap() + val);
}
// prefix = [0, 2, 6, 7, 10, 15]
// range_sum(0, 2) = prefix[3] - prefix[0] = 7 - 0 = 7  ✓
```

💡 **Tại sao:** `P[0] = 0` đại diện cho "chưa cộng phần tử nào". Nếu thiếu nó, bạn không thể tính tổng bắt đầu từ index 0 đúng cách.

---

## Subarray Sum = k (HashMap trick)

Bài toán: đếm có bao nhiêu subarray (mảng con liên tiếp) có tổng bằng `target`.

### Ý tưởng

Nếu duyệt brute force tất cả cặp (i, j), mất O(n^2). Nhưng nếu dùng prefix sum + HashMap, chỉ cần O(n)!

Gọi `current_sum` = prefix sum tại vị trí hiện tại. Nếu tồn tại prefix sum trước đó bằng `current_sum - target`, thì đoạn giữa hai prefix sum đó có tổng đúng bằng `target`.

Nhớ [HashMap ở Chương 4](../04-hashing/01-hash-map.md) không? Đây là lúc nó tỏa sáng -- tra cứu O(1) giúp bài toán chạy nhanh gấp bội.

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
  [2, 1]        (index 3..4) -- current_sum=5, tìm 5-3=2 trong map,
                               2 xuất hiện tại index 2, nên subarray là index 3..4
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

## Prefix XOR -- Biến thể hay

### Đây là gì?

Prefix Sum cộng dồn. **Prefix XOR** thì XOR dồn. Ý tưởng y hệt, chỉ thay phép `+` bằng phép `^` (XOR).

Tại sao XOR lại hữu ích? Vì XOR có tính chất đặc biệt:
- `a ^ a = 0` (XOR chính mình = 0)
- `a ^ 0 = a` (XOR với 0 = giữ nguyên)
- XOR có tính kết hợp và giao hoán

Nên nếu `prefix_xor[right+1] ^ prefix_xor[left]`, ta được XOR của đoạn `arr[left..=right]` -- giống hệt prefix sum nhưng thay trừ bằng XOR!

### Ví dụ: Tìm subarray có XOR = target

```
arr = [4, 2, 2, 6, 4], target = 6

Tính prefix XOR:
PX[0] = 0
PX[1] = 0 ^ 4 = 4
PX[2] = 4 ^ 2 = 6
PX[3] = 6 ^ 2 = 4
PX[4] = 4 ^ 6 = 2
PX[5] = 2 ^ 4 = 6

Muốn tìm subarray XOR = 6:
  PX[right+1] ^ PX[left] = target
  => PX[left] = PX[right+1] ^ target

Duyệt qua, dùng HashMap giống subarray sum:

index | PX  | PX ^ target | HashMap       | count
------|-----|-------------|---------------|------
  -   |  0  |     -       | {0: 1}        | 0
  0   |  4  | 4 ^ 6 = 2  | {0:1, 4:1}    | 0
  1   |  6  | 6 ^ 6 = 0  | {0:1, 4:1, 6:1}| 1    <-- subarray [4,2]
  2   |  4  | 4 ^ 6 = 2  | {..., 4:2}    | 1
  3   |  2  | 2 ^ 6 = 4  | {..., 2:1}    | 3    <-- 4 xuất hiện 2 lần!
  4   |  6  | 6 ^ 6 = 0  | {..., 6:2}    | 4    <-- 0 xuất hiện 1 lần

Đáp án: 4 subarrays có XOR = 6
```

### Code

```rust
use std::collections::HashMap;

pub fn subarray_xor_count(arr: &[u64], target: u64) -> usize {
    let mut count: usize = 0;
    let mut current_xor: u64 = 0;
    let mut map: HashMap<u64, usize> = HashMap::new();
    map.insert(0, 1);

    for &val in arr {
        current_xor ^= val;
        // Tìm prefix XOR trước đó sao cho current_xor ^ prev = target
        // => prev = current_xor ^ target
        if let Some(&c) = map.get(&(current_xor ^ target)) {
            count += c;
        }
        *map.entry(current_xor).or_insert(0) += 1;
    }
    count
}
```

So sánh nhỏ cho dễ nhớ:

```
Prefix Sum  + HashMap:  tìm subarray tổng = k     (trừ để "khử")
Prefix XOR  + HashMap:  tìm subarray XOR  = k     (XOR để "khử")
```

Cả hai đều dựa trên cùng một insight: phép toán nghịch đảo triệt tiêu phần "thừa".

---

## Difference Array -- Prefix Sum ngược

### Đây là gì?

Prefix Sum giúp **truy vấn** nhanh. **Difference Array** (mảng hiệu) giúp **cập nhật** nhanh.

Tưởng tượng bạn là giáo viên, có 30 học sinh. Bạn muốn:
- Cộng 5 điểm cho học sinh 3 đến 10
- Cộng 3 điểm cho học sinh 7 đến 20
- Cộng 2 điểm cho học sinh 1 đến 15

Cách chậm: mỗi lần duyệt qua tất cả học sinh trong đoạn, cộng điểm. Nếu có Q lệnh cập nhật, mất O(Q * n).

Cách nhanh: dùng difference array -- mỗi lần cập nhật chỉ mất O(1)!

### Cách hoạt động

Mảng `diff` ban đầu toàn 0. Muốn cộng `val` vào đoạn `[left, right]`:

```
diff[left]     += val    (bắt đầu cộng từ đây)
diff[right+1]  -= val    (ngừng cộng sau đây)
```

Sau khi xong tất cả updates, chạy prefix sum trên `diff` để ra mảng kết quả!

### Ví dụ

```
n = 6, ban đầu arr = [0, 0, 0, 0, 0, 0]

Update 1: cộng 3 vào đoạn [1, 4]
  diff = [0, +3, 0, 0, 0, -3]

Update 2: cộng 2 vào đoạn [2, 5]
  diff = [0, +3, +2, 0, 0, -3+2] = [0, 3, 2, 0, 0, -1]
  (chú ý: right+1 = 6 nằm ngoài mảng nên bỏ qua)

Chạy prefix sum trên diff:
  result[0] = 0
  result[1] = 0 + 3 = 3
  result[2] = 3 + 2 = 5
  result[3] = 5 + 0 = 5
  result[4] = 5 + 0 = 5
  result[5] = 5 + (-1) = 4

result = [0, 3, 5, 5, 5, 4]
```

Kiểm tra bằng tay:
```
Index:      0  1  2  3  4  5
Update 1:   0  3  3  3  3  0    (+3 cho [1,4])
Update 2:   0  0  2  2  2  2    (+2 cho [2,5])
Tổng:       0  3  5  5  5  2    Hmm... chờ đã
```

Ủa, sao kết quả không khớp? Kiểm tra lại Update 2: cộng 2 vào đoạn [2, 5]:
- `diff[2] += 2` OK
- `diff[6] -= 2` --> index 6 nằm ngoài mảng kích thước 6, nên ta cần mảng diff kích thước `n+1`!

```
diff kích thước n+1 = 7:
  diff = [0, 3, 2, 0, 0, -3, -2]

Prefix sum:
  [0, 3, 5, 5, 5, 2, 0]

Lấy 6 phần tử đầu: [0, 3, 5, 5, 5, 2]  ✓
```

### Code

```rust
pub fn apply_range_updates(n: usize, updates: &[(usize, usize, i64)]) -> Vec<i64> {
    let mut diff = vec![0i64; n + 1];

    for &(left, right, val) in updates {
        diff[left] += val;
        if right + 1 <= n {
            diff[right + 1] -= val;
        }
    }

    // Chạy prefix sum trên diff
    let mut result = Vec::with_capacity(n);
    let mut running = 0i64;
    for i in 0..n {
        running += diff[i];
        result.push(running);
    }
    result
}
```

Dùng thử:

```rust
let updates = vec![
    (1, 4, 3),   // cộng 3 vào [1, 4]
    (2, 5, 2),   // cộng 2 vào [2, 5]
];
let result = apply_range_updates(6, &updates);
// [0, 3, 5, 5, 5, 2]
```

- **Mỗi update:** O(1)
- **Q updates:** O(Q)
- **Build kết quả:** O(n)
- **Tổng:** O(n + Q) thay vì O(n * Q)

### Mối liên hệ Prefix Sum ↔ Difference Array

Hai kỹ thuật này là **nghịch đảo** của nhau:

```
Mảng gốc ──prefix_sum──> Prefix Sum Array
                              |
Mảng gốc <──prefix_sum── Difference Array
```

- Prefix Sum của Difference Array = mảng gốc
- Difference Array của Prefix Sum = mảng gốc

Giống như đạo hàm và tích phân trong toán! Nếu bạn biết calculus thì thấy quen. Nếu không biết cũng không sao, chỉ cần nhớ: **prefix sum và difference array khử nhau**.

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

### Pitfall 3: Inclusion-Exclusion sai trong 2D

Đây là lỗi kinh điển khi viết 2D prefix sum. Hãy nhìn kỹ:

❌ **Sai:** Quên cộng lại góc trên-trái

```rust
// Thiếu + prefix[r1][c1]
fn range_sum_2d_wrong(prefix: &[Vec<i64>], r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
    prefix[r2+1][c2+1] - prefix[r1][c2+1] - prefix[r2+1][c1]
    // Vùng góc trên-trái bị trừ HAI LẦN nhưng chưa cộng lại!
}
```

✅ **Đúng:**

```rust
fn range_sum_2d(prefix: &[Vec<i64>], r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
    prefix[r2+1][c2+1] - prefix[r1][c2+1] - prefix[r2+1][c1]
    + prefix[r1][c1]  // CỘNG LẠI vùng bị trừ 2 lần
}
```

💡 **Tại sao:** Vẽ ra hình sẽ thấy ngay. Khi trừ "phía trên" và "bên trái", vùng góc trên-trái nằm trong CẢ HAI vùng trừ, nên bị trừ 2 lần. Phải cộng lại 1 lần.

```
  +-------+-------+
  |  BỊ   | Trừ   |
  | TRỪ   | (trên)|
  | 2 LẦN |       |
  +-------+-------+
  | Trừ   | VÙNG  |
  | (trái)| CẦN   |
  +-------+-------+

Nên: TỔNG - TRÊN - TRÁI + GÓC = VÙNG CẦN
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

## Bảng tổng hợp: "Khi nào dùng gì?"

| Bạn cần... | Kỹ thuật | Time |
|-------------|----------|------|
| Tổng đoạn [l, r] nhiều lần | **1D Prefix Sum** | Build O(n), query O(1) |
| Đếm subarray tổng = k | **Prefix Sum + HashMap** | O(n) |
| Đếm subarray XOR = k | **Prefix XOR + HashMap** | O(n) |
| Tổng hình chữ nhật con 2D nhiều lần | **2D Prefix Sum** | Build O(m*n), query O(1) |
| Cộng/trừ hàng loạt vào đoạn [l, r] | **Difference Array** | Update O(1), build O(n) |
| Tổng đoạn + cập nhật giá trị | **Fenwick Tree / Segment Tree** (chương sau) | Update & query O(log n) |

**Quy tắc chọn:**
- Mảng **không đổi**, hỏi nhiều lần --> Prefix Sum
- Mảng bị **cập nhật hàng loạt theo đoạn** --> Difference Array
- Mảng bị **cập nhật từng phần tử** + hỏi tổng đoạn --> Fenwick Tree (ngoài phạm vi chương này)

---

## Bảng tổng hợp độ phức tạp

| Thao tác | Time | Space |
|----------|------|-------|
| Xây prefix sum 1D | O(n) | O(n) |
| Range sum 1D | O(1) | O(1) |
| Subarray sum = k (HashMap) | O(n) | O(n) |
| Subarray XOR = k (HashMap) | O(n) | O(n) |
| Xây prefix sum 2D | O(rows * cols) | O(rows * cols) |
| Range sum 2D | O(1) | O(1) |
| Range update (Difference Array) | O(1) mỗi update | O(n) |
| Build result từ Difference Array | O(n) | O(n) |

---

## Rust Ecosystem

### `std` -- Đã đủ dùng

Prefix sum đơn giản đến mức bạn không cần crate nào. Dùng `Vec<i64>` và `HashMap` từ standard library là xong.

Một vài tips Rust khi implement:

```rust
// Cách 1: Dùng scan (iterator style, rất Rust-idiomatic)
fn prefix_sum_iter(arr: &[i64]) -> Vec<i64> {
    std::iter::once(0)
        .chain(arr.iter().scan(0i64, |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect()
}

// Cách 2: Dùng windows() cho sliding prefix
// (hữu ích khi kết hợp với Sliding Window ở chương sau)
```

### `itertools` crate

Crate `itertools` có method `.scan()` mạnh hơn standard `Iterator::scan`, nhưng cho prefix sum thì standard library đã đủ.

### Overflow

Trong Rust, `i64` chứa được giá trị tới ~9.2 * 10^18. Nhưng nếu mảng có 10^5 phần tử, mỗi phần tử tới 10^9, prefix sum tối đa = 10^14 -- vẫn trong phạm vi `i64`. Nếu lo lắng, dùng `i128` hoặc kiểm tra overflow với `checked_add()`:

```rust
// Prefix sum an toàn -- panic nếu overflow
fn prefix_sum_checked(arr: &[i64]) -> Vec<i64> {
    let mut prefix = Vec::with_capacity(arr.len() + 1);
    prefix.push(0);
    for &val in arr {
        let last = *prefix.last().unwrap();
        prefix.push(last.checked_add(val).expect("prefix sum overflow!"));
    }
    prefix
}
```

### KaCrab integration

Trong project `rust-ds2a`, bạn có thể chạy:

```bash
cargo test prefix_sum    # Chạy tất cả test cho prefix sum
```

Xem file `src/prefix_sum.rs` để đọc implementation đầy đủ với tests.

---

## Practice — Luyện tập

### LeetCode #303: Range Sum Query - Immutable

Đúng y chang bài 1D prefix sum. Build prefix array trong constructor, trả lời query trong O(1).

```rust
struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(nums.len() + 1);
        prefix.push(0);
        for &val in &nums {
            prefix.push(prefix.last().unwrap() + val);
        }
        Self { prefix }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[right as usize + 1] - self.prefix[left as usize]
    }
}
```

### LeetCode #560: Subarray Sum Equals K

Chính là bài Subarray Sum = k ở trên. Copy paste code `subarray_sum_count`, chỉ cần đổi type sang `i32`.

### LeetCode #238: Product of Array Except Self

Bài này **không dùng prefix sum** theo nghĩa cộng, mà dùng **prefix product** -- cùng ý tưởng nhưng thay `+` bằng `*`.

Trick: tính `left_product[i]` = tích các phần tử bên trái i, và `right_product[i]` = tích các phần tử bên phải i. Kết quả = `left_product[i] * right_product[i]`.

```rust
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];

    // Left pass: result[i] = product of nums[0..i]
    let mut left = 1;
    for i in 0..n {
        result[i] = left;
        left *= nums[i];
    }

    // Right pass: multiply by product of nums[i+1..n]
    let mut right = 1;
    for i in (0..n).rev() {
        result[i] *= right;
        right *= nums[i];
    }

    result
}
```

Bài này hay ở chỗ: nó chứng minh **prefix idea** không chỉ giới hạn ở phép cộng. Bất kỳ phép toán nào có tính kết hợp (associative) đều có thể dùng prefix.

---

## Tiếp theo

Prefix Sum giúp bạn tính tổng đoạn O(1), nhưng cần O(n) space cho mảng prefix.

Ở [chương tiếp theo -- Sliding Window](./10-sliding-window.md), bạn sẽ học một kỹ thuật khác: thay vì tính trước tất cả, bạn **trượt một cửa sổ** qua mảng, mỗi bước chỉ thêm 1 phần tử mới và bỏ 1 phần tử cũ. Không cần mảng phụ, O(1) space!

Hai kỹ thuật bổ trợ nhau:
- **Prefix Sum**: trả lời bất kỳ range query nào, nhưng tốn O(n) space
- **Sliding Window**: chỉ xử lý các cửa sổ liên tiếp, nhưng O(1) space

Ở chương Two Pointers trước, bạn đã thấy cách di chuyển hai con trỏ. Sliding Window chính là "two pointers cùng hướng" -- cả hai di chuyển từ trái sang phải, tạo thành một "cửa sổ" trượt trên mảng.

---

## Tóm tắt

Prefix sum đơn giản nhưng cực kỳ mạnh. Nó biến truy vấn O(n) thành O(1) chỉ với một bước tiền xử lý!

Những gì bạn đã học trong chương này:
1. **1D Prefix Sum** -- tổng đoạn O(1)
2. **Subarray Sum = k** -- prefix sum + HashMap, O(n)
3. **Prefix XOR** -- cùng pattern, khác phép toán
4. **Difference Array** -- "prefix sum ngược", range update O(1)
5. **2D Prefix Sum** -- mở rộng ra ma trận, inclusion-exclusion
6. **Prefix Product** -- mở rộng sang phép nhân

Pattern chung: **precompute + tra cứu nhanh**. Gặp bài nào hỏi nhiều lần về "tổng/tích/XOR của đoạn", nghĩ ngay đến prefix!

---

[← Two Pointers](./08-two-pointers.md) | [Sliding Window →](./10-sliding-window.md)
