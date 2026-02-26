# Fenwick Tree (Binary Indexed Tree)

## Đây là gì?

Bạn đã học **Prefix Sum** ở Phần 6 và **Bit Manipulation** ở Phần 7. Fenwick Tree kết hợp cả hai — như prefix sum nhưng **cập nhật được trong O(log n)**.

Hãy tưởng tượng bạn là người quản lý **bảng điểm giải đấu game**. Có 8 người chơi, mỗi người có điểm riêng. Bạn phải:

1. **Cập nhật** điểm một người chơi bất kỳ (ví dụ: "Player 3 được thêm 50 điểm")
2. **Tính tổng** điểm từ Player 1 đến Player k bất kỳ (ví dụ: "Tổng điểm từ Player 1 đến Player 5 là bao nhiêu?")

Nếu dùng mảng thường:
- Cập nhật: O(1) — nhanh
- Tính tổng: O(n) — chậm, phải cộng từ đầu

Nếu dùng prefix sum array:
- Tính tổng: O(1) — siêu nhanh
- Cập nhật: O(n) — chậm, phải tính lại toàn bộ prefix sum

**Fenwick Tree** giải quyết cả hai: **O(log n) cho cả update lẫn query**. Bí mật nằm ở cách nó dùng **bit thấp nhất** của chỉ số để chia nhỏ trách nhiệm.

---

## So sánh với Prefix Sum và Segment Tree

| Thao tác | Mảng thường | Prefix Sum Array | Fenwick Tree | Segment Tree |
|----------|-------------|-----------------|--------------|--------------|
| Xây dựng | O(n) | O(n) | O(n) | O(n) |
| Cập nhật 1 phần tử | O(1) | O(n) | **O(log n)** | O(log n) |
| Tính tổng [0, k] | O(n) | O(1) | **O(log n)** | O(log n) |
| Tính tổng [l, r] | O(n) | O(1) | **O(log n)** | O(log n) |
| Bộ nhớ | O(n) | O(n) | **O(n)** | O(4n) |
| Độ phức tạp code | Thấp | Thấp | **Trung bình** | Cao |

Fenwick Tree đơn giản hơn Segment Tree rất nhiều, ít code hơn, ít bộ nhớ hơn. Nhưng nó chỉ hỗ trợ các phép toán **có nghịch đảo** (cộng/trừ). Segment Tree mạnh hơn — hỗ trợ min, max, GCD...

> **Quy tắc ngón tay cái:** Nếu bài chỉ cần prefix sum + update -> dùng Fenwick Tree. Nếu cần min/max/GCD -> dùng Segment Tree.

---

## Bit trick: `i & (-i)` — lowbit

Đây là trái tim của Fenwick Tree. Hàm `lowbit(i)` trả về **bit thấp nhất** (lowest set bit) của `i`.

```
i       | nhị phân  | lowbit(i) | ý nghĩa
--------|-----------|-----------|------------------
1       | 0001      | 1         | quản lý 1 phần tử
2       | 0010      | 2         | quản lý 2 phần tử
3       | 0011      | 1         | quản lý 1 phần tử
4       | 0100      | 4         | quản lý 4 phần tử
5       | 0101      | 1         | quản lý 1 phần tử
6       | 0110      | 2         | quản lý 2 phần tử
7       | 0111      | 1         | quản lý 1 phần tử
8       | 1000      | 8         | quản lý 8 phần tử
```

Tại sao `i & (-i)` hoạt động? Trong biểu diễn **two's complement**, `-i` lật tất cả bit rồi cộng 1. Kết quả: chỉ bit thấp nhất còn lại.

```
Ví dụ: i = 6 (0110)
  -i = NOT(0110) + 1 = 1001 + 1 = 1010
  i & (-i) = 0110 & 1010 = 0010 = 2  ✓
```

Trong Rust, vì chỉ số là `usize` (unsigned), ta dùng `i & i.wrapping_neg()` thay cho `i & (-i)`.

---

## Cấu trúc cây

Fenwick Tree dùng mảng 1-indexed. Mỗi vị trí `i` **chịu trách nhiệm** cho `lowbit(i)` phần tử liên tiếp kết thúc tại `i`.

```
Mảng gốc (0-indexed):  [a0, a1, a2, a3, a4, a5, a6, a7]
                          1   2   3   4   5   6   7   8   <- 1-indexed

Fenwick Tree (tree[]):
  tree[1] = a0                              (lowbit(1)=1, quản lý 1 phần tử)
  tree[2] = a0 + a1                         (lowbit(2)=2, quản lý 2 phần tử)
  tree[3] = a2                              (lowbit(3)=1, quản lý 1 phần tử)
  tree[4] = a0 + a1 + a2 + a3              (lowbit(4)=4, quản lý 4 phần tử)
  tree[5] = a4                              (lowbit(5)=1, quản lý 1 phần tử)
  tree[6] = a4 + a5                         (lowbit(6)=2, quản lý 2 phần tử)
  tree[7] = a6                              (lowbit(7)=1, quản lý 1 phần tử)
  tree[8] = a0 + a1 + ... + a7             (lowbit(8)=8, quản lý 8 phần tử)
```

Nhìn theo dạng cây:

```
Tầng 3 (quản lý 8):          [=======tree[8]=======]
                              /                      \
Tầng 2 (quản lý 4):   [==tree[4]==]            [==   ==]
                       /           \            /        \
Tầng 1 (quản lý 2): [tree[2]]  [    ]     [tree[6]]  [    ]
                     /    \     /    \     /    \       /    \
Tầng 0 (quản lý 1): t[1] .   t[3]  .   t[5]  .     t[7]   .
```

---

## Thao tác Query — prefix_sum(idx)

Để tính tổng `a[0] + a[1] + ... + a[idx]`, ta bắt đầu từ `i = idx + 1` (chuyển sang 1-indexed) và **nhảy lùi** bằng cách bỏ bit thấp nhất.

```
Ví dụ: prefix_sum(6) — tức tổng a[0]..a[6], i bắt đầu = 7

  i = 7  (0111)  -> cộng tree[7]  -> i -= lowbit(7) = i - 1 = 6
  i = 6  (0110)  -> cộng tree[6]  -> i -= lowbit(6) = i - 2 = 4
  i = 4  (0100)  -> cộng tree[4]  -> i -= lowbit(4) = i - 4 = 0
  i = 0  -> dừng

  Kết quả = tree[7] + tree[6] + tree[4]
           = a6 + (a4+a5) + (a0+a1+a2+a3)
           = a0 + a1 + a2 + a3 + a4 + a5 + a6  ✓
```

Mỗi bước bỏ 1 bit -> tối đa **log₂(n)** bước.

```rust
pub fn prefix_sum(&self, idx: usize) -> i64 {
    let mut sum = 0;
    let mut i = idx + 1; // chuyển sang 1-indexed
    while i > 0 {
        sum += self.tree[i];
        i -= i & i.wrapping_neg(); // bỏ bit thấp nhất
    }
    sum
}
```

---

## Thao tác Update — update(idx, delta)

Để cộng thêm `delta` vào `a[idx]`, ta bắt đầu từ `i = idx + 1` và **nhảy tới** bằng cách cộng bit thấp nhất — đi ngược chiều query.

```
Ví dụ: update(2, +10) — cộng 10 vào a[2], i bắt đầu = 3

  i = 3  (0011)  -> cập nhật tree[3]  -> i += lowbit(3) = i + 1 = 4
  i = 4  (0100)  -> cập nhật tree[4]  -> i += lowbit(4) = i + 4 = 8
  i = 8  (1000)  -> cập nhật tree[8]  -> i += lowbit(8) = i + 8 = 16
  i = 16 -> vượt quá n, dừng

  Cập nhật tree[3], tree[4], tree[8] — tất cả các ô "chứa" a[2]
```

```rust
pub fn update(&mut self, idx: usize, delta: i64) {
    let mut i = idx + 1; // chuyển sang 1-indexed
    while i <= self.n {
        self.tree[i] += delta;
        i += i & i.wrapping_neg(); // cộng bit thấp nhất
    }
}
```

---

## Xây dựng từ mảng — from_vec

Cách naive: tạo tree rỗng rồi gọi `update` cho từng phần tử -> O(n log n).

Cách tốt hơn: copy dữ liệu vào, rồi lan truyền lên cha -> **O(n)**.

```rust
pub fn from_vec(data: &[i64]) -> Self {
    let n = data.len();
    let mut tree = vec![0i64; n + 1];

    // Copy vào vị trí 1-indexed
    for i in 0..n {
        tree[i + 1] = data[i];
    }

    // Lan truyền: mỗi node cộng giá trị vào cha trực tiếp
    for i in 1..=n {
        let parent = i + (i & i.wrapping_neg());
        if parent <= n {
            tree[parent] += tree[i];
        }
    }

    Self { tree, n }
}
```

---

## Range Sum — tổng đoạn [l, r]

Dùng hiệu prefix sum:

```
range_sum(l, r) = prefix_sum(r) - prefix_sum(l - 1)
```

Chú ý trường hợp `l == 0`: không cần trừ, vì `prefix_sum(r)` đã là đáp án.

```rust
pub fn range_sum(&self, l: usize, r: usize) -> i64 {
    if l == 0 {
        self.prefix_sum(r)
    } else {
        self.prefix_sum(r) - self.prefix_sum(l - 1)
    }
}
```

---

## Bảng độ phức tạp

| Thao tác | Thời gian | Ghi chú |
|----------|-----------|---------|
| `new(n)` | O(n) | Khởi tạo mảng 0 |
| `from_vec(data)` | O(n) | Xây từ mảng có sẵn |
| `update(idx, delta)` | O(log n) | Cộng delta vào 1 vị trí |
| `prefix_sum(idx)` | O(log n) | Tổng [0, idx] |
| `range_sum(l, r)` | O(log n) | Tổng [l, r], gọi prefix_sum 2 lần |
| Bộ nhớ | O(n) | Mảng n+1 phần tử |

---

## Khi nào dùng Fenwick Tree?

- Bài toán prefix sum mà **mảng bị thay đổi** liên tục
- Đếm số phần tử nhỏ hơn x (dùng Fenwick Tree trên giá trị)
- Đếm **inversions** trong mảng
- Competitive programming — Fenwick Tree code rất ngắn, dễ nhớ

> **Mẹo:** Trong competitive programming, Fenwick Tree là vũ khí yêu thích vì code ngắn gọn (khoảng 15 dòng) mà hiệu quả cao. Nếu bài chỉ cần tổng + cập nhật, hãy nghĩ đến Fenwick Tree trước Segment Tree.
