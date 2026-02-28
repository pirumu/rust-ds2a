# LFU Cache

> 💡 **Đừng lo lắng:** Chương này dùng lại 100% kỹ thuật bạn đã học ở chương LRU Cache: HashMap + Doubly Linked List + arena allocation. Nếu bạn đã hiểu LRU, bạn hiểu được LFU. Chỉ thêm **một ý tưởng mới**: thay vì một linked list, ta có **nhiều linked list** — mỗi cái cho một mức frequency. Nếu chưa đọc chương LRU Cache, quay lại đọc trước. Chương này build on top of nó.

## Đây là gì?

> **Bridge từ LRU**: LRU Cache evict phần tử **least RECENTLY used** — lâu nhất chưa ai đụng. LFU Cache evict phần tử **least FREQUENTLY used** — ít lần dùng nhất (tổng cộng).

Tưởng tượng bạn là thủ thư quản lý một kệ sách nhỏ chỉ chứa được 5 cuốn. Kệ đầy rồi, có người muốn mượn cuốn mới. Bạn phải bỏ bớt 1 cuốn ra. Bạn sẽ bỏ cuốn nào?

- **LRU Cache**: bỏ cuốn **lâu nhất chưa ai mượn** (cuốn nằm im lâu nhất)
- **LFU Cache**: bỏ cuốn **ít người mượn nhất** (cuốn tổng số lần mượn thấp nhất)

LFU = **Least Frequently Used** -- ít được dùng nhất (tính theo tổng số lần).

## Tại sao cần LFU?

LRU có một điểm yếu: nó chỉ nhớ **lần cuối** bạn dùng, không nhớ bạn dùng **bao nhiêu lần**.

Ví dụ thực tế: bạn có một CDN cache. Video A được xem 10 triệu lần mỗi ngày. Video B vừa được upload 5 phút trước, có 1 view. Nếu dùng LRU và video B được xem sau video A, thì LRU nghĩ B "mới hơn" và có thể đá A ra. Vô lý!

LFU giải quyết: nó đếm số lần truy cập. Video A (10 triệu lần) sẽ không bao giờ bị đá bởi video B (1 lần).

### Khi nào dùng LFU?

- **CDN cache**: file nào được request nhiều nhất thì giữ lại, file ít ai truy cập thì bỏ.
- **Database query cache**: câu query nào chạy đi chạy lại nhiều lần thì cache, câu hiếm gặp thì bỏ.
- **DNS cache**: domain nào được resolve nhiều nhất thì giữ.

## So sánh LRU vs LFU

| | LRU Cache | LFU Cache |
|---|---|---|
| **Tiêu chí đuổi** | Lâu nhất chưa dùng | Ít lần dùng nhất |
| **Theo dõi** | Thứ tự truy cập gần đây | Số lần truy cập (frequency) |
| **Bias** | **Recency bias** — ưu tiên cái mới | **Frequency bias** — ưu tiên cái hay dùng |
| **Ưu điểm** | Đơn giản, thích ứng nhanh với thay đổi | Giữ item "hot" tốt hơn |
| **Nhược điểm** | Có thể đuổi item dùng nhiều nếu tạm thời không dùng | Item cũ tần suất cao khó bị đuổi (**cache pollution**) |
| **Tiebreaker** | Không cần | Khi 2 item cùng frequency → dùng LRU để phá hòa |
| **Độ phức tạp code** | Đơn giản (1 HashMap + 1 DLL) | Phức tạp hơn (2 HashMap + nhiều DLL) |

### Trade-off cốt lõi

```
LRU: "Cái gì MỚI DÙNG thì quan trọng"
     → Thích ứng nhanh, nhưng dễ quên item hay dùng

LFU: "Cái gì DÙNG NHIỀU thì quan trọng"
     → Giữ hot item tốt, nhưng item cũ frequency cao
       dính mãi trong cache (cache pollution)
```

**Frequency bias** là con dao hai lưỡi: item A được dùng 1000 lần tuần trước nhưng tuần này không ai dùng nữa. LFU vẫn giữ A vì frequency cao. Đây là lý do thực tế LRU thường được dùng nhiều hơn — nó đơn giản và "đủ tốt" cho hầu hết trường hợp.

## Thiết kế: Double HashMap + DLL Architecture

Để đạt **O(1)** cho cả `get` và `put`, ta cần 3 thành phần:

### 1. `key_map`: HashMap<key, node_index>

Tra cứu nhanh node theo key. Giống hệt LRU.

### 2. `freq_map`: HashMap<freq, DoublyLinkedList>

Đây là điểm khác biệt lớn nhất so với LRU. Thay vì **một** linked list, ta có **nhiều** linked list — mỗi cái chứa tất cả node có cùng frequency.

Trong mỗi bucket, node được sắp theo thứ tự truy cập: head = cũ nhất (LRU), tail = mới nhất (MRU).

### 3. `min_freq`: tần suất thấp nhất hiện tại

Một con số duy nhất. Khi cần evict, ta biết ngay bucket nào chứa victim — không cần duyệt tìm.

```
                    Double HashMap + DLL Architecture
                    ─────────────────────────────────

  key_map (HashMap)           freq_map (HashMap)
  ┌─────────────┐             ┌──────────────────────────────┐
  │ key → index │             │                              │
  │ A   →  0    │             │  freq=1:  [D] ←→ [E]        │
  │ B   →  1    │             │            ▲                 │
  │ C   →  2    │             │            │                 │
  │ D   →  3    │             │          head (evict here)   │
  │ E   →  4    │             │                              │
  └─────────────┘             │  freq=2:  [B] ←→ [C]        │
                              │                              │
  min_freq ──► 1              │  freq=4:  [A]                │
                              │                              │
                              └──────────────────────────────┘

  Arena (Vec<Node>): [A, B, C, D, E]
                      0  1  2  3  4
```

**So sánh với LRU**:
- LRU: 1 HashMap + 1 DLL → đơn giản
- LFU: 2 HashMap + nhiều DLL + `min_freq` → phức tạp hơn, nhưng vẫn O(1)

## O(1) LFU Design — Tại sao O(1)?

Câu hỏi quan trọng: làm sao biết ngay item nào có frequency thấp nhất mà không cần duyệt?

Bí mật nằm ở `min_freq`:

```
                    Tại sao mọi thứ đều O(1)?
                    ─────────────────────────

get(key):
  1. key_map.get(key)           → O(1) HashMap lookup
  2. Gỡ node khỏi bucket cũ    → O(1) DLL remove (có index trực tiếp)
  3. freq += 1                  → O(1)
  4. Thêm node vào bucket mới  → O(1) DLL append
  5. Cập nhật min_freq          → O(1) chỉ check 1 điều kiện

put(key, value):
  Evict:
    1. freq_map[min_freq].head  → O(1) biết ngay victim!
    2. Remove victim            → O(1) DLL remove
  Insert:
    1. Tạo node, freq = 1      → O(1)
    2. Thêm vào bucket freq=1  → O(1)
    3. min_freq = 1             → O(1) (node mới luôn có freq thấp nhất)
```

**Không có vòng lặp nào.** Mọi thao tác đều trực tiếp nhờ HashMap + con trỏ DLL + `min_freq`.

`min_freq` chỉ thay đổi trong 2 trường hợp:
1. **`put` node mới**: `min_freq = 1` (luôn luôn, vì node mới có freq = 1)
2. **`touch` node cũ**: nếu bucket `min_freq` rỗng sau khi gỡ node ra → `min_freq += 1`

Không cần duyệt tìm min. Không cần sort. O(1).

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

## Những lỗi hay gặp

### Pitfall 1: Quên tăng frequency khi `put` update

❌ **Sai**: `put(key, new_value)` khi key đã tồn tại → chỉ cập nhật value, không tăng freq.

✅ **Đúng**: `put` update phải tăng freq giống hệt `get` — gọi `touch()`.

💡 **Tại sao**: `put` update nghĩa là bạn đang *sử dụng* key đó. LeetCode 460 test case kiểm tra điều này. Nếu không tăng freq, item bị evict sai.

### Pitfall 2: Quên cập nhật `min_freq` khi bucket rỗng

❌ **Sai**: Gỡ node khỏi bucket, bucket rỗng, nhưng `min_freq` vẫn trỏ tới bucket rỗng.

✅ **Đúng**: Sau khi gỡ node, check xem bucket cũ có rỗng không. Nếu rỗng VÀ đó là bucket `min_freq` → `min_freq += 1`.

💡 **Tại sao**: Nếu `min_freq` trỏ tới bucket rỗng, lần evict tiếp theo sẽ panic (bucket không có node nào để đuổi).

### Pitfall 3: Tiebreaker — LRU trong cùng frequency

❌ **Sai**: Khi 2 node cùng frequency, evict ngẫu nhiên hoặc theo key.

✅ **Đúng**: Dùng **LRU** làm tiebreaker — trong mỗi frequency bucket, node ở head là node cũ nhất (LRU), evict node đó.

💡 **Tại sao**: Đề bài LeetCode 460 nói rõ: "If there is a tie, the least recently used key would be evicted." Mỗi bucket là một mini-LRU list.

### Pitfall 4: Quên set `min_freq = 1` khi insert node mới

❌ **Sai**: Insert node mới nhưng giữ `min_freq` cũ (có thể là 5, 10...).

✅ **Đúng**: Mỗi lần insert node mới → `min_freq = 1`. Luôn luôn.

💡 **Tại sao**: Node mới có freq = 1. Đó chắc chắn là frequency thấp nhất (không ai có freq = 0). Nếu không set `min_freq = 1`, lần evict tiếp sẽ tìm sai bucket.

## Code trong Rust

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

### Implement đầy đủ

```rust
impl LFUCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            key_to_idx: HashMap::new(),
            nodes: Vec::new(),
            freq_buckets: HashMap::new(),
            min_freq: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        if self.capacity == 0 {
            return None;
        }
        let &idx = self.key_to_idx.get(&key)?;
        let value = self.nodes[idx].value;
        self.touch(idx);  // ← tăng freq + di chuyển bucket
        Some(value)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // Update existing key
        if let Some(&idx) = self.key_to_idx.get(&key) {
            self.nodes[idx].value = value;
            self.touch(idx);  // ← Pitfall 1: PHẢI tăng freq!
            return;
        }

        // Evict if full
        if self.key_to_idx.len() == self.capacity {
            self.evict();
        }

        // Insert new node
        let idx = self.nodes.len();
        self.nodes.push(Node {
            key,
            value,
            freq: 1,
            prev: NONE,
            next: NONE,
        });
        self.key_to_idx.insert(key, idx);
        self.add_to_bucket(1, idx);
        self.min_freq = 1;  // ← Pitfall 4: luôn set = 1
    }

    fn touch(&mut self, idx: usize) {
        let old_freq = self.nodes[idx].freq;
        let new_freq = old_freq + 1;
        self.nodes[idx].freq = new_freq;

        self.remove_from_bucket(old_freq, idx);

        // Pitfall 2: cập nhật min_freq nếu bucket cũ rỗng
        if old_freq == self.min_freq {
            if let Some(bucket) = self.freq_buckets.get(&old_freq) {
                if bucket.is_empty() {
                    self.min_freq = new_freq;
                }
            }
        }

        self.add_to_bucket(new_freq, idx);
    }

    fn evict(&mut self) {
        let bucket = self.freq_buckets
            .get(&self.min_freq)
            .expect("min_freq bucket must exist");
        let victim_idx = bucket.head;  // ← Pitfall 3: LRU tiebreaker

        let victim_key = self.nodes[victim_idx].key;
        self.remove_from_bucket(self.min_freq, victim_idx);
        self.key_to_idx.remove(&victim_key);
    }

    fn add_to_bucket(&mut self, freq: usize, idx: usize) {
        let bucket = self.freq_buckets
            .entry(freq)
            .or_insert(FreqBucket { head: NONE, tail: NONE });

        if bucket.tail == NONE {
            bucket.head = idx;
            bucket.tail = idx;
        } else {
            let old_tail = bucket.tail;
            self.nodes[old_tail].next = idx;
            self.nodes[idx].prev = old_tail;
            self.nodes[idx].next = NONE;
            bucket.tail = idx;
        }
    }

    fn remove_from_bucket(&mut self, freq: usize, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        let bucket = self.freq_buckets
            .get_mut(&freq)
            .expect("bucket must exist");

        if prev != NONE {
            self.nodes[prev].next = next;
        } else {
            bucket.head = next;
        }

        if next != NONE {
            self.nodes[next].prev = prev;
        } else {
            bucket.tail = prev;
        }

        self.nodes[idx].prev = NONE;
        self.nodes[idx].next = NONE;
    }
}
```

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

## Khi nào dùng?

| Tình huống | Chọn | Lý do |
|---|---|---|
| Workload có "hot items" rõ ràng (80/20 rule) | **LFU** | Giữ hot items tốt hơn |
| Workload thay đổi theo thời gian | **LRU** | LFU bị cache pollution, item cũ dính mãi |
| Cần đơn giản, dễ implement | **LRU** | 1 HashMap + 1 DLL vs 2 HashMap + nhiều DLL |
| CDN, DNS cache — traffic pattern ổn định | **LFU** | Frequency phản ánh đúng popularity |
| Browser cache, OS page cache | **LRU** | Trang web bạn đọc hôm nay khác hôm qua |
| Không biết chọn gì | **LRU** | Đơn giản hơn và "đủ tốt" cho 90% trường hợp |

## Lưu ý thực tế

1. **Cache pollution**: Item cũ có frequency cao sẽ "dính" trong cache rất lâu, dù không còn được dùng nữa. Một số biến thể LFU giảm frequency theo thời gian (aging/decay). Ví dụ: chia frequency cho 2 mỗi giờ — gọi là **LFU with aging**.

2. **So với LRU**: LRU đơn giản hơn và thường "đủ tốt" cho hầu hết use case. LFU chỉ tốt hơn khi workload có rõ ràng "hot items" vs "cold items".

3. **Window-TinyLFU**: Đây là thuật toán kết hợp LRU + LFU, được dùng trong Caffeine (Java) và nhiều cache library hiện đại. Ý tưởng: dùng LRU cho "admission window" (item mới) + LFU cho "main cache" (item đã chứng minh được).

## Practice

| Bài | Gợi ý |
|---|---|
| [LFU Cache — LeetCode #460](https://leetcode.com/problems/lfu-cache/) (Hard) | Chính xác thiết kế trong chương này. Double HashMap + DLL + `min_freq`. |

Bài #460 là bài Hard nổi tiếng. Nếu bạn hiểu được thiết kế frequency buckets + `min_freq` + LRU tiebreaker, bạn đã giải được nó. Copy code Rust ở trên, thay kiểu dữ liệu cho phù hợp LeetCode, submit.

## Rust Ecosystem

| Crate | Mô tả |
|---|---|
| [`lfu`](https://crates.io/crates/lfu) | LFU cache đơn giản |
| [`cached`](https://crates.io/crates/cached) | Macro-based caching, hỗ trợ nhiều eviction policy |
| [`moka`](https://crates.io/crates/moka) | Concurrent cache lấy cảm hứng từ Caffeine (Java), dùng Window-TinyLFU — kết hợp LRU + LFU |
| [`quick_cache`](https://crates.io/crates/quick_cache) | Concurrent cache nhẹ, cũng dùng frequency-based eviction |

Trong production, hầu như không ai tự viết LFU cache. Dùng `moka` hoặc `quick_cache` — chúng thread-safe, đã tối ưu, và có frequency aging built-in. Tự viết chỉ để học và phỏng vấn.

## Tiếp theo

Tiếp theo ta sẽ học **Merkle Tree** — cấu trúc dữ liệu đứng sau Git, blockchain, và hệ thống phân tán. Thay vì cache, Merkle Tree giải quyết một vấn đề khác: **làm sao biết dữ liệu có bị thay đổi không?**

---

[← LRU Cache](./04-lru-cache.md) | [Merkle Tree →](./06-merkle-tree.md)
