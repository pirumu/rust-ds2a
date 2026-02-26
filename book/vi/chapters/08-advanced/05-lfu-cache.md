# LFU Cache

## Đây là gì?

Bạn đã học **LRU Cache** ở chương trước. LFU Cache khác ở chỗ: thay vì đuổi người *mới đến gần nhất ít được dùng*, nó đuổi người **ít dùng nhất**.

Tưởng tượng bạn là thủ thư quản lý một kệ sách nhỏ chỉ chứa được 5 cuốn. Kệ đầy rồi, có người muốn mượn cuốn mới. Bạn phải bỏ bớt 1 cuốn ra. Bạn sẽ bỏ cuốn nào?

- **LRU Cache**: bỏ cuốn **lâu nhất chưa ai mượn** (cuốn nằm im lâu nhất)
- **LFU Cache**: bỏ cuốn **ít người mượn nhất** (cuốn tổng số lần mượn thấp nhất)

LFU = **Least Frequently Used** -- ít được dùng nhất (tính theo tổng số lần).

### Khi nào dùng LFU?

- **CDN cache**: file nào được request nhiều nhất thì giữ lại, file ít ai truy cập thì bỏ.
- **Database query cache**: câu query nào chạy đi chạy lại nhiều lần thì cache, câu hiếm gặp thì bỏ.
- **DNS cache**: domain nào được resolve nhiều nhất thì giữ.

## So sánh LRU vs LFU

| | LRU Cache | LFU Cache |
|---|---|---|
| **Tiêu chí đuổi** | Lâu nhất chưa dùng | Ít lần dùng nhất |
| **Theo dõi** | Thứ tự truy cập gần đây | Số lần truy cập (frequency) |
| **Ưu điểm** | Đơn giản, nhanh | Giữ item "hot" tốt hơn |
| **Nhược điểm** | Có thể đuổi item dùng nhiều nếu tạm thời không dùng | Item cũ tần suất cao khó bị đuổi (cache pollution) |
| **Tiebreaker** | Không cần | Khi 2 item cùng frequency → dùng LRU để phá hòa |

## Thiết kế: Frequency Buckets

Đây là phần hay nhất. Để đạt **O(1)** cho cả `get` và `put`, ta dùng 3 thứ:

### 1. HashMap: key → node

Tra cứu nhanh node theo key.

### 2. Frequency buckets: freq → danh sách node

Mỗi "bucket" chứa tất cả node có cùng frequency. Trong mỗi bucket, node được sắp theo thứ tự truy cập (doubly linked list).

### 3. min_freq: tần suất thấp nhất hiện tại

Để biết ngay bucket nào cần lấy victim khi evict.

```
min_freq = 1
     │
     ▼
┌─────────────────────────────────────────────────┐
│  freq_buckets (HashMap<freq, DoublyLinkedList>)  │
├─────────────────────────────────────────────────┤
│                                                  │
│  freq=1:  [D] ←→ [E]                            │
│            ▲                                     │
│            │                                     │
│          head                                    │
│       (đuổi trước)                               │
│                                                  │
│  freq=2:  [B] ←→ [C]                            │
│                                                  │
│  freq=4:  [A]                                    │
│                                                  │
└─────────────────────────────────────────────────┘

key_to_idx (HashMap<key, node_index>):
  A → 0,  B → 1,  C → 2,  D → 3,  E → 4
```

Trong mỗi bucket, **head = ít dùng gần đây nhất** (LRU), **tail = dùng gần đây nhất** (MRU).

Khi cần evict: lấy head của bucket có `freq = min_freq`. Đó là node ít dùng nhất, và trong nhóm đó là node cũ nhất.

## Thao tác Get

```
get(key):
  1. Tìm node trong HashMap        → O(1)
  2. Nếu không có → return None
  3. Gỡ node khỏi bucket cũ (freq) → O(1)
  4. freq += 1
  5. Thêm node vào bucket mới      → O(1)
  6. Nếu bucket cũ rỗng VÀ freq cũ == min_freq:
       min_freq += 1
  7. Return value
```

### Ví dụ từng bước

Cache capacity = 3. Đã có: A(freq=2), B(freq=1), C(freq=1).

```
Trước get(B):
  min_freq = 1
  freq=1:  [B] ←→ [C]      ← B ở head
  freq=2:  [A]

get(B): gỡ B khỏi freq=1, tăng freq lên 2, thêm vào freq=2

Sau get(B):
  min_freq = 1              ← freq=1 vẫn còn C, nên min_freq không đổi
  freq=1:  [C]
  freq=2:  [A] ←→ [B]      ← B thêm vào tail
```

## Thao tác Put

```
put(key, value):
  1. Nếu key đã tồn tại:
       - Cập nhật value
       - Gọi logic giống get (tăng freq)
       - Return

  2. Nếu cache đầy:
       - Evict head của bucket min_freq  → O(1)
       - Xóa key khỏi HashMap

  3. Tạo node mới với freq = 1
  4. Thêm vào bucket freq=1             → O(1)
  5. min_freq = 1  (node mới luôn có freq thấp nhất)
```

### Ví dụ Eviction

Cache capacity = 2. Có: A(freq=3), B(freq=1).

```
Trước put(C, 30):
  min_freq = 1
  freq=1:  [B]
  freq=3:  [A]

Cache đầy → evict head của freq=1 = B. Bye B!

Sau put(C, 30):
  min_freq = 1
  freq=1:  [C]              ← C mới, freq=1
  freq=3:  [A]
```

### Ví dụ Tiebreaker (cùng frequency)

Cache capacity = 2. Có: A(freq=1), B(freq=1). A được thêm trước B.

```
Trước put(C, 30):
  min_freq = 1
  freq=1:  [A] ←→ [B]      ← A ở head (thêm trước = cũ hơn)

Cache đầy → evict head của freq=1 = A (LRU tiebreaker)

Sau put(C, 30):
  min_freq = 1
  freq=1:  [B] ←→ [C]
```

## Code Rust

```rust
use std::collections::HashMap;

const NONE: usize = usize::MAX;

struct Node {
    key: i32,
    value: i32,
    freq: usize,
    prev: usize,  // index trong arena
    next: usize,
}

struct FreqBucket {
    head: usize,  // LRU end (đuổi trước)
    tail: usize,  // MRU end (mới nhất)
}

pub struct LFUCache {
    capacity: usize,
    key_to_idx: HashMap<i32, usize>,  // key → index trong arena
    nodes: Vec<Node>,                  // arena allocation
    freq_buckets: HashMap<usize, FreqBucket>,
    min_freq: usize,
}
```

**Arena allocation** là gì? Thay vì dùng pointer (`Box`, `Rc`), ta lưu tất cả node trong 1 `Vec`. Mỗi node tham chiếu node khác bằng **index** (số nguyên). Đơn giản hơn, không cần `unsafe`, không lo memory leak.

### Tại sao O(1)?

Mỗi thao tác chỉ gồm:
- **HashMap lookup**: O(1) trung bình
- **Linked list insert/remove**: O(1) vì ta có con trỏ trực tiếp đến node
- **min_freq update**: chỉ cần check bucket cũ có rỗng không → O(1)

Không có vòng lặp nào duyệt qua tất cả phần tử. Mọi thứ đều trực tiếp.

## Bảng độ phức tạp

| Thao tác | Time | Space |
|---|---|---|
| `new(capacity)` | O(1) | O(capacity) |
| `get(key)` | O(1) | O(1) |
| `put(key, value)` | O(1) amortized | O(1) |
| `len()` | O(1) | O(1) |

**Space tổng thể**: O(capacity) -- mỗi entry chiếm 1 node trong arena + 1 entry trong HashMap.

## Lưu ý thực tế

1. **Cache pollution**: Item cũ có frequency cao sẽ "dính" trong cache rất lâu, dù không còn được dùng nữa. Một số biến thể LFU giảm frequency theo thời gian (aging/decay).

2. **So với LRU**: LRU đơn giản hơn và thường "đủ tốt" cho hầu hết use case. LFU chỉ tốt hơn khi workload có rõ ràng "hot items" vs "cold items".

3. **LeetCode 460**: Bài "LFU Cache" là bài Hard nổi tiếng. Nếu bạn hiểu được thiết kế frequency buckets + min_freq, bạn đã giải được nó.
