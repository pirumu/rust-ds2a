# Queue

## Đây là gì?

Bạn xếp hàng mua trà sữa. Ai đến trước được phục vụ trước. Ai đến sau đứng cuối hàng. Không ai được chen ngang (trừ khi muốn bị nhìn dữ).

Đó chính là **queue** (hàng đợi) -- cấu trúc dữ liệu hoạt động theo nguyên tắc **FIFO** (First-In, First-Out: vào trước, ra trước).

So sánh nhanh:
- **Stack** = chồng đĩa → vào sau ra trước (LIFO)
- **Queue** = hàng chờ mua trà sữa → vào trước ra trước (FIFO)

### Tại sao cần biết queue?

Queue xuất hiện khắp nơi trong lập trình:

- **Hàng đợi in ấn** -- Gửi 5 file in, máy in xử lý theo thứ tự gửi.
- **Xử lý request trên server** -- 1000 người cùng truy cập website, server xếp request vào queue và xử lý lần lượt.
- **BFS (Breadth-First Search)** -- Duyệt đồ thị theo chiều rộng dùng queue.
- **Message queue** -- Microservice gửi tin nhắn cho nhau qua hàng đợi (RabbitMQ, Kafka).

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
Ring buffer (dung lượng 8):

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

Khi `tail` đến cuối mảng, nó quay vòng lại đầu (vì thế gọi là "ring" -- vòng). Cả enqueue và dequeue đều O(1).

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

Queue-based BFS duyệt: 1, 2, 3, 4, 5

Bước 1: enqueue(1)
Bước 2: dequeue() → 1, enqueue con của 1: enqueue(2), enqueue(3)
Bước 3: dequeue() → 2, enqueue con của 2: enqueue(4), enqueue(5)
Bước 4: dequeue() → 3, không có con
Bước 5: dequeue() → 4, không có con
Bước 6: dequeue() → 5, không có con
```

Queue đảm bảo tất cả node ở tầng d được xử lý trước bất kỳ node nào ở tầng d+1. Đó là lý do BFS dùng queue, không dùng stack (stack sẽ cho DFS -- duyệt theo chiều sâu).
