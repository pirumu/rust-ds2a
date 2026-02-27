# Bloom Filter

## Đây là gì?

Tưởng tượng bạn có một bảng đăng ký hiến máu. Mỗi người đến đăng ký, bạn không ghi tên họ (tốn giấy lắm). Thay vào đó, bạn dùng 3 cây bút màu khác nhau, mỗi bút tô vào một ô trên bảng theo "công thức" riêng từ tên người đó. Ví dụ: "Tuấn" → bút đỏ tô ô 3, bút xanh tô ô 7, bút vàng tô ô 11.

Khi ai đó hỏi "Tuấn đăng ký chưa?", bạn kiểm tra 3 ô đó:
- **Có ô nào còn trắng?** → **Chắc chắn chưa đăng ký**. Tin được 100%.
- **Cả 3 ô đều đã tô?** → **Có thể đã đăng ký**. Nhưng đôi khi những ô đó bị tô bởi người KHÁC, không phải Tuấn. Nhầm!

Đó chính là **Bloom filter** -- cấu trúc dữ liệu xác suất, cực kỳ tiết kiệm bộ nhớ, dùng để kiểm tra "phần tử này có trong tập hợp không?"

Đặc điểm quan trọng:
- **Không bao giờ sai khi nói "không"** (no false negative) -- ô trắng thì chắc chắn chưa ai tô
- **Đôi khi sai khi nói "có"** (có false positive) -- ô đã tô có thể do người khác tô trùng

### Tại sao chấp nhận sai?

Vì đổi lại, Bloom filter **cực kỳ nhỏ gọn**. Lưu 1 triệu phần tử với tỷ lệ sai 1%? Chỉ cần khoảng 1.2 MB. HashSet lưu tương tự? Có thể cần 50+ MB.

### Dùng ở đâu trong thực tế?

- **Spam filter** -- Email này có trong danh sách spam không? "Chắc chắn không" → cho vào inbox. "Có thể có" → kiểm tra kỹ hơn.
- **Cache checking** -- Dữ liệu này có trong cache không? "Chắc chắn không" → đọc từ database luôn, khỏi tìm cache.
- **Database query** -- Google Bigtable dùng Bloom filter để tránh đọc file không chứa dữ liệu cần tìm.

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

### Khi nào dùng Bloom filter thay vì HashSet?

| Tình huống | Dùng cái gì? |
|------------|-------------|
| Cần chính xác 100% | HashSet |
| Bộ nhớ giới hạn, chấp nhận sai nhỏ | Bloom filter |
| Tập dữ liệu cực lớn (hàng triệu) | Bloom filter |
| Cần xóa phần tử | HashSet (Bloom filter không xóa được) |
| Bước lọc sơ bộ trước khi kiểm tra kỹ | Bloom filter |
