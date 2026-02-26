# Bit Manipulation

## Đây là gì?

Hình dung bạn đứng trước bảng điều khiển có 8 công tắc đèn. Mỗi công tắc chỉ có 2 trạng thái: **bật** (1) hoặc **tắt** (0). Chỉ cần nhìn vào bảng, bạn biết ngay đèn nào sáng, đèn nào tối.

Máy tính lưu mọi thứ bằng **bit** — 0 và 1. Một số nguyên như `42` thực ra được lưu dưới dạng:

```
42 = 0b00101010
         │ │ │
         │ │ └── bit 1: bật
         │ └──── bit 3: bật
         └────── bit 5: bật
```

Hiểu cách thao tác trực tiếp với bit giúp bạn giải nhiều bài toán **cực nhanh** — nhanh hơn cả dùng phép nhân chia, vì CPU xử lý bit trong **một nhịp đồng hồ**.

### Tại sao cần học?

- Kiểm tra chẵn/lẻ, kiểm tra lũy thừa 2 — chỉ cần **một phép AND**
- Đếm bit, tìm phần tử duy nhất — trick kinh điển trong phỏng vấn
- Liệt kê tập con — dùng bitmask thay vì đệ quy
- Tiết kiệm bộ nhớ khi lưu trạng thái (flag, permission...)

---

## Các phép toán cơ bản

### AND (`&`) — Cả hai đều bật thì mới bật

Giống như cửa an ninh cần **cả thẻ VÀ mật khẩu** mới mở.

```
  1 0 1 1        a
& 1 1 0 1        b
--------
  1 0 0 1        a & b
```

| a | b | a & b |
|---|---|-------|
| 0 | 0 |   0   |
| 0 | 1 |   0   |
| 1 | 0 |   0   |
| 1 | 1 |   1   |

**Ứng dụng:** Kiểm tra bit cụ thể, xóa bit.

### OR (`|`) — Một trong hai bật là bật

Giống như đèn phòng có 2 công tắc — bật **một trong hai** là sáng.

```
  1 0 1 1        a
| 1 1 0 1        b
--------
  1 1 1 1        a | b
```

| a | b | a \| b |
|---|---|--------|
| 0 | 0 |   0    |
| 0 | 1 |   1    |
| 1 | 0 |   1    |
| 1 | 1 |   1    |

**Ứng dụng:** Bật bit (set bit).

### XOR (`^`) — Khác nhau thì bật

Giống **trò chơi "ngược lại"**: hai người giơ tay — nếu khác nhau thì được điểm.

```
  1 0 1 1        a
^ 1 1 0 1        b
--------
  0 1 1 0        a ^ b
```

| a | b | a ^ b |
|---|---|-------|
| 0 | 0 |   0   |
| 0 | 1 |   1   |
| 1 | 0 |   1   |
| 1 | 1 |   0   |

**Tính chất quan trọng:**
- `a ^ a = 0` (XOR với chính nó = 0)
- `a ^ 0 = a` (XOR với 0 = giữ nguyên)
- Giao hoán & kết hợp: thứ tự không quan trọng

**Ứng dụng:** Tìm phần tử duy nhất, đổi giá trị không cần biến tạm, toggle bit.

### NOT (`!`) — Đảo ngược tất cả

Bật thành tắt, tắt thành bật.

```
! 1 0 1 1
--------
  0 1 0 0
```

**Ứng dụng:** Tạo mask để xóa bit.

### Left Shift (`<<`) — Dịch trái, nhân 2

Đẩy tất cả bit sang trái, thêm 0 vào bên phải.

```
  0 0 1 0 1 0    = 10
       << 1
  0 1 0 1 0 0    = 20   (nhân 2!)
       << 1
  1 0 1 0 0 0    = 40   (nhân 2 lần nữa!)
```

### Right Shift (`>>`) — Dịch phải, chia 2

```
  0 1 0 1 0 0    = 20
       >> 1
  0 0 1 0 1 0    = 10   (chia 2!)
```

---

## Thao tác bit cơ bản

Bốn thao tác nền tảng mà mọi thứ khác đều xây trên đó:

```
Số n = 0b1010  (10 trong hệ 10)
Vị trí:  3210

┌──────────────┬───────────────────────┬──────────┐
│  Thao tác    │  Cách làm             │  Kết quả │
├──────────────┼───────────────────────┼──────────┤
│  get_bit(1)  │  (n >> 1) & 1         │  1 (bật) │
│  set_bit(0)  │  n | (1 << 0)         │  0b1011  │
│  clear_bit(1)│  n & !(1 << 1)        │  0b1000  │
│  toggle_bit(3)│ n ^ (1 << 3)         │  0b0010  │
└──────────────┴───────────────────────┴──────────┘
```

### Ví dụ chi tiết: set_bit

```
n   = 0b1010
pos = 0

mask = 1 << 0  = 0b0001

  1 0 1 0   n
| 0 0 0 1   mask
---------
  1 0 1 1   kết quả: bit 0 đã bật!
```

---

## Tricks hay

### 1. Kiểm tra lũy thừa 2 — `is_power_of_two`

Một số là lũy thừa 2 khi nó chỉ có **đúng 1 bit** được bật:

```
1   = 0b0001  ✓
2   = 0b0010  ✓
4   = 0b0100  ✓
8   = 0b1000  ✓

3   = 0b0011  ✗ (2 bit bật)
6   = 0b0110  ✗ (2 bit bật)
```

**Trick:** `n & (n - 1)` xóa bit thấp nhất. Nếu kết quả = 0 thì chỉ có 1 bit.

```
n     = 0b1000  (8)
n - 1 = 0b0111  (7)

  1 0 0 0
& 0 1 1 1
---------
  0 0 0 0  → bằng 0 → là lũy thừa 2!
```

```rust
pub fn is_power_of_two(n: u64) -> bool {
    n != 0 && (n & (n - 1)) == 0
}
```

### 2. Đếm số bit 1 — Brian Kernighan's Algorithm

Thay vì kiểm tra từng bit (32 hoặc 64 lần), ta xóa bit thấp nhất mỗi lần:

```
n = 0b1011_0100  (có 4 bit 1)

Bước 1: n & (n-1) = 0b1011_0100 & 0b1011_0011 = 0b1011_0000  (count=1)
Bước 2: n & (n-1) = 0b1011_0000 & 0b1010_1111 = 0b1010_0000  (count=2)
Bước 3: n & (n-1) = 0b1010_0000 & 0b1001_1111 = 0b1000_0000  (count=3)
Bước 4: n & (n-1) = 0b1000_0000 & 0b0111_1111 = 0b0000_0000  (count=4)

Chỉ lặp 4 lần thay vì 8!
```

```rust
pub fn count_ones(n: u64) -> u32 {
    let mut count = 0u32;
    let mut x = n;
    while x != 0 {
        x &= x - 1;
        count += 1;
    }
    count
}
```

### 3. Tìm phần tử duy nhất — XOR trick

Bài toán: mảng có n phần tử, mọi phần tử xuất hiện **2 lần** trừ **1 phần tử** xuất hiện 1 lần. Tìm nó.

Tính chất XOR: `a ^ a = 0` và `a ^ 0 = a`.

```
nums = [4, 1, 2, 1, 2]

XOR tất cả:  4 ^ 1 ^ 2 ^ 1 ^ 2
           = 4 ^ (1 ^ 1) ^ (2 ^ 2)
           = 4 ^ 0 ^ 0
           = 4           ← đáp án!
```

```rust
pub fn single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}
```

### 4. Đổi 2 giá trị không cần biến tạm — XOR swap

```
a = 5, b = 3

a ^= b   →  a = 5^3 = 6,  b = 3
b ^= a   →  b = 3^6 = 5,  a = 6
a ^= b   →  a = 6^5 = 3,  b = 5

Kết quả: a = 3, b = 5   (đã đổi!)
```

---

## Bitmask Subsets — Liệt kê tập con

Cho mảng `[A, B, C]`, liệt kê **tất cả tập con** bằng cách dùng số từ 0 đến 2^n - 1.

Mỗi số là một "mặt nạ bit" (bitmask): bit thứ `i` = 1 nghĩa là phần tử thứ `i` nằm trong tập con.

```
n = 3 phần tử [A, B, C]
2^3 = 8 tập con

mask  binary  C B A   Tập con
─────────────────────────────
 0     000    0 0 0   { }
 1     001    0 0 1   { A }
 2     010    0 1 0   { B }
 3     011    0 1 1   { A, B }
 4     100    1 0 0   { C }
 5     101    1 0 1   { A, C }
 6     110    1 1 0   { B, C }
 7     111    1 1 1   { A, B, C }
```

```rust
pub fn subsets_bitmask(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1u64 << n;
    let mut result = Vec::new();
    for mask in 0..total {
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1u64 << i) != 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }
    result
}
```

**So sánh với đệ quy:** Cùng kết quả, nhưng bitmask ngắn gọn hơn và dễ hiểu khi `n` nhỏ (thường n <= 20).

---

## Bảng độ phức tạp

| Hàm                 | Time           | Space         |
|----------------------|----------------|---------------|
| `is_power_of_two`   | O(1)           | O(1)          |
| `count_ones`        | O(k), k = số bit 1 | O(1)     |
| `single_number`     | O(n)           | O(1)          |
| `subsets_bitmask`   | O(n * 2^n)     | O(n * 2^n)    |
| `get_bit`           | O(1)           | O(1)          |
| `set_bit`           | O(1)           | O(1)          |
| `clear_bit`         | O(1)           | O(1)          |
| `toggle_bit`        | O(1)           | O(1)          |
| `swap_without_temp` | O(1)           | O(1)          |

---

## Ghi nhớ

```
Bit tricks cheat sheet:

n & (n - 1)     Xóa bit thấp nhất      → đếm bit, check lũy thừa 2
n & (-n)        Lấy bit thấp nhất       → Fenwick tree
n | (1 << i)    Bật bit i               → set flag
n & ~(1 << i)   Tắt bit i              → clear flag
n ^ (1 << i)    Đảo bit i              → toggle
a ^ a = 0       XOR triệt tiêu         → tìm phần tử duy nhất
x << k          Nhân x cho 2^k
x >> k          Chia x cho 2^k
```

Bit manipulation không phải phép thuật — chỉ là cách nói chuyện trực tiếp với máy tính bằng ngôn ngữ của nó: **0 và 1**.
