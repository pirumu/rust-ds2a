# Hash Map

## Đây là gì?

Bạn đi siêu thị, mua xong muốn gửi đồ. Bạn đưa túi cho nhân viên, họ đưa lại bạn một **số tủ** -- ví dụ tủ số 7. Lúc quay lại, bạn chỉ cần đưa số 7 là lấy được đồ ngay. Không cần mở từng tủ tìm.

**Hash Map** (bảng băm) hoạt động y hệt vậy.

- **Key** (khóa) = đồ bạn gửi -- cái tên để nhận diện.
- **Value** (giá trị) = thông tin đi kèm với key đó.
- **Hash function** (hàm băm) = cách nhân viên tạo ra "số tủ" từ key của bạn.
- **Bucket** (ngăn chứa) = từng tủ trong dãy tủ.

Kết quả? Thay vì dò từng tủ một (O(n)), bạn nhảy thẳng đến đúng tủ cần tìm. Trung bình chỉ mất O(1) -- nhanh gần như tức thì.

## Hoạt động như thế nào?

### Mảng bucket -- dãy tủ gửi đồ

Hash Map bên trong là một mảng. Mỗi ô là một "tủ". Mỗi tủ có thể chứa một chuỗi (chain) các cặp key-value:

```
Dãy tủ (Bucket Array)
┌───────┐
│ Tủ 0  │ -> Trống
├───────┤
│ Tủ 1  │ -> ("apple", 5) -> ("fig", 8) -> Trống
├───────┤
│ Tủ 2  │ -> Trống
├───────┤
│ Tủ 3  │ -> ("banana", 3) -> Trống
├───────┤
│ Tủ 4  │ -> ("cherry", 7) -> Trống
├───────┤
│ Tủ 5  │ -> Trống
├───────┤
│ Tủ 6  │ -> Trống
├───────┤
│ Tủ 7  │ -> ("date", 2) -> Trống
└───────┘
```

### Thêm phần tử (Insert)

Muốn thêm cặp `("grape", 4)` vào hash map:

**Bước 1:** Chạy hash function trên key: `hash("grape") = 0x7a3f...`

**Bước 2:** Lấy phần dư để tìm số tủ: `0x7a3f... % 8 = 3`

**Bước 3:** Mở tủ 3 ra xem. Nếu key `"grape"` đã có rồi thì cập nhật value.

**Bước 4:** Nếu chưa có, thêm vào đầu chuỗi:

```
Trước:   tủ[3] -> ("banana", 3) -> Trống

Sau:     tủ[3] -> ("grape", 4) -> ("banana", 3) -> Trống
```

### Hash function -- cách tạo "số tủ"

Tại sao cần hash function? Vì key có thể là bất cứ thứ gì -- chuỗi ký tự, số, struct. Hash function biến key thành một con số, từ đó ta tính ra vị trí tủ.

Chúng ta dùng **FNV-1a** (Fowler-Noll-Vo) -- một hash function đơn giản, nhanh, phân bổ đều:

```
state = FNV_OFFSET_BASIS  (một số nguyên tố lớn)

Với mỗi byte trong key:
    state = state XOR byte
    state = state * FNV_PRIME

Kết quả: state chính là hash value
```

Giống như cách nhân viên siêu thị có một công thức riêng để tính số tủ từ tên khách hàng vậy.

### Collision -- khi 2 người bị gán cùng 1 tủ

Hãy tưởng tượng 2 khách hàng khác nhau nhưng nhân viên tính ra cùng một số tủ. Đây gọi là **collision** (va chạm).

Giải pháp của chúng ta: **separate chaining** -- mỗi tủ chứa một danh sách liên kết. Nếu 2 key cùng rơi vào tủ 1, cả hai đều nằm trong danh sách của tủ đó:

```
Collision ở tủ 1:

tủ[1] -> ("apple", 5) -> ("fig", 8) -> Trống
           ↑                 ↑
     hash = 1           hash cũng = 1 (collision!)
```

Khi tìm kiếm, ta duyệt qua danh sách trong tủ và so sánh key.

### Load factor -- khi tủ quá đông

**Load factor** (hệ số tải) = số phần tử / số tủ.

Ví dụ: 12 phần tử, 16 tủ => load factor = 12/16 = 0.75.

Khi load factor cao, mỗi tủ phải chứa nhiều phần tử hơn. Danh sách dài ra. Tìm kiếm chậm lại.

Giống như siêu thị giờ cao điểm -- mỗi tủ nhét 3-4 túi đồ. Lúc tìm phải lục hết.

**Giải pháp:** Khi load factor vượt **0.75**, ta nhân đôi số tủ và phân bổ lại toàn bộ phần tử:

```
Trước resize (load factor = 13/16 = 0.81):
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
│2│0│1│3│0│1│0│2│1│0│1│0│0│1│0│1│  <- số phần tử mỗi tủ
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
              16 tủ (chật!)

Sau resize (load factor = 13/32 = 0.41):
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
│1│0│0│1│0│1│0│0│1│0│1│0│0│1│0│0│1│0│0│1│0│0│1│0│1│0│0│1│0│0│0│1│
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
                      32 tủ (thoáng hơn nhiều!)
```

## Code Rust

Code đầy đủ nằm trong `src/hash_map.rs`. Dưới đây là các phần quan trọng.

### Cấu trúc dữ liệu

```rust
struct Entry<K, V> {
    key: K,
    value: V,
    next: Option<Box<Entry<K, V>>>,  // con trỏ đến phần tử tiếp theo trong chuỗi
}

pub struct HashMap<K, V> {
    buckets: Vec<Option<Box<Entry<K, V>>>>,  // dãy tủ
    len: usize,                               // tổng số phần tử
}
```

Mỗi bucket là `Option<Box<Entry>>`. `None` nghĩa là tủ trống. `Some(...)` là đầu chuỗi.

### Hash function (FNV-1a)

```rust
struct SimpleHasher {
    state: u64,
}

impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;           // XOR từng byte
            self.state = self.state.wrapping_mul(FNV_PRIME);  // nhân với số nguyên tố
        }
    }

    fn finish(&self) -> u64 {
        self.state  // trả về hash value cuối cùng
    }
}
```

Ta implement trait `std::hash::Hasher` để dùng được với bất kỳ kiểu nào có trait `Hash`.

### Insert -- xử lý collision

```rust
pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    let idx = self.bucket_index(&key);  // tính số tủ

    // Duyệt chuỗi -- nếu key đã tồn tại, cập nhật value
    // ...

    // Key chưa có -- thêm vào đầu chuỗi
    let head = self.buckets[idx].take();
    self.buckets[idx] = Some(Box::new(Entry { key, value, next: head }));
    self.len += 1;

    // Mở rộng nếu load factor > 0.75
    if self.load_factor() > 0.75 {
        self.resize();
    }
    None
}
```

### Resize -- mở rộng dãy tủ

```rust
fn resize(&mut self) {
    let new_cap = self.buckets.len() * 2;        // nhân đôi số tủ
    let mut new_buckets = vec![None; new_cap];

    for bucket in &mut self.buckets {
        let mut current = bucket.take();
        while let Some(mut entry) = current {
            current = entry.next.take();
            let idx = hash(&entry.key) % new_cap;   // tính lại vị trí tủ mới
            entry.next = new_buckets[idx].take();
            new_buckets[idx] = Some(entry);
        }
    }
    self.buckets = new_buckets;
}
```

Mỗi phần tử được **move** (di chuyển) sang mảng mới, không clone. Nên dù phải duyệt hết, vẫn hiệu quả về bộ nhớ.

## Độ phức tạp

| Thao tác       | Trung bình | Xấu nhất |
|----------------|------------|-----------|
| `insert`       | O(1)*      | O(n)      |
| `get`          | O(1)*      | O(n)      |
| `remove`       | O(1)*      | O(n)      |
| `contains_key` | O(1)*      | O(n)      |
| `resize`       | O(n)       | O(n)      |

\* Amortized (phân bổ). Trường hợp xấu nhất xảy ra khi tất cả key đều rơi vào cùng một tủ (hash function tệ).

**Bộ nhớ:** O(n) cho n phần tử + O(m) cho dãy tủ.

**Ý nghĩa thực tế:**
- Trung bình, mọi thao tác đều gần như tức thì -- giống bạn đưa số tủ, lấy đồ ngay.
- Xấu nhất là O(n) -- giống như tất cả khách đều bị gán cùng 1 tủ, phải lục hết.
- Resize tốn O(n) nhưng chỉ xảy ra thỉnh thoảng. Được phân bổ ra, mỗi insert vẫn là O(1).

## Ví dụ

```rust
use rust_ds2a::hash_map::HashMap;

let mut scores = HashMap::new();

// Thêm điểm của các bạn
scores.insert("Alice", 95);
scores.insert("Bob", 87);
scores.insert("Carol", 92);

// Tìm kiếm -- đưa tên, lấy điểm
assert_eq!(scores.get(&"Alice"), Some(&95));   // Alice có 95 điểm
assert_eq!(scores.get(&"Dave"), None);          // Dave không có trong danh sách

// Cập nhật -- Bob thi lại, được 91 điểm
scores.insert("Bob", 91);          // trả về Some(87) -- điểm cũ
assert_eq!(scores.get(&"Bob"), Some(&91));

// Xóa -- Carol nghỉ học
let old = scores.remove(&"Carol");  // trả về Some(92)
assert_eq!(scores.len(), 2);        // còn 2 người
assert!(!scores.contains_key(&"Carol"));  // Carol không còn trong danh sách
```
