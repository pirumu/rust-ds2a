# Hash Map (Bảng băm)

> 💡 **Đừng lo lắng:** HashMap là cấu trúc bạn đã dùng hàng ngày -- `HashMap::new()`, `map.insert()`, `map.get()`. Giờ bạn sẽ hiểu **bên trong** nó hoạt động ra sao. Tin vui: concept core rất đơn giản -- chỉ là "mảng + hàm tính vị trí". Phần phức tạp (collision handling, resize) cũng chỉ là 2 ý tưởng. Nếu bạn đã hiểu Vec (chương 1) và linked list concept, bạn đã có đủ nền tảng.

## Đây là gì?

Bạn đi siêu thị, mua xong muốn gửi đồ. Bạn đưa túi cho nhân viên, họ đưa lại bạn một **số tủ** -- ví dụ tủ số 7. Lúc quay lại, bạn chỉ cần đưa số 7 là lấy được đồ ngay. Không cần mở từng tủ tìm.

**Hash Map** (bảng băm) hoạt động y hệt vậy.

- **Key** (khóa) = đồ bạn gửi -- cái tên để nhận diện.
- **Value** (giá trị) = thông tin đi kèm với key đó.
- **Hash function** (hàm băm) = cách nhân viên tạo ra "số tủ" từ key của bạn.
- **Bucket** (ngăn chứa) = từng tủ trong dãy tủ.

Kết quả? Thay vì dò từng tủ một (O(n)), bạn nhảy thẳng đến đúng tủ cần tìm. Trung bình chỉ mất O(1) -- nhanh gần như tức thì.

---

## O(1) vs O(log n) -- the great trade-off

Suốt series tree, mọi thao tác đều O(log n). Với 1 triệu phần tử, O(log n) ≈ 20 bước. Rất nhanh. Nhưng có cách nào **1 bước** không?

**CÓ -- Hash Map.**

Nhớ lại chương Trie? Mỗi node trong Trie dùng `HashMap<char, TrieNode>` để lưu children. Lúc đó bạn đã "chạm" HashMap mà chưa hiểu bên trong. Giờ là lúc mở nắp ra xem.

| | HashMap | BTreeMap | Vec (sorted) |
|---|--------|---------|-------------|
| Lookup | **O(1)** avg | O(log n) | O(log n) binary search |
| Insert | **O(1)** avg | O(log n) | O(n) shift |
| Delete | **O(1)** avg | O(log n) | O(n) shift |
| Sorted iteration | ❌ **Không hỗ trợ** | ✅ O(n) | ✅ O(n) |
| Range query | ❌ | ✅ O(log n + k) | ✅ O(log n + k) |
| Min/Max | O(n) | O(log n) | O(1) |
| Memory | Hash overhead | Pointer overhead | Compact |
| Worst case | O(n) collision | O(log n) guaranteed | O(log n) |

HashMap thắng tuyệt đối về speed cho lookup/insert/delete. Đổi lại, mất hoàn toàn sorted order. Đây là lý do cả `HashMap` VÀ `BTreeMap` đều tồn tại trong Rust std -- mỗi cái giải quyết bài toán khác.

Quay lại ẩn dụ tủ gửi đồ:
- **BTreeMap** = tủ sắp xếp theo tên khách hàng. Tìm bằng binary search. Nhanh (O(log n)) và có thể duyệt theo thứ tự.
- **HashMap** = tủ gán số bằng công thức toán. Nhảy thẳng đến tủ đúng. Nhanh hơn (O(1)) nhưng không có thứ tự.

---

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

---

## Hash function deep dive

### Hash function -- cách tạo "số tủ"

Tại sao cần hash function? Vì key có thể là bất cứ thứ gì -- chuỗi ký tự, số, struct. Hash function biến key thành một con số, từ đó ta tính ra vị trí tủ.

### Tại sao cần hash function TỐT?

Xem hai ví dụ:

```
Hash function TỆ (chỉ lấy ký tự đầu):
  hash("apple")   = 'a' % 8 = 1
  hash("ant")     = 'a' % 8 = 1
  hash("avocado") = 'a' % 8 = 1
  hash("banana")  = 'b' % 8 = 2

  Tủ 1: apple → ant → avocado   (3 collision!)
  Tủ 2: banana
  → Tìm "avocado" cần duyệt 3 phần tử = O(n)

Hash function TỐT (FNV-1a, xét MỌI byte):
  hash("apple")   = 0x3a7f... % 8 = 5
  hash("ant")     = 0x8bc2... % 8 = 2
  hash("avocado") = 0x1de4... % 8 = 7
  hash("banana")  = 0x5f91... % 8 = 3

  Tủ 2: ant
  Tủ 3: banana
  Tủ 5: apple
  Tủ 7: avocado
  → Mỗi tủ 1 phần tử → O(1) lookup
```

Giống nhân viên siêu thị: nếu chỉ nhìn chữ cái đầu tên khách, tất cả Anh/An/Ân sẽ bị nhét cùng 1 tủ. Nhưng nếu xét toàn bộ tên, mỗi người sẽ có tủ riêng.

### Hash function tốt cần gì?

- **Deterministic**: cùng input → luôn cùng output. Gửi đồ ngày nào cũng cùng tủ.
- **Uniform distribution**: phân bổ đều vào các bucket. Không để 10 người chen 1 tủ.
- **Avalanche effect**: thay đổi 1 bit input → ~50% bit output thay đổi. "apple" và "bpple" phải ra số tủ hoàn toàn khác.
- **Nhanh**: hash tính toán phải nhanh, nếu không O(1) lookup vô nghĩa.

### FNV-1a -- hash function trong code của chúng ta

Chúng ta dùng **FNV-1a** (Fowler-Noll-Vo) -- một hash function đơn giản, nhanh, phân bổ đều:

```
state = FNV_OFFSET_BASIS  (một số nguyên tố lớn)

Với mỗi byte trong key:
    state = state XOR byte
    state = state * FNV_PRIME

Kết quả: state chính là hash value
```

### SipHash -- hash function mặc định của Rust

Rust std HashMap **KHÔNG** dùng FNV-1a mà dùng **SipHash-1-3**.

Tại sao? Vì **HashDoS attack**. Hacker có thể cố tình tạo ra hàng ngàn key mà hash giống nhau → tất cả rơi vào cùng 1 tủ → O(n) lookup → server chết. SipHash dùng random seed nên hacker không đoán được collision.

Trade-off:
- SipHash **chậm hơn** FNV/xxHash
- Nhưng **an toàn hơn** -- chống HashDoS

Muốn nhanh hơn (và chấp nhận rủi ro HashDoS)?
- `rustc_hash::FxHashMap` -- dùng trong Rust compiler, rất nhanh
- `ahash::AHashMap` -- dùng trong hashbrown (backend của std HashMap)

---

## Collision handling -- 2 trường phái

### Collision -- khi 2 người bị gán cùng 1 tủ

Hãy tưởng tượng 2 khách hàng khác nhau nhưng nhân viên tính ra cùng một số tủ. Đây gọi là **collision** (va chạm). Có 2 cách giải quyết:

### Trường phái 1: Separate Chaining (code của chúng ta dùng)

Mỗi tủ chứa một danh sách liên kết. Nếu 2 key cùng rơi vào tủ 1, cả hai đều nằm trong danh sách của tủ đó:

```
Mỗi bucket = linked list
  Bucket 3: ("grape",4) → ("banana",3) → None

Collision ở tủ 1:
  tủ[1] -> ("apple", 5) -> ("fig", 8) -> Trống
              ↑                 ↑
        hash = 1           hash cũng = 1 (collision!)
```

Khi tìm kiếm, ta duyệt qua danh sách trong tủ và so sánh key.

```
✅ Đơn giản, dễ implement
✅ Load factor có thể > 1 (nhiều phần tử hơn bucket)
❌ Pointer chasing (linked list trên heap) → cache-unfriendly
❌ Mỗi entry cần thêm pointer → tốn memory
```

### Trường phái 2: Open Addressing (Rust std HashMap dùng)

Khi collision, thay vì tạo linked list, ta **tìm bucket trống khác** trong cùng mảng:

**Linear probing:** thử bucket+1, bucket+2, bucket+3...

```
Insert "grape" (hash=3), "mango" (hash=3), "kiwi" (hash=5):

Bước 1: "grape" → bucket 3 (trống) → đặt luôn
  [_, _, _, grape, _, _, _, _]

Bước 2: "mango" → bucket 3 (đã có grape) → probe 4 (trống) → đặt
  [_, _, _, grape, mango, _, _, _]

Bước 3: "kiwi" → bucket 5 (trống) → đặt luôn
  [_, _, _, grape, mango, kiwi, _, _]

Lookup "mango": hash=3 → bucket 3 = grape (không match) →
  probe 4 = mango (match!) → trả về
```

**Robin Hood hashing** (Rust hashbrown dùng): phần tử "giàu" (gần bucket gốc) nhường chỗ cho phần tử "nghèo" (xa bucket gốc). Cân bằng probe distance → lookup đều hơn.

```
✅ Cache-friendly (data liên tiếp trong mảng)
✅ Nhanh hơn chaining trên modern CPU
❌ Phức tạp hơn khi delete (cần tombstone hoặc backward shift)
❌ Load factor phải < 1 (ít phần tử hơn bucket)
```

> **Note:** Code trong sách này implement separate chaining (đơn giản, tốt cho học). Rust std dùng open addressing + Robin Hood hashing (nhanh hơn). Cả hai đều O(1) amortized.

---

## Load factor, Resize, và Amortized analysis

### Load factor -- khi tủ quá đông

**Load factor** (hệ số tải) = số phần tử / số tủ.

Ví dụ: 12 phần tử, 16 tủ => load factor = 12/16 = 0.75.

Khi load factor cao, mỗi tủ phải chứa nhiều phần tử hơn. Danh sách dài ra. Tìm kiếm chậm lại.

Giống như siêu thị giờ cao điểm -- mỗi tủ nhét 3-4 túi đồ. Lúc tìm phải lục hết.

### Resize -- mở rộng dãy tủ

Khi load factor vượt **0.75**, ta nhân đôi số tủ và phân bổ lại toàn bộ phần tử:

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

Tại sao mỗi phần tử phải **rehash**? Vì vị trí tủ = `hash % số_tủ`. Khi số tủ thay đổi, vị trí tủ cũng thay đổi.

### Amortized O(1) -- tại sao resize không phá O(1)?

Resize tốn O(n) -- phải copy toàn bộ phần tử. Nghe kinh khủng. Nhưng:

```
Amortized O(1) insert:

Insert 1-7:  O(1) mỗi cái = 7 operations
Insert 8:    bucket đầy → resize (copy 8 items) + insert = 9 operations
Insert 9-15: O(1) mỗi cái = 7 operations
Insert 16:   resize (copy 16 items) + insert = 17 operations

Tổng: 7 + 9 + 7 + 17 = 40 operations cho 16 inserts
Trung bình: 40/16 = 2.5 operations/insert ≈ O(1)

Resize xảy ra ở insert 8, 16, 32, 64, 128...
Mỗi resize tốn O(n) nhưng khoảng cách giữa 2 resize tăng gấp đôi
→ Phân bổ ra mỗi insert = O(1)
```

Resize giống chuyển nhà sang nhà lớn hơn. Tốn công 1 lần, nhưng sau đó sống thoải mái lâu. Nếu tính trung bình chi phí chuyển nhà ra mỗi ngày sống → rất rẻ.

---

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

---

## Rust std HashMap -- bí mật bên trong

Bạn đã hiểu HashMap hoạt động thế nào qua code chúng ta viết. Giờ xem Rust std làm khác gì.

### Backend: hashbrown (SwissTable)

Rust std `HashMap` dùng **hashbrown** -- port của Google's SwissTable:

- **Collision handling:** Open addressing (không phải chaining như code của chúng ta)
- **Probing:** Robin Hood hashing + SIMD lookup (dùng CPU vector instructions để check nhiều bucket cùng lúc)
- **Hash function:** SipHash-1-3 (chống HashDoS)

### API quan trọng

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// with_capacity -- tránh resize nếu biết trước size
let mut map = HashMap::with_capacity(1000);

// entry API -- pattern quan trọng nhất
map.entry("key")
    .and_modify(|v| *v += 1)  // nếu có → modify
    .or_insert(0);             // nếu chưa → insert default

// entry + or_default (cho Vec, String, etc.)
let items: &mut Vec<i32> = map.entry("key").or_default();
items.push(42);

// drain -- lấy hết phần tử ra mà không clone
for (k, v) in map.drain() {
    println!("{}: {}", k, v);
}

// retain -- filter in-place
map.retain(|_k, v| *v > 10);
```

`entry()` là API đặc biệt quan trọng. Nó tránh double lookup (không cần `get()` rồi `insert()` riêng) và là Rust idiom mà bạn sẽ dùng cực kỳ nhiều.

### Benchmark vs BTreeMap

```
1M random integers:
  HashMap insert:    ~120ms   BTreeMap insert:    ~180ms
  HashMap lookup:    ~80ms    BTreeMap lookup:    ~150ms
  HashMap iterate:   ~25ms    BTreeMap sorted:    ~45ms
  HashMap range:     ❌        BTreeMap range:     ~0.5ms ✓

HashMap thắng tốc độ. BTreeMap thắng sorted + range.
```

---

## Hash Trait trong Rust

Khi dùng `HashMap`, key phải implement `Hash + Eq`. Với primitive types (`i32`, `String`, `&str`...) thì Rust đã lo sẵn. Nhưng khi dùng custom struct làm key thì sao?

```rust
// Tự động derive Hash -- phổ biến nhất
#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let mut map = HashMap::new();
map.insert(Point { x: 1, y: 2 }, "origin-ish");
```

**Quy tắc quan trọng nhất:** Hash + Eq phải **consistent**.

```
Nếu a == b thì hash(a) PHẢI == hash(b)
Ngược lại không bắt buộc (collision OK)
```

Nếu vi phạm: 2 object bằng nhau nhưng hash khác → `get()` sẽ tìm sai tủ → lookup fail.

**f64 KHÔNG implement Hash** vì `NaN != NaN` (vi phạm Eq). Muốn dùng float làm key:

```rust
// ❌ Không compile
// let mut map: HashMap<f64, String> = HashMap::new();

// ✅ Dùng ordered_float
use ordered_float::OrderedFloat;
let mut map: HashMap<OrderedFloat<f64>, String> = HashMap::new();
map.insert(OrderedFloat(3.14), "pi".into());
```

**Rule of thumb:** Nếu struct chỉ chứa integer, string, bool → `derive(Hash)`. Nếu chứa float → cần xử lý đặc biệt.

---

## HashMap vs HashSet

`HashSet` = HashMap mà bạn chỉ quan tâm key, không cần value. Bên trong implementation giống hệt -- `HashSet<K>` thực chất là `HashMap<K, ()>`.

```rust
use std::collections::HashSet;

let mut seen = HashSet::new();
seen.insert("apple");
seen.insert("banana");
seen.insert("apple");  // duplicate → bị bỏ qua

assert_eq!(seen.len(), 2);  // chỉ 2 phần tử unique
assert!(seen.contains("apple"));
```

Khi nào dùng?
- **HashMap**: cần lưu key-value pairs (tên → điểm, id → user)
- **HashSet**: chỉ cần kiểm tra "có hay không" (dedup, membership check, set operations như union/intersection)

---

## HashMap trong thực tế

### a) Counting / Frequency (phổ biến nhất)

Đời thực: đếm số lần xuất hiện của mỗi từ trong văn bản. Giống nhân viên siêu thị đếm bao nhiêu khách đến mỗi ngày -- mỗi ngày là 1 key, số khách là value.

```rust
use std::collections::HashMap;

// Đếm frequency mỗi từ
let text = "con mèo con chó con mèo";
let mut freq = HashMap::new();
for word in text.split_whitespace() {
    *freq.entry(word).or_insert(0) += 1;
}
// freq = {"con": 3, "mèo": 2, "chó": 1}
```

`entry()` API tránh double lookup -- đây là Rust idiom quan trọng.

### b) Caching / Memoization

Đời thực: bạn tính Fibonacci(50). Không cache → 2^50 operations (hàng năm). Có cache → 50 operations (tức thì). HashMap lưu kết quả đã tính, lần sau chỉ cần tra tủ.

```rust
fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&val) = cache.get(&n) {
        return val;  // cache hit → O(1)
    }
    let result = fib(n - 1, cache) + fib(n - 2, cache);
    cache.insert(n, result);  // cache miss → compute + store
    result
}
```

### c) Two Sum -- bài phỏng vấn kinh điển nhất

Bài toán: cho mảng `[2, 7, 11, 15]` và target = 9, tìm 2 index sao cho `nums[i] + nums[j] = target`.

- Brute force: O(n^2) -- check mọi cặp
- HashMap: O(n) -- với mỗi num, check `target - num` có trong map không

```
Trace:
  map = {}
  num=2: complement = 9-2 = 7, 7 not in map → insert (2 → index 0)
  num=7: complement = 9-7 = 2, 2 IN MAP at index 0 → return [0, 1] ✓
```

Ý tưởng: thay vì hỏi "ai + tôi = target?", ta gửi mỗi số vào tủ (HashMap). Khi số tiếp theo đến, nó check "tủ có complement của tôi không?" -- O(1) lookup thay vì O(n) duyệt.

### d) Graph adjacency list

```rust
use std::collections::HashMap;

// Graph biểu diễn bằng HashMap
let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
graph.entry("A").or_default().push("B");
graph.entry("A").or_default().push("C");
graph.entry("B").or_default().push("D");
// A → [B, C], B → [D]
// HashMap cho O(1) lookup neighbors -- quan trọng cho BFS/DFS
```

---

## Độ phức tạp

| Thao tác       | Trung bình | Xấu nhất |
|----------------|------------|-----------|
| `insert`       | O(1)*      | O(n)      |
| `get`          | O(1)*      | O(n)      |
| `remove`       | O(1)*      | O(n)      |
| `contains_key` | O(1)*      | O(n)      |
| `resize`       | O(n)       | O(n)      |

\* **Amortized** (phân bổ). Trường hợp xấu nhất xảy ra khi tất cả key đều rơi vào cùng một tủ (hash function tệ hoặc bị HashDoS attack).

**Tại sao insert là "amortized" O(1)?** Vì resize (O(n)) chỉ xảy ra khi load factor > 0.75. Khoảng cách giữa 2 lần resize tăng gấp đôi. Phân bổ chi phí resize ra mỗi insert → O(1).

**Bộ nhớ:** O(n) cho n phần tử + O(m) cho dãy tủ.

**Ý nghĩa thực tế:**
- Trung bình, mọi thao tác đều gần như tức thì -- giống bạn đưa số tủ, lấy đồ ngay.
- Xấu nhất là O(n) -- giống như tất cả khách đều bị gán cùng 1 tủ, phải lục hết.
- Resize tốn O(n) nhưng chỉ xảy ra thỉnh thoảng. Được phân bổ ra, mỗi insert vẫn là O(1).

---

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

---

## Những cái bẫy hay gặp

### a) Dùng HashMap khi cần sorted order

❌ Iterate HashMap và mong kết quả theo thứ tự

✅ Dùng `BTreeMap` nếu cần sorted iteration. Dùng `indexmap::IndexMap` nếu cần insertion order.

💡 HashMap không giữ thứ tự insert cũng không sorted. Thứ tự iterate phụ thuộc vào hash value -- thay đổi giữa các lần chạy.

### b) Quên Hash + Eq consistency

❌ Implement `Hash` và `Eq` riêng rẽ, không nhất quán

✅ Derive cả `Hash`, `Eq`, `PartialEq` cùng lúc: `#[derive(Hash, Eq, PartialEq)]`

💡 Nếu `a == b` nhưng `hash(a) != hash(b)` → `get()` tìm sai tủ → lookup fail dù key tồn tại.

### c) Dùng f64 làm key

❌ `HashMap<f64, String>` -- không compile

✅ Dùng `OrderedFloat<f64>` từ crate `ordered_float`. Hoặc convert sang integer (nhân 100 để lưu cent thay vì dollar).

💡 `f64` không implement `Hash` vì `NaN != NaN` -- vi phạm Eq contract.

### d) Không dùng with_capacity khi biết trước size

❌ Insert 1 triệu phần tử vào `HashMap::new()` → resize ~20 lần

✅ `HashMap::with_capacity(1_000_000)` → 0 resize. Nhanh hơn ~30%.

💡 Mỗi resize tốn O(n). 20 resize = rất nhiều copy thừa.

### e) Nghĩ HashMap luôn O(1)

❌ "HashMap O(1) nên luôn nhanh nhất"

✅ Worst case là O(n) khi tất cả key collision. Data nhỏ (< 20 items) thì `Vec` + linear search có thể nhanh hơn vì cache-friendly.

💡 SipHash giảm rủi ro collision nhưng không triệt tiêu. Nếu cần guaranteed worst-case O(log n) → dùng `BTreeMap`.

---

## Khi nào dùng / không nên dùng

| Tình huống | HashMap? | Thay bằng gì? | Tại sao? |
|------------|---------|---------------|----------|
| Key-value lookup nhanh | ✅ | -- | O(1) amortized |
| Counting / frequency | ✅ | -- | entry() API |
| Caching / memoization | ✅ | -- | O(1) cache hit |
| Dedup (kiểm tra trùng) | ✅ (HashSet) | -- | O(1) contains |
| Two Sum, group by | ✅ | -- | Classic pattern |
| Cần sorted iteration | ❌ | BTreeMap | HashMap unordered |
| Cần range query | ❌ | BTreeMap | HashMap không support |
| Cần insertion order | ❌ | IndexMap | HashMap shuffle order |
| Key là float | ❌ | BTreeMap hoặc OrderedFloat | f64 no Hash |
| Guaranteed O(log n) | ❌ | BTreeMap | HashMap worst O(n) |
| Data nhỏ (< 20 items) | ⚠️ | Vec + linear search | Hash overhead > linear scan |
| Prefix search | ❌ | Trie | HashMap không hỗ trợ prefix |

---

## Luyện nhận diện Pattern

### Bài 1: Two Sum (LeetCode #1)

Cho mảng `nums` và `target`, tìm 2 index sao cho `nums[i] + nums[j] = target`.

**Gợi ý:** Với mỗi `num`, complement = `target - num`. Check complement có trong HashMap không? Nếu có → trả về. Nếu chưa → insert num vào map.

### Bài 2: Group Anagrams (LeetCode #49)

Cho mảng strings, group các anagram lại. Ví dụ:
```
Input:  ["eat","tea","tan","ate","nat","bat"]
Output: [["eat","tea","ate"], ["tan","nat"], ["bat"]]
```

**Gợi ý:** 2 string là anagram nếu sorted bằng nhau. Dùng sorted string làm key trong `HashMap<String, Vec<String>>`.

### Bài 3: LRU Cache (LeetCode #146)

Implement cache có capacity cố định, khi đầy thì xóa phần tử **ít dùng nhất** (Least Recently Used). `get()` và `put()` đều O(1).

**Gợi ý:** HashMap cho O(1) lookup + doubly linked list cho O(1) move-to-front. Đây là combo HashMap + LinkedList -- 2 cấu trúc bạn đã học.

---

## HashMap trong Rust ecosystem

### Standard library

- `std::collections::HashMap` -- SipHash, hashbrown/SwissTable backend
- `std::collections::HashSet` -- `HashMap<K, ()>` wrapper

### Crates phổ biến

- `indexmap::IndexMap` -- giữ insertion order, iterate theo thứ tự insert
- `dashmap::DashMap` -- concurrent HashMap, lock-free reads. Dùng khi multi-thread.
- `rustc_hash::FxHashMap` -- hash nhanh hơn SipHash, dùng trong Rust compiler (không chống HashDoS)
- `ahash::AHashMap` -- hash nhanh, dùng bởi hashbrown (backend của std HashMap)

### KaCrab -- HashMap trong thực tế

Trong project KaCrab (Kafka client bằng Rust), HashMap xuất hiện khắp nơi:

- **Topic metadata cache**: `HashMap<TopicName, PartitionConfig>` -- O(1) tra cứu config
- **Consumer group tracking**: `HashMap<MemberId, Assignment>` -- theo dõi member nào xử lý partition nào
- **In-flight request tracking**: `HashMap<CorrelationId, Callback>` -- match response với request

Tất cả đều dùng HashMap vì cần O(1) lookup by key -- đúng bài toán mà HashMap sinh ra để giải.

---

## Chương tiếp theo

HashMap cho O(1) lookup bằng cách biến key thành index. Chương tiếp theo sẽ giới thiệu **Hash Set** -- khi bạn chỉ cần biết "phần tử này có tồn tại không?" mà không cần value. Sau đó, chúng ta sẽ khám phá **Bloom Filter** -- cấu trúc xác suất cho phép false positive nhưng KHÔNG bao giờ false negative, tiết kiệm memory cực kỳ. Và cuối cùng là **Consistent Hashing** -- kỹ thuật phân tán data trên nhiều server, nền tảng của mọi hệ thống distributed từ Cassandra đến Redis Cluster.

Xa hơn nữa, khi bạn qua phần Graph, bạn sẽ thấy HashMap xuất hiện lại -- biểu diễn adjacency list, tracking visited nodes trong BFS/DFS. HashMap là công cụ nền tảng mà bạn sẽ dùng ở mọi nơi.

---

---

[← Trie](../03-trees-and-heaps/08-trie.md) | [Hash Set →](./02-hash-set.md)
