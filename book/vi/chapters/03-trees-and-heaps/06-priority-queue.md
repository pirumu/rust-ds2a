# Priority Queue

> 💡 **Đừng lo lắng:** Đây có thể là chương dễ nhất series. Nếu bạn hiểu Binary Heap ở chương trước, bạn đã hiểu 100% Priority Queue -- vì nó chỉ là heap đổi tên `push`/`pop` thành `enqueue`/`dequeue`. Không algorithm mới, không cấu trúc mới. Relax.

## Đây là gì?

> **Chương này là chương DỄ NHẤT toàn series.** Nếu bạn đã hiểu Binary Heap ở chương trước, bạn đã hiểu 100% Priority Queue -- vì nó chỉ đổi tên: `push` → `enqueue`, `pop` → `dequeue`. Không có algorithm mới, không có cấu trúc mới. Giá trị thực sự của chương này là **ứng dụng**: bạn sẽ thấy Heap được dùng ở đâu trong thực tế, từ tìm đường ngắn nhất (Dijkstra) đến lập lịch hệ điều hành. Hít thở sâu, relax, và enjoy.

Bạn đến **phòng cấp cứu bệnh viện**. Ở đây không phải ai đến trước được khám trước. Người bị đau tim được khám trước người bị trầy xước, dù người bị trầy xước đến trước 2 tiếng. Ai **nặng nhất** (priority cao nhất) được phục vụ trước.

**Priority queue** (hàng đợi ưu tiên) hoạt động đúng như vậy. Khác với queue thường (FIFO -- ai vào trước ra trước), priority queue luôn lấy ra phần tử có **ưu tiên cao nhất**, bất kể thứ tự chèn.

**Ở đâu trong thực tế?**
- **Thuật toán Dijkstra**: tìm đường ngắn nhất. Luôn xử lý đỉnh gần nhất trước.
- **Task scheduler** trong OS: process quan trọng chạy trước.
- **Huffman coding**: nén dữ liệu, luôn ghép 2 node nhỏ nhất.
- **Event-driven systems**: game engine, Tokio runtime -- event sớm nhất trigger trước.

Bên trong, priority queue dùng **binary heap** (chương trước) làm "động cơ". Interface bên ngoài đơn giản và rõ ràng: `enqueue` (thêm), `dequeue` (lấy ra), `peek` (xem).

## Priority Queue vs Queue vs Stack -- ba anh em

Đặt Priority Queue trong bức tranh tổng series. Bạn đã học Stack (chương 3), Queue (chương 4), giờ là anh em cuối cùng:

| | Stack | Queue | Priority Queue |
|---|-------|-------|---------------|
| Quy tắc lấy ra | Mới nhất (LIFO) | Cũ nhất (FIFO) | **Ưu tiên cao nhất** |
| Phép ẩn dụ | Xếp đĩa | Hàng trà sữa | Phòng cấp cứu |
| `push`/`enqueue` | O(1) | O(1) | O(log n) |
| `pop`/`dequeue` | O(1) | O(1) | O(log n) |
| `peek` | O(1) | O(1) | O(1) |
| Engine bên trong | Vec | VecDeque | **BinaryHeap** |
| Chương | 3 | 4 | 6 (này) |

**"Ba anh em: Stack biết ai *mới nhất*, Queue biết ai *cũ nhất*, Priority Queue biết ai *quan trọng nhất*."**

Stack và Queue nhanh hơn (O(1) mọi thao tác) vì chúng chỉ cần nhìn đầu/cuối. Priority Queue phải trả giá O(log n) cho enqueue/dequeue vì phải **sắp xếp lại** mỗi lần -- nhưng đổi lại, nó biết ai quan trọng nhất bất kể thứ tự chèn.

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

### Tại sao cần wrapper? -- Intent & Abstraction

Tài liệu trên nói PQ "chỉ là lớp vỏ". Vậy tại sao không dùng thẳng BinaryHeap?

```rust
// Không có wrapper -- reader phải đoán ý nghĩa
let mut heap = BinaryHeap::new();
heap.push(Task { priority: 5, name: "render" });
heap.pop();  // lấy cái gì? max? min? task nào?

// Có wrapper -- ý đồ rõ ràng
let mut pq = PriorityQueue::new();
pq.enqueue(Task { priority: 5, name: "render" });
pq.dequeue();  // rõ ràng: lấy task ưu tiên cao nhất
```

3 lý do:

1. **Abstraction barrier**: user của PriorityQueue không cần biết bên trong là heap, array, hay BST. Ngày mai đổi engine → API không đổi. Code gọi `pq.enqueue()` vẫn hoạt động.

2. **Domain language**: `enqueue`/`dequeue` nói rõ đây là queue (có thứ tự ưu tiên). `push`/`pop` mơ hồ hơn -- stack? heap? Tên hàm kể câu chuyện.

3. **Adapter pattern**: đây là pattern kinh điển trong design patterns -- wrap 1 interface (heap) thành interface khác (queue) phù hợp hơn với domain. Bạn sẽ gặp pattern này khắp nơi trong production code.

Trong production Rust thực tế, nhiều người dùng thẳng `std::collections::BinaryHeap` vì nó đã đủ rõ ràng. Nhưng hiểu tại sao wrapper tồn tại giúp bạn thiết kế API tốt hơn.

## Custom Priority -- Ord trait trong Rust

Ví dụ trên dùng integer làm priority. Thực tế, bạn thường cần **custom struct** -- ví dụ task có tên và priority:

```rust
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    priority: u32,
    name: String,
}

// Priority Queue cần Ord trait
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // So sánh theo priority (cao = ưu tiên hơn)
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Sử dụng
let mut pq = PriorityQueue::new();
pq.enqueue(Task { priority: 1, name: "backup".into() });
pq.enqueue(Task { priority: 5, name: "render".into() });
pq.enqueue(Task { priority: 2, name: "log".into() });

assert_eq!(pq.dequeue().unwrap().name, "render");  // priority 5 ra trước
```

### Min-priority queue (cho Dijkstra)

Max-priority queue lấy **lớn nhất** ra trước. Dijkstra cần **nhỏ nhất** ra trước (node gần nhất). Hai cách:

```rust
// Cách 1: đảo cmp trong Ord
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()  // đảo!
    }
}

// Cách 2: wrap trong Reverse (nếu dùng BinaryHeap trực tiếp)
use std::cmp::Reverse;
heap.push(Reverse(Node { distance: 5, ... }));
// pop() sẽ lấy distance nhỏ nhất
```

**Note:** Implement `Ord` cho custom struct là kỹ năng Rust quan trọng. Bạn sẽ cần nó mỗi khi dùng `BinaryHeap`, `BTreeMap`, hoặc `.sort()`. Nếu struct có `f64` (không implement `Ord` vì `NaN`), dùng crate `ordered_float::OrderedFloat` hoặc implement thủ công với `.total_cmp()`.

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

## Dijkstra chi tiết -- PQ trong hành động

Dijkstra tìm đường ngắn nhất từ một đỉnh đến tất cả các đỉnh khác. Đây là **ứng dụng quan trọng nhất** của Priority Queue.

### Graph đơn giản (warm-up)

```
Đồ thị:  A --1--> B --2--> D
          A --4--> C --1--> D

Dùng min-priority queue (ưu tiên khoảng cách nhỏ nhất):

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

### Full trace -- graph 6 node

Giờ trace kỹ hơn với graph lớn hơn. Đây là lúc bạn thấy PQ **thực sự tỏa sáng**:

```
Graph (weighted, directed):

    A ---1--- B ---3--- D
    |         |         |
    4         1         2
    |         |         |
    C ---2--- E ---1--- F

Tìm đường ngắn nhất từ A đến tất cả node.

Min-Priority Queue trace:

Bước  dequeue      Update neighbors            PQ state                  dist[]
──────────────────────────────────────────────────────────────────────────────────
 0    --           enqueue A(0)                [(A,0)]                   A=0
 1    A(0)         B:0+1=1, C:0+4=4           [(B,1), (C,4)]            A=0,B=1,C=4
 2    B(1)         D:1+3=4, E:1+1=2           [(E,2), (C,4), (D,4)]    B=1,E=2
 3    E(2)         C:2+2=4(skip,đã=4)         [(F,3), (C,4), (D,4)]    E=2,F=3
                   F:2+1=3
 4    F(3)         D:3+2=5(skip,đã có 4)      [(C,4), (D,4)]           F=3
 5    C(4)         (no better paths)           [(D,4)]                  C=4
 6    D(4)         (done)                      []                       D=4

Shortest paths from A:
  A→B = 1   (A→B)
  A→E = 2   (A→B→E)
  A→F = 3   (A→B→E→F)
  A→C = 4   (A→C)
  A→D = 4   (A→B→D)
```

**Tại sao PQ quan trọng cho Dijkstra?**

Nhìn bước 3: PQ cho ta `E(2)` thay vì `C(4)` hay `D(4)`. Nhờ xử lý E trước, ta tìm được `F(3)` sớm hơn -- nếu xử lý C trước, ta sẽ không tìm được đường tốt hơn đến F.

```
Không có PQ → mỗi bước phải scan TẤT CẢ node chưa visited để tìm min
           → O(V²) tổng

Có PQ      → mỗi bước dequeue O(log V) + update neighbors
           → O((V+E) log V) tổng

Với graph 10,000 node: V² = 100 triệu vs (V+E)logV ≈ 200,000
                        → nhanh hơn 500 lần!
```

Nếu dùng queue thường (FIFO), ta sẽ xử lý theo thứ tự chèn, không phải theo khoảng cách. Kết quả có thể sai hoặc chậm.

## Event-Driven Systems

Game engine, Tokio runtime, network server -- tất cả đều dùng concept tương tự Priority Queue để quản lý events theo thời gian.

### Game engine event loop

```
PQ contents (sorted by timestamp):
  [t=100ms: render_frame]
  [t=150ms: physics_update]
  [t=200ms: AI_decision]
  [t=250ms: network_sync]

Loop:
  now = current_time()
  while pq.peek().timestamp <= now:
    event = pq.dequeue()         // lấy event sớm nhất
    event.execute()
    if event.repeating:
      event.timestamp += event.interval
      pq.enqueue(event)          // reschedule
```

Mỗi frame, game chỉ xử lý events **đến hạn** -- không cần scan toàn bộ event list. PQ đảm bảo event sớm nhất luôn ở root.

### Ứng dụng thực tế

- **Tokio runtime**: internally dùng timer structure tương tự PQ để schedule `sleep()` và timeout. Khi bạn viết `tokio::time::sleep(Duration::from_secs(5))`, Tokio enqueue 1 timer event, rồi dequeue khi đến hạn.
- **Network server**: process request by deadline (earliest deadline first). Request nào sắp timeout → xử lý trước.
- **OS scheduler**: process ưu tiên cao chạy trước. Linux CFS dùng Red-Black tree, nhưng nhiều OS khác (Windows, FreeBSD) dùng heap-based scheduler.

## Huffman Coding -- nén dữ liệu

Doc trước nhắc "Huffman coding dùng PQ" nhưng chưa giải thích. Đây là concept:

**Ý tưởng**: ký tự xuất hiện nhiều → mã ngắn. Ký tự xuất hiện ít → mã dài. Để xây cây mã tối ưu, luôn ghép 2 node **nhỏ nhất** → dùng min-PQ.

```
Input: "aabbbcccc"
Frequency: a=2, b=3, c=4

Bước 1: Bỏ tất cả vào min-PQ
  PQ: [a(2), b(3), c(4)]

Bước 2: Ghép 2 nhỏ nhất: a(2) + b(3) = ab(5)
  PQ: [c(4), ab(5)]

Bước 3: Ghép 2 nhỏ nhất: c(4) + ab(5) = abc(9)
  PQ: [abc(9)]  ← done, chỉ còn 1 node = root

Cây Huffman:
      abc(9)
     /     \
   c(4)   ab(5)
          /   \
        a(2)  b(3)

Gán mã: trái=0, phải=1
  c = 0        (1 bit)
  a = 10       (2 bit)
  b = 11       (2 bit)

"aabbbcccc" = 10,10,11,11,11,0,0,0,0 = 17 bits
Không nén: 9 chars × 8 bits = 72 bits
→ Tiết kiệm 76%!
```

c xuất hiện nhiều nhất (4 lần) → mã ngắn nhất (1 bit). a xuất hiện ít nhất (2 lần) → mã dài hơn (2 bit). PQ đảm bảo luôn ghép 2 node nhỏ nhất → cây optimal. Không có PQ, Huffman coding không thể hoạt động hiệu quả.

## Decrease Key -- thao tác bị thiếu

Trong Dijkstra, khi tìm được đường ngắn hơn đến node X, ta cần **cập nhật** priority của X trong PQ. Nhưng `BinaryHeap` / `PriorityQueue` không hỗ trợ "tìm và update" hiệu quả -- search là O(n).

**3 cách giải quyết thực tế:**

### 1. Lazy deletion (phổ biến nhất, đơn giản nhất)

```
Không update, chỉ enqueue thêm bản mới (priority thấp hơn).
Khi dequeue, skip nếu đã visited.

Ví dụ: node D ban đầu distance=10, sau tìm được distance=4
  → Không xóa D(10), chỉ enqueue D(4)
  → PQ chứa cả D(4) và D(10) (duplicate)
  → Khi dequeue D(4), xử lý bình thường
  → Khi dequeue D(10) sau, thấy D đã visited → skip

Trade-off: PQ chứa duplicate → dùng nhiều memory hơn
Nhưng code CỰC KỲ đơn giản → dùng trong 90% trường hợp
```

### 2. Index heap (decrease-key O(log n))

```
HashMap lưu vị trí mỗi element trong heap array.
Khi update priority → sửa trực tiếp + sift-up/sift-down.

Trade-off: code phức tạp hơn nhiều, nhưng optimal về memory và time.
Dùng khi PQ cực lớn và memory là constraint.
```

### 3. `BinaryHeap::peek_mut()` trong Rust std

```rust
// Modify root in-place, tự sift-down khi PeekMut drop
if let Some(mut top) = heap.peek_mut() {
    *top = new_value;
}
// Chỉ modify được ROOT, không phải arbitrary element
// Nhưng hữu ích trong 1 số pattern đặc biệt
```

**Recommend**: Dùng lazy deletion cho hầu hết trường hợp. Index heap chỉ cần khi PQ cực lớn và memory là constraint. Crate `priority-queue` trên crates.io implement index heap sẵn nếu bạn cần.

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

### So sánh các cách implement PQ

```
                    enqueue    dequeue    peek
Mảng chưa sắp xếp   O(1)      O(n)      O(n)     ← chèn nhanh, lấy chậm
Mảng đã sắp xếp      O(n)      O(1)      O(1)     ← chèn chậm, lấy nhanh
Linked list sorted    O(n)      O(1)      O(1)     ← chèn chậm, lấy nhanh
BST balanced          O(log n)  O(log n)  O(log n) ← cân bằng nhưng overhead cao
Binary heap           O(log n)  O(log n)  O(1)     ← CÂN BẰNG + peek O(1)!
```

Heap thắng vì: enqueue và dequeue đều O(log n) (như BST), nhưng peek O(1) (tốt hơn BST), và cache-friendly (array-based, không pointer chasing).

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

### Lập lịch task

```
enqueue(Task("backup",  priority=1))  // thấp
enqueue(Task("render",  priority=5))  // cao
enqueue(Task("log",     priority=2))  // trung bình

dequeue() → render  (priority 5)  // chạy trước
dequeue() → log     (priority 2)
dequeue() → backup  (priority 1)  // chạy sau cùng
```

Hệ điều hành dùng cách này để quyết định process nào chạy tiếp.

## Những cái bẫy hay gặp

### a) Nhầm Priority Queue với sorted array

- ❌ "PQ sorted nên phần tử thứ 2 chắc chắn lớn thứ 2"
- ✅ PQ chỉ đảm bảo phần tử **TOP** (max/min) ra trước. Phần tử thứ 2, 3 không nhất thiết theo thứ tự cho đến khi dequeue.
- 💡 Muốn full sorted order → dùng `BTreeSet`. PQ chỉ biết ai **số 1**.

### b) Quên implement Ord cho custom struct

- ❌ `BinaryHeap::new()` với struct không có `Ord` → compiler error
- ✅ `BinaryHeap` cần `T: Ord`. Phải implement `Ord`, `PartialOrd`, `Eq`, `PartialEq`.
- 💡 Nếu struct có `f64` (không implement `Ord` vì `NaN`), dùng `ordered_float::OrderedFloat` hoặc implement thủ công với `f64::total_cmp()`.

### c) Dùng PQ khi chỉ cần min/max 1 lần

- ❌ Build PQ chỉ để lấy 1 phần tử lớn nhất
- ✅ `vec.iter().min()` là O(n) -- nhanh hơn build PQ rồi dequeue (O(n) heapify + O(log n) pop).
- 💡 PQ chỉ đáng khi cần **repeated** min/max operations. 1 lần thì `iter().min()` đủ rồi.

### d) Không handle duplicate priorities

- ❌ 2 task cùng priority 5, nghĩ task nào vào trước ra trước
- ✅ `BinaryHeap` Rust **không stable** -- không đảm bảo FIFO trong cùng priority. Thứ tự dequeue giữa các phần tử cùng priority là **không xác định**.
- 💡 Nếu cần FIFO tie-breaking, thêm sequence number:

```rust
// (priority, sequence_number) -- sequence nhỏ = vào trước
// Reverse sequence để vào trước ra trước
struct Entry {
    priority: u32,
    seq: u64,        // tăng dần mỗi lần enqueue
    task: String,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)           // priority cao trước
            .then(other.seq.cmp(&self.seq))          // cùng priority → seq nhỏ trước
    }
}
```

## Khi nào dùng / không nên dùng

| Tình huống | PQ? | Thay bằng gì? | Tại sao? |
|------------|-----|---------------|----------|
| Dijkstra/Prim shortest path | ✅ | -- | Lấy node gần nhất liên tục |
| OS task scheduler | ✅ | -- | Task priority cao chạy trước |
| Event-driven system / timer | ✅ | -- | Event sớm nhất trigger trước |
| Huffman coding | ✅ | -- | Luôn ghép 2 node nhỏ nhất |
| Top-K elements | ✅ | -- | Min-PQ size K |
| Merge K sorted lists | ✅ | -- | O(N log K) |
| Cần sorted iteration | ❌ | BTreeSet | PQ không giữ full sorted order |
| Cần search by value | ❌ | HashMap | PQ search O(n) |
| Task queue FIFO đơn giản | ❌ | Queue (VecDeque) | Không cần priority |
| Cần stable ordering (FIFO tie-break) | ⚠️ | PQ + sequence number | BinaryHeap không stable |

## Luyện nhận diện Pattern

### Bài 1: K Closest Points to Origin (LeetCode #973)

Cho mảng points trên 2D plane, tìm K điểm gần gốc tọa độ nhất.

Ví dụ: `points = [[1,3], [-2,2]], K = 1` → `[[-2,2]]` (khoảng cách √8 < √10)

<details>
<summary>Gợi ý</summary>

"Gần nhất" = ưu tiên khoảng cách **nhỏ**. Nhưng dùng **max-PQ size K** (không phải min-PQ!). Tại sao?

Max-PQ size K giữ K điểm gần nhất. Root = điểm **xa nhất** trong K ứng viên. Khi điểm mới gần hơn root → pop root, push điểm mới. Giống Top-K pattern ở chương Heap nhưng đảo ngược.

```rust
use std::collections::BinaryHeap;

fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    // Max-heap by distance (default BinaryHeap)
    let mut heap: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();
    for p in points {
        let dist = p[0] * p[0] + p[1] * p[1];  // không cần sqrt
        heap.push((dist, p));
        if heap.len() > k {
            heap.pop();  // loại xa nhất
        }
    }
    heap.into_iter().map(|(_, p)| p).collect()
}
```

</details>

### Bài 2: Reorganize String (LeetCode #767)

Cho string `"aab"`, sắp xếp lại sao cho không có 2 ký tự giống nhau liên tiếp. Trả về `""` nếu không thể.

Ví dụ: `"aab"` → `"aba"`. `"aaab"` → `""` (không thể).

<details>
<summary>Gợi ý</summary>

Ký tự xuất hiện nhiều nhất → khó xếp nhất → nên ưu tiên đặt trước. Dùng **max-PQ theo frequency**.

Mỗi bước: pop ký tự nhiều nhất, đặt vào result. Nếu ký tự vừa đặt lần trước còn count > 0, push lại vào PQ.

Điều kiện không thể: ký tự nào có count > (len + 1) / 2.

</details>

### Bài 3: Network Delay Time (LeetCode #743)

Cho network N node, danh sách edges có weight (truyền tín hiệu mất bao lâu), gửi tín hiệu từ node K. Bao lâu tất cả node nhận được?

Ví dụ: `times = [[2,1,1],[2,3,1],[3,4,1]], n=4, k=2` → `2`

<details>
<summary>Gợi ý</summary>

Đây **chính là Dijkstra**! Tìm shortest path từ K đến tất cả node. Answer = max distance trong tất cả shortest paths. Nếu có node unreachable → return -1.

Dùng min-PQ, y hệt trace ở phần Dijkstra trên. Sau khi chạy xong, answer = `dist.values().max()`.

</details>

## PQ trong Rust ecosystem

### Standard library

- `std::collections::BinaryHeap` -- max-heap, dùng `Reverse` cho min. Đây là PQ built-in của Rust. Trong production, nhiều người dùng trực tiếp thay vì wrapper.

### Crates hữu ích

- **`priority-queue`** crate -- PQ với decrease-key support (index heap). Khi bạn cần update priority của phần tử đã có trong PQ.
- **`keyed_priority_queue`** crate -- PQ cho phép update priority by key. API: `pq.set_priority(&key, new_priority)`.
- **`ordered_float`** crate -- `OrderedFloat<f64>` implement `Ord`, giải quyết vấn đề f64 + BinaryHeap.

### Trong async Rust

- **Tokio**: `tokio::time::sleep` internally dùng timer structure tương tự PQ để schedule wakeup. Mỗi `sleep()` = 1 entry trong timer heap.
- **Message ordering**: nếu consumer cần xử lý messages by timestamp thay vì by partition order → PQ cho cross-partition ordering. Merge K sorted streams = min-PQ pattern.

## Tiếp theo: B-Tree

Priority Queue kết thúc hành trình "lấy max/min hiệu quả". Bạn đã đi từ cây đơn giản nhất (Binary Tree) qua search trees (BST → AVL → Red-Black), đến heap-based structures (Binary Heap → Priority Queue).

Chương tiếp theo chuyển sang **B-Tree** -- "ông trùm" cuối cùng của gia đình tree. B-Tree không giới hạn 2 con per node mà cho phép **hàng trăm key per node**. Đây là cấu trúc đằng sau database index (MySQL, PostgreSQL) và `BTreeMap` trong Rust std. Nếu bạn từng thắc mắc "tại sao database nhanh thế?" -- câu trả lời nằm ở chương tiếp.

---

---

[← Binary Heap](./05-binary-heap.md) | [B-Tree →](./07-b-tree.md)
