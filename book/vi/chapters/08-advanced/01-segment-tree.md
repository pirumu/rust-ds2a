# Segment Tree

> 💡 **Đừng lo lắng:** Bạn chỉ cần hai thứ:
> - **Binary Tree** (chương 3) — cách cây nhị phân chia nhánh trái/phải
> - **Recursion** (chương 6) — gọi hàm chính nó, chia nhỏ bài toán
>
> Nếu đã qua hai chương đó thì yên tâm. Segment Tree chỉ là **mỗi node lưu sẵn kết quả tổng hợp (aggregate) cho một đoạn liên tục** của mảng. Vậy thôi.

## Đây là gì?

Hãy tưởng tượng bạn là **quản lý chuỗi cửa hàng** có 16 chi nhánh. Sếp hay hỏi: "Doanh thu từ chi nhánh 5 đến 12 là bao nhiêu?" Nếu mỗi lần phải gọi điện từng chi nhánh hỏi số, rất chậm. Thay vào đó, bạn tổ chức theo **cấp bậc**: mỗi quản lý vùng phụ trách 2 quản lý nhỏ hơn, mỗi người đó lại phụ trách 2 nhóm nhỏ hơn nữa, cho đến từng chi nhánh. Mỗi quản lý luôn **cập nhật sẵn tổng doanh thu** vùng mình. Khi sếp hỏi, bạn chỉ cần hỏi vài quản lý vùng, gộp lại là xong. Khi một chi nhánh thay đổi doanh thu, chỉ cần cập nhật **dọc theo chuỗi quản lý** lên trên — không cần tính lại tất cả.

Segment Tree hoạt động chính xác như vậy. Nó chia mảng thành các đoạn (segment) theo cấu trúc cây nhị phân, lưu kết quả sẵn cho mỗi đoạn, rồi kết hợp nhanh khi cần truy vấn hoặc cập nhật.

---

## Tại sao cần Segment Tree?

### Cách thông thường: duyệt từng phần tử

Cho mảng `[2, 5, 1, 4, 9, 3]`. Muốn tính tổng từ index 1 đến 4?

```
Duyệt: 5 + 1 + 4 + 9 = 19    --> O(n) mỗi lần hỏi
```

Nếu hỏi 100.000 lần trên mảng 100.000 phần tử? Đó là 10^10 phép tính. Quá chậm.

### Prefix Sum — giải pháp một nửa

Prefix sum tính trước tổng tích lũy, trả lời mỗi query trong O(1):

```
Mảng gốc:    [2, 5, 1, 4, 9, 3]
Prefix sum:  [2, 7, 8, 12, 21, 24]

Tổng [1..4] = prefix[4] - prefix[0] = 21 - 2 = 19   --> O(1)!
```

Nhưng nếu **cập nhật** giá trị (ví dụ: chi nhánh 3 báo doanh thu mới)? Phải tính lại toàn bộ prefix sum --> O(n).

### Cầu nối: Prefix Sum --> Segment Tree

Đây là trade-off kinh điển:

```
Prefix Sum:    Query O(1)      Update O(n)     ← đọc nhanh, sửa chậm
Segment Tree:  Query O(log n)  Update O(log n) ← cả hai đều nhanh
```

Prefix Sum giống **cuốn sổ ghi tay** — tra cực nhanh, nhưng mỗi lần sửa một dòng thì phải viết lại cả trang. Segment Tree giống **bảng tính Excel có công thức** — tra chậm hơn một chút, nhưng sửa một ô thì các ô liên quan tự cập nhật.

### Segment Tree — cân bằng cả hai

| Thao tác       | Duyệt thường | Prefix Sum | Segment Tree |
|----------------|:---:|:---:|:---:|
| Build          | — | O(n) | O(n) |
| Query (range)  | O(n) | O(1) | **O(log n)** |
| Update (point) | O(1) | O(n) | **O(log n)** |

Segment Tree không nhanh bằng prefix sum cho query, nhưng update cũng chỉ O(log n). Nếu bài toán có **vừa query vừa update**, Segment Tree là lựa chọn tốt nhất.

---

## Cấu trúc cây

Cho mảng `[2, 5, 1, 4, 9, 3]` (6 phần tử, index 0..5):

```
                        [0..5]=24
                       /          \
               [0..2]=8            [3..5]=16
              /       \           /        \
         [0..1]=7    [2..2]=1  [3..4]=13   [5..5]=3
         /     \               /      \
    [0..0]=2  [1..1]=5    [3..3]=4  [4..4]=9
```

Mỗi node lưu **tổng của một đoạn liên tục**:
- Root lưu tổng toàn bộ mảng `[0..5] = 24`
- Node trái lưu nửa đầu `[0..2] = 8`, node phải lưu nửa sau `[3..5] = 16`
- Lá (leaf) lưu từng phần tử riêng lẻ

### Lưu trữ trong mảng phẳng

Giống Binary Heap (chương 3), ta dùng mảng 1-indexed:
- Node `i` có con trái ở `2*i`, con phải ở `2*i+1`
- Kích thước mảng: **`4 * n`** (xem phần Pitfalls bên dưới để hiểu tại sao)

```
Index:  1    2    3    4    5    6    7    8    9   10   11
Value: 24    8   16    7    1   13    3    2    5    4    9
       root  |    |    |    |    |    |    |    |    |    |
            [0..2][3..5][0..1][2][3..4][5] [0] [1] [3] [4]
```

---

## Build — Xây dựng cây

Dùng **recursion** (đệ quy) từ trên xuống:

1. Nếu đoạn chỉ có 1 phần tử (lá): gán giá trị
2. Nếu không: chia đôi, build hai nửa, rồi cộng kết quả

```
build(data, node, start, end):
    if start == end:
        tree[node] = data[start]     // lá
    else:
        mid = (start + end) / 2
        build(data, 2*node, start, mid)       // nửa trái
        build(data, 2*node+1, mid+1, end)     // nửa phải
        tree[node] = tree[2*node] + tree[2*node+1]  // tổng hợp
```

**Time:** O(n) — mỗi phần tử được xử lý đúng 1 lần.

---

## Query — Truy vấn đoạn

Muốn tính tổng `[l, r]`. Bắt đầu từ root, có 3 trường hợp:

```
1. Đoạn [start..end] nằm hoàn toàn NGOÀI [l..r]  --> return 0
2. Đoạn [start..end] nằm hoàn toàn TRONG [l..r]   --> return tree[node]
3. Giao nhau một phần --> chia đôi, hỏi cả hai con
```

### Ví dụ: query(1, 4) trên `[2, 5, 1, 4, 9, 3]`

```
                        [0..5]=24
                       /          \
               [0..2]=8            [3..5]=16
              /       \           /        \
         [0..1]=7    [2..2]=1  [3..4]=13   [5..5]=3
         /     \               /      \
    [0..0]=2  [1..1]=5    [3..3]=4  [4..4]=9

Query [1..4]:

[0..5] giao [1..4] một phần → đi cả hai con
  ├─ [0..2] giao [1..4] một phần → đi cả hai con
  │    ├─ [0..1] giao [1..4] một phần → đi cả hai con
  │    │    ├─ [0..0] NGOÀI [1..4] → return 0
  │    │    └─ [1..1] TRONG [1..4] → return 5  ✓
  │    └─ [2..2] TRONG [1..4] → return 1       ✓
  └─ [3..5] giao [1..4] một phần → đi cả hai con
       ├─ [3..4] TRONG [1..4] → return 13      ✓
       └─ [5..5] NGOÀI [1..4] → return 0

Kết quả: 5 + 1 + 13 = 19 ✓
```

**Time:** O(log n) — mỗi tầng chỉ ghé thăm tối đa 2 node.

---

## Update — Cập nhật một phần tử

Giả sử đổi index 2 từ `1` thành `10`:

```
Trước:                              Sau:
        [0..5]=24                           [0..5]=33
       /          \                        /          \
   [0..2]=8      [3..5]=16            [0..2]=17     [3..5]=16
  /       \                           /       \
[0..1]=7  [2..2]=1                [0..1]=7  [2..2]=10  ← thay đổi
                                                ↑
                                           cập nhật lá

Đi từ lá lên root, cập nhật lại tổng mỗi node trên đường đi.
```

Giống hệt việc quản lý chi nhánh báo doanh thu mới — chỉ cần cập nhật **dọc theo chuỗi quản lý** lên trên, không cần động tới các nhánh khác.

**Time:** O(log n) — chỉ cập nhật các node trên một nhánh.

---

## Lazy Propagation — Cập nhật đoạn

### Vấn đề

Nếu muốn **cộng thêm 5 vào tất cả phần tử từ index 1 đến 4**? Với update thường, phải gọi update 4 lần → O(n log n). Với mảng lớn và nhiều thao tác, vẫn chậm.

### Ý tưởng: "Lười biếng" (Lazy)

Quay lại ẩn dụ chuỗi cửa hàng: sếp nói "thưởng thêm 5 triệu cho chi nhánh 1 đến 4". Bạn **không** gọi từng chi nhánh. Bạn ghi vào sổ quản lý vùng: "vùng này cần cộng thêm 5 triệu". Chỉ khi nào ai đó hỏi chi tiết từng chi nhánh, bạn mới thực sự phân phối xuống.

```
range_update [1..4], +5:

                [0..5] tree=24+?
               /          \
        [0..2]              [3..5]
       /       \           /        \
  [0..1]      [2..2]   [3..4]     [5..5]
  /     \               /      \
[0] [1]              [3]     [4]

Bước 1: [0..5] giao [1..4] → đi cả hai con
Bước 2: [0..2] giao [1..4] → đi cả hai con
  - [0..1] giao [1..4] → đi cả hai con
    - [0..0] NGOÀI → bỏ
    - [1..1] TRONG → tree += 5, lazy += 5
  - [2..2] TRONG → tree += 5, lazy += 5
Bước 3: [3..5] giao [1..4] → đi cả hai con
  - [3..4] TRONG [1..4] → tree += 5*2=10, lazy += 5
    (DỪNG! Không đi xuống con nữa — lazy sẽ đẩy sau)
  - [5..5] NGOÀI → bỏ
```

Khi query, trước khi đi xuống con, ta **push down** lazy:

```
push_down(node):
    if lazy[node] != 0:
        // Đẩy lazy xuống con trái
        tree[left] += lazy[node] * size(left)
        lazy[left] += lazy[node]
        // Đẩy lazy xuống con phải
        tree[right] += lazy[node] * size(right)
        lazy[right] += lazy[node]
        // Xóa lazy ở node hiện tại
        lazy[node] = 0
```

**Time:** O(log n) cho cả range update lẫn query.

---

## Code trong Rust

### SegmentTree (Range Sum)

```rust
use rust_ds2a::segment_tree::SegmentTree;

let data = vec![2, 5, 1, 4, 9, 3];
let mut st = SegmentTree::new(&data);

// Query: tổng [1..4]
assert_eq!(st.query(1, 4), 19);  // 5 + 1 + 4 + 9

// Update: đổi index 2 thành 10
st.update(2, 10);
assert_eq!(st.query(1, 4), 28);  // 5 + 10 + 4 + 9
```

### SegmentTreeMin (Range Minimum)

```rust
use rust_ds2a::segment_tree::SegmentTreeMin;

let data = vec![5, 1, 8, 3, 9, 2];
let mut st = SegmentTreeMin::new(&data);

assert_eq!(st.query(0, 5), 1);   // min toàn bộ
assert_eq!(st.query(2, 4), 3);   // min [8, 3, 9]

st.update(1, 10);                 // đổi 1 → 10
assert_eq!(st.query(0, 1), 5);   // min [5, 10] = 5
```

### LazySegmentTree (Range Update)

```rust
use rust_ds2a::segment_tree::LazySegmentTree;

let data = vec![1, 2, 3, 4, 5];
let mut st = LazySegmentTree::new(&data);

// Cộng 10 vào tất cả phần tử [1..3]
st.range_update(1, 3, 10);
// Bây giờ mảng logic: [1, 12, 13, 14, 5]

assert_eq!(st.query(0, 4), 45);  // 1+12+13+14+5
assert_eq!(st.query(1, 3), 39);  // 12+13+14
```

---

## Segment Tree cho Min thay vì Sum

Thay đổi duy nhất: **phép kết hợp**.

| Loại | Kết hợp hai con | Giá trị "trung tính" (identity) |
|------|:---:|:---:|
| Sum  | `left + right` | `0` |
| Min  | `min(left, right)` | `i64::MAX` |
| Max  | `max(left, right)` | `i64::MIN` |
| GCD  | `gcd(left, right)` | `0` |
| XOR  | `left ^ right` | `0` |

Cấu trúc cây và thuật toán query/update **hoàn toàn giống nhau**. Đây là sức mạnh của Segment Tree — bạn chỉ cần thay đổi **một dòng code** (phép kết hợp) để giải một loại bài toán hoàn toàn khác.

---

## Bảng tổng hợp độ phức tạp

| Thao tác | Time | Space |
|----------|:----:|:-----:|
| `new(data)` — build | O(n) | O(n) |
| `query(l, r)` — range query | O(log n) | O(log n) stack |
| `update(idx, val)` — point update | O(log n) | O(log n) stack |
| `range_update(l, r, val)` — lazy | O(log n) | O(log n) stack |

---

## Những lỗi hay gặp

### 1. Kích thước mảng: 4n, không phải 2n

❌ **Sai:** `tree = vec![0; 2 * n]` — "Cây nhị phân có 2n node mà?"

✅ **Đúng:** `tree = vec![0; 4 * n]`

💡 **Tại sao:** Khi `n` không phải lũy thừa của 2, cây không hoàn hảo (perfect binary tree). Một số node ở tầng cuối bị lệch, index có thể vượt `2n`. Dùng `4n` là an toàn cho mọi trường hợp. Trong Rust code của crate này, bạn thấy dòng `vec![0i64; 4 * n.max(1)]` — đó là lý do.

```
n = 5, dùng 2*n = 10 slots?

          1
        /   \
       2     3
      / \   / \
     4   5 6   7       ← index 7 rồi
    / \  |
   8  9 10              ← index 10, vẫn ổ nếu 2n

Nhưng nếu n = 6:
          1
        /   \
       2     3
      / \   / \
     4   5 6   7
    / \ / \  |   |
   8  9 10 11 12 13     ← index 13 > 2*6 = 12. BÙM! 💥
```

### 2. Lazy push_down — PHẢI gọi trước khi đi xuống con

❌ **Sai:** Query hoặc update con mà quên push_down trước

```rust
// Sai: đi xuống con mà chưa push lazy
fn query_inner(&mut self, node, start, end, l, r) {
    let mid = (start + end) / 2;
    // Ủa, node này có lazy chưa đẩy xuống mà đã hỏi con???
    self.query_inner(2 * node, start, mid, l, r)  // SAI!
}
```

✅ **Đúng:** Luôn `push_down` trước khi chia đôi

```rust
fn query_inner(&mut self, node, start, end, l, r) {
    self.push_down(node, start, end);  // ← LUÔN gọi trước
    let mid = (start + end) / 2;
    self.query_inner(2 * node, start, mid, l, r)
}
```

💡 **Tại sao:** Nếu node cha đang giữ lazy value mà bạn hỏi con, con sẽ trả về giá trị **cũ** (chưa được cộng thêm). Kết quả sai mà khó debug vì chỉ sai trong một số trường hợp cụ thể.

### 3. Range boundaries — inclusive hay exclusive?

❌ **Sai:** Nhầm lẫn `[l, r]` inclusive với `[l, r)` exclusive

✅ **Đúng:** Chọn một convention và giữ nhất quán. Code trong crate này dùng **inclusive `[l, r]`** — cả `l` và `r` đều tính.

💡 **Tại sao:** Nhầm boundary off-by-one là bug phổ biến nhất với Segment Tree. Khi đi từ LeetCode (0-indexed, thường inclusive) sang competitive programming (đôi khi 1-indexed), phải cẩn thận.

### 4. Overflow khi tính mid

❌ **Sai:** `mid = (start + end) / 2` — có thể overflow nếu `start + end` lớn

✅ **Đúng:** `mid = start + (end - start) / 2`

💡 **Tại sao:** Với `usize` trong Rust thì ít gặp overflow (vì usize rất lớn), nhưng đây là thói quen tốt. Code trong `segment_tree.rs` đã dùng cách đúng.

---

## Segment Tree vs Fenwick Tree — Khi nào dùng cái nào?

Chương tiếp theo sẽ nói chi tiết về Fenwick Tree (Binary Indexed Tree), nhưng đây là bảng so sánh nhanh:

| Tiêu chí | Segment Tree | Fenwick Tree |
|----------|:---:|:---:|
| Range Sum query | O(log n) | O(log n) |
| Point update | O(log n) | O(log n) |
| Range update (lazy) | O(log n) | Phức tạp hơn |
| Range Min/Max query | O(log n) | Khó / Không hỗ trợ |
| Bộ nhớ | 4n | n |
| Code complexity | Dài hơn (~60 dòng) | Ngắn (~20 dòng) |
| Constant factor | Chậm hơn | **Nhanh hơn** |
| Linh hoạt | **Rất cao** | Trung bình |

**Tóm lại:**
- **Fenwick Tree** nếu bài toán chỉ cần **sum + point update** — code ngắn, chạy nhanh hơn
- **Segment Tree** nếu cần **min/max**, **range update** (lazy), hoặc các phép kết hợp phức tạp (GCD, merge sort tree...)
- Khi không chắc → dùng Segment Tree. Nó xử lý được mọi thứ Fenwick Tree làm được, và nhiều hơn.

---

## Khi nào dùng Segment Tree?

| Tình huống | Dùng gì? | Tại sao? |
|-----------|----------|----------|
| Chỉ query sum, không update | Prefix Sum | O(1) query, đơn giản |
| Query sum + point update | Segment Tree hoặc Fenwick Tree | Cả hai O(log n) |
| Query min/max + point update | **Segment Tree** | Fenwick Tree không hỗ trợ min/max tốt |
| Range update + range query | **Segment Tree + Lazy** | Fenwick Tree phức tạp hơn nhiều |
| Dữ liệu rất nhỏ (n < 1000) | Brute force | Đơn giản, đủ nhanh |
| Query GCD/XOR + update | **Segment Tree** | Chỉ cần đổi phép kết hợp |
| Đếm phần tử < x trong range | **Segment Tree + Merge Sort Tree** | Nâng cao, nhưng cùng ý tưởng |

**Không cần** Segment Tree nếu:
- Chỉ query, không update → dùng prefix sum (O(1) query)
- Chỉ update, không query → dùng mảng thường
- Dữ liệu rất nhỏ → duyệt brute force cũng được

---

## Persistent Segment Tree — Nâng cao

> Đây là kiến thức nâng cao. Bạn không cần hiểu ngay, nhưng nên biết nó tồn tại.

Persistent Segment Tree cho phép bạn **giữ lại lịch sử** — sau mỗi update, bạn có thể query **phiên bản cũ** của cây. Giống như Git: mỗi commit tạo version mới, nhưng version cũ vẫn truy cập được.

**Ý tưởng:** Khi update một node, thay vì sửa trực tiếp, tạo **node mới** và chỉ copy đường đi từ lá lên root. Các node không bị ảnh hưởng vẫn share với version cũ.

```
Version 0:         Version 1 (sau update index 2):

    [0..3]=10          [0..3]=15      ← node mới
   /        \         /        \
 [0..1]=3  [2..3]=7  [0..1]=3  [2..3]=12  ← node mới
                        │ share    /    \
                                [2]=10  [3]=2  ← node mới
                                  ↑
                                (cũ là 5)
```

**Ứng dụng:** Query "tổng từ i đến j **tại thời điểm t**", hoặc bài K-th smallest element in range.

**Space:** O(n log n) vì mỗi update tạo O(log n) node mới.

---

## Rust Ecosystem

### Crate `rust_ds2a`

Thư viện của chúng ta có 3 struct trong module `segment_tree`:

| Struct | Chức năng |
|--------|-----------|
| `SegmentTree` | Range sum + point update |
| `SegmentTreeMin` | Range min + point update |
| `LazySegmentTree` | Range sum + lazy range update |

Code dùng `Vec<i64>` với size `4 * n`, 1-indexed. Xem file `src/segment_tree.rs` để đọc implementation đầy đủ (~90 dòng cho mỗi variant).

### Crates bên ngoài

| Crate | Mô tả |
|-------|-------|
| [`ac-library-rs`](https://crates.io/crates/ac-library-rs) | Port của AtCoder Library. Có `Segtree` generic với trait `Monoid` — bạn chỉ cần implement phép kết hợp |
| [`competitive-programming-rs`](https://github.com/kenkoooo/competitive-programming-rs) | Collection cho competitive programming, bao gồm lazy segment tree |

Nếu bạn tham gia competitive programming trên AtCoder, `ac-library-rs` rất tiện vì nó match API chính thức.

### Tip: Generic Segment Tree với trait

Trong Rust, bạn có thể tạo Segment Tree generic bằng trait:

```rust
trait Monoid {
    fn identity() -> Self;       // phần tử trung tính
    fn combine(&self, other: &Self) -> Self;  // phép kết hợp
}

// Sum
impl Monoid for i64 {
    fn identity() -> Self { 0 }
    fn combine(&self, other: &Self) -> Self { self + other }
}
```

Như vậy một struct `SegmentTree<T: Monoid>` xử lý được sum, min, max, GCD... chỉ bằng cách implement trait khác nhau. Đây là cách `ac-library-rs` làm.

---

## Practice — Bài tập

### LeetCode

| # | Bài | Gợi ý |
|---|-----|-------|
| 307 | [Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable/) | Bài "hello world" của Segment Tree. Build + point update + range sum query. Dùng đúng code trong chương này. |
| 315 | [Count of Smaller Numbers After Self](https://leetcode.com/problems/count-of-smaller-numbers-after-self/) | Duyệt từ phải sang trái, dùng Segment Tree đếm số phần tử < giá trị hiện tại. Cần coordinate compression. |

### Competitive Programming

| Nguồn | Bài | Gợi ý |
|-------|-----|-------|
| CSES | [Range Sum Queries II](https://cses.fi/problemset/task/1648) | Segment Tree cơ bản |
| CSES | [Range Update Queries](https://cses.fi/problemset/task/1651) | Lazy Propagation |
| Codeforces | [Sereja and Brackets](https://codeforces.com/problemset/problem/380/C) | Segment Tree với merge phức tạp hơn |

### Cách tiếp cận

1. **Bắt đầu với #307** — nếu AC được bài này, bạn đã hiểu Segment Tree cơ bản
2. Thử **CSES Range Update** — để luyện lazy propagation
3. **#315** là bài ứng dụng sáng tạo — đừng nản nếu chưa nghĩ ra ngay

---

## Tóm tắt

Segment Tree giống như **hệ thống quản lý chuỗi cửa hàng**. Thay vì gọi điện từng chi nhánh hỏi doanh thu, bạn tổ chức theo cấp bậc — mỗi quản lý vùng nắm sẵn tổng. Hỏi nhanh O(log n), cập nhật cũng nhanh O(log n).

Lazy Propagation là phiên bản "lười biếng thông minh" — ghi nhớ thay đổi, chỉ thực hiện khi thực sự cần. Giống như bạn ghi note "thưởng thêm 5 triệu cho cả khu A" thay vì gọi từng chi nhánh thông báo.

Nhớ:
- **4n** cho mảng, không phải 2n
- **push_down trước** khi đi xuống con
- Segment Tree > Fenwick Tree khi cần min/max hoặc lazy
- Fenwick Tree > Segment Tree khi chỉ cần sum (ngắn hơn, nhanh hơn)

---

[← Tree Patterns](../07-patterns/09-tree-patterns.md) | [Fenwick Tree →](./02-fenwick-tree.md)
