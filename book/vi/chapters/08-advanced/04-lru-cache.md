# LRU Cache

> 💡 **Đừng lo lắng:** LRU Cache nghe "xịn" nhưng chỉ là ghép 2 thứ bạn đã biết: HashMap (Phần 4) để tìm nhanh O(1) + Doubly Linked List (Phần 2) để di chuyển nhanh O(1). Hai cấu trúc cũ, chỉ nối lại với nhau. Không có gì mới.

## Đây là gì?

Mở điện thoại lên, vuốt lên để xem **danh sách ứng dụng gần đây** (recent apps). Bạn thấy gì?

- App vừa mở nằm **trên cùng**.
- App lâu không dùng nằm **dưới cùng**.
- Khi bộ nhớ đầy, hệ điều hành tự **đóng app cũ nhất** (ít dùng nhất).

Đó chính là **LRU Cache** — Least Recently Used Cache. "Least Recently Used" = "ít được dùng gần đây nhất". Khi cache đầy, ta đá bỏ phần tử lâu nhất chưa ai đụng tới.

## Tại sao cần?

Cache (bộ nhớ đệm) xuất hiện ở khắp nơi:

- **Browser cache** — Trang web bạn vừa xem được lưu lại. Lần sau truy cập nhanh hơn. Nhưng ổ cứng có hạn, phải xóa trang cũ nhất.
- **OS page cache** — Hệ điều hành giữ dữ liệu từ ổ cứng trong RAM. RAM đầy? Xóa trang ít dùng nhất.
- **Database query cache** — Database lưu kết quả query hay dùng. Bộ nhớ có hạn? Xóa kết quả cũ nhất.
- **CDN cache** — File ảnh, video được cache ở server gần bạn. Server đầy? Xóa file ít ai xem.

Vấn đề chung: **bộ nhớ có hạn, phải chọn cái nào giữ, cái nào xóa**. LRU là chiến lược đơn giản và hiệu quả nhất: xóa cái lâu nhất không ai dùng.

## LRU vs các chiến lược eviction khác

LRU không phải cách duy nhất. Hãy so sánh:

| Chiến lược | Evict cái nào? | Ưu | Nhược |
|-----------|----------------|-----|-------|
| **FIFO** | Cái vào **sớm nhất** | Đơn giản nhất, chỉ cần queue | Không quan tâm tần suất dùng — có thể xóa cái hay dùng |
| **LRU** | Cái **lâu nhất chưa dùng** | Tốt cho hầu hết workload | Không tối ưu nếu có item dùng nhiều nhưng bị gián đoạn |
| **LFU** | Cái **ít dùng nhất** (đếm lần) | Giữ item "hot" rất tốt | Phức tạp hơn, item cũ dùng nhiều khó bị evict dù đã "nguội" |
| **Random** | **Ngẫu nhiên** | Code siêu đơn giản | Không thông minh, nhưng surprisingly không tệ lắm |

**Thực tế:** LRU thắng trong hầu hết trường hợp vì nó đủ thông minh mà lại đơn giản. Đó là lý do CPU cache, database buffer pool, browser cache đều dùng LRU (hoặc biến thể của nó).

## Yêu cầu

LRU Cache cần 2 thao tác, **cả hai đều O(1)**:

| Thao tác | Ý nghĩa |
|----------|---------|
| `get(key)` | Lấy value. Đánh dấu key này là "vừa dùng". |
| `put(key, value)` | Thêm/cập nhật. Nếu đầy → đá bỏ phần tử LRU. |

O(1) cho cả hai! Đây là lý do ta cần kết hợp 2 cấu trúc dữ liệu.

## Thiết kế: Tại sao HashMap + Doubly Linked List?

Hãy thử từng cấu trúc một:

**Chỉ dùng HashMap?**
- Tìm key → O(1). Tốt.
- Nhưng HashMap không biết **thứ tự** — bạn không biết cái nào dùng gần nhất, cái nào lâu nhất. Evict ai bây giờ?

**Chỉ dùng Doubly Linked List?**
- Di chuyển node → O(1). Xóa node → O(1). Tốt.
- Nhưng tìm key → phải duyệt từ đầu đến cuối, O(n). Chậm.

**Kết hợp cả hai:**
- HashMap trỏ vào node trong linked list → tìm nhanh O(1).
- Linked list giữ thứ tự "gần đây → cũ" → biết evict ai trong O(1).

```
                          HashMap
                     ┌──────────────┐
                     │ key → index  │
                     │ ─────────── │
                     │  1  →  [2]  │─ ─ ─ ┐
                     │  2  →  [3]  │─ ─ ─ ─│─ ─ ┐
                     │  3  →  [4]  │─ ─ ─ ─│─ ─ ─│─ ┐
                     └──────────────┘       │     │   │
                                            │     │   │
        Doubly Linked List (Vec-based)      │     │   │
                                            v     v   v
    ┌───────┐    ┌───────┐    ┌───────┐    ┌───────┐    ┌───────┐
    │ HEAD  │◄──►│ key=3 │◄──►│ key=2 │◄──►│ key=1 │◄──►│ TAIL  │
    │ dummy │    │ val=30│    │ val=20│    │ val=10│    │ dummy │
    └───────┘    └───────┘    └───────┘    └───────┘    └───────┘
                  MRU ◄─────────────────────────► LRU
              (mới nhất)                     (cũ nhất)
```

**HEAD** và **TAIL** là 2 node giả (sentinel/dummy). Chúng luôn tồn tại, giúp code đơn giản hơn vì không cần xử lý trường hợp danh sách rỗng.

- Node ngay sau HEAD = **mới nhất** (Most Recently Used).
- Node ngay trước TAIL = **cũ nhất** (Least Recently Used).

## Thao tác Get

`get(key)`:

1. Tìm key trong HashMap → được index.
2. Đọc value từ `nodes[index]`.
3. Gỡ node đó ra khỏi vị trí hiện tại (detach).
4. Gắn node ngay sau HEAD (attach — đánh dấu "vừa dùng").

```
Trước get(2):
    HEAD ◄──► [3] ◄──► [2] ◄──► [1] ◄──► TAIL

Bước 1: Detach node [2]
    HEAD ◄──► [3]  ◄──►  [1] ◄──► TAIL        [2] (rời ra)

Bước 2: Attach [2] ngay sau HEAD
    HEAD ◄──► [2] ◄──► [3] ◄──► [1] ◄──► TAIL
               ^
          bây giờ mới nhất!
```

Tất cả là O(1) — HashMap lookup O(1), detach O(1), attach O(1).

## Thao tác Put

`put(key, value)`:

**Nếu key đã tồn tại:**
1. Cập nhật value.
2. Gỡ node ra, gắn lại sau HEAD (giống `get`).

**Nếu key mới:**
1. Nếu cache đầy → **evict** (xóa node ngay trước TAIL + xóa key trong HashMap).
2. Tạo node mới, thêm vào Vec.
3. Gắn sau HEAD.
4. Thêm key → index vào HashMap.

```
Cache đầy (capacity=2): HEAD ◄──► [2] ◄──► [1] ◄──► TAIL

put(3, 30):

Bước 1: Evict LRU — node trước TAIL là [1]
    HEAD ◄──► [2] ◄──► TAIL                    [1] bị xóa

Bước 2: Thêm node [3] sau HEAD
    HEAD ◄──► [3] ◄──► [2] ◄──► TAIL
               ^
             mới nhất
```

## Vec-based Doubly Linked List — Giải pháp "Rust-friendly"

### Tại sao Doubly Linked List khó trong Rust?

Trong C/C++, doubly linked list dùng pointer. Dễ. Nhưng Rust có **ownership rules** — một giá trị chỉ có 1 owner. Mà mỗi node trong DLL bị trỏ bởi **cả node trước lẫn node sau**. Rust sẽ không cho phép.

Ba cách giải quyết:

| Cách | Ưu | Nhược |
|------|-----|-------|
| **Vec-based** (dùng index) | An toàn, đơn giản, cache-friendly | Không thể thực sự xóa node (chỉ detach) |
| `Rc<RefCell<>>` | Đúng kiểu linked list | Nhiều overhead, code dài dòng |
| `unsafe` pointer | Hiệu suất cao nhất | Dễ bug, cần kinh nghiệm |

**Trong bài này ta dùng Vec-based** — đơn giản nhất, an toàn nhất, và đủ tốt cho LRU Cache.

### Cách hoạt động

Mỗi node lưu `prev` và `next` là **index** trong Vec (không phải pointer):

```rust
struct Node {
    key: i32,
    value: i32,
    prev: usize,   // index trong Vec
    next: usize,   // index trong Vec
}
```

- Index 0 = HEAD (dummy)
- Index 1 = TAIL (dummy)
- Index 2, 3, 4, ... = các node thật

Detach node tại index `i`:

```rust
fn detach(&mut self, idx: usize) {
    let prev = self.nodes[idx].prev;
    let next = self.nodes[idx].next;
    self.nodes[prev].next = next;
    self.nodes[next].prev = prev;
}
```

Attach node sau HEAD:

```rust
fn attach_after_head(&mut self, idx: usize) {
    let head = 0;
    let old_first = self.nodes[head].next;
    self.nodes[idx].prev = head;
    self.nodes[idx].next = old_first;
    self.nodes[head].next = idx;
    self.nodes[old_first].prev = idx;
}
```

Gọn, an toàn, không cần `unsafe`.

## Những lỗi hay gặp

### Pitfall 1: Capacity = 0

❌ **Sai:** Cho phép tạo cache với capacity 0, rồi `put` crash.

✅ **Đúng:** Kiểm tra `capacity > 0` ngay khi tạo.

💡 **Tại sao:** Cache capacity 0 vô nghĩa — mỗi `put` sẽ phải evict ngay lập tức, hoặc tệ hơn, access vào node không tồn tại.

```rust
pub fn new(capacity: usize) -> Self {
    assert!(capacity > 0, "LRU cache capacity must be > 0");
    // ...
}
```

### Pitfall 2: Update key nhưng quên move to front

❌ **Sai:** `put(key, new_value)` chỉ update value, không di chuyển node.

✅ **Đúng:** `put` key đã tồn tại = update value **VÀ** move to front.

💡 **Tại sao:** `put` cũng đánh dấu key là "vừa dùng". Nếu không move to front, key đó có thể bị evict sai.

```rust
// SAI — chỉ update value
if let Some(&idx) = self.map.get(&key) {
    self.nodes[idx].value = value;
    // quên detach + attach!
}

// ĐÚNG — update value + move to front
if let Some(&idx) = self.map.get(&key) {
    self.nodes[idx].value = value;
    self.detach(idx);           // gỡ ra
    self.attach_after_head(idx); // gắn lại đầu
}
```

### Pitfall 3: Sai thứ tự pointer khi detach/attach

❌ **Sai:** Update pointer của node mới trước khi lưu pointer cũ.

✅ **Đúng:** Luôn đọc hết giá trị cũ trước, rồi mới ghi giá trị mới.

💡 **Tại sao:** Nếu ghi đè `next` trước khi đọc `old_next`, bạn mất thông tin và linked list bị đứt.

```
Sai:
    nodes[head].next = idx;           // ghi đè!
    let old_first = nodes[head].next; // = idx, không phải node cũ!

Đúng:
    let old_first = nodes[head].next; // đọc trước
    nodes[head].next = idx;           // ghi sau
```

## LRU Cache trong thực tế — Production Systems

LRU Cache không phải bài tập lý thuyết. Nó chạy trong máy tính của bạn ngay lúc này:

### CPU Cache

CPU có L1/L2/L3 cache. Khi CPU đọc dữ liệu từ RAM, nó lưu vào cache. Cache đầy? Evict dòng ít dùng nhất. Thuật toán thực tế là **pseudo-LRU** (LRU gần đúng) vì LRU chính xác tốn quá nhiều transistor.

### Database Buffer Pool

MySQL, PostgreSQL giữ các trang dữ liệu (data page) trong RAM. Mỗi trang ~16KB. Buffer pool có thể lên tới vài GB. Khi đầy, evict trang LRU. MySQL dùng biến thể gọi là **LRU with midpoint insertion** — trang mới vào giữa list thay vì đầu, tránh full table scan đẩy hết hot page ra.

### CDN Cache (Cloudflare, Akamai)

Ảnh, video, CSS, JS được cache ở server gần user. Mỗi edge server có dung lượng giới hạn. File ít ai truy cập bị evict trước.

### Browser Cache

Chrome, Firefox cache file CSS/JS/ảnh. Bạn có thể thấy `304 Not Modified` trong DevTools — nghĩa là browser đang dùng cache thay vì download lại.

## Bảng độ phức tạp

| Thao tác | Time | Space | Giải thích |
|----------|------|-------|------------|
| `new(cap)` | O(1) | O(cap) | Khởi tạo HashMap + Vec |
| `get(key)` | O(1)* | O(1) | HashMap lookup + detach/attach |
| `put(key, val)` | O(1)* | O(1) | HashMap lookup + linked list ops |
| `len()` | O(1) | O(1) | Đọc HashMap.len() |

\* Amortised O(1) — HashMap có thể resize nhưng trung bình vẫn O(1).

## Khi nào dùng LRU Cache?

| Tình huống | Dùng LRU? | Lý do |
|-----------|-----------|-------|
| Cache API response, hạn chế bộ nhớ | Dùng | Classic use case |
| Cache database query results | Dùng | Rất phổ biến, hầu hết DB đều làm |
| Cache kết quả tính toán đắt (memoization) | Dùng | Bounded memoization |
| Dữ liệu có "hot set" rõ ràng (vài item dùng rất nhiều) | Cân nhắc **LFU** | LFU giữ hot item tốt hơn |
| Không cần eviction (bộ nhớ đủ) | **Không** | Dùng HashMap thường, đơn giản hơn |
| Cần TTL (time-to-live) | **Không** đủ | LRU không biết thời gian — kết hợp LRU + TTL |
| Cache size rất nhỏ (< 10 items) | Không cần thiết | Dùng Vec + linear scan cũng được |

## Chạy thử trong Rust

```rust
use rust_ds2a::lru_cache::LRUCache;

let mut cache = LRUCache::new(2);

cache.put(1, 1);
cache.put(2, 2);
assert_eq!(cache.get(1), Some(1));   // key 1 lên đầu

cache.put(3, 3);                     // cache đầy → evict key 2
assert_eq!(cache.get(2), None);      // key 2 đã bị xóa

cache.put(4, 4);                     // evict key 1
assert_eq!(cache.get(1), None);
assert_eq!(cache.get(3), Some(3));   // vẫn còn
assert_eq!(cache.get(4), Some(4));   // vẫn còn
```

## Rust Ecosystem

| Crate | Mô tả |
|-------|--------|
| [`lru`](https://crates.io/crates/lru) | LRU Cache đơn giản, API giống `HashMap`. Production-ready. |
| [`cached`](https://crates.io/crates/cached) | Macro `#[cached]` để tự động cache kết quả function. Hỗ trợ LRU, TTL. |
| [`moka`](https://crates.io/crates/moka) | Concurrent cache lấy cảm hứng từ Java Caffeine. Thread-safe, hỗ trợ LRU + TTL + async. |
| [`quick_cache`](https://crates.io/crates/quick_cache) | Cache hiệu suất cao, concurrent, hỗ trợ weighted entries. |

Khi nào dùng crate thay vì tự viết? **Hầu như luôn luôn** trong production. Tự viết để học, dùng crate để ship.

## Practice — LeetCode

| Bài | Tên | Gợi ý |
|-----|-----|-------|
| [#146](https://leetcode.com/problems/lru-cache/) | **LRU Cache** | Bài kinh điển. Implement `get` + `put` đúng y như chương này. |
| [#460](https://leetcode.com/problems/lfu-cache/) | **LFU Cache** | Nâng cấp — sẽ học ở chương sau. |
| [#1171](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/) | Remove Zero Sum | Kết hợp HashMap + Linked List — cùng pattern. |

**Gợi ý cho #146:** Copy nguyên code trong chương này, đổi `i32` thành type phù hợp. Bài này là interview classic — nếu bạn hiểu chương này, bạn đã giải được.

## Tổng kết

LRU Cache là bài toán kinh điển trong interview (LeetCode #146) và cực kỳ phổ biến trong thực tế. Điểm then chốt:

- **HashMap** → tìm nhanh O(1).
- **Doubly Linked List** → di chuyển/xóa nhanh O(1).
- **Kết hợp cả hai** → mọi thao tác O(1).
- **Sentinel nodes** (HEAD/TAIL giả) → code gọn, không cần xử lý edge case.
- **Vec-based DLL** → giải pháp Rust-friendly, tránh ownership issues.

Khi ai hỏi "tại sao cần doubly linked list?", đây là câu trả lời hoàn hảo: vì LRU Cache cần xóa node ở giữa danh sách trong O(1), mà chỉ doubly linked list mới làm được.

---

[← Skip List](./03-skip-list.md) | [LFU Cache →](./05-lfu-cache.md)
