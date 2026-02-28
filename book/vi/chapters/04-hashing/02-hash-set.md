# Hash Set

> 💡 **Đừng lo lắng:** Nếu bạn hiểu HashMap ở chương trước, bạn đã hiểu **100% HashSet** -- vì HashSet = HashMap mà value là rỗng `()`. Chương này giống hệt chương Priority Queue (wrapper quanh Heap): **không có algorithm mới**, chỉ thay đổi **cách nhìn** và **ứng dụng**. Giá trị thực sự nằm ở: set operations (union, intersection, difference) và các pattern phỏng vấn dùng HashSet (dedup, contains, counting unique). Nói cách khác: **HashSet = HashMap mà quên mang value theo** -- vậy thôi!

## Đây là gì?

Hãy tưởng tượng buổi điểm danh đầu giờ. Cô giáo không cần biết "em ngồi bàn mấy" hay "em đến lúc mấy giờ". Cô chỉ cần biết: **"em có mặt hay vắng?"**

Đó chính là **hash set** -- tập hợp các phần tử **không trùng lặp**, chỉ quan tâm phần tử **có tồn tại hay không**. Không có giá trị kèm theo (khác với hash map lưu cặp key-value).

So sánh nhanh:
- **HashMap** = danh bạ điện thoại: tên → số điện thoại (key → value)
- **HashSet** = danh sách điểm danh: chỉ có tên (chỉ có key)

### Khi nào cần hash set?

- **Phát hiện trùng lặp** -- "Username này đã có người dùng chưa?"
- **Kiểm tra thành viên** -- "Email này có trong danh sách VIP không?"
- **Phép toán tập hợp** -- union (gộp), intersection (giao), difference (hiệu)

---

## HashSet vs BTreeSet vs Vec

Trước khi đi sâu, hãy hiểu khi nào nên dùng cái gì. Ba cách lưu "tập hợp không trùng" phổ biến nhất trong Rust:

| | HashSet | BTreeSet | Vec (unique) |
|---|---------|----------|-------------|
| Contains | **O(1)** | O(log n) | O(n) linear scan |
| Insert (dedup) | **O(1)** | O(log n) | O(n) scan + push |
| Remove | **O(1)** | O(log n) | O(n) scan + shift |
| Sorted iteration | ❌ | ✅ | Cần sort trước |
| Union | O(n+m) | O(n+m) | O(n+m) merge |
| Intersection | O(min(n,m)) | O(min(n,m)) | O(n+m) if sorted |
| Memory | Hash overhead | Pointer overhead | Compact |

**Tóm gọn:** HashSet cho tốc độ. BTreeSet cho sorted order. Vec chỉ khi data nhỏ (< 20 phần tử).

Giống như điểm danh: HashSet là cô giáo có trí nhớ siêu phàm -- nghe tên là biết ngay "có rồi" hay "chưa". BTreeSet là cô giáo xếp danh sách theo bảng chữ cái -- tìm cũng nhanh, nhưng không bằng trí nhớ siêu phàm. Vec là cô giáo đọc lại từ đầu danh sách mỗi lần -- ổn nếu lớp có 10 người, nhưng lớp 1000 người thì... chào thua.

---

## Hoạt động như thế nào?

### HashSet = HashMap nhưng value là rỗng

Bí mật nhỏ: hash set chỉ là hash map mà value luôn là `()` (unit type -- kiểu rỗng trong Rust). Đây cũng là cách `std::collections::HashSet` của Rust hoạt động.

```
HashMap<K, V>                    HashSet<T>
┌──────────────────┐             ┌──────────────────┐
│ key   │ value    │             │ key   │ value     │
├───────┼──────────┤             ├───────┼───────────┤
│ "Lan" │ 42       │             │ "Lan" │ ()        │
│ "Mai" │ 17       │             │ "Mai" │ ()        │
│ "Hoa" │ 99       │             │ "Hoa" │ ()        │
└───────┴──────────┘             └───────┴───────────┘

  Lưu cặp key-value               Chỉ key, value rỗng
```

Mỗi method của HashSet đều gọi thẳng qua HashMap:

| HashSet method | HashMap method tương ứng |
|----------------|--------------------------|
| `insert(val)` | `insert(val, ())` |
| `contains(val)` | `contains_key(val)` |
| `remove(val)` | `remove(val)` |

Cô giáo (HashSet) không cần ghi thêm thông tin gì -- chỉ cần đánh dấu "có mặt" bằng `()`.

---

## Phép toán tập hợp (Set Operations)

Đây là phần thú vị nhất -- và cũng là lý do chính bạn dùng HashSet thay vì chỉ dùng HashMap.

### Ví dụ đời thực

Hãy tưởng tượng 2 lớp học:

- Lớp A = {Lan, Mai, Hoa, Tú}
- Lớp B = {Hoa, Tú, Linh, Nam}

```
Union (A ∪ B) -- gộp 2 lớp lại:
  {Lan, Mai, Hoa, Tú, Linh, Nam}
  = tất cả học sinh, không ai bị lặp

    ┌─────┐   ┌─────┐
    │xxxxx│xxx│xxxxx│     Tô hết
    │x A x│xxx│x B x│
    │xxxxx│xxx│xxxxx│
    └─────┘   └─────┘

Intersection (A ∩ B) -- học sinh cả 2 lớp:
  {Hoa, Tú}
  = ai có mặt ở cả A và B?

    ┌─────┐   ┌─────┐
    │  A  │xxx│  B  │     Chỉ phần chồng
    │     │xxx│     │
    └─────┘   └─────┘

Difference (A \ B) -- chỉ có trong lớp A:
  {Lan, Mai}
  = ai chỉ ở A mà không ở B?

    ┌─────┐   ┌─────┐
    │xxA  │   │  B  │     A trừ phần chồng
    │xx   │   │     │
    └─────┘   └─────┘

Symmetric Difference (A △ B) -- chỉ thuộc 1 trong 2 lớp:
  {Lan, Mai, Linh, Nam}
  = ai KHÔNG có mặt ở cả 2 lớp?

    ┌─────┐   ┌─────┐
    │xxA  │   │  Bxx│     Hai bên, trừ phần chồng
    │xx   │   │   xx│
    └─────┘   └─────┘
```

### Trace chi tiết: Intersection tối ưu

Mẹo quan trọng: luôn **duyệt set NHỎ hơn**, check trong set LỚN hơn. Vì mỗi `contains()` là O(1), số lần gọi = kích thước set nhỏ.

```
A = {1, 2, 3, 4, 5}  (5 phần tử)
B = {3, 5, 7}          (3 phần tử)

Duyệt B (nhỏ hơn), check trong A:
  3 in A? → YES → add to result
  5 in A? → YES → add to result
  7 in A? → NO

Result: {3, 5}
Operations: 3 contains checks (O(1) each) = O(min(n,m))

Nếu duyệt A thay B: 5 checks → tốn hơn!
```

### Trace chi tiết: Symmetric Difference (A △ B)

Phần tử chỉ thuộc **đúng 1** trong 2 set:

```
A = {1, 2, 3}
B = {2, 3, 4}

Bước 1 -- Duyệt A, tìm phần tử KHÔNG thuộc B:
  1 in B? → NO  → add to result
  2 in B? → YES → skip
  3 in B? → YES → skip

Bước 2 -- Duyệt B, tìm phần tử KHÔNG thuộc A:
  2 in A? → YES → skip
  3 in A? → YES → skip
  4 in A? → NO  → add to result

A △ B = {1, 4}
= (A \ B) ∪ (B \ A) = {1} ∪ {4}
```

---

## is_subset, is_superset, is_disjoint

Ba operations "kiểm tra quan hệ" giữa 2 set -- rất hay dùng:

```
A = {1, 2, 3}
B = {1, 2, 3, 4, 5}
C = {6, 7}

A ⊆ B?  (is_subset)    → true  -- mọi phần tử A đều ở B
B ⊇ A?  (is_superset)  → true  -- B chứa mọi phần tử A
A ∩ C = ∅? (is_disjoint) → true -- A và C không có phần tử chung
```

Ví dụ đời thực: cô giáo kiểm tra "tất cả học sinh lớp Toán có nằm trong danh sách đi dã ngoại không?" -- đó là `is_subset`.

### Implement

```rust
/// Tất cả phần tử self đều ở other?
pub fn is_subset(&self, other: &HashSet<T>) -> bool {
    self.map.keys().iter().all(|k| other.contains(k))
}
// O(n) -- duyệt self, check mỗi phần tử trong other = O(1)

/// other có phải subset của self?
pub fn is_superset(&self, other: &HashSet<T>) -> bool {
    other.is_subset(self)
}

/// self và other có phần tử chung nào không?
pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
    self.map.keys().iter().all(|k| !other.contains(k))
}
```

---

## Code Rust

Code đầy đủ nằm trong `src/hash_set.rs`.

### Cấu trúc

```rust
use crate::hash_map::HashMap;

pub struct HashSet<T: Hash + Eq + Debug> {
    map: HashMap<T, ()>,
}
```

Đơn giản: bọc HashMap, value luôn là `()`.

### Thao tác cơ bản

```rust
/// Thêm phần tử. Trả về true nếu là phần tử mới.
pub fn insert(&mut self, val: T) -> bool {
    self.map.insert(val, ()).is_none()
}

/// Kiểm tra phần tử có tồn tại không.
pub fn contains(&self, val: &T) -> bool {
    self.map.contains_key(val)
}

/// Xóa phần tử. Trả về true nếu phần tử có tồn tại.
pub fn remove(&mut self, val: &T) -> bool {
    self.map.remove(val).is_some()
}
```

### Phép toán tập hợp

```rust
/// Union -- gộp 2 set
pub fn union(&self, other: &HashSet<T>) -> HashSet<T> {
    let mut result = HashSet::new();
    for key in self.map.keys() {
        result.insert(key.clone());
    }
    for key in other.map.keys() {
        result.insert(key.clone());  // trùng thì bỏ qua
    }
    result
}

/// Intersection -- phần tử chung
pub fn intersection(&self, other: &HashSet<T>) -> HashSet<T> {
    let mut result = HashSet::new();
    for key in self.map.keys() {
        if other.contains(key) {
            result.insert(key.clone());
        }
    }
    result
}

/// Difference -- chỉ có trong self, không có trong other
pub fn difference(&self, other: &HashSet<T>) -> HashSet<T> {
    let mut result = HashSet::new();
    for key in self.map.keys() {
        if !other.contains(key) {
            result.insert(key.clone());
        }
    }
    result
}

/// Symmetric Difference -- phần tử chỉ thuộc 1 trong 2 set
pub fn symmetric_difference(&self, other: &HashSet<T>) -> HashSet<T> {
    let mut result = HashSet::new();
    for key in self.map.keys() {
        if !other.contains(key) {
            result.insert(key.clone());
        }
    }
    for key in other.map.keys() {
        if !self.contains(key) {
            result.insert(key.clone());
        }
    }
    result
}
```

Các phép toán cần `T: Clone` vì ta tạo set mới với các giá trị owned.

---

## HashSet trong thực tế

### a) Dedup -- loại bỏ trùng lặp

Đây là use case phổ biến **nhất** của HashSet. Tưởng tượng bạn có danh sách email đăng ký newsletter, và cùng 1 người đăng ký 3 lần:

```rust
use std::collections::HashSet;

let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
let unique: HashSet<_> = data.iter().copied().collect();
// unique = {1, 2, 3, 4, 5, 6, 9}  (thứ tự random)
```

Nhưng nếu muốn **giữ thứ tự xuất hiện đầu tiên**:

```rust
let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
let mut seen = HashSet::new();
let deduped: Vec<_> = data.iter()
    .filter(|x| seen.insert(**x))  // insert trả true nếu MỚI
    .copied()
    .collect();
// deduped = [3, 1, 4, 5, 9, 2, 6]  -- giữ nguyên thứ tự
```

Mẹo hay: `insert()` trả `true` nếu phần tử mới được thêm, `false` nếu đã tồn tại. Lợi dụng điều này để filter!

### b) Membership test trong validation

```rust
let valid_roles: HashSet<_> = ["admin", "editor", "viewer"]
    .into_iter().collect();

fn validate_role(role: &str, valid: &HashSet<&str>) -> bool {
    valid.contains(role)  // O(1) thay vì match hoặc if/else chain
}
```

Khi có 3 role thì `match` cũng được. Nhưng khi có 50 role valid? HashSet O(1) thắng tuyệt đối.

### c) Tìm phần tử chung giữa 2 danh sách (bài phỏng vấn kinh điển)

```
Bài toán: cho 2 mảng, tìm phần tử xuất hiện ở cả hai.

Brute force: O(n × m) -- 2 vòng lặp lồng nhau
HashSet:     O(n + m) -- insert mảng 1 vào set, check mảng 2

nums1 = [1, 2, 2, 1], nums2 = [2, 2]
set1 = {1, 2}
result = nums2.filter(|x| set1.contains(x)) → {2}
```

### d) Cycle detection (phát hiện vòng lặp)

Khi duyệt graph hoặc linked list, cần biết "node này đã visit chưa?":

```rust
// Detect cycle trong graph
let mut visited = HashSet::new();
let mut current = start_node;
loop {
    if !visited.insert(current) {
        // current đã visit trước đó → CYCLE!
        break;
    }
    current = next(current);
}
```

Giống cô giáo điểm danh: nếu gọi đến tên mà đã điểm rồi → "em đi vòng vòng à?"

---

## Độ phức tạp

| Thao tác | Trung bình | Trường hợp xấu nhất |
|----------|------------|----------------------|
| `insert` | O(1) | O(n) |
| `contains` | O(1) | O(n) |
| `remove` | O(1) | O(n) |
| `union` | O(n+m) | O((n+m)*n) |
| `intersection` | O(min(n,m)) | O(n*m) |
| `difference` | O(n) | O(n*m) |
| `symmetric_difference` | O(n+m) | O((n+m)*n) |
| `is_subset` | O(n) | O(n*m) |
| `is_disjoint` | O(n) | O(n*m) |

Trong đó n = kích thước set 1, m = kích thước set 2.

Trường hợp xấu nhất (O(n)) xảy ra khi tất cả phần tử hash vào cùng 1 bucket -- rất hiếm nếu hash function tốt.

**Nôm na:** Insert, contains, remove gần như ngay lập tức. Nhanh hơn nhiều so với duyệt danh sách (O(n)).

**Bộ nhớ:** O(n) cho n phần tử (giống HashMap bên dưới).

---

## Ví dụ

```rust
use rust_ds2a::hash_set::HashSet;

// === Điểm danh ===
let mut lop_a = HashSet::new();
lop_a.insert("Lan");
lop_a.insert("Mai");
lop_a.insert("Hoa");
lop_a.insert("Lan");   // trùng → trả về false, không thêm

assert_eq!(lop_a.len(), 3);          // chỉ 3 người
assert!(lop_a.contains(&"Lan"));     // Lan có mặt
assert!(!lop_a.contains(&"Nam"));    // Nam vắng

// === Phép toán tập hợp ===
let mut lop_b = HashSet::new();
lop_b.insert("Hoa");
lop_b.insert("Tú");
lop_b.insert("Lan");

// Học sinh cả 2 lớp
let ca_hai_lop = lop_a.intersection(&lop_b);
// ca_hai_lop chứa {"Lan", "Hoa"}

// Gộp 2 lớp
let tat_ca = lop_a.union(&lop_b);
// tat_ca chứa {"Lan", "Mai", "Hoa", "Tú"}

// Chỉ ở lớp A
let chi_lop_a = lop_a.difference(&lop_b);
// chi_lop_a chứa {"Mai"}

// Chỉ thuộc 1 lớp (không phải cả 2)
let mot_lop = lop_a.symmetric_difference(&lop_b);
// mot_lop chứa {"Mai", "Tú"}

// === Kiểm tra quan hệ ===
let mut lop_nho = HashSet::new();
lop_nho.insert("Lan");
lop_nho.insert("Hoa");

assert!(lop_nho.is_subset(&lop_a));   // lop_nho ⊆ lop_a
assert!(lop_a.is_superset(&lop_nho)); // lop_a ⊇ lop_nho

let mut lop_c = HashSet::new();
lop_c.insert("Minh");
lop_c.insert("Phúc");

assert!(lop_a.is_disjoint(&lop_c));   // không ai chung
```

---

## Những cái bẫy hay gặp

### 1. Dùng Vec để check duplicate thay vì HashSet

❌ Sai:
```rust
let mut seen = Vec::new();
for x in data {
    if seen.contains(&x) { /* duplicate */ }  // O(n) mỗi lần check!
    seen.push(x);
}
```

✅ Đúng:
```rust
let mut seen = HashSet::new();
for x in data {
    if !seen.insert(x) { /* duplicate */ }  // O(1) mỗi lần check
}
```

💡 `vec.contains()` là O(n), HashSet O(1). Với 1 triệu phần tử: Vec cần scan cả triệu, HashSet cần 1 phép hash.

### 2. Quên HashSet không giữ thứ tự

❌ Sai:
```rust
let set: HashSet<_> = vec![3, 1, 2].into_iter().collect();
for x in set { /* thứ tự RANDOM, không phải 3, 1, 2 */ }
```

✅ Đúng:
```rust
// Muốn sorted → BTreeSet
let set: BTreeSet<_> = vec![3, 1, 2].into_iter().collect();
// iterate: 1, 2, 3

// Muốn insertion order → indexmap::IndexSet
```

💡 HashSet dùng hash → thứ tự nội bộ phụ thuộc hash function, không phải thứ tự bạn insert. Đừng bao giờ dựa vào thứ tự iterate của HashSet.

### 3. Clone khi làm set operations mà chỉ cần iterate

❌ Tốn bộ nhớ:
```rust
// Tạo set MỚI chứa clone của mọi phần tử chung
let common = a.intersection(&b);  // allocate + clone tất cả
for x in common { /* ... */ }
```

✅ Tiết kiệm (với std HashSet):
```rust
// std::collections::HashSet trả iterator, không allocate
for x in a.intersection(&b) {
    // x là &T, không clone
}
```

💡 Implementation của chúng ta tạo set mới (cần Clone). Nhưng std trả iterator -- dùng std khi chỉ cần iterate, dùng `.collect()` khi cần set mới.

### 4. Dùng HashSet cho data nhỏ

❌ Overkill:
```rust
let valid = HashSet::from(["a", "b", "c"]);  // 3 phần tử
valid.contains(&role);
```

✅ Đơn giản hơn:
```rust
let valid = ["a", "b", "c"];
valid.contains(&role);  // Vec/slice linear scan
```

💡 Với < 20 phần tử, Vec + linear scan nhanh hơn HashSet. Hash overhead (tính hash, allocate bảng) lớn hơn lợi ích O(1) khi n nhỏ. Chỉ dùng HashSet khi n lớn hoặc cần set operations.

---

## Khi nào dùng / không nên dùng

| Tình huống | HashSet? | Thay bằng gì? | Tại sao? |
|------------|:--------:|---------------|----------|
| Kiểm tra trùng lặp | ✅ | -- | O(1) contains |
| Loại bỏ duplicate | ✅ | -- | Insert tự dedup |
| Set operations (union, intersect) | ✅ | -- | Built-in |
| Membership validation | ✅ | -- | O(1) lookup |
| Visited tracking (BFS/DFS) | ✅ | -- | O(1) insert + contains |
| Cần sorted unique elements | ❌ | BTreeSet | HashSet unordered |
| Cần insertion order | ❌ | IndexSet | HashSet random order |
| Data nhỏ (< 20 phần tử) | ❌ | Vec | Hash overhead |
| Cần count per element | ❌ | HashMap<T, usize> | HashSet không count |
| Multiset (cho phép trùng) | ❌ | HashMap<T, usize> | HashSet tự dedup |

---

## Luyện nhận diện Pattern

Ba bài kinh điển dùng HashSet -- gặp rất nhiều trong phỏng vấn:

### 1. Contains Duplicate (LeetCode #217)

**Đề bài:** Cho mảng `nums`, trả `true` nếu có phần tử xuất hiện >= 2 lần.

```
nums = [1, 2, 3, 1]  → true  (1 xuất hiện 2 lần)
nums = [1, 2, 3, 4]  → false (không ai trùng)
```

**Gợi ý:** Insert từng phần tử vào HashSet. Nếu `insert()` trả `false` → đã có trước đó → duplicate!

```rust
fn contains_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    nums.iter().any(|x| !seen.insert(x))
}
```

### 2. Intersection of Two Arrays (LeetCode #349)

**Đề bài:** Cho 2 mảng, trả mảng chứa phần tử xuất hiện ở cả hai (unique).

```
nums1 = [1, 2, 2, 1], nums2 = [2, 2]  → [2]
nums1 = [4, 9, 5], nums2 = [9, 4, 9, 8, 4]  → [4, 9]
```

**Gợi ý:** 2 HashSet + intersection, hoặc 1 HashSet + filter.

### 3. Longest Consecutive Sequence (LeetCode #128) -- Khó

**Đề bài:** Cho mảng unsorted, tìm dãy số liên tiếp dài nhất. Yêu cầu O(n).

```
nums = [100, 4, 200, 1, 3, 2]  → 4  (dãy [1, 2, 3, 4])
```

**Gợi ý:** Đưa tất cả vào HashSet. Với mỗi `num`, nếu `num - 1` **KHÔNG** có trong set → `num` là đầu dãy → đếm `num+1`, `num+2`,... cho đến khi không tìm thấy.

```
Set = {100, 4, 200, 1, 3, 2}

num = 100: 99 in set? NO → đầu dãy → 101? NO → dài 1
num = 4:   3 in set? YES → skip (không phải đầu dãy)
num = 200: 199 in set? NO → đầu dãy → 201? NO → dài 1
num = 1:   0 in set? NO → đầu dãy → 2? YES → 3? YES → 4? YES → 5? NO → dài 4
num = 3:   2 in set? YES → skip
num = 2:   1 in set? YES → skip

Longest = 4
```

Tổng vẫn O(n) vì mỗi phần tử được visit tối đa 2 lần (1 lần scan, 1 lần đếm).

---

## HashSet trong Rust ecosystem

| Crate / Type | Đặc điểm |
|-------------|-----------|
| `std::collections::HashSet` | Standard -- wrapper quanh HashMap<T, ()>. Dùng SipHash (chống HashDoS). |
| `std::collections::BTreeSet` | Sorted, hỗ trợ range queries (`range(1..5)`). |
| `indexmap::IndexSet` | Giữ insertion order. Nhanh gần bằng HashSet. |
| `bit-set` crate | BitSet cho integer set -- cực kỳ memory efficient (1 bit/phần tử). |
| `hashbrown::HashSet` | Backend thực sự của std HashSet từ Rust 1.36+. Swiss table algorithm. |

Ví dụ thực tế trong hệ thống: tracking which partition IDs are assigned (HashSet of IDs), dedup message IDs cho exactly-once delivery, tracking active consumer members trong consumer group.

---

## Chương tiếp theo

HashSet kết thúc phần hash-based structures "đơn giản". Bạn đã có đủ "vũ khí": HashMap cho key-value, HashSet cho membership, BTreeMap/BTreeSet cho sorted. Chương tiếp theo sẽ giới thiệu **Bloom Filter** -- cấu trúc probabilistic (xác suất) cực kỳ tiết kiệm bộ nhớ. Bloom Filter trả lời câu hỏi "phần tử này có trong set không?" giống HashSet, nhưng dùng **ít bộ nhớ hơn gấp nhiều lần** -- đổi lại, đôi khi nó... **nói dối** (false positive). Nghe thú vị chưa?

---

[← Hash Map](./01-hash-map.md) | [Bloom Filter →](./03-bloom-filter.md)
