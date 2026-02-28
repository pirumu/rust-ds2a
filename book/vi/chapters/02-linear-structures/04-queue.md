# Queue

> 💡 **Đừng lo lắng:** Queue chỉ là hàng chờ mua trà sữa thôi -- ai đến trước được phục vụ trước. Nếu bạn hiểu Stack ở chương trước, Queue chỉ là anh em sinh đôi nhưng tính cách ngược lại. Dễ lắm.

## Đây là gì?

Bạn đã dùng queue mỗi ngày mà không biết. Mỗi lần bạn gửi tin nhắn trên Zalo, đặt đồ ăn trên ShopeeFood, hay đợi Grab đến đón -- đằng sau tất cả đều có queue đang chạy. Hiểu queue là bước đệm để sau này bạn hiểu cách hệ thống lớn (server, message broker, OS scheduler) vận hành.

Bạn xếp hàng mua trà sữa. Ai đến trước được phục vụ trước. Ai đến sau đứng cuối hàng. Không ai được chen ngang (trừ khi muốn bị nhìn dữ).

Đó chính là **queue** (hàng đợi) -- cấu trúc dữ liệu hoạt động theo nguyên tắc **FIFO** (First-In, First-Out: vào trước, ra trước).

So sánh nhanh:
- **Stack** = chồng đĩa → vào sau ra trước (LIFO)
- **Queue** = hàng chờ mua trà sữa → vào trước ra trước (FIFO)

Stack và Queue là hai anh em sinh đôi nhưng tính cách ngược nhau. Stack thích ai đến sau phục vụ trước (kiểu thiên vị!). Queue thì công bằng -- ai đến trước phục vụ trước.

## Tại sao cần biết queue?

Queue xuất hiện khắp nơi trong lập trình:

- **Hàng đợi in ấn** -- Gửi 5 file in, máy in xử lý theo thứ tự gửi.
- **Xử lý request trên server** -- 1000 người cùng truy cập website, server xếp request vào queue và xử lý lần lượt.
- **BFS (Breadth-First Search)** -- Duyệt đồ thị theo chiều rộng dùng queue.
- **Message queue** -- Microservice gửi tin nhắn cho nhau qua hàng đợi (RabbitMQ, Kafka).

### Message Queue -- use case quan trọng nhất

Hãy tưởng tượng bạn đặt hàng trên Shopee. Bạn bấm "Đặt hàng" và thấy màn hình "Đơn hàng đã được tiếp nhận". Nhưng thực tế, đơn hàng chưa được xử lý ngay. Nó được đẩy vào một **message queue**.

Tại sao không xử lý ngay luôn?

```
❌ Không dùng queue:

  User → [Order Service] → [Payment Service] → [Inventory Service] → [Shipping Service]
         ↑
         User phải chờ TẤT CẢ bước này xong mới thấy "Đặt hàng thành công"
         Nếu Payment Service đang chậm? User đợi... đợi... timeout.
         Nếu Inventory Service sập? Toàn bộ đơn hàng mất.

✅ Dùng queue:

  User → [Order Service] → | QUEUE | → [Payment Service]
                                     → [Inventory Service]
                                     → [Shipping Service]

  User thấy "Đặt hàng thành công" ngay lập tức.
  Các service lấy đơn từ queue khi nào rảnh thì xử lý.
  Payment Service sập? Đơn vẫn nằm trong queue, đợi nó hồi lại rồi xử lý tiếp.
```

Ba lý do chính:

1. **Decoupling** (tách rời) -- Order Service không cần biết Payment Service ở đâu, chạy bằng gì. Nó chỉ cần đẩy message vào queue. Giống như bạn bỏ thư vào hòm thư -- bạn không cần biết bưu tá đi đường nào.

2. **Backpressure** (kiểm soát tải) -- Flash sale 11.11, 100,000 đơn đổ vào trong 1 phút. Không có queue thì server sập. Có queue thì đơn xếp hàng, server xử lý từ từ theo tốc độ nó chịu được.

3. **Retry khi fail** -- Payment Service lỗi? Message vẫn nằm trong queue. Khi service hồi lại, nó lấy message ra xử lý tiếp. Không mất đơn.

Kafka, RabbitMQ, Amazon SQS -- tất cả đều là distributed queue. Cái queue đơn giản bạn đang học chính là nền tảng để hiểu những hệ thống này.

## Hoạt động như thế nào?

Queue chỉ có 2 thao tác chính:

| Thao tác | Nghĩa là gì? |
|----------|---------------|
| **enqueue** | Xếp vào cuối hàng |
| **dequeue** | Phục vụ người đầu hàng (lấy ra) |
| **peek** | Xem ai đang đứng đầu hàng (không lấy ra) |

```
enqueue(1)   enqueue(2)   enqueue(3)   dequeue()→1   dequeue()→2
+--------+   +--------+   +--------+   +--------+    +--------+
| đầu    |   | đầu    |   | đầu    |   | đầu    |    | đầu    |
|   1    |   |   1    |   |   1    |   |   2    |    |   3    |
|        |   |   2    |   |   2    |   |   3    |    |        |
|        |   |        |   |   3    |   |        |    |        |
| cuối   |   | cuối   |   | cuối   |   | cuối   |    | cuối   |
+--------+   +--------+   +--------+   +--------+    +--------+
```

### Tại sao không dùng Vec?

Bạn **có thể** dùng `Vec` và gọi `remove(0)` để dequeue. Nhưng đây là ý tưởng tệ. Mỗi lần xóa phần tử đầu, **tất cả** phần tử còn lại phải dịch sang trái. Với 1 triệu phần tử? Dịch 1 triệu lần. O(n) -- chậm.

Giải pháp: dùng **ring buffer** (bộ đệm vòng). Rust cung cấp sẵn `VecDeque` dùng ring buffer.

Ring buffer giống như một vòng tròn. Thay vì dịch phần tử, ta chỉ di chuyển 2 con trỏ: `head` (đầu hàng) và `tail` (cuối hàng).

```
Ring buffer (dung lượng 8) -- nhìn như mảng thẳng:

  index:    0   1   2   3   4   5   6   7
          +---+---+---+---+---+---+---+---+
          | _ | _ | A | B | C | _ | _ | _ |
          +---+---+---+---+---+---+---+---+
                    ^           ^
                   head        tail

Sau dequeue (xóa A) và enqueue(D):

          +---+---+---+---+---+---+---+---+
          | _ | _ | _ | B | C | D | _ | _ |
          +---+---+---+---+---+---+---+---+
                        ^           ^
                       head        tail

Không dịch phần tử nào! Chỉ di chuyển head và tail.
```

Nhưng mảng thẳng chưa thấy hết cái hay. Hãy nghĩ nó như **vòng tròn** thực sự:

```
         ┌───┐
      7 →│   │← 0
     ┌───┘   └───┐
   6 │             │ 1
     │             │
   5 │      D      │ 2(head)
     │     ↑tail   │  ↓
   4 │             │ B
     └───┐   ┌───┘
      3 →│ C │← (3)
         └───┘

  head = index 2 (B)     tail = index 5 (D vừa enqueue)
```

Tại sao "vòng tròn" quan trọng? Khi `tail` chạy đến cuối mảng (index 7), nó **không bị kẹt** -- nó quay về index 0 tiếp tục. Công thức: `next_index = (current + 1) % capacity`. Đó là tại sao gọi là "ring" (vòng). Cả enqueue và dequeue đều O(1) -- không bao giờ cần dịch phần tử.

## VecDeque trong Rust

`VecDeque<T>` là ring buffer được implement sẵn trong thư viện chuẩn (`std::collections`). Tại sao dùng nó thay vì tự viết?

**Tự implement ring buffer phức tạp hơn bạn nghĩ:**
- Khi buffer đầy, cần resize (cấp phát mảng mới lớn hơn, copy dữ liệu sang). `VecDeque` handle tự động.
- Wrap-around index: tính `(head + offset) % capacity` cho mọi truy cập. Dễ sai, khó debug.
- Memory alignment: dữ liệu trên heap cần align đúng cho từng kiểu `T`. `VecDeque` lo hết.

**Rust-specific -- tại sao index thay vì pointer?**

Trong C/C++, linked list dùng pointer để trỏ đến node kế tiếp. Nhưng trong Rust, borrow checker không cho phép nhiều mutable reference cùng lúc. `VecDeque<T>` giải quyết bằng cách:
- Store data trong **một mảng liên tục** trên heap
- `head` và `tail` là `usize` index, không phải pointer
- Truy cập phần tử qua index: `data[head]`, `data[tail]` -- borrow checker vui vẻ vì chỉ có một owner (mảng)

```
VecDeque<T> bên trong:

  heap: [ _, _, B, C, D, _, _, _ ]
                ^        ^
  head: 2      tail: 5       capacity: 8

  Chỉ có 2 con số (usize) quản lý toàn bộ queue.
  Không pointer nào cả → borrow checker không phàn nàn.
```

## Code Rust

Code đầy đủ nằm trong `src/queue.rs`.

```rust
use std::collections::VecDeque;

pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Tạo queue rỗng
    pub fn new() -> Self {
        Self { data: VecDeque::new() }
    }

    /// Xếp vào cuối hàng
    pub fn enqueue(&mut self, val: T) {
        self.data.push_back(val);
    }

    /// Lấy người đầu hàng ra (trả về None nếu hàng trống)
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Xem ai đang đầu hàng (không lấy ra)
    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    /// Hàng có trống không?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Có bao nhiêu người trong hàng?
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
```

Giống Stack, code rất ngắn vì `VecDeque` đã lo hết phần ring buffer. `Queue` chỉ **giới hạn interface** -- chỉ cho enqueue ở cuối và dequeue ở đầu, không cho push ở đầu hay pop ở cuối. FIFO được bảo vệ.

## Độ phức tạp

| Thao tác | Thời gian (trung bình) | Bộ nhớ |
|----------|------------------------|--------|
| `enqueue` | O(1) | O(1) |
| `dequeue` | O(1) | O(1) |
| `peek` | O(1) | O(1) |
| `is_empty` | O(1) | O(1) |
| `size` | O(1) | O(1) |
| **Tổng bộ nhớ** | -- | **O(n)** |

**Nôm na:** Mọi thao tác đều siêu nhanh, bất kể hàng dài bao nhiêu.

So sánh với dùng `Vec` thô:

| Cách làm | enqueue | dequeue |
|----------|---------|---------|
| `Vec` + `remove(0)` | O(1) | **O(n)** -- chậm! |
| `VecDeque` (ring buffer) | O(1) | O(1) -- nhanh! |

## Ví dụ

### Sử dụng cơ bản

```rust
use rust_ds2a::queue::Queue;

let mut q = Queue::new();

q.enqueue("Alice");    // Alice xếp hàng
q.enqueue("Bob");      // Bob xếp sau Alice
q.enqueue("Carol");    // Carol xếp sau Bob

assert_eq!(q.peek(), Some(&"Alice"));    // Alice đang đầu hàng
assert_eq!(q.dequeue(), Some("Alice"));  // Alice được phục vụ
assert_eq!(q.dequeue(), Some("Bob"));    // Bob được phục vụ
assert_eq!(q.size(), 1);                 // Còn Carol
```

### BFS -- Duyệt cây theo chiều rộng

BFS (Breadth-First Search) dùng queue để duyệt từng tầng, từ trên xuống dưới:

```
Cây:        1
           / \
          2   3
         / \
        4   5
```

Trace từng bước -- chú ý trạng thái queue tại mỗi bước:

| Bước | Action | Queue sau action | Visited |
|------|--------|-----------------|---------|
| 1 | enqueue(1) | [**1**] | [] |
| 2 | dequeue() → 1, enqueue con: 2, 3 | [**2**, 3] | [1] |
| 3 | dequeue() → 2, enqueue con: 4, 5 | [**3**, 4, 5] | [1, 2] |
| 4 | dequeue() → 3, không có con | [**4**, 5] | [1, 2, 3] |
| 5 | dequeue() → 4, không có con | [**5**] | [1, 2, 3, 4] |
| 6 | dequeue() → 5, không có con | [] | [1, 2, 3, 4, 5] |

Kết quả: **1, 2, 3, 4, 5** -- đúng thứ tự từng tầng (tầng 0 → tầng 1 → tầng 2).

Queue đảm bảo tất cả node ở tầng d được xử lý trước bất kỳ node nào ở tầng d+1. Đó là lý do BFS dùng queue, không dùng stack.

**Câu hỏi tư duy:** Nếu thay queue bằng stack ở đây, thứ tự duyệt sẽ thế nào?

Thử lại cùng cây đó với stack:

| Bước | Action | Stack sau action | Visited |
|------|--------|-----------------|---------|
| 1 | push(1) | [1] | [] |
| 2 | pop() → 1, push con: 3, 2 | [3, **2**] | [1] |
| 3 | pop() → 2, push con: 5, 4 | [3, 5, **4**] | [1, 2] |
| 4 | pop() → 4, không có con | [3, **5**] | [1, 2, 4] |
| 5 | pop() → 5, không có con | [**3**] | [1, 2, 4, 5] |
| 6 | pop() → 3, không có con | [] | [1, 2, 4, 5, 3] |

Kết quả: **1, 2, 4, 5, 3** -- đi sâu trước (DFS), không đi theo tầng nữa. Thay queue bằng stack → BFS biến thành DFS. Đó là tại sao cấu trúc dữ liệu bạn chọn quyết định hành vi thuật toán -- không phải chỉ là lý thuyết suông.

### Rate Limiter -- Giới hạn tốc độ request

Bài toán thực tế: API của bạn chỉ cho phép **5 request trong 10 giây**. Request thứ 6 trong cùng cửa sổ 10 giây bị reject.

Ý tưởng: dùng queue lưu timestamp của mỗi request. Mỗi lần có request mới:
1. Xóa tất cả timestamp cũ hơn 10 giây (dequeue các phần tử quá hạn)
2. Nếu queue còn >= 5 phần tử → reject
3. Ngược lại → enqueue timestamp mới, cho phép request

```
Timeline:

  t=0s   t=1s   t=2s   t=3s   t=4s   t=5s   t=11s
  req1   req2   req3   req4   req5   req6   req7
  ✅     ✅     ✅     ✅     ✅     ❌     ✅
                                      ↑       ↑
                                  đã đủ 5   req1 đã quá 10s,
                                  trong 10s  bị dequeue → còn 4 slot
```

Đây chính là thuật toán **sliding window** -- bạn sẽ gặp lại pattern này trong chương Sliding Window sau này.

```rust
use std::collections::VecDeque;

/// Rate limiter dùng sliding window với queue
pub struct RateLimiter {
    /// Queue lưu timestamp (giây) của mỗi request
    timestamps: VecDeque<f64>,
    /// Số request tối đa trong cửa sổ
    max_requests: usize,
    /// Kích thước cửa sổ (giây)
    window_secs: f64,
}

impl RateLimiter {
    /// Tạo rate limiter mới
    /// Ví dụ: max_requests=5, window_secs=10.0 → 5 request/10 giây
    pub fn new(max_requests: usize, window_secs: f64) -> Self {
        Self {
            timestamps: VecDeque::new(),
            max_requests,
            window_secs,
        }
    }

    /// Kiểm tra request tại thời điểm `now` có được phép không
    /// Trả về true = cho phép, false = reject
    pub fn allow_request(&mut self, now: f64) -> bool {
        // Bước 1: Xóa tất cả timestamp quá hạn
        // Queue được sắp xếp theo thời gian (cũ nhất ở đầu)
        // nên chỉ cần dequeue từ đầu cho đến khi gặp timestamp còn hạn
        while let Some(&oldest) = self.timestamps.front() {
            if now - oldest > self.window_secs {
                self.timestamps.pop_front(); // dequeue timestamp quá hạn
            } else {
                break; // timestamp đầu còn hạn → tất cả sau nó cũng còn
            }
        }

        // Bước 2: Kiểm tra có còn slot không
        if self.timestamps.len() < self.max_requests {
            self.timestamps.push_back(now); // enqueue timestamp mới
            true  // cho phép
        } else {
            false // reject -- đã đầy
        }
    }
}
```

## Những cái bẫy hay gặp

**❌ Dùng `Vec` + `remove(0)` thay vì `VecDeque`**

```rust
// ❌ Sai
let mut q: Vec<i32> = vec![1, 2, 3, 4, 5];
let first = q.remove(0); // O(n) -- dịch 4 phần tử sang trái!
```

✅ **Đúng:** Dùng `VecDeque` và `pop_front()` -- O(1), không dịch gì cả.

💡 **Tại sao:** `remove(0)` trên `Vec` phải dịch toàn bộ phần tử còn lại sang trái. Với 1 triệu phần tử, mỗi dequeue dịch 1 triệu lần. `VecDeque` chỉ cần tăng `head` lên 1.

---

**❌ Nhầm enqueue/dequeue với push/pop của Stack**

```rust
// ❌ Nghĩ rằng dequeue lấy phần tử CUỐI (như stack pop)
q.enqueue(1);
q.enqueue(2);
q.enqueue(3);
// dequeue() → 3?  SAI! dequeue() → 1 (FIFO, không phải LIFO)
```

✅ **Đúng:** Nhớ hàng trà sữa. Ai đến **trước** được phục vụ **trước**. `enqueue` = xếp cuối hàng, `dequeue` = phục vụ đầu hàng.

💡 **Tại sao:** Stack là LIFO (vào sau ra trước). Queue là FIFO (vào trước ra trước). Hai cái ngược nhau hoàn toàn. Nếu bạn cần LIFO → dùng Stack. Cần FIFO → dùng Queue.

---

**❌ Không kiểm tra `is_empty()` trước khi dequeue**

```rust
// ❌ Giả sử queue luôn có phần tử
let value = q.dequeue().unwrap(); // PANIC nếu queue rỗng!
```

✅ **Đúng:** Dùng pattern matching hoặc `if let`:

```rust
if let Some(value) = q.dequeue() {
    println!("Xử lý: {}", value);
} else {
    println!("Queue rỗng, không có gì để xử lý");
}
```

💡 **Tại sao:** Trong Rust, `dequeue()` trả về `Option<T>` -- `Some(value)` nếu có, `None` nếu trống. Đây là cách Rust thay thế exception -- an toàn hơn nhiều vì compiler bắt bạn phải xử lý cả hai trường hợp.

---

**❌ Tạo Queue mới mỗi lần cần xử lý batch**

```rust
// ❌ Tạo queue mới mỗi batch
for batch in batches {
    let mut q = Queue::new(); // allocate mới mỗi lần!
    for item in batch {
        q.enqueue(item);
    }
    process(&mut q);
}
```

✅ **Đúng:** Tạo một lần, dùng lại nhiều lần:

```rust
let mut q = Queue::new(); // allocate 1 lần
for batch in batches {
    for item in batch {
        q.enqueue(item);
    }
    process(&mut q);
    // Sau khi process dequeue hết, queue tự rỗng
    // VecDeque giữ lại capacity đã cấp phát → không cần allocate lại
}
```

💡 **Tại sao:** Mỗi lần `Queue::new()` là một lần cấp phát bộ nhớ trên heap. Khi reuse, `VecDeque` giữ lại capacity cũ -- lần sau enqueue không cần allocate thêm.

## Khi nào dùng / không nên dùng

| Tình huống | Queue? | Thay bằng gì? |
|------------|--------|---------------|
| Xử lý task theo thứ tự đến | ✅ | -- |
| Undo/Redo | ❌ | Stack |
| Cần truy cập phần tử bất kỳ | ❌ | Vec |
| BFS traversal | ✅ | -- |
| Cần thêm/xóa cả hai đầu | ❌ | Deque (chương sau) |
| Priority-based processing | ❌ | Priority Queue |
| Rate limiting / sliding window | ✅ | -- |
| Xử lý event theo thứ tự thời gian | ✅ | -- |

**Quy tắc ngón tay cái:** Nếu bạn cần xử lý theo thứ tự "ai đến trước phục vụ trước" → queue. Nếu cần thứ tự khác → tìm cấu trúc dữ liệu phù hợp hơn.

## Luyện nhận diện Pattern

**Bài 1:** Implement hệ thống in tài liệu: nhiều user gửi lệnh in cùng lúc, máy in xử lý theo thứ tự, nhưng job nào urgent được đẩy lên đầu.

*Gợi ý: Cần thêm/xóa ở cả hai đầu không?*

<details>
<summary>Đáp án</summary>

Có dùng Queue thông thường không? **Không hoàn toàn.** Queue thường chỉ cho enqueue ở cuối. Nhưng urgent job cần đẩy lên đầu → cần **Deque** (double-ended queue) hoặc **Priority Queue**. Deque cho phép `push_front()` cho urgent job và `push_back()` cho job thường. Priority Queue tự động sắp xếp theo mức ưu tiên.

Complexity: enqueue O(1), dequeue O(1) với Deque. O(log n) cho enqueue với Priority Queue.

</details>

---

**Bài 2:** Level-order traversal của cây nhị phân -- in ra từng tầng trên một dòng riêng.

*Gợi ý: Giống BFS nhưng cần biết khi nào tầng mới bắt đầu.*

<details>
<summary>Đáp án</summary>

Có dùng Queue không? **Có!** Đây chính là BFS. Trick để in từng tầng: trước khi bắt đầu xử lý một tầng, ghi nhớ `queue.size()`. Dequeue đúng bấy nhiêu phần tử = hết một tầng. Phần tử mới enqueue trong lúc đó thuộc tầng tiếp theo.

Complexity: Time O(n) -- mỗi node được enqueue/dequeue đúng 1 lần. Space O(w) -- w là chiều rộng lớn nhất của cây.

</details>

---

**Bài 3:** Sliding window maximum: cho mảng và window size k, tìm max trong mỗi window.

*Gợi ý: Cần queue thông thường hay loại đặc biệt?*

<details>
<summary>Đáp án</summary>

Có dùng Queue thông thường không? **Không.** Cần **Monotonic Deque** (deque đơn điệu) -- một deque mà phần tử luôn giảm dần từ đầu đến cuối. Khi thêm phần tử mới, loại bỏ tất cả phần tử nhỏ hơn ở cuối deque. Phần tử đầu deque luôn là max của window hiện tại.

Queue thường không đủ vì cần xóa ở cả hai đầu và cần truy cập max hiệu quả.

Complexity: Time O(n) -- mỗi phần tử được thêm/xóa khỏi deque tối đa 1 lần. Space O(k).

</details>

## Tiếp theo: Deque

Queue chỉ cho enqueue ở cuối và dequeue ở đầu. Nhưng nếu bạn cần thêm/xóa ở **cả hai đầu** thì sao?

**Deque** (Double-Ended Queue) là sự kết hợp của cả Stack và Queue -- push/pop được cả đầu lẫn cuối. Thực ra `VecDeque` mà ta đang dùng bên trong đã là Deque rồi -- ta chỉ đang *giới hạn* nó lại thành Queue thôi.

Chương sau sẽ mở khóa toàn bộ sức mạnh của Deque.

---

[← Stack](./03-stack.md) | [Deque →](./05-deque.md)
