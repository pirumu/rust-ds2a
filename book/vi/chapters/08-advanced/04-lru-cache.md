# LRU Cache

## Đây là gì?

Bạn đã học HashMap ở Phần 4 và Doubly Linked List ở Phần 2. LRU Cache kết hợp cả hai để tạo ra một cấu trúc dữ liệu cực kỳ thực tế.

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

## Yêu cầu

LRU Cache cần 2 thao tác, **cả hai đều O(1)**:

| Thao tác | Ý nghĩa |
|----------|---------|
| `get(key)` | Lấy value. Đánh dấu key này là "vừa dùng". |
| `put(key, value)` | Thêm/cập nhật. Nếu đầy → đá bỏ phần tử LRU. |

O(1) cho cả hai! Đây là lý do ta cần kết hợp 2 cấu trúc dữ liệu.

## Thiết kế: HashMap + Doubly Linked List

Tại sao cần cả hai?

- **HashMap** cho O(1) lookup theo key — nhưng không biết thứ tự.
- **Doubly Linked List** cho O(1) di chuyển/xóa node — nhưng không tìm nhanh theo key.

Kết hợp lại: HashMap trỏ vào node trong linked list. Linked list giữ thứ tự "gần đây → cũ".

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

## Vec-based Doubly Linked List

Trong Rust, doubly linked list thường cần `unsafe` hoặc `Rc<RefCell<>>`. Nhưng ta có cách đơn giản hơn: **dùng Vec**.

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

## Bảng độ phức tạp

| Thao tác | Time | Space | Giải thích |
|----------|------|-------|------------|
| `new(cap)` | O(1) | O(cap) | Khởi tạo HashMap + Vec |
| `get(key)` | O(1)* | O(1) | HashMap lookup + detach/attach |
| `put(key, val)` | O(1)* | O(1) | HashMap lookup + linked list ops |
| `len()` | O(1) | O(1) | Đọc HashMap.len() |

\* Amortised O(1) — HashMap có thể resize nhưng trung bình vẫn O(1).

## Chạy thử trong Rust

```rust
use rust_dsa::lru_cache::LRUCache;

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

## Tổng kết

LRU Cache là bài toán kinh điển trong interview (LeetCode #146) và cực kỳ phổ biến trong thực tế. Điểm then chốt:

- **HashMap** → tìm nhanh O(1).
- **Doubly Linked List** → di chuyển/xóa nhanh O(1).
- **Kết hợp cả hai** → mọi thao tác O(1).
- **Sentinel nodes** (HEAD/TAIL giả) → code gọn, không cần xử lý edge case.

Khi ai hỏi "tại sao cần doubly linked list?", đây là câu trả lời hoàn hảo: vì LRU Cache cần xóa node ở giữa danh sách trong O(1), mà chỉ doubly linked list mới làm được.
