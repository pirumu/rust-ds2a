# Design-Oriented Data Structures

## Đây là gì?

Ở các chương trước, bạn học những cấu trúc dữ liệu "chuẩn" — stack, queue, heap, HashMap. Chương này khác. Ta sẽ **kết hợp** chúng lại theo cách thông minh để giải quyết 4 bài toán thiết kế kinh điển.

Mỗi bài đều có chung 1 pattern: **một cấu trúc đơn lẻ không đủ, phải ghép 2 cái lại**. Giống như đũa — một chiếc thì vô dụng, nhưng hai chiếc cùng nhau thì gắp được mọi thứ.

4 bài toán:

| # | Tên | Ý tưởng chính | Nguồn |
|---|-----|---------------|-------|
| 1 | **MinStack** | Stack + auxiliary stack → O(1) get_min | LeetCode #155 |
| 2 | **MedianFinder** | Max-heap + min-heap → O(1) find_median | LeetCode #295 |
| 3 | **RandomizedSet** | Vec + HashMap → O(1) insert/remove/getRandom | LeetCode #380 |
| 4 | **NestedIterator** | Stack → làm phẳng danh sách lồng nhau | LeetCode #341 |

---

## 1. MinStack — Stack với O(1) get_min

### Bài toán

Thiết kế một stack hỗ trợ:

| Thao tác | Ý nghĩa | Yêu cầu |
|----------|---------|----------|
| `push(val)` | Đẩy phần tử vào | O(1) |
| `pop()` | Lấy phần tử trên cùng ra | O(1) |
| `top()` | Xem phần tử trên cùng | O(1) |
| `get_min()` | Lấy giá trị nhỏ nhất trong stack | **O(1)** |

Ba cái đầu thì stack bình thường làm được. Cái khó là `get_min()` trong O(1).

### Tại sao khó?

Nếu dùng stack bình thường, muốn tìm min phải duyệt hết → O(n). Nếu chỉ lưu 1 biến `current_min`, khi pop phần tử min ra thì sao? Phải tìm min mới → lại O(n).

### Ví dụ thực tế

Tưởng tượng bạn có **chồng đĩa** trong bồn rửa. Mỗi đĩa có kích thước khác nhau. Bạn muốn bất cứ lúc nào cũng biết **đĩa nhỏ nhất** trong chồng là bao nhiêu — mà không cần lục cả chồng.

Giải pháp: Bạn dán thêm 1 tờ giấy nhớ lên mỗi đĩa ghi "đĩa nhỏ nhất từ đây trở xuống là ___". Khi thêm đĩa mới, bạn so sánh nó với giấy nhớ của đĩa dưới, rồi ghi giấy nhớ mới.

### Trick: Auxiliary min-stack

Dùng **2 stack song song**:

- `data`: stack chính, lưu dữ liệu bình thường.
- `mins`: stack phụ, mỗi phần tử là **min tính từ đáy đến vị trí tương ứng**.

Hai stack luôn cùng kích thước. `mins.top()` chính là min hiện tại.

### Step-by-step

```
push(5):
    data: [5]           mins: [5]           ← min = 5

push(3):
    data: [5, 3]        mins: [5, 3]        ← min(3, 5) = 3

push(7):
    data: [5, 3, 7]     mins: [5, 3, 3]     ← min(7, 3) = 3

push(2):
    data: [5, 3, 7, 2]  mins: [5, 3, 3, 2]  ← min(2, 3) = 2

get_min() → 2  ✓  (đọc mins.top())

pop() → 2:
    data: [5, 3, 7]     mins: [5, 3, 3]     ← pop cả hai

get_min() → 3  ✓  (min tự động đúng!)

pop() → 7:
    data: [5, 3]        mins: [5, 3]

get_min() → 3  ✓
```

Mỗi lần pop, min tự động quay về giá trị đúng vì `mins` lưu lịch sử min tại từng thời điểm.

### ASCII diagram

```
    data stack          mins stack
    ┌───────┐           ┌───────┐
    │   2   │  top →    │   2   │  ← get_min() trả về đây
    ├───────┤           ├───────┤
    │   7   │           │   3   │
    ├───────┤           ├───────┤
    │   3   │           │   3   │
    ├───────┤           ├───────┤
    │   5   │           │   5   │
    └───────┘           └───────┘
```

### Rust struct

```rust
pub struct MinStack {
    data: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self { ... }
    pub fn push(&mut self, val: i32) { ... }
    pub fn pop(&mut self) -> Option<i32> { ... }
    pub fn top(&self) -> Option<i32> { ... }
    pub fn get_min(&self) -> Option<i32> { ... }
}
```

### Key insight

Khi push `val`:
- Push `val` vào `data`.
- Push `min(val, mins.top())` vào `mins`.

Khi pop:
- Pop cả `data` và `mins`.

`get_min()` = đọc `mins.last()`. Done. O(1) cho mọi thao tác.

### Độ phức tạp

| Thao tác | Time | Space |
|----------|------|-------|
| `push` | O(1) | O(1) |
| `pop` | O(1) | O(1) |
| `top` | O(1) | O(1) |
| `get_min` | O(1) | O(1) |
| **Tổng space** | | **O(n)** — mins stack tốn thêm n |

---

## 2. MedianFinder — Tìm trung vị từ luồng dữ liệu

### Bài toán

Dữ liệu đến liên tục (streaming). Tại bất kỳ thời điểm nào, bạn cần trả lời: **trung vị (median) hiện tại là bao nhiêu?**

| Thao tác | Ý nghĩa | Yêu cầu |
|----------|---------|----------|
| `add_num(num)` | Thêm 1 số vào luồng | O(log n) |
| `find_median()` | Trả về trung vị | **O(1)** |

Trung vị = số ở **giữa** khi sắp xếp. Nếu có chẵn phần tử → trung bình 2 số giữa.

### Tại sao khó?

- Cách naive: Mỗi lần thêm, sắp xếp lại mảng → O(n log n). Quá chậm.
- Dùng sorted array + binary search insert → O(n) vì phải dịch phần tử.

Ta cần cách thêm O(log n) và đọc median O(1).

### Ví dụ thực tế

Tưởng tượng bạn là giáo viên. Lớp đang xếp hàng theo **chiều cao từ thấp đến cao**. Mỗi khi có học sinh mới vào lớp, bạn muốn biết **học sinh đứng ở giữa hàng** cao bao nhiêu.

Trick: **Chia hàng thành 2 nhóm**.

- **Nhóm thấp** (nửa bên trái): Bạn chỉ cần biết **người cao nhất** nhóm này.
- **Nhóm cao** (nửa bên phải): Bạn chỉ cần biết **người thấp nhất** nhóm này.

Median = trung bình của "người cao nhất nhóm thấp" và "người thấp nhất nhóm cao"!

### Trick: Two-heap

- **max_heap** (left half): Chứa nửa nhỏ hơn. Top = số lớn nhất nửa nhỏ.
- **min_heap** (right half): Chứa nửa lớn hơn. Top = số nhỏ nhất nửa lớn.

Luôn giữ: `max_heap.len()` = `min_heap.len()` hoặc `max_heap.len()` = `min_heap.len() + 1`.

```
        max_heap (nửa nhỏ)          min_heap (nửa lớn)
        ┌─────────────┐             ┌─────────────┐
        │  top = lớn  │             │ top = nhỏ   │
        │  nhất nửa   │    median   │ nhất nửa    │
        │  nhỏ        │◄───────────►│ lớn         │
        └─────────────┘             └─────────────┘

        [1, 2, 3]                   [4, 5, 6]
              ^                      ^
          max_heap.top=3        min_heap.top=4

        median = (3 + 4) / 2 = 3.5
```

### Step-by-step

```
add_num(3):
    max_heap: [3]       min_heap: []
    median = 3.0

add_num(1):
    Thêm 1 vào max_heap → [3, 1]
    max_heap.len (2) > min_heap.len (0) + 1 → chuyển top sang
    max_heap: [1]       min_heap: [3]
    median = (1 + 3) / 2 = 2.0

add_num(5):
    5 > max_heap.top (1) → thêm vào min_heap
    max_heap: [1]       min_heap: [3, 5]
    min_heap.len (2) > max_heap.len (1) → chuyển top sang
    max_heap: [1, 3]    min_heap: [5]
    median = 3.0

add_num(2):
    2 ≤ max_heap.top (3) → thêm vào max_heap
    max_heap: [1, 2, 3]  min_heap: [5]
    max_heap.len (3) > min_heap.len (1) + 1 → chuyển top sang
    max_heap: [1, 2]     min_heap: [3, 5]
    median = (2 + 3) / 2 = 2.5

add_num(4):
    4 > max_heap.top (2) → thêm vào min_heap
    max_heap: [1, 2]     min_heap: [3, 4, 5]
    min_heap.len (3) > max_heap.len (2) → chuyển top sang
    max_heap: [1, 2, 3]  min_heap: [4, 5]
    median = 3.0
```

Sorted: [1, 2, **3**, 4, 5] → median = 3. Correct!

### Rust struct

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct MedianFinder {
    lo: BinaryHeap<i32>,           // max-heap (nửa nhỏ)
    hi: BinaryHeap<Reverse<i32>>,  // min-heap (nửa lớn)
}

impl MedianFinder {
    pub fn new() -> Self { ... }
    pub fn add_num(&mut self, num: i32) { ... }
    pub fn find_median(&self) -> f64 { ... }
}
```

### Key insight

Mọi lần thêm:
1. So sánh `num` với `lo.peek()` để quyết định thêm vào heap nào.
2. **Rebalance**: nếu 2 heap chênh lệch > 1, chuyển top từ heap lớn sang heap nhỏ.

`find_median()`:
- Nếu cùng size → trung bình 2 top.
- Nếu `lo` lớn hơn → `lo.peek()`.

### Độ phức tạp

| Thao tác | Time | Space |
|----------|------|-------|
| `add_num` | O(log n) | O(1) |
| `find_median` | O(1) | O(1) |
| **Tổng space** | | **O(n)** |

---

## 3. RandomizedSet — Tập hợp O(1) insert/remove/getRandom

### Bài toán

Thiết kế tập hợp hỗ trợ:

| Thao tác | Ý nghĩa | Yêu cầu |
|----------|---------|----------|
| `insert(val)` | Thêm phần tử (nếu chưa có) | O(1) |
| `remove(val)` | Xóa phần tử (nếu có) | O(1) |
| `get_random()` | Trả về phần tử ngẫu nhiên, mỗi phần tử có xác suất bằng nhau | **O(1)** |

### Tại sao khó?

- **HashSet**: insert/remove O(1), nhưng `get_random` phải duyệt → O(n). Không có cách chọn ngẫu nhiên O(1) từ HashSet vì phần tử nằm rải rác trong bộ nhớ.
- **Vec**: get_random O(1) (chọn index ngẫu nhiên), nhưng remove O(n) vì phải dịch phần tử.

Không cái nào đủ một mình. Phải ghép lại!

### Ví dụ thực tế

Tưởng tượng bạn tổ chức **rút thăm trúng thưởng**.

- Bạn có 1 **hộp thăm** (Vec) — các tấm thăm nằm cạnh nhau, có thể bốc ngẫu nhiên.
- Bạn có 1 **sổ tay** (HashMap) ghi "thăm X nằm ở vị trí nào trong hộp".

Khi ai đó rời khỏi (remove): Bạn không rút thăm ra khỏi giữa hộp (sẽ tạo lỗ trống). Thay vào đó, bạn **đổi chỗ thăm cần xóa với thăm cuối cùng**, rồi bỏ thăm cuối. Không ai biết thứ tự thay đổi!

### Trick: Vec + HashMap swap-remove

```
HashMap: val → index trong Vec
Vec:     [val₀, val₁, val₂, ...]
```

**Insert** `val`:
1. Kiểm tra HashMap — nếu đã có → return false.
2. Push `val` vào cuối Vec.
3. Ghi `val → vec.len()-1` vào HashMap.

**Remove** `val`:
1. Tìm index `i` của `val` trong HashMap.
2. Swap `vec[i]` với `vec[last]`.
3. Cập nhật HashMap cho phần tử bị swap.
4. Pop phần tử cuối Vec.
5. Xóa `val` khỏi HashMap.

**get_random**:
1. Chọn index ngẫu nhiên trong `0..vec.len()`.
2. Return `vec[index]`.

### Step-by-step

```
insert(10):
    Vec: [10]              Map: {10→0}

insert(20):
    Vec: [10, 20]          Map: {10→0, 20→1}

insert(30):
    Vec: [10, 20, 30]      Map: {10→0, 20→1, 30→2}

remove(20):
    Bước 1: index của 20 = 1
    Bước 2: Swap vec[1] với vec[2] (phần tử cuối)
        Vec: [10, 30, 20]
    Bước 3: Cập nhật Map: 30→1
    Bước 4: Pop cuối
        Vec: [10, 30]
    Bước 5: Xóa 20 khỏi Map
        Map: {10→0, 30→1}

get_random():
    random index trong [0, 2) → trả 10 hoặc 30
```

### ASCII diagram

```
remove(20):

Trước:
    Vec:  ┌────┬────┬────┐
          │ 10 │ 20 │ 30 │     Map: {10→0, 20→1, 30→2}
          └────┴────┴────┘
                 ↑    ↑
              remove  last

Swap:
    Vec:  ┌────┬────┬────┐
          │ 10 │ 30 │ 20 │     Map: {10→0, 30→1, 20→2}
          └────┴────┴────┘
                       ↑
                    pop this

Sau:
    Vec:  ┌────┬────┐
          │ 10 │ 30 │          Map: {10→0, 30→1}
          └────┴────┘
```

### Tại sao mỗi thao tác O(1)?

| Thao tác | Phân tích |
|----------|-----------|
| `insert` | HashMap insert O(1) + Vec push O(1)* |
| `remove` | HashMap lookup O(1) + Vec swap O(1) + Vec pop O(1) + HashMap update O(1) |
| `get_random` | Random index O(1) + Vec index O(1) |

\* Amortised O(1) — Vec có thể resize nhưng trung bình O(1).

Swap-remove trick là điểm then chốt: xóa ở giữa Vec bình thường là O(n) vì phải dịch, nhưng swap với cuối rồi pop → O(1). Ta không cần giữ thứ tự.

### Rust struct

```rust
use std::collections::HashMap;

pub struct RandomizedSet {
    vals: Vec<i32>,
    map: HashMap<i32, usize>,  // val → index trong vals
}

impl RandomizedSet {
    pub fn new() -> Self { ... }
    pub fn insert(&mut self, val: i32) -> bool { ... }
    pub fn remove(&mut self, val: i32) -> bool { ... }
    pub fn get_random(&self) -> i32 { ... }
}
```

### Độ phức tạp

| Thao tác | Time | Space |
|----------|------|-------|
| `insert` | O(1)* | O(1) |
| `remove` | O(1)* | O(1) |
| `get_random` | O(1) | O(1) |
| **Tổng space** | | **O(n)** |

---

## 4. NestedIterator — Làm phẳng danh sách lồng nhau

### Bài toán

Cho một danh sách lồng nhau (nested list), ví dụ:

```
[[1, 1], 2, [1, 1]]
```

Viết iterator duyệt ra: `1, 1, 2, 1, 1`.

| Thao tác | Ý nghĩa |
|----------|---------|
| `next()` | Trả về số tiếp theo |
| `has_next()` | Còn số nào không? |

Vấn đề: danh sách có thể lồng **nhiều tầng**:

```
[1, [2, [3, [4, 5]]]]  →  1, 2, 3, 4, 5
```

### Ví dụ thực tế

Tưởng tượng bạn nhận một **hộp quà** lớn. Mở ra thì bên trong có... thêm hộp nhỏ và vài món quà lẻ. Mở hộp nhỏ ra lại thấy... hộp nhỏ hơn nữa!

Bạn muốn **lấy tất cả món quà ra** (flatten). Cách tự nhiên nhất: khi gặp hộp → mở ra, đặt đồ bên trong lên bàn. Khi gặp món quà → lấy luôn.

### Trick: Stack-based flattening

Ta dùng 1 stack. Mỗi phần tử trên stack là 1 "con trỏ" đang duyệt qua 1 danh sách.

Ý tưởng: khi gặp danh sách con, **push** nó lên stack và bắt đầu duyệt danh sách con đó. Khi duyệt xong danh sách con, **pop** stack và quay lại danh sách cha.

### Step-by-step

```
Input: [[1, 2], 3, [4, [5]]]

Bắt đầu:
    Stack: [ iter([  [1,2], 3, [4,[5]]  ]) ]
                      ^
                   đang ở đây

next() → gặp [1, 2] — đây là list, push vào stack:
    Stack: [ iter([[1,2], 3, [4,[5]]]),  iter([1, 2]) ]
                    ^                          ^

next() → stack.top đang ở iter([1, 2]), phần tử = 1 → trả về 1
    Stack: [ iter([[1,2], 3, [4,[5]]]),  iter([1, 2]) ]
                    ^                            ^

next() → trả về 2
    Stack: [ iter([[1,2], 3, [4,[5]]]),  iter([1, 2]) ]
                    ^                               ^ hết!

    iter([1, 2]) hết rồi → pop stack
    Stack: [ iter([[1,2], 3, [4,[5]]]) ]
                          ^
                    quay lại đây

next() → gặp 3 (số) → trả về 3
    Stack: [ iter([[1,2], 3, [4,[5]]]) ]
                              ^

next() → gặp [4, [5]] — list, push:
    Stack: [ iter([...]),  iter([4, [5]]) ]
                                  ^

next() → gặp 4 → trả về 4

next() → gặp [5] — list, push:
    Stack: [ iter([...]),  iter([4, [5]]),  iter([5]) ]
                                                 ^

next() → trả về 5

    iter([5]) hết → pop. iter([4,[5]]) hết → pop. iter([...]) hết → pop.
    Stack rỗng → has_next() = false.
```

Kết quả: `1, 2, 3, 4, 5`. Correct!

### ASCII diagram

```
Input: [[1, 2], 3, [4, [5]]]

         Stack (mỗi tầng = 1 danh sách đang duyệt)

         ┌─────────────────┐
    top  │ iter([5])        │   ← danh sách sâu nhất
         ├─────────────────┤
         │ iter([4, [5]])   │   ← tầng giữa
         ├─────────────────┤
    bot  │ iter([...], 3, .)│   ← danh sách gốc
         └─────────────────┘

    Gặp số → trả về.
    Gặp list → push lên stack, duyệt tiếp.
    Hết list → pop, quay lại tầng trước.
```

### Cách tiếp cận đơn giản hơn: Flatten trước

Nếu không cần lazy evaluation, ta có thể flatten toàn bộ nested list thành 1 Vec ngay từ đầu bằng recursion:

```rust
fn flatten(nested: &[NestedItem]) -> Vec<i32> {
    let mut result = vec![];
    for item in nested {
        match item {
            NestedItem::Int(n) => result.push(*n),
            NestedItem::List(list) => result.extend(flatten(list)),
        }
    }
    result
}
```

Sau đó iterator chỉ là duyệt Vec. Đơn giản và đủ dùng cho hầu hết trường hợp.

### Rust struct

```rust
pub enum NestedItem {
    Int(i32),
    List(Vec<NestedItem>),
}

pub struct NestedIterator {
    stack: Vec<i32>,  // flattened values (reversed for O(1) pop)
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedItem>) -> Self { ... }
    pub fn next(&mut self) -> Option<i32> { ... }
    pub fn has_next(&self) -> bool { ... }
}
```

### Độ phức tạp

| Thao tác | Time | Space |
|----------|------|-------|
| `new` (flatten) | O(n) | O(n) |
| `next` | O(1) | O(1) |
| `has_next` | O(1) | O(1) |

n = tổng số integers trong nested list.

---

## Tổng kết

4 bài toán, 4 trick khác nhau — nhưng chung 1 tư duy: **ghép 2 cấu trúc dữ liệu để bù đắp nhược điểm của nhau**.

| Bài toán | Trick | Tại sao? |
|----------|-------|----------|
| **MinStack** | Stack + stack phụ | Stack không biết min, stack phụ ghi lại lịch sử min |
| **MedianFinder** | Max-heap + min-heap | Chia dữ liệu thành 2 nửa, median nằm ở ranh giới |
| **RandomizedSet** | Vec + HashMap | Vec cho random O(1), HashMap cho lookup O(1), swap-remove cho delete O(1) |
| **NestedIterator** | Stack (hoặc recursion) | Mỗi tầng lồng = 1 frame trên stack, y hệt cách compiler xử lý recursion |

Đây là những bài interview phổ biến nhất trong dạng "Design Data Structure". Hiểu trick rồi thì code chỉ là chuyện nhỏ.

## Chạy thử trong Rust

```rust
use rust_dsa::design_structures::{MinStack, MedianFinder, RandomizedSet};

// MinStack
let mut ms = MinStack::new();
ms.push(5);
ms.push(3);
ms.push(7);
assert_eq!(ms.get_min(), Some(3));
ms.pop();
assert_eq!(ms.get_min(), Some(3));

// MedianFinder
let mut mf = MedianFinder::new();
mf.add_num(1);
mf.add_num(2);
assert_eq!(mf.find_median(), 1.5);
mf.add_num(3);
assert_eq!(mf.find_median(), 2.0);

// RandomizedSet
let mut rs = RandomizedSet::new();
assert!(rs.insert(10));
assert!(rs.insert(20));
assert!(!rs.insert(10));    // đã có rồi
assert!(rs.remove(10));
assert!(!rs.remove(10));    // không còn nữa
let _random = rs.get_random();  // trả về 20
```
