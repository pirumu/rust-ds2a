# Segment Tree

## Đây là gì?

Bạn đã học Binary Tree ở Phần 3 và Recursion ở Phần 6. Segment Tree kết hợp cả hai để trả lời câu hỏi **"tổng/min/max từ i đến j"** trong O(log n).

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

Nhưng nếu **cập nhật** giá trị (ví dụ: thay đổi sách trên kệ)? Phải tính lại toàn bộ prefix sum --> O(n).

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

Giống Binary Heap, ta dùng mảng 1-indexed:
- Node `i` có con trái ở `2*i`, con phải ở `2*i+1`
- Kích thước mảng: `4 * n` (đủ cho mọi trường hợp)

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

**Time:** O(log n) — chỉ cập nhật các node trên một nhánh.

---

## Lazy Propagation — Cập nhật đoạn

### Vấn đề

Nếu muốn **cộng thêm 5 vào tất cả phần tử từ index 1 đến 4**? Với update thường, phải gọi update 4 lần → O(n log n). Với mảng lớn và nhiều thao tác, vẫn chậm.

### Ý tưởng: "Lười biếng" (Lazy)

Thay vì cập nhật ngay tất cả node con, ta **ghi nhớ** (lazy) ở node cha: "Các con tôi cần cộng thêm 5, nhưng tôi chưa làm". Chỉ khi nào thực sự cần truy cập node con, ta mới **đẩy** (push down) giá trị lazy xuống.

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

Cấu trúc cây và thuật toán query/update **hoàn toàn giống nhau**.

---

## Bảng tổng hợp độ phức tạp

| Thao tác | Time | Space |
|----------|:----:|:-----:|
| `new(data)` — build | O(n) | O(n) |
| `query(l, r)` — range query | O(log n) | O(log n) stack |
| `update(idx, val)` — point update | O(log n) | O(log n) stack |
| `range_update(l, r, val)` — lazy | O(log n) | O(log n) stack |

---

## Khi nào dùng Segment Tree?

- Bài toán có **nhiều lần query** trên đoạn (tổng, min, max, GCD...)
- **Vừa query vừa update** — prefix sum không đủ
- Dữ liệu thay đổi liên tục (online)
- Competitive programming: rất phổ biến!

**Không cần** nếu:
- Chỉ query, không update → dùng prefix sum (O(1) query)
- Chỉ update, không query → dùng mảng thường
- Dữ liệu rất nhỏ → duyệt brute force cũng được

---

## Tóm tắt

Segment Tree giống như **cuốn sổ tổng hợp** của thư viện. Thay vì đếm từng kệ sách, bạn tra sổ và tìm đáp án nhanh chóng. Khi có sách mới hoặc bớt sách, bạn chỉ cần sửa vài dòng trong sổ — không cần viết lại toàn bộ.

Lazy Propagation là phiên bản "lười biếng thông minh" — ghi nhớ thay đổi, chỉ thực hiện khi thực sự cần. Giống như bạn ghi note "cộng thêm 5 sách cho cả khu A" thay vì đi sửa từng kệ một.
