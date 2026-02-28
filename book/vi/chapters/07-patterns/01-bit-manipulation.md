# Bit Manipulation

> 💡 **Đừng lo lắng:** Nghe "bit manipulation" có vẻ đáng sợ? Thật ra chỉ là chơi với **0 và 1**. Bạn chỉ cần nhớ **6 phép toán**: AND, OR, XOR, NOT, shift left, shift right. Thế thôi. Không có gì phức tạp hơn bật/tắt công tắc đèn.

---

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
- Nếu bạn đã quen [Hash Set](../../chapters/04-hashing/02-hash-set.md) — bitmask thực ra là một "set siêu nhẹ" khi phần tử chỉ là số nhỏ

### Khi nào dùng bit manipulation?

| Tình huống | Kỹ thuật | Ví dụ |
|---|---|---|
| Kiểm tra tính chất số | AND, shift | Chẵn/lẻ, lũy thừa 2 |
| Tìm phần tử duy nhất/thiếu | XOR | Single Number, Missing Number |
| Lưu tập hợp nhỏ (n <= 20) | Bitmask | Subsets, permissions, flags |
| Liệt kê tập con | Bitmask + loop | Subset enumeration |
| Đếm bit | Brian Kernighan / built-in | Hamming weight, Hamming distance |
| Tối ưu phép nhân/chia cho 2^k | Shift | Thay `* 4` bằng `<< 2` |
| Toggle trạng thái | XOR | Bật/tắt feature flag |

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

**Ứng dụng:** Tìm phần tử duy nhất, đổi giá trị không cần biến tạm, toggle bit.

### NOT (`!`) — Đảo ngược tất cả

Bật thành tắt, tắt thành bật.

```
! 1 0 1 1
--------
  0 1 0 0
```

**Lưu ý:** Trong Rust, toán tử NOT cho số nguyên là `!` (không phải `~` như C/C++).

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

## XOR — Vũ khí bí mật

XOR là phép toán "lạ" nhất nhưng cũng mạnh nhất. Hãy hiểu thật kỹ 3 tính chất này:

### Tính chất 1: Tự triệt tiêu — `a ^ a = 0`

Bất kỳ số nào XOR với chính nó đều bằng 0. Tưởng tượng bạn bật đèn rồi tắt đèn — về lại trạng thái ban đầu.

```
  0 1 0 1    (5)
^ 0 1 0 1    (5)
--------
  0 0 0 0    (0)    ← tất cả khác nhau? Không! Giống nhau hết → 0
```

### Tính chất 2: Giữ nguyên — `a ^ 0 = a`

XOR với 0 không thay đổi gì. Như nhân với 1 vậy.

```
  0 1 0 1    (5)
^ 0 0 0 0    (0)
--------
  0 1 0 1    (5)    ← giữ nguyên
```

### Tính chất 3: Giao hoán & kết hợp — thứ tự không quan trọng

`a ^ b ^ c = c ^ a ^ b = b ^ c ^ a`

Giống cộng vậy: `1 + 2 + 3 = 3 + 1 + 2`. Tính chất này cực kỳ hữu ích — bạn có thể "gom" các cặp giống nhau lại để triệt tiêu.

### Combo sát thủ: Tìm Missing Number

Cho mảng `[0, 1, 3]` với n = 3. Thiếu số nào?

```
XOR tất cả index:  0 ^ 1 ^ 2 ^ 3  (từ 0 đến n)
XOR tất cả nums:   0 ^ 1 ^ 3

Gộp lại:  (0^0) ^ (1^1) ^ (2) ^ (3^3)
        =   0   ^   0   ^ 2  ^   0
        =   2                       ← số bị thiếu!
```

```rust
pub fn missing_number(nums: &[i32]) -> i32 {
    let n = nums.len() as i32;
    let mut xor = n; // bắt đầu với n
    for i in 0..nums.len() {
        xor ^= i as i32 ^ nums[i];
    }
    xor
}
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

### Bảng nhanh — Interview tricks

Giữ bảng này trong đầu, dùng được trong hầu hết bài bit manipulation:

```
┌────────────────────────┬──────────────────┬──────────────────────────┐
│  Trick                 │  Công thức       │  Giải thích              │
├────────────────────────┼──────────────────┼──────────────────────────┤
│  Isolate lowest set bit│  n & (-n)        │  Chỉ giữ bit 1 thấp nhất│
│  Clear lowest set bit  │  n & (n - 1)     │  Tắt bit 1 thấp nhất    │
│  Check bit i           │  (n >> i) & 1    │  Lấy giá trị bit thứ i  │
│  Set bit i             │  n | (1 << i)    │  Bật bit thứ i lên 1    │
│  Clear bit i           │  n & !(1 << i)   │  Tắt bit thứ i về 0     │
│  Toggle bit i          │  n ^ (1 << i)    │  Đảo bit thứ i          │
│  Check power of 2      │  n & (n-1) == 0  │  Chỉ 1 bit bật?         │
│  Count set bits        │  Brian Kernighan │  Lặp n &= n-1           │
│  All 1s mask (k bits)  │  (1 << k) - 1    │  VD: k=4 → 0b1111      │
└────────────────────────┴──────────────────┴──────────────────────────┘
```

**Trace `n & (-n)` — isolate lowest set bit:**

```
n  = 0b1010_1100
-n = 0b0101_0100   (đảo tất cả bit + 1, tức two's complement)

  1 0 1 0 1 1 0 0    n
& 0 1 0 1 0 1 0 0   -n
------------------
  0 0 0 0 0 1 0 0    ← chỉ còn bit thấp nhất!
```

Trick này là nền tảng của **Fenwick Tree** (Binary Indexed Tree) mà bạn sẽ gặp ở phần nâng cao.

---

## Bit manipulation cho tập hợp (Sets)

Nhớ [Hash Set](../../chapters/04-hashing/02-hash-set.md) không? Bitmask là phiên bản "siêu nhẹ" khi phần tử là số nhỏ (0 đến 31 cho `u32`, 0 đến 63 cho `u64`).

Ý tưởng: mỗi **bit** đại diện cho sự có mặt của một phần tử.

```
Tập {0, 2, 5} →  bit 0, 2, 5 bật  →  0b00100101 = 37

     bit: 7 6 5 4 3 2 1 0
          0 0 1 0 0 1 0 1
                ↑     ↑   ↑
                5     2   0
```

### Phép toán tập hợp = phép toán bit

| Tập hợp | Bit | Ví dụ |
|---|---|---|
| Union (hợp) | A \| B | `{1,2} ∪ {2,3}` = `0b0110 \| 0b1100` = `0b1110` = `{1,2,3}` |
| Intersection (giao) | A & B | `{1,2} ∩ {2,3}` = `0b0110 & 0b1100` = `0b0100` = `{2}` |
| Difference (hiệu) | A & !B | `{1,2} \ {2,3}` = `0b0110 & !0b1100` = `{1}` |
| Symmetric diff | A ^ B | `{1,2} △ {2,3}` = `0b0110 ^ 0b1100` = `{1,3}` |
| Add element i | A \| (1 << i) | Thêm 4: `set \| 0b10000` |
| Remove element i | A & !(1 << i) | Xóa 2: `set & !0b00100` |
| Contains i? | A & (1 << i) != 0 | Có 2 không? `set & 0b00100` |
| Size (đếm phần tử) | `count_ones()` | Đếm bit 1 |
| Empty? | A == 0 | Không bit nào bật |

**Khi nào dùng bitmask set thay vì HashSet?**
- Phần tử là số nguyên nhỏ (0..63)
- Cần tốc độ cực nhanh (O(1) cho mọi phép toán)
- Cần tiết kiệm bộ nhớ
- Ví dụ thực tế: permission flags (READ = 1, WRITE = 2, EXECUTE = 4), trạng thái game, visited mask trong DP bitmask

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
           = 4 ^ (1 ^ 1) ^ (2 ^ 2)    ← giao hoán, gom cặp
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

> **Thực tế:** Trong Rust, bạn chỉ cần `std::mem::swap(&mut a, &mut b)` hoặc `let (a, b) = (b, a)`. XOR swap là trick hay để hiểu XOR, nhưng code thật thì dùng cách đọc được hơn.

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

## Rust-specific: Built-in bit methods

Rust có sẵn các method cực tiện cho bit manipulation. Không cần tự viết!

```rust
fn rust_bit_methods() {
    let n: u32 = 0b1010_1100;

    // Đếm bit 1
    assert_eq!(n.count_ones(), 4);

    // Đếm bit 0
    assert_eq!(n.count_zeros(), 28);  // 32 - 4

    // Số bit 0 đầu tiên bên trái (leading zeros)
    assert_eq!(n.leading_zeros(), 24); // 32-bit, 24 bit 0 phía trước

    // Số bit 0 cuối cùng bên phải (trailing zeros)
    assert_eq!(n.trailing_zeros(), 2); // ...1100 → 2 số 0 cuối

    // Xoay bit (rotate) — không mất bit như shift
    let r = 0b1000_0001u8;
    assert_eq!(r.rotate_left(1), 0b0000_0011);  // bit cao nhất xoay sang phải
    assert_eq!(r.rotate_right(1), 0b1100_0000); // bit thấp nhất xoay sang trái

    // Đảo byte order
    assert_eq!(0x1234u16.swap_bytes(), 0x3412);

    // Check power of 2 — có sẵn luôn!
    assert!(8u32.is_power_of_two());
    assert!(!6u32.is_power_of_two());
}
```

**Tại sao dùng built-in?**
- `count_ones()` dùng instruction `POPCNT` của CPU — nhanh hơn Brian Kernighan rất nhiều
- `leading_zeros()` / `trailing_zeros()` dùng `LZCNT` / `TZCNT` — O(1) thật sự
- Compiler tối ưu hơn code tự viết

### i32 vs u32 — Cẩn thận!

Các bit method có trên **cả** `i32` và `u32`, nhưng behavior khác nhau ở right shift:

```rust
fn signed_vs_unsigned() {
    // Unsigned: shift phải điền 0
    let u: u32 = 0xFF000000;   // 1111_1111 ...
    println!("{:032b}", u >> 4); // 0000_1111_1111 ...

    // Signed: shift phải điền bit dấu (arithmetic shift)
    let s: i32 = -16;          // 1111...10000
    println!("{:032b}", s >> 2); // 1111...11100  (vẫn âm!)
}
```

---

## Pitfalls — Cạm bẫy thường gặp

### Pitfall 1: Signed vs Unsigned shift

❌ Sai:
```rust
let n: i32 = -1;
let result = n >> 1; // -1, không phải giá trị dương lớn!
```

✅ Đúng:
```rust
let n: i32 = -1;
let result = (n as u32) >> 1; // 2147483647 = 0x7FFFFFFF
```

💡 Tại sao: Right shift trên signed integer là **arithmetic shift** — nó copy bit dấu. Muốn **logical shift** (điền 0) thì cast sang unsigned trước.

### Pitfall 2: Shift overflow

❌ Sai:
```rust
let n: u32 = 1;
let result = n << 32; // PANIC trong debug mode!
```

✅ Đúng:
```rust
let n: u32 = 1;
let result = n.checked_shl(32); // Returns None
// hoặc
let result = (n as u64) << 32;  // Dùng type lớn hơn
```

💡 Tại sao: Shift bằng hoặc hơn số bit của type là undefined behavior trong C, và panic trong Rust (debug mode) / wrap (release mode). Luôn kiểm tra shift amount.

### Pitfall 3: NOT trong Rust khác C

❌ Sai (nghĩ theo C):
```rust
let mask: u32 = !(1u32 << 3); // Kết quả: 0xFFFF_FFF7 — đúng!
// Nhưng nếu quên type annotation:
let mask = !(1 << 3);         // i32, có thể gây lỗi sign khi dùng tiếp
```

✅ Đúng:
```rust
let mask: u32 = !(1u32 << 3);
// Hoặc rõ ràng hơn:
let bit_pos = 3;
let mask = u32::MAX ^ (1u32 << bit_pos);
```

💡 Tại sao: Rust dùng `!` cho NOT (không phải `~`). Và type inference có thể cho bạn `i32` thay vì `u32` — gây bug khi dùng tiếp trong AND/OR.

### Pitfall 4: Endianness — thứ tự byte

Bit manipulation hoạt động trên **giá trị logic**, không phụ thuộc endianness. Nhưng khi bạn đọc/ghi byte từ file hoặc network:

```
Số 0x1234 (2 byte):
  Big-endian:    [0x12, 0x34]   (byte quan trọng nhất trước)
  Little-endian: [0x34, 0x12]   (byte ít quan trọng nhất trước)
```

Khi làm bit manipulation thuần túy (XOR, AND, OR, shift), bạn **không cần lo** endianness. Chỉ cần cẩn thận khi serialize/deserialize byte.

---

## Bảng độ phức tạp

| Hàm                 | Time           | Space         |
|----------------------|----------------|---------------|
| `is_power_of_two`   | O(1)           | O(1)          |
| `count_ones`        | O(k), k = số bit 1 | O(1)     |
| `single_number`     | O(n)           | O(1)          |
| `missing_number`    | O(n)           | O(1)          |
| `subsets_bitmask`   | O(n * 2^n)     | O(n * 2^n)    |
| `get/set/clear/toggle_bit` | O(1)    | O(1)          |
| `swap_without_temp` | O(1)           | O(1)          |
| `u32::count_ones()` | O(1) hw        | O(1)          |

*"O(1) hw" = hardware instruction, nhanh hơn cả O(k)*

---

## Practice — Luyện tập

### Bài 1: Single Number (LeetCode #136)

**Đề:** Mảng mỗi phần tử xuất hiện 2 lần trừ 1 phần tử. Tìm phần tử đó.

**Approach:** XOR tất cả. Cặp giống nhau triệt tiêu, còn lại đáp án.

```rust
// Đã có ở trên: nums.iter().fold(0, |acc, &x| acc ^ x)
```

**Time:** O(n), **Space:** O(1). Không cách nào tốt hơn.

---

### Bài 2: Number of 1 Bits (LeetCode #191)

**Đề:** Đếm số bit 1 trong một số nguyên.

**Approach 1:** Brian Kernighan — xóa bit thấp nhất mỗi lần.

**Approach 2 (Rust):** Dùng built-in.

```rust
pub fn hamming_weight(n: u32) -> u32 {
    // Cách 1: Brian Kernighan
    let mut count = 0;
    let mut x = n;
    while x != 0 {
        x &= x - 1;
        count += 1;
    }
    count

    // Cách 2: built-in (nhanh hơn, dùng POPCNT instruction)
    // n.count_ones()
}
```

**Trace:**
```
n = 0b1011 (11)

Bước 1: x = 1011 & 1010 = 1010, count = 1
Bước 2: x = 1010 & 1001 = 1000, count = 2
Bước 3: x = 1000 & 0111 = 0000, count = 3

Kết quả: 3 bit
```

---

### Bài 3: Subsets via Bitmask (LeetCode #78)

**Đề:** Cho mảng số nguyên không trùng, trả về tất cả tập con.

**Approach:** Dùng bitmask từ 0 đến 2^n - 1. Đã giải ở phần Bitmask Subsets phía trên.

**Trace cho nums = [1, 2, 3]:**

```
mask=0 (000): []           ← tập rỗng
mask=1 (001): [1]          ← chỉ bit 0 bật → lấy nums[0]
mask=2 (010): [2]          ← chỉ bit 1 bật → lấy nums[1]
mask=3 (011): [1, 2]       ← bit 0,1 bật → lấy nums[0], nums[1]
mask=4 (100): [3]
mask=5 (101): [1, 3]
mask=6 (110): [2, 3]
mask=7 (111): [1, 2, 3]    ← tất cả bit bật → lấy hết
```

**Nhắc lại:** Approach này chỉ tốt khi n <= 20 (vì 2^20 ~ 1 triệu). Nếu n lớn hơn, quay lại dùng [đệ quy/backtracking](../../chapters/06-sorting-and-searching/08-backtracking.md).

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
a ^ 0 = a       XOR giữ nguyên         → missing number trick
x << k          Nhân x cho 2^k
x >> k          Chia x cho 2^k
bitmask         = tập hợp siêu nhẹ     → union = OR, giao = AND
```

Bit manipulation không phải phép thuật — chỉ là cách nói chuyện trực tiếp với máy tính bằng ngôn ngữ của nó: **0 và 1**. Bạn đã nắm 6 phép toán, vài trick hay, và biết khi nào dùng bitmask thay vì HashSet. Đủ để giải hầu hết bài phỏng vấn về bit rồi.

---

## Tiếp theo

---

[← Backtracking](../06-algorithms/14-backtracking.md) | [Monotonic Stack →](./02-monotonic-stack.md)
