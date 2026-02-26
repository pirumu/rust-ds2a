# Hash Set

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

### Phép toán tập hợp

Đây là phần thú vị. Hãy tưởng tượng 2 lớp học:

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
```

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
```

Các phép toán cần `T: Clone` vì ta tạo set mới với các giá trị owned.

## Độ phức tạp

| Thao tác | Trung bình | Trường hợp xấu nhất |
|----------|------------|----------------------|
| `insert` | O(1) | O(n) |
| `contains` | O(1) | O(n) |
| `remove` | O(1) | O(n) |
| `union` | O(n+m) | O((n+m)*n) |
| `intersection` | O(min(n,m)) | O(n*m) |
| `difference` | O(n) | O(n*m) |

Trong đó n = kích thước set 1, m = kích thước set 2.

Trường hợp xấu nhất (O(n)) xảy ra khi tất cả phần tử hash vào cùng 1 bucket -- rất hiếm nếu hash function tốt.

**Nôm na:** Insert, contains, remove gần như ngay lập tức. Nhanh hơn nhiều so với duyệt danh sách (O(n)).

**Bộ nhớ:** O(n) cho n phần tử (giống HashMap bên dưới).

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
```
