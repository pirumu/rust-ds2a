# Priority Queue

## Đây là gì?

Bạn đến **phòng cấp cứu bệnh viện**. Ở đây không phải ai đến trước được khám trước. Người bị đau tim được khám trước người bị trầy xước, dù người bị trầy xước đến trước 2 tiếng. Ai **nặng nhất** (priority cao nhất) được phục vụ trước.

**Priority queue** (hàng đợi ưu tiên) hoạt động đúng như vậy. Khác với queue thường (FIFO -- ai vào trước ra trước), priority queue luôn lấy ra phần tử có **ưu tiên cao nhất**, bất kể thứ tự chèn.

**Ở đâu trong thực tế?**
- **Thuật toán Dijkstra**: tìm đường ngắn nhất. Luôn xử lý đỉnh gần nhất trước.
- **Task scheduler** trong OS: process quan trọng chạy trước.
- **Huffman coding**: nén dữ liệu, luôn ghép 2 node nhỏ nhất.
- **Game AI**: xử lý event quan trọng nhất trước.

Bên trong, priority queue dùng **binary heap** (chương trước) làm "động cơ". Interface bên ngoài đơn giản và rõ ràng: `enqueue` (thêm), `dequeue` (lấy ra), `peek` (xem).

## Hoạt động như thế nào?

Priority queue chỉ là "lớp vỏ" đẹp bên ngoài heap:

```
enqueue(3)   enqueue(1)   enqueue(5)   dequeue()→5   dequeue()→3

Heap:        Heap:        Heap:        Heap:          Heap:
  [3]         [3]          [5]          [3]            [1]
              [1]         [1][3]        [1]

Luôn lấy ra phần tử LỚN NHẤT (max-heap)
```

### Ánh xạ thao tác

| Priority Queue | Heap tương ứng | Mô tả |
|---------------|----------------|-------|
| `enqueue(val)` | `push(val)` | Thêm phần tử |
| `dequeue()` | `pop()` | Lấy phần tử ưu tiên cao nhất |
| `peek()` | `peek()` | Xem mà không lấy ra |
| `size()` | `size()` | Đếm phần tử |
| `is_empty()` | `is_empty()` | Kiểm tra rỗng |

### Ví dụ: Thuật toán Dijkstra

Dijkstra tìm đường ngắn nhất từ A đến D. Dùng min-priority queue (ưu tiên khoảng cách nhỏ nhất):

```
Đồ thị:  A --1--> B --2--> D
          A --4--> C --1--> D

Bước 1: Xử lý A (khoảng cách 0)
   enqueue B(1), C(4)
   Queue: [B(1), C(4)]

Bước 2: dequeue → B (khoảng cách 1, nhỏ nhất)
   Từ B đến D: 1+2 = 3
   enqueue D(3)
   Queue: [D(3), C(4)]

Bước 3: dequeue → D (khoảng cách 3)
   Đến đích! Đường ngắn nhất A→D = 3
   (qua A→B→D, không phải A→C→D = 5)
```

Nếu dùng queue thường (FIFO), ta sẽ xử lý theo thứ tự chèn, không phải theo khoảng cách. Kết quả có thể sai hoặc chậm.

### Ví dụ: Lập lịch task

```
enqueue(Task("backup",  priority=1))  // thấp
enqueue(Task("render",  priority=5))  // cao
enqueue(Task("log",     priority=2))  // trung bình

dequeue() → render  (priority 5)  // chạy trước
dequeue() → log     (priority 2)
dequeue() → backup  (priority 1)  // chạy sau cùng
```

Hệ điều hành dùng cách này để quyết định process nào chạy tiếp.

## Code Rust

Code đầy đủ nằm trong `src/priority_queue.rs`.

```rust
use crate::heap::BinaryHeap;

pub struct PriorityQueue<T: Ord> {
    heap: BinaryHeap<T>,   // "động cơ" bên trong
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }

    pub fn enqueue(&mut self, val: T) {
        self.heap.push(val);       // thêm vào heap
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop()            // lấy phần tử lớn nhất ra
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()           // xem mà không lấy
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn size(&self) -> usize {
        self.heap.size()
    }
}
```

Cực kỳ đơn giản! Mọi logic nặng đã nằm trong `BinaryHeap`. Priority queue chỉ đổi tên cho dễ hiểu: `push` thành `enqueue`, `pop` thành `dequeue`.

**Tại sao cần wrapper này?** Vì tên hàm quan trọng. Khi đọc code thấy `pq.enqueue()` và `pq.dequeue()`, bạn biết ngay đây là hàng đợi ưu tiên. Nếu thấy `heap.push()` và `heap.pop()`, bạn phải suy nghĩ thêm.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Giải thích |
|----------|-----------|--------|------------|
| `enqueue` | O(log n) | O(1) | = heap push |
| `dequeue` | O(log n) | O(1) | = heap pop |
| `peek` | O(1) | O(1) | Chỉ nhìn root |
| `is_empty` | O(1) | O(1) | Kiểm tra len |
| `size` | O(1) | O(1) | Đọc len |
| **Tổng bộ nhớ** | -- | **O(n)** | |

Mọi thao tác giống hệt binary heap vì priority queue chỉ là wrapper.

### So sánh các cách implement

```
                    enqueue    dequeue    peek
Mảng chưa sắp xếp   O(1)      O(n)      O(n)     ← chèn nhanh, lấy chậm
Mảng đã sắp xếp      O(n)      O(1)      O(1)     ← chèn chậm, lấy nhanh
Binary heap           O(log n)  O(log n)  O(1)     ← cân bằng cả hai!
```

Heap là sự cân bằng tốt nhất giữa chèn và lấy ra.

## Ví dụ

### Sử dụng cơ bản

```rust
use rust_ds2a::priority_queue::PriorityQueue;

let mut pq = PriorityQueue::new();
pq.enqueue(3);
pq.enqueue(1);
pq.enqueue(5);

// Luôn lấy ra phần tử lớn nhất trước
assert_eq!(pq.peek(), Some(&5));
assert_eq!(pq.dequeue(), Some(5));
assert_eq!(pq.dequeue(), Some(3));
assert_eq!(pq.dequeue(), Some(1));
assert_eq!(pq.dequeue(), None);  // hết rồi
```

### Với string (sắp xếp theo thứ tự từ điển)

```rust
use rust_ds2a::priority_queue::PriorityQueue;

let mut pq = PriorityQueue::new();
pq.enqueue("apple");
pq.enqueue("cherry");
pq.enqueue("banana");

// Theo thứ tự từ điển: "cherry" > "banana" > "apple"
assert_eq!(pq.dequeue(), Some("cherry"));
assert_eq!(pq.dequeue(), Some("banana"));
assert_eq!(pq.dequeue(), Some("apple"));
```
