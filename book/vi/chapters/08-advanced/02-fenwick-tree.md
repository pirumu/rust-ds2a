# Fenwick Tree (Binary Indexed Tree)

## Đừng sợ!

> **"Fenwick Tree nghe fancy nhưng thực ra đơn giản hơn Segment Tree rất nhiều."**
>
> Toàn bộ cấu trúc chỉ là **1 array** + **2 functions** (`update` và `query`), mỗi function chỉ **3 dòng code** thực sự. Nếu bạn hiểu được `i & (-i)`, bạn hiểu được Fenwick Tree. Thế thôi.
>
> So sánh nhanh:
> - Segment Tree: ~50-80 dòng code, tree phức tạp, dễ bug
> - **Fenwick Tree: ~15 dòng code, 1 mảng, siêu gọn**

---

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

### Bit trick visualization — mỗi index "phụ trách" đoạn nào?

`lowbit(i)` quyết định index `i` quản lý bao nhiêu phần tử. Hãy nhìn bằng hình:

```
Index (nhị phân)   lowbit   Đoạn phụ trách          Hình dung
─────────────────────────────────────────────────────────────
1  (0001)           1       [1, 1]                   █
2  (0010)           2       [1, 2]                   ██
3  (0011)           1       [3, 3]                       █
4  (0100)           4       [1, 4]                   ████
5  (0101)           1       [5, 5]                           █
6  (0110)           2       [5, 6]                           ██
7  (0111)           1       [7, 7]                               █
8  (1000)           8       [1, 8]                   ████████
```

**Quy luật:** Index `i` quản lý đoạn `[i - lowbit(i) + 1, i]`. Đoạn dài đúng bằng `lowbit(i)` phần tử.

Nhìn kỹ nhị phân: **bit 1 thấp nhất ở vị trí nào thì đoạn dài 2^vị_trí_đó**. Ví dụ:
- `6 = 0110` -> bit thấp nhất ở vị trí 1 -> đoạn dài 2^1 = 2
- `8 = 1000` -> bit thấp nhất ở vị trí 3 -> đoạn dài 2^3 = 8

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

## Những lỗi hay gặp (Pitfalls)

### 1. Dùng 0-indexed thay vì 1-indexed

❌ **Sai:**
```rust
// Quên chuyển sang 1-indexed
let mut i = idx; // BUG! Nếu idx = 0, vòng while không chạy
while i > 0 {
    sum += self.tree[i];
    i -= i & i.wrapping_neg();
}
```

✅ **Đúng:**
```rust
let mut i = idx + 1; // Chuyển sang 1-indexed
while i > 0 {
    sum += self.tree[i];
    i -= i & i.wrapping_neg();
}
```

💡 **Tại sao:** Fenwick Tree **bắt buộc** dùng 1-indexed. `lowbit(0) = 0`, nên nếu `i = 0` thì `i -= lowbit(i)` sẽ loop vô tận hoặc bỏ qua phần tử đầu tiên. Luôn nhớ `i = idx + 1` khi chuyển từ 0-indexed API sang internal 1-indexed.

### 2. Nhầm công thức lowbit

❌ **Sai:**
```rust
// Viết sai lowbit cho unsigned type
let lowbit = i & (-i); // Lỗi compile! usize không có số âm
```

✅ **Đúng:**
```rust
let lowbit = i & i.wrapping_neg(); // wrapping_neg() cho unsigned
```

💡 **Tại sao:** Rust dùng `usize` cho index (unsigned). Toán tử `-` không dùng được trên unsigned. `wrapping_neg()` thực hiện phép NOT + 1, cho kết quả tương đương `i & (-i)` trong two's complement.

### 3. Query đoạn [l, r] quên trừ prefix(l-1)

❌ **Sai:**
```rust
// Tính tổng [2, 5] bằng cách trừ sai
let result = prefix_sum(5) - prefix_sum(2); // Thiếu a[2]!
```

✅ **Đúng:**
```rust
let result = prefix_sum(5) - prefix_sum(1); // prefix(r) - prefix(l-1)
```

💡 **Tại sao:** `prefix_sum(k)` là tổng `[0, k]` (inclusive). Muốn tổng `[l, r]`, phải trừ đi tổng `[0, l-1]`, tức `prefix_sum(l-1)`. Nếu trừ `prefix_sum(l)` thì sẽ mất phần tử `a[l]`. Và đừng quên xử lý `l == 0` riêng!

### 4. Khởi tạo mảng tree sai kích thước

❌ **Sai:**
```rust
let tree = vec![0i64; n]; // Thiếu 1 slot!
```

✅ **Đúng:**
```rust
let tree = vec![0i64; n + 1]; // n+1 vì 1-indexed, tree[0] không dùng
```

💡 **Tại sao:** Fenwick Tree dùng index từ 1 đến n. Cần mảng size `n + 1`. `tree[0]` luôn là 0 và không bao giờ được truy cập.

---

## 2D Fenwick Tree — mở rộng lên ma trận

Khi bài toán yêu cầu **cập nhật 1 ô** trong ma trận và **tính tổng sub-matrix**, ta mở rộng Fenwick Tree lên 2 chiều.

Ý tưởng: thay vì 1 mảng 1D, dùng **mảng 2D**. Update và query chạy nested loop, mỗi chiều dùng cùng bit trick.

```rust
pub struct FenwickTree2D {
    tree: Vec<Vec<i64>>,
    rows: usize,
    cols: usize,
}

impl FenwickTree2D {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            tree: vec![vec![0i64; cols + 1]; rows + 1],
            rows,
            cols,
        }
    }

    /// Cộng delta vào ô (r, c) — 0-indexed
    pub fn update(&mut self, r: usize, c: usize, delta: i64) {
        let mut i = r + 1;
        while i <= self.rows {
            let mut j = c + 1;
            while j <= self.cols {
                self.tree[i][j] += delta;
                j += j & j.wrapping_neg();
            }
            i += i & i.wrapping_neg();
        }
    }

    /// Tổng sub-matrix từ (0,0) đến (r,c) — 0-indexed, inclusive
    pub fn prefix_sum(&self, r: usize, c: usize) -> i64 {
        let mut sum = 0;
        let mut i = r + 1;
        while i > 0 {
            let mut j = c + 1;
            while j > 0 {
                sum += self.tree[i][j];
                j -= j & j.wrapping_neg();
            }
            i -= i & i.wrapping_neg();
        }
        sum
    }

    /// Tổng sub-matrix từ (r1,c1) đến (r2,c2)
    /// Dùng inclusion-exclusion:
    /// sum(r2,c2) - sum(r1-1,c2) - sum(r2,c1-1) + sum(r1-1,c1-1)
    pub fn range_sum(
        &self, r1: usize, c1: usize, r2: usize, c2: usize
    ) -> i64 {
        let mut result = self.prefix_sum(r2, c2);
        if r1 > 0 { result -= self.prefix_sum(r1 - 1, c2); }
        if c1 > 0 { result -= self.prefix_sum(r2, c1 - 1); }
        if r1 > 0 && c1 > 0 { result += self.prefix_sum(r1 - 1, c1 - 1); }
        result
    }
}
```

**Complexity:** Update O(log R * log C), Query O(log R * log C), Space O(R * C).

> Cùng logic như 1D, chỉ lồng thêm 1 vòng cho chiều thứ 2. Nếu bạn hiểu 1D, 2D chỉ là copy-paste thêm 1 tầng.

---

## Fenwick Tree vs Segment Tree — so sánh cuối cùng

| Tiêu chí | Fenwick Tree | Segment Tree |
|----------|-------------|-------------|
| **Code** | ~15 dòng | ~50-80 dòng |
| **Bộ nhớ** | n + 1 | 4n |
| **Hằng số ẩn** | Rất nhỏ (nhanh hơn thực tế) | Lớn hơn |
| **Sum + Update** | O(log n) | O(log n) |
| **Min / Max** | Không hỗ trợ | O(log n) |
| **GCD / LCM** | Không hỗ trợ | O(log n) |
| **Lazy propagation** | Khó (cần biến thể) | Tự nhiên |
| **Range update** | Cần 2 Fenwick Trees | Dễ hơn |
| **2D mở rộng** | Dễ | Phức tạp |
| **Debug** | Khó (bit magic) | Dễ hơn (cấu trúc cây rõ ràng) |

**Tóm lại:**
- Bài chỉ cần **sum + point update** -> **Fenwick Tree** (nhanh, gọn, ít bug)
- Bài cần **min/max/GCD** hoặc **range update + range query** -> **Segment Tree**
- Trong competitive programming, nhiều người thích Fenwick Tree vì code nhanh hơn, ít bug hơn

---

## Khi nào dùng Fenwick Tree?

| Tình huống | Dùng gì? | Lý do |
|-----------|---------|------|
| Prefix sum, **không** có update | Prefix Sum Array | O(1) query, đơn giản nhất |
| Prefix sum + point update | **Fenwick Tree** | O(log n) cho cả hai, code ngắn |
| Range min/max query | Segment Tree | Fenwick Tree không hỗ trợ |
| Range update + range query | Segment Tree + Lazy | Fenwick Tree cần biến thể phức tạp |
| 2D sum + point update | **2D Fenwick Tree** | Dễ code hơn 2D Segment Tree |
| Đếm inversions | **Fenwick Tree** | Classic use case |
| Đếm phần tử nhỏ hơn x | **Fenwick Tree** | Fenwick trên giá trị |
| Competitive programming, cần code nhanh | **Fenwick Tree** | 15 dòng, khó viết sai |

---

## Khi nào dùng Fenwick Tree?

- Bài toán prefix sum mà **mảng bị thay đổi** liên tục
- Đếm số phần tử nhỏ hơn x (dùng Fenwick Tree trên giá trị)
- Đếm **inversions** trong mảng
- Competitive programming — Fenwick Tree code rất ngắn, dễ nhớ

> **Mẹo:** Trong competitive programming, Fenwick Tree là vũ khí yêu thích vì code ngắn gọn (khoảng 15 dòng) mà hiệu quả cao. Nếu bài chỉ cần tổng + cập nhật, hãy nghĩ đến Fenwick Tree trước Segment Tree.

---

## Practice — Luyện tập

### LeetCode 307: Range Sum Query - Mutable

**Bài toán:** Cho mảng `nums`, implement 2 thao tác:
- `update(index, val)` — thay `nums[index]` thành `val`
- `sumRange(left, right)` — tính tổng `nums[left..=right]`

**Gợi ý:** Đây chính xác là bài toán Fenwick Tree sinh ra để giải. Chú ý: `update` trong LeetCode là **set** giá trị mới, không phải **cộng thêm** delta. Nên cần tính `delta = new_val - old_val` trước khi gọi Fenwick update.

```rust
// Ý tưởng:
// - Giữ mảng gốc để tính delta khi set giá trị mới
// - Fenwick Tree để query range sum

struct NumArray {
    nums: Vec<i64>,
    tree: FenwickTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let vals: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let tree = FenwickTree::from_vec(&vals);
        Self { nums: vals, tree }
    }

    fn update(&mut self, index: i32, val: i32) {
        let i = index as usize;
        let delta = val as i64 - self.nums[i];
        self.nums[i] = val as i64;
        self.tree.update(i, delta);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.tree.range_sum(left as usize, right as usize) as i32
    }
}
```

### LeetCode 315: Count of Smaller Numbers After Self

**Bài toán:** Cho mảng `nums`, trả về mảng `counts` trong đó `counts[i]` = số phần tử bên phải `nums[i]` mà nhỏ hơn `nums[i]`.

**Gợi ý:** Duyệt từ phải sang trái. Với mỗi phần tử, query Fenwick Tree để đếm "có bao nhiêu giá trị nhỏ hơn đã được thêm vào?" rồi update Fenwick Tree tại vị trí giá trị đó.

```
Ví dụ: nums = [5, 2, 6, 1]

Duyệt từ phải:
  i=3, val=1: query(0..0) = 0 phần tử nhỏ hơn 1. Update tại 1. counts[3]=0
  i=2, val=6: query(0..5) = 1 phần tử nhỏ hơn 6. Update tại 6. counts[2]=1
  i=1, val=2: query(0..1) = 1 phần tử nhỏ hơn 2. Update tại 2. counts[1]=1
  i=0, val=5: query(0..4) = 2 phần tử nhỏ hơn 5. Update tại 5. counts[0]=2

Kết quả: [2, 1, 1, 0]
```

Cần coordinate compression nếu giá trị lớn. Fenwick Tree size = số giá trị unique.

---

## Rust Ecosystem

Trong Rust ecosystem, có vài crate hỗ trợ Fenwick Tree:

| Crate | Ghi chú |
|-------|---------|
| [`fenwick-tree`](https://crates.io/crates/fenwick-tree) | Crate nhỏ, API đơn giản, generic over type |
| [`superslice`](https://crates.io/crates/superslice) | Có Fenwick Tree trong bộ competitive programming utilities |

Tuy nhiên, Fenwick Tree code quá ngắn (15-20 dòng) nên hầu hết mọi người tự viết thay vì thêm dependency. Trong competitive programming, bạn cũng không dùng được external crate.

> **Lời khuyên thực tế:** Tự viết Fenwick Tree. Code chỉ 15 dòng, dễ customize (ví dụ: thay `+` bằng XOR), và bạn hiểu rõ hơn khi debug. Xem implementation ở `src/fenwick_tree.rs` trong repo này.

---

## Tổng kết

```
                    Fenwick Tree cheat sheet
┌──────────────────────────────────────────────────┐
│  Cấu trúc: 1 array, size n+1, 1-indexed         │
│  Bit trick: lowbit(i) = i & i.wrapping_neg()     │
│                                                  │
│  Query:  i = idx+1; while i>0: sum+=t[i], i-=lb  │
│  Update: i = idx+1; while i<=n: t[i]+=d, i+=lb   │
│                                                  │
│  Range [l,r] = prefix(r) - prefix(l-1)           │
│  Complexity: O(log n) update, O(log n) query     │
│  Space: O(n)                                     │
└──────────────────────────────────────────────────┘
```

---

## Tiếp theo

---

[← Segment Tree](./01-segment-tree.md) | [Skip List →](./03-skip-list.md)
