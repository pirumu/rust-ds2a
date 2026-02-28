# Bloom Filter

## Đây là gì?

> **Đừng lo!** Bloom Filter nghe tên lạ, và ý tưởng "chấp nhận sai" có vẻ phản trực giác. Nhưng thực ra cấu trúc cực kỳ đơn giản: chỉ là **mảng bit + vài hash function**. Insert = bật vài bit. Lookup = check vài bit. Code ngắn hơn hầu hết mọi cấu trúc trong series này. Phần khó duy nhất là hiểu **tại sao** chấp nhận sai lại hữu ích -- và đó là câu hỏi về engineering trade-off, không phải thuật toán phức tạp. Bloom Filter xuất hiện trong **phỏng vấn system design** (không phải coding), và chạy trong production ở hầu hết hệ thống lớn: Google Bigtable, Apache Cassandra, LevelDB, Chrome browser...

Tưởng tượng bạn có một bảng đăng ký hiến máu. Mỗi người đến đăng ký, bạn không ghi tên họ (tốn giấy lắm). Thay vào đó, bạn dùng 3 cây bút màu khác nhau, mỗi bút tô vào một ô trên bảng theo "công thức" riêng từ tên người đó. Ví dụ: "Tuấn" → bút đỏ tô ô 3, bút xanh tô ô 7, bút vàng tô ô 11.

Khi ai đó hỏi "Tuấn đăng ký chưa?", bạn kiểm tra 3 ô đó:
- **Có ô nào còn trắng?** → **Chắc chắn chưa đăng ký**. Tin được 100%.
- **Cả 3 ô đều đã tô?** → **Có thể đã đăng ký**. Nhưng đôi khi những ô đó bị tô bởi người KHÁC, không phải Tuấn. Nhầm!

Đó chính là **Bloom filter** -- cấu trúc dữ liệu xác suất, cực kỳ tiết kiệm bộ nhớ, dùng để kiểm tra "phần tử này có trong tập hợp không?"

Đặc điểm quan trọng:
- **Không bao giờ sai khi nói "không"** (no false negative) -- ô trắng thì chắc chắn chưa ai tô
- **Đôi khi sai khi nói "có"** (có false positive) -- ô đã tô có thể do người khác tô trùng

## Từ HashSet đến Bloom Filter

Chương trước, ta học HashSet -- cho `contains` O(1), chính xác 100%. Tuyệt vời. Nhưng...

Lưu **1 tỷ URL** (spam filter) trong HashSet? Mỗi URL trung bình ~100 bytes → **100 GB RAM**. Máy chủ email không có nhiều RAM đến vậy.

**Câu hỏi**: nếu bạn chấp nhận **đôi khi nhầm** (1% trường hợp), bạn có thể giảm memory xuống **~1.2 GB** -- ít hơn **80 lần**. Đáng không?

| | HashSet | Bloom Filter |
|---|---------|-------------|
| Contains accuracy | **100%** | ~99% (tunable) |
| Memory (1M items) | ~50 MB | **~1.2 MB** (40x ít hơn) |
| Memory (1B items) | ~50 GB | **~1.2 GB** |
| Insert | O(1) | O(k) ≈ O(1) |
| Lookup | O(1) | O(k) ≈ O(1) |
| Delete | ✅ | ❌ (standard), ⚠️ (counting variant) |
| False negative | ❌ Không | ❌ **Không bao giờ** |
| False positive | ❌ Không | ⚠️ Có (~1%) |
| Enumerate items | ✅ iterate | ❌ Không thể |

Bloom Filter là **HashSet với deal tuyệt vời**: giảm 40x memory, đổi lại ~1% false positive. Nhiều bài toán thực tế chấp nhận deal này.

Nhớ lại: HashSet dùng hash function để ánh xạ phần tử vào bucket. Bloom Filter cũng dùng hash function -- nhưng thay vì lưu phần tử, nó chỉ **bật bit**. Không lưu data = tiết kiệm memory cực lớn. Cái giá? Không thể biết chính xác phần tử nào đã insert.

## Hoạt động như thế nào?

### Bước 1: Mảng bit

Bloom filter bắt đầu với một mảng bit (mảng toàn số 0):

```
Index:   0   1   2   3   4   5   6   7   8   9  10  11
       ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
Bits:  │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │ 0 │
       └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
```

### Bước 2: Thêm phần tử (insert)

Để thêm 1 phần tử, ta dùng **k hash function** khác nhau. Mỗi hash function cho ra 1 vị trí. Ta bật bit ở vị trí đó lên 1.

Thêm `"apple"` với k=3 hash function, ra vị trí 1, 4, 9:

```
       h1("apple")=1    h2("apple")=4    h3("apple")=9
              │                │                │
              v                v                v
       ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
Bits:  │ 0 │ 1 │ 0 │ 0 │ 1 │ 0 │ 0 │ 0 │ 0 │ 1 │ 0 │ 0 │
       └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
```

Thêm `"banana"` với hash ra vị trí 1, 5, 10:

```
       ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
Bits:  │ 0 │ 1 │ 0 │ 0 │ 1 │ 1 │ 0 │ 0 │ 0 │ 1 │ 1 │ 0 │
       └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
             ^               ^   ^               ^   ^
             |               |   |               |   |
          chung với       từ      từ           từ    từ
          "apple"       "apple" "banana"     "apple" "banana"
```

Vị trí 1 bị chung giữa "apple" và "banana". Không sao cả, bình thường.

### Bước 3: Kiểm tra (lookup)

Kiểm tra `"cherry"` có trong set không? Hash ra vị trí 2, 4, 7:

```
       ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
Bits:  │ 0 │ 1 │ 0 │ 0 │ 1 │ 1 │ 0 │ 0 │ 0 │ 1 │ 1 │ 0 │
       └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
                 ^           ^               ^
             bit[2]=0    bit[4]=1        bit[7]=0
```

Bit 2 = 0! Vậy `"cherry"` **chắc chắn không** có trong set. Dừng luôn, không cần kiểm tra thêm.

### Bước 4: False positive -- khi bảo vệ nhầm

Kiểm tra `"date"` -- hash ra vị trí 1, 5, 9:

```
       ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
Bits:  │ 0 │ 1 │ 0 │ 0 │ 1 │ 1 │ 0 │ 0 │ 0 │ 1 │ 1 │ 0 │
       └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
             ^               ^                   ^
         bit[1]=1        bit[5]=1            bit[9]=1
```

Tất cả bit đều = 1! Nhưng ta chưa bao giờ thêm `"date"`. Các bit này được bật bởi "apple" và "banana". Đây là **false positive** -- bảo vệ nói "có thể có" nhưng thực ra không có.

### Công thức tỷ lệ false positive

```
p ≈ (1 - e^(-k*n/m))^k

Trong đó:
  m = kích thước mảng bit
  n = số phần tử đã thêm
  k = số hash function
```

Mối quan hệ:
- **m lớn hơn** (nhiều bit hơn) → ít false positive hơn
- **n lớn hơn** (nhiều phần tử hơn) → nhiều false positive hơn
- **k tối ưu** = (m/n) * ln(2) ≈ 0.693 * (m/n)

### Mẹo double hashing

Thay vì cài k hash function riêng biệt (phiền phức!), ta dùng **double hashing**: tính 2 hash `h1` và `h2`, rồi kết hợp:

```
h(i) = h1 + i * h2     (với i = 0, 1, 2, ..., k-1)
```

Chỉ cần 2 hash function, tạo ra được k hash khác nhau. Đơn giản mà hiệu quả.

## Tại sao false positive chấp nhận được?

Đây là insight quan trọng nhất của chương: **chấp nhận sai nhỏ không phải là yếu kém -- đó là engineering trade-off thông minh**.

Bloom filter dùng làm **bước lọc sơ bộ** (pre-filter):
- Nói **"không"** → tin tuyệt đối → **skip** thao tác tốn kém
- Nói **"có"** → check lại bằng cách chính xác (disk read, DB query, ...)

Tiết kiệm phần lớn thao tác tốn kém, đổi lại vài thao tác thừa rẻ tiền.

### Spam filter

```
False positive: email tốt bị đánh dấu "có thể spam"
  → kiểm tra thêm bằng cách chính xác = mất vài ms
  → Không mất email. Chỉ chậm 1 chút.

False negative: email spam lọt qua?
  → KHÔNG BAO GIỜ xảy ra với Bloom filter!
  → Nếu email nằm trong danh sách spam, Bloom filter luôn bắt được.
```

### Cache check

```
False positive: tưởng có trong cache → đi tìm → không có → đọc DB
  → Lãng phí 1 cache lookup (~1μs). Trivial.

False negative: tưởng không có → bỏ qua cache → đọc DB trực tiếp?
  → Bloom KHÔNG cho false negative
  → Không bao giờ miss cache nếu data thực sự có trong cache.
```

### Database (Bigtable / Cassandra)

```
Mỗi SSTable file có 1 Bloom filter.
Query "row X có trong SSTable này không?"

False positive: mở file đọc → row không có → lãng phí 1 disk read (~10ms)
No Bloom filter: phải mở MỌI SSTable file → lãng phí N disk reads

Bloom filter giảm disk reads từ N xuống ~1.01 (gần 1).
Tiết kiệm (N-1) × 10ms mỗi query!
```

### Pattern chung

```
┌─────────────────────┐
│   Bloom Filter      │
│   "X có không?"     │
└─────────┬───────────┘
          │
    ┌─────┴─────┐
    │           │
  "Không"     "Có thể"
    │           │
  SKIP!     Verify bằng
  (tiết      cách chính
  kiệm!)    xác (DB, disk)
```

## Tuning -- chọn m, k, n

Doc có công thức, nhưng quan trọng hơn là **biết cách dùng**.

### Cho trước: n phần tử, mong muốn false positive rate p

**Bước 1**: Tính m (số bit)

```
m = -(n × ln(p)) / (ln(2))²

Ví dụ: n = 1,000,000 (1M), p = 1% (0.01)
m = -(1M × ln(0.01)) / (0.693)²
m = -(1M × (-4.605)) / 0.480
m ≈ 9,585,058 bits ≈ 1.2 MB
```

**Bước 2**: Tính k (số hash function)

```
k = (m/n) × ln(2)
k = (9.58M / 1M) × 0.693
k ≈ 6.64 → làm tròn thành 7
```

**Bước 3**: Verify

```
p ≈ (1 - e^(-7 × 1M / 9.58M))^7 ≈ 0.0082 ≈ 0.82% ✓
```

### Bảng quick reference

| Items (n) | Target p | Bits (m) | Hashes (k) | Memory |
|-----------|----------|----------|------------|--------|
| 10K | 1% | 96K | 7 | 12 KB |
| 100K | 1% | 960K | 7 | 120 KB |
| 1M | 1% | 9.6M | 7 | **1.2 MB** |
| 1M | 0.1% | 14.4M | 10 | 1.8 MB |
| 10M | 1% | 96M | 7 | 12 MB |
| 1B | 1% | 9.6B | 7 | **1.2 GB** |

**Rule of thumb**: ~10 bits per item cho 1% false positive. Muốn 0.1%? ~14.4 bits per item. Rẻ đến kinh ngạc.

Quay lại ẩn dụ bảng hiến máu: bảng cần ~10 ô cho mỗi người đăng ký. 1 triệu người = 10 triệu ô. Mỗi ô chỉ 1 bit = 1.2 MB. Ghi tên đầy đủ (HashSet)? 50+ MB.

## Counting Bloom Filter

Bloom filter tiêu chuẩn có 1 hạn chế lớn: **không xóa được**. Tại sao? Vì 1 bit có thể được bật bởi nhiều phần tử. Đặt bit = 0 có thể "xóa nhầm" phần tử khác.

**Counting Bloom Filter** giải quyết bằng cách thay mỗi bit bằng **counter**:

```
Standard Bloom Filter:           Counting Bloom Filter:
  Mỗi slot = 1 bit                Mỗi slot = counter (thường 4 bits)
  Insert: set bit = 1              Insert: counter += 1
  Delete: ❌ không thể              Delete: counter -= 1
  Memory: m bits                   Memory: m × 4 bits (4x hơn)
```

### Trace từng bước

```
Bắt đầu: tất cả counter = 0

Insert "apple" → positions [1, 4, 9]: counter += 1
  Counter: [0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0]

Insert "banana" → positions [1, 5, 10]: counter += 1
  Counter: [0, 2, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0]
                ^
           2 = apple + banana cùng hash vào vị trí 1

Delete "apple" → positions [1, 4, 9]: counter -= 1
  Counter: [0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0]
                ^         ^               ^
           1 (còn banana) 0 (apple đã xóa) 0

might_contain("apple")  → counter[4] = 0 → FALSE ✓ (đã xóa)
might_contain("banana") → counter[1]=1, [5]=1, [10]=1 → TRUE ✓
```

**Trade-off**: 4x memory nhưng cho phép delete. Dùng khi items có thể bị xóa (ví dụ: URL bị gỡ khỏi danh sách spam, session hết hạn).

## Code Rust

Code đầy đủ nằm trong `src/bloom_filter.rs`.

### Cấu trúc

```rust
pub struct BloomFilter {
    bits: Vec<bool>,      // mảng bit
    size: usize,          // kích thước mảng (m)
    num_hashes: usize,    // số hash function (k)
    count: usize,         // số phần tử đã thêm (n)
}
```

### Thêm phần tử

```rust
pub fn insert(&mut self, item: &str) {
    for i in 0..self.num_hashes {
        let idx = self.hash(item, i);
        self.bits[idx] = true;     // bật bit lên 1
    }
    self.count += 1;
}
```

Duyệt qua k hash function, bật k bit tương ứng.

### Kiểm tra phần tử

```rust
pub fn might_contain(&self, item: &str) -> bool {
    for i in 0..self.num_hashes {
        let idx = self.hash(item, i);
        if !self.bits[idx] {
            return false;   // bit = 0 → chắc chắn KHÔNG có
        }
    }
    true  // tất cả bit = 1 → CÓ THỂ có
}
```

Để ý tên hàm: `might_contain` -- "có thể chứa". Không phải `contains`. Tên hàm đã cảnh báo bạn rồi.

### Double hashing

```rust
fn hash(&self, item: &str, i: usize) -> usize {
    let h1 = self.fnv1a(item);            // hash function 1
    let h2 = self.djb2(item);             // hash function 2
    let combined = h1.wrapping_add(i.wrapping_mul(h2));  // h1 + i * h2
    combined % self.size                   // giới hạn trong mảng
}
```

Dùng FNV-1a và DJB2 làm 2 hash function nền.

### Ước tính tỷ lệ false positive

```rust
pub fn false_positive_rate(&self) -> f64 {
    let k = self.num_hashes as f64;
    let n = self.count as f64;
    let m = self.size as f64;
    (1.0 - (-k * n / m).exp()).powf(k)
}
```

## Bloom Filter trong thực tế

### Database -- LSM Tree / Bigtable / Cassandra

Đây là ứng dụng quan trọng nhất. LSM Tree (Log-Structured Merge Tree) lưu data thành nhiều file SSTable:

```
LSM Tree structure:
  Level 0: [SSTable 1] [SSTable 2] [SSTable 3]
  Level 1: [SSTable 4] [SSTable 5]
  Level 2: [SSTable 6]

Query "find key X":
  Không có Bloom filter:
    → Mở tất cả 6 SSTable → 6 disk reads → 60ms

  Có Bloom filter (mỗi SSTable có 1 Bloom filter in-memory):
    → Check Bloom filter của SSTable 1: "definitely not" → skip
    → Check Bloom filter của SSTable 2: "maybe"         → mở file → 1 disk read
    → Check Bloom filter của SSTable 3: "definitely not" → skip
    → Check Bloom filter của SSTable 4: "definitely not" → skip
    → Check Bloom filter của SSTable 5: "definitely not" → skip
    → Check Bloom filter của SSTable 6: "definitely not" → skip

    Kết quả: 1 disk read thay vì 6 → tiết kiệm 50ms per query!
```

Google Bigtable, Apache Cassandra, LevelDB, RocksDB đều dùng pattern này. Mỗi ngày query hàng tỷ lần → tiết kiệm khổng lồ.

### Web Crawler -- URL dedup

Google crawl hàng tỷ URL. Trước khi crawl mỗi URL, cần check "URL này đã crawl chưa?":
- HashSet 10 tỷ URL = ~1 TB RAM
- Bloom filter = ~12 GB (chỉ 1.2% bộ nhớ)

False positive (crawl lại URL đã crawl)? Lãng phí 1 request. Không nghiêm trọng.
False negative (bỏ sót URL chưa crawl)? Không bao giờ xảy ra.

### CDN / Cache -- cache digest

CDN node chia sẻ Bloom filter với nhau: "tôi có những content nào".

```
Node A cần file X:
  1. Check Bloom filter của Node B (lưu local): "B có X không?"
  2. "Definitely not" → skip, không gửi network request
  3. "Maybe" → hỏi Node B qua network

Tiết kiệm hàng triệu network requests mỗi giây.
Bloom filter chỉ ~1 MB, truyền qua network rất nhanh.
```

### Bitcoin -- SPV clients

Bitcoin SPV (Simplified Payment Verification) clients không download toàn bộ blockchain (~500 GB). Thay vào đó, client gửi Bloom filter cho full node: "tôi quan tâm đến những address/transaction nào". Full node dùng Bloom filter để lọc, chỉ gửi transactions liên quan. Tiết kiệm bandwidth khổng lồ.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ |
|----------|-----------|--------|
| `insert` | O(k) | -- |
| `might_contain` | O(k) | -- |
| **Tổng bộ nhớ** | -- | **O(m)** |

Trong đó k = số hash function, m = kích thước mảng bit. Cả hai đều là hằng số chọn lúc khởi tạo. Nên thực tế mọi thao tác đều **O(1)**.

### So sánh bộ nhớ

| Cấu trúc | 1 triệu phần tử | False positive |
|-----------|------------------|----------------|
| HashSet | ~50+ MB | 0% (chính xác) |
| Bloom filter | ~1.2 MB | 1% |

Bloom filter dùng ít bộ nhớ hơn **~40 lần**, đổi lại chấp nhận 1% sai sót. Đáng không? Tùy bài toán.

## Ví dụ

```rust
use rust_ds2a::bloom_filter::BloomFilter;

// Tạo filter: 10,000 bit, 5 hash function
let mut filter = BloomFilter::new(10_000, 5);

// Thêm vài phần tử
filter.insert("apple");
filter.insert("banana");
filter.insert("cherry");

// Kiểm tra
assert!(filter.might_contain("apple"));    // true -- đã thêm
assert!(filter.might_contain("banana"));   // true -- đã thêm

// "date" chưa thêm → THƯỜNG là false
// nhưng CÓ THỂ là true (false positive)
let result = filter.might_contain("date");
println!("date: {}", result);  // rất có thể false

// Xem tỷ lệ false positive hiện tại
let rate = filter.false_positive_rate();
println!("Tỷ lệ false positive: {:.6}", rate);  // rất nhỏ với 10k bit
```

### Ví dụ thực tế: Spam filter đơn giản

```rust
// Giả sử ta có danh sách 100K email spam đã biết
// HashSet: ~5 MB. Bloom filter: ~120 KB (40x nhỏ hơn)
let mut spam_filter = BloomFilter::new(960_000, 7); // ~120 KB, 1% false positive

// Load danh sách spam
spam_filter.insert("buy-cheap-watches@spam.com");
spam_filter.insert("nigerian-prince@scam.net");
// ... 99,998 emails khác ...

// Khi email mới đến:
fn check_email(filter: &BloomFilter, sender: &str) -> &str {
    if !filter.might_contain(sender) {
        "✅ Chắc chắn không spam → inbox"   // tin tuyệt đối
    } else {
        "⚠️ Có thể spam → kiểm tra kỹ hơn" // verify thêm
    }
}
```

## Những cái bẫy hay gặp

### ❌ Nghĩ false positive = false negative

**Sai**: "Bloom filter đôi khi bỏ sót phần tử đã thêm"

**Đúng**: Bloom filter **KHÔNG BAO GIỜ** cho false negative. Nếu phần tử đã insert, `might_contain` luôn trả về `true`. Chỉ có false positive (nói "có" khi thực ra "không"). Hai hướng sai hoàn toàn khác nhau về hậu quả.

💡 Nhớ: "Không" = tin tuyệt đối. "Có" = kiểm tra lại.

### ❌ Quên rằng Bloom filter không enumerable

**Sai**: "Cho tôi danh sách tất cả phần tử trong Bloom filter"

**Đúng**: Bloom filter chỉ lưu bit, không lưu data gốc. Không thể liệt kê phần tử đã insert. Chỉ trả lời "X có trong đây không?". Nếu cần iterate → dùng HashSet.

💡 Quay lại ẩn dụ bảng hiến máu: bảng chỉ có ô tô màu, không có tên. Không thể xem bảng mà biết "ai đã đăng ký".

### ❌ Không tuning m và k

**Sai**: Dùng Bloom filter với m quá nhỏ → false positive rate cao đến mức vô dụng.

**Đúng**: Luôn tính m từ n (số phần tử dự kiến) và p (false positive rate mong muốn) trước khi khởi tạo. Rule of thumb: ~10 bits per item cho 1% false positive.

💡 Bloom filter với m quá nhỏ giống bảng hiến máu chỉ có 10 ô cho 1000 người -- tất cả ô đều tô hết, mọi lookup đều trả về "có thể có". Vô nghĩa.

### ❌ Cố xóa phần tử trong standard Bloom filter

**Sai**: "Xóa apple bằng cách đặt bit[1]=0, bit[4]=0, bit[9]=0"

**Đúng**: Bit[1] được chia sẻ giữa "apple" và "banana". Đặt bit[1]=0 sẽ "xóa nhầm" banana! Cần **Counting Bloom Filter** nếu muốn delete.

💡 Trên bảng hiến máu: ô 1 bị tô bởi cả Tuấn và Minh. Xóa ô 1 = xóa cả hai. Không được!

## Khi nào dùng / không nên dùng?

| Tình huống | Bloom Filter? | Thay bằng gì? | Tại sao? |
|------------|:---:|---------------|----------|
| Pre-filter trước disk/network lookup | ✅ | -- | Tiết kiệm I/O tốn kém |
| Dedup URL/email (tỷ items, memory limited) | ✅ | -- | 40x ít memory hơn HashSet |
| Database SSTable lookup | ✅ | -- | Giảm disk reads |
| Cache digest giữa nodes | ✅ | -- | Compact để truyền qua network |
| System design interview | ✅ | -- | Rất hay được hỏi! |
| Cần chính xác 100% | ❌ | HashSet | Bloom có false positive |
| Cần xóa phần tử | ❌ | HashSet / Counting BF | Standard BF không delete |
| Cần liệt kê phần tử | ❌ | HashSet | BF không enumerable |
| Data nhỏ (< 10K items) | ❌ | HashSet | Memory saving không đáng |

**Tóm lại**: Bloom filter tỏa sáng khi data **lớn**, false positive **chấp nhận được**, và bạn cần **tiết kiệm memory** hoặc **tránh I/O tốn kém**.

## Luyện nhận diện Pattern

### Bài 1: URL Shortener (System Design)

Hệ thống tạo short URL (như bit.ly). Khi tạo short code mới, cần check "code này đã dùng chưa?" trước khi insert vào DB. Có hàng tỷ codes.

**Vấn đề**: Query DB mỗi lần tạo code mới = chậm và tốn resource.

**Gợi ý**: Bloom filter làm pre-check:
- "Definitely not used" → dùng luôn, không cần query DB
- "Maybe used" → query DB để verify
- ~99% code mới sẽ là unique → 99% trường hợp skip DB query

<details>
<summary>Lời giải chi tiết</summary>

```
Bloom filter chứa tất cả short codes đã dùng.
n = 1 tỷ codes, p = 0.1% → m ≈ 14.4 tỷ bits ≈ 1.8 GB, k = 10

Tạo code mới "abc123":
  might_contain("abc123") = false → INSERT luôn, 0 DB query ✓
  might_contain("xyz789") = true  → SELECT từ DB → không có → INSERT

Tiết kiệm ~99% DB queries cho operation tạo code.
Khi insert thành công, cũng insert vào Bloom filter.
```

</details>

### Bài 2: Spell Checker (System Design)

Browser cần check chính tả offline (không có internet). Từ điển tiếng Anh ~170K từ. Cách lưu compact nhất trên device?

<details>
<summary>Lời giải chi tiết</summary>

```
HashSet: ~170K words × ~30 bytes/word = ~5 MB
Bloom filter: n=170K, p=1% → m ≈ 1.7M bits ≈ 200 KB

200 KB vs 5 MB = 25x nhỏ hơn. Trên mobile, memory matters.

False positive: từ sai chính tả nhưng Bloom filter nói "có thể đúng"
  → 1% từ sai không bị gạch chân. Chấp nhận được.
False negative: từ đúng bị gạch chân?
  → KHÔNG BAO GIỜ xảy ra. Từ trong từ điển luôn được nhận dạng.
```

</details>

### Bài 3: Tính toán

Cho n = 5M items, target false positive rate p = 0.5%.

**a)** Tính m (bits), k (hashes), và memory cần thiết.

**b)** Nếu sau đó insert thêm 5M items (tổng 10M) mà không resize, false positive rate mới là bao nhiêu?

<details>
<summary>Lời giải chi tiết</summary>

**a)**
```
m = -(5M × ln(0.005)) / (ln(2))²
m = -(5M × (-5.298)) / 0.480
m ≈ 55,189,583 bits ≈ 6.9 MB
k = (m/n) × ln(2) = (55.19M / 5M) × 0.693 ≈ 7.65 → 8
```

**b)** Với n = 10M (gấp đôi), m và k giữ nguyên:
```
p = (1 - e^(-8 × 10M / 55.19M))^8
p = (1 - e^(-1.449))^8
p = (1 - 0.235)^8
p = (0.765)^8
p ≈ 0.111 = 11.1%
```

False positive rate tăng từ 0.5% lên **11.1%** -- gần như vô dụng! Bài học: **luôn sizing Bloom filter cho expected capacity**. Quá tải = false positive rate tăng nhanh chóng.

</details>

## Bloom Filter trong Rust ecosystem

### Crates phổ biến

- **`bloomfilter`** -- basic Bloom filter, đơn giản, dễ dùng
- **`bloom`** -- optimized, hỗ trợ counting variant
- **`probabilistic-collections`** -- bộ sưu tập đầy đủ: Bloom filter, Cuckoo filter, Count-Min Sketch, HyperLogLog

### Cuckoo Filter -- thế hệ tiếp theo

**Cuckoo Filter** là variant mới hơn của Bloom filter:
- Hỗ trợ **delete** (không cần counting)
- False positive rate tốt hơn ở cùng memory
- Lookup nhanh hơn (cache-friendly)
- Dùng trong nhiều hệ thống mới thay thế Bloom filter

Trade-off: Insert có thể fail nếu filter quá đầy (Bloom filter không bao giờ fail insert).

### Liên hệ Kafka

Nếu bạn đang build hệ thống message queue (như KaCrab -- Kafka client bằng Rust):
- Consumer cần check "message ID này đã processed chưa?" cho exactly-once delivery
- Bloom filter compact hơn HashSet nhiều lần cho dedup tracking
- Kafka log compaction cần check "key này có trong segment không?" → Bloom filter per segment (giống Bigtable SSTable pattern)

## Tiếp theo

Bloom Filter là cấu trúc xác suất đầu tiên trong series -- chấp nhận sai nhỏ để tiết kiệm memory cực lớn. Chương tiếp theo, ta sẽ học **Consistent Hashing** -- kỹ thuật phân phối data đều giữa nhiều server, và xử lý gracefully khi server thêm/bớt. Consistent Hashing dùng trong hầu hết hệ thống phân tán: CDN, distributed cache (Memcached, Redis Cluster), database sharding. Nếu Bloom Filter trả lời "có hay không?", thì Consistent Hashing trả lời "data này nên nằm ở server nào?"

---

---

[← Hash Set](./02-hash-set.md) | [Consistent Hashing →](./04-consistent-hashing.md)
