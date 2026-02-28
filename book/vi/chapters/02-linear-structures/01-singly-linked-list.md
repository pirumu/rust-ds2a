# Singly Linked List

> 💡 **Đừng lo lắng:** Linked list trong Rust nổi tiếng khó vì ownership, nhưng bản thân ý tưởng cực kỳ đơn giản -- chỉ là mỗi ô nhớ trỏ tới ô tiếp theo, như đoàn tàu nối toa. Rust compiler sẽ giữ lưng bạn, không cho sai memory. Cứ thử thoải mái.

## Đây là gì?

Hình dung một **đoàn tàu hỏa**. Mỗi toa tàu chứa hàng hóa bên trong, và có một móc nối duy nhất -- nối tới toa **phía sau**. Toa cuối cùng không nối với gì cả. Và quan trọng: mỗi toa **chỉ biết toa tiếp theo**, không biết toa trước mình là toa nào.

```
  đầu tàu
    |
    v
 [Toa 1] --> [Toa 2] --> [Toa 3] --> không có gì (None)
```

Đó chính là **singly linked list** (danh sách liên kết đơn).

Mỗi toa tàu gọi là một **node** (nút). Mỗi node chứa 2 thứ:
1. **val** -- giá trị (hàng hóa trong toa)
2. **next** -- con trỏ tới node tiếp theo (móc nối tới toa sau)

Toa đầu tiên gọi là **head** (đầu danh sách). Chúng ta chỉ cần nhớ head là đủ. Muốn tìm toa nào, cứ đi từ head, lần theo móc nối.

**Khác gì với array?**

Array giống như một dãy tủ đồ đánh số 0, 1, 2, 3... Bạn muốn tủ số 5? Đi thẳng tới. Nhanh lắm -- O(1).

Linked list thì không có số. Muốn tới node thứ 5? Phải đi từ đầu, đếm 1, 2, 3, 4, 5. Chậm hơn -- O(n).

Nhưng linked list có lợi thế: **thêm/xóa ở đầu danh sách cực nhanh** -- O(1). Không cần dịch chuyển gì cả. Với array, thêm ở đầu phải đẩy tất cả phần tử sang phải -- O(n).

### Bộ nhớ: tại sao random access nhanh với array nhưng chậm với linked list?

Để hiểu sâu hơn, hãy nhìn cách máy tính lưu trữ hai cấu trúc này trong bộ nhớ:

```
                       BỘ NHỚ MÁY TÍNH (RAM)
 Địa chỉ:   0x100  0x104  0x108  0x10C  0x110  ...

 ┌─── ARRAY ─────────────────────────────────┐
 │  [10]   [20]   [30]   [40]   [50]         │  ← các ô NẰM LIỀN NHAU
 │  0x100  0x104  0x108  0x10C  0x110         │
 └────────────────────────────────────────────┘
   Muốn phần tử thứ 3?
   → Tính: 0x100 + 3×4 = 0x10C → nhảy thẳng tới! O(1)

 ┌─── LINKED LIST ────────────────────────────┐
 │  [10|→0x250]      [30|→0x800]              │
 │  0x100             0x108                    │
 │                                             │
 │        [20|→0x108]           [40|→None]     │  ← các node NẰM RẢI RÁC
 │        0x250                 0x800          │
 └─────────────────────────────────────────────┘
   Muốn phần tử thứ 3?
   → Phải đi: 0x100 → 0x250 → 0x108 → 0x800    O(n)
     Không tính được địa chỉ, phải lần theo từng pointer!
```

Array nằm liền nhau trong bộ nhớ, nên máy tính tính được địa chỉ bằng phép cộng đơn giản. Linked list nằm rải rác -- mỗi node có thể ở bất cứ đâu trên heap -- nên phải đi từng bước.

### So sánh Array vs Linked List

| Thao tác | Array / Vec | Linked List | Tại sao? |
|----------|-------------|-------------|----------|
| Truy cập vị trí thứ k | **O(1)** | O(n) | Array tính địa chỉ, LL phải đi bộ |
| Thêm vào đầu | O(n) | **O(1)** | Array phải đẩy tất cả sang phải |
| Thêm vào cuối | **O(1)\*** | O(n) | Array có sẵn vị trí cuối, LL phải đi tìm |
| Thêm vào giữa | O(n) | O(n) | Cả hai đều phải tìm vị trí |
| Xóa ở đầu | O(n) | **O(1)** | Array phải dịch trái, LL chỉ đổi head |
| Xóa ở cuối | **O(1)** | O(n) | LL phải tìm node áp cuối |
| Xóa ở giữa | O(n) | O(n) | Cả hai phải tìm + dịch/nối lại |
| Tìm kiếm | O(n) | O(n) | Cả hai phải duyệt qua |

_\* Vec amortized O(1) -- đôi khi phải resize, nhưng trung bình vẫn O(1)._

> **Rule of thumb:** Nếu hay truy cập theo vị trí → dùng Array/Vec. Nếu hay thêm/xóa ở đầu → dùng Linked List.

### Tại sao Rust cần `Box`?

Khi bạn viết code, Rust compiler cần biết **kích thước** của mỗi kiểu dữ liệu lúc compile.

Vấn đề: một `Node` chứa một `Node` khác bên trong, mà node đó lại chứa node khác... Kích thước là vô hạn! Compiler không thể tính được.

```
// Compiler la lên: "Kích thước vô hạn!"
struct Node {
    val: i32,
    next: Option<Node>,  // Node chứa Node chứa Node...
}
```

**`Box`** là giải pháp. `Box<T>` là một con trỏ tới dữ liệu trên **heap** (vùng nhớ động). Con trỏ luôn có kích thước cố định (8 byte trên máy 64-bit). Compiler vui rồi!

```
// OK! Box có kích thước cố định
struct Node {
    val: i32,
    next: Option<Box<Node>>,  // Box = 8 byte, compiler biết rồi
}
```

Còn **`Option`** thì sao? `Option<Box<Node>>` nghĩa là: "có thể có node tiếp theo (`Some`), hoặc không có gì (`None`)". Đây là cách Rust thay thế cho null pointer. An toàn hơn nhiều.

## Hoạt động như thế nào?

### Cấu trúc node

```
  head
   |
   v
+-------+     +-------+     +-------+
| val: 3|     | val: 7|     | val: 1|
| next:--+--->| next:--+--->| next:--+--> None
+-------+     +-------+     +-------+
```

### push_front -- Thêm vào đầu (O(1))

Giống như nối thêm một toa mới vào **đầu** đoàn tàu.

**Bước 1:** Tạo node mới, cho `next` của nó trỏ vào head cũ.

**Bước 2:** Cập nhật head = node mới.

```
Trước: head --> [3] --> [7] --> [1] --> None

Bước 1: Tạo node mới [5], trỏ vào head cũ [3]

         new_node     head cũ
            |            |
            v            v
         [5] -------> [3] --> [7] --> [1] --> None

Bước 2: head = new_node

  head
   |
   v
  [5] --> [3] --> [7] --> [1] --> None
```

Trong Rust, `self.head.take()` lấy head cũ ra (thay bằng `None`). Rồi gán nó làm `next` của node mới. Cuối cùng, node mới trở thành head. Tất cả là O(1) -- không cần duyệt gì cả.

### pop_front -- Xóa ở đầu (O(1))

Giống như tháo toa đầu tiên ra khỏi đoàn tàu.

```
Trước:  head --> [3] --> [7] --> [1] --> None

Bước 1: Lấy node đầu [3] ra
Bước 2: head = node tiếp theo [7]

Sau:    head --> [7] --> [1] --> None    (trả về giá trị 3)
```

### push_back -- Thêm vào cuối (O(n))

Giống như nối thêm toa vào **cuối** đoàn tàu. Nhưng vì mỗi toa chỉ biết toa tiếp theo, ta phải **đi bộ từ đầu tới cuối** mới biết toa cuối ở đâu!

```
Trước: head --> [3] --> [7] --> [1] --> None

Bước 1: Đi từ head [3] -> [7] -> [1] (tìm toa cuối)
Bước 2: Gán [1].next = [9]

Sau:   head --> [3] --> [7] --> [1] --> [9] --> None
```

Phải đi qua n node, nên là O(n).

### reverse -- Đảo ngược (O(n))

Giống như đảo ngược hướng tất cả móc nối trong đoàn tàu. Toa cuối thành toa đầu. Toa đầu thành toa cuối.

```
Trước: head --> [1] --> [2] --> [3] --> None
Sau:   head --> [3] --> [2] --> [1] --> None
```

Cách làm: dùng 3 biến -- `prev`, `current`, `next` -- đi qua từng node một lần.

```
Bước 0:  prev=None   current=[1]-->  [2]-->  [3]-->None

Bước 1:  Lưu next = [2]
         Đổi [1].next = prev (None)
         prev = [1]    current = [2]

         None <--[1]   [2]-->  [3]-->None
                  ^     ^
                 prev  current

Bước 2:  Lưu next = [3]
         Đổi [2].next = prev ([1])
         prev = [2]    current = [3]

         None <--[1] <--[2]   [3]-->None
                         ^     ^
                        prev  current

Bước 3:  Lưu next = None
         Đổi [3].next = prev ([2])
         prev = [3]    current = None

         None <--[1] <--[2] <--[3]
                                ^
                               prev

Kết thúc: head = prev = [3]
```

### Custom Drop -- tại sao cần và tại sao có thể stack overflow

Mặc định, khi Rust xóa một linked list, nó xóa **đệ quy**. Nghĩa là:

```
drop(node1)
  └→ drop(node1.next) = drop(node2)
       └→ drop(node2.next) = drop(node3)
            └→ drop(node3.next) = drop(node4)
                 └→ ...1 triệu tầng sâu...
```

Mỗi lần gọi `drop()`, máy tính phải **tạo một stack frame** -- một khung nhớ trên stack để lưu thông tin của lần gọi hàm đó. Stack có giới hạn (thường 8MB). Mỗi frame chiếm khoảng vài chục byte, nhưng 1 triệu frame = hàng chục MB → **stack overflow!**

```
  STACK (giới hạn ~8MB)            HEAP
  ┌──────────────────┐
  │ drop(node1)      │ ──→ [node1]
  │ drop(node2)      │ ──→ [node2]
  │ drop(node3)      │ ──→ [node3]
  │ ...              │
  │ drop(node999999) │ ──→ [node999999]
  │ drop(node1000000)│ ──→ [node1000000]
  │ 💥 STACK OVERFLOW│
  └──────────────────┘
```

**Giải pháp:** viết `Drop` thủ công, xóa từng node bằng **vòng lặp** thay vì đệ quy. Vòng lặp chỉ dùng 1 stack frame duy nhất, bất kể list dài bao nhiêu:

```
  STACK (luôn chỉ 1 frame)        HEAP
  ┌──────────────────┐
  │ drop (vòng lặp)  │ ──→ xóa [node1] → xóa [node2] → ... → xóa [node1000000]
  │                  │     ✅ An toàn! O(n) thời gian, O(1) stack
  └──────────────────┘
```

Liên hệ Big-O: đệ quy sâu n tầng = **O(n) stack space**. Với n = 1,000,000 thì vượt giới hạn. Vòng lặp = **O(1) stack space**.

## Two Pointer Patterns trên Linked List

Đây là 2 pattern phỏng vấn quan trọng nhất của linked list. Cả hai đều dùng ý tưởng: **hai "con trỏ" di chuyển trên list với tốc độ khác nhau**.

### Pattern 1: Slow/Fast Pointer (Rùa và Thỏ)

Hình dung hai người chạy trên cùng một con đường. Người nhanh chạy **gấp đôi** tốc độ người chậm. Khi người nhanh về đích, người chậm mới đi được **nửa đường**. Đó chính là nguyên lý slow/fast pointer.

#### Ứng dụng 1: Tìm node ở giữa danh sách

Bài toán: cho list `[1] → [2] → [3] → [4] → [5]`, tìm node giữa.

Cách "ngây thơ": đếm chiều dài n, rồi đi tới node n/2. Phải duyệt list **2 lần**.

Cách slow/fast: **chỉ 1 lần duyệt!**

```
Bắt đầu: slow = [1], fast = [1]

Bước 1:  slow đi 1 bước → [2]
         fast đi 2 bước → [3]

  [1]  [2]  [3]  [4]  [5] → None
        ^         ^
       slow      fast

Bước 2:  slow đi 1 bước → [3]
         fast đi 2 bước → [5]

  [1]  [2]  [3]  [4]  [5] → None
              ^              ^
             slow           fast

Bước 3:  fast.next = None → DỪNG!

  slow = [3] → Đó là node giữa! ✓
```

Tại sao đúng? Fast đi nhanh gấp đôi. Khi fast hết đường (tới cuối list), slow mới đi được nửa đường → đúng ở giữa.

```rust
pub fn find_middle(&self) -> Option<&T> {
    let mut slow = self.head.as_deref()?;
    let mut fast = self.head.as_deref()?;

    while let Some(next) = fast.next.as_deref() {
        if let Some(next_next) = next.next.as_deref() {
            fast = next_next;
            slow = slow.next.as_deref().unwrap();
        } else {
            break;
        }
    }
    Some(&slow.val)
}
```

`as_deref()` giúp ta **mượn** node để đọc mà không lấy ownership. `slow` và `fast` đều là `&SinglyNode<T>` -- chỉ đọc, không sửa.

#### Ứng dụng 2: Phát hiện vòng lặp (Cycle Detection)

Quay lại ví dụ đoàn tàu: nếu toa cuối **nối ngược** về một toa ở giữa, đoàn tàu sẽ chạy vòng vòng mãi mãi. Đó là **cycle** (vòng lặp).

```
  [1] → [2] → [3] → [4] → [5]
                ^              |
                └──────────────┘   ← cycle! [5].next trỏ về [3]
```

Nếu duyệt list bình thường, chương trình sẽ **chạy mãi không dừng**.

**Slow/fast pointer giải quyết:** nếu có cycle, fast chạy vòng vòng, slow cũng chạy vòng vòng. Vì fast nhanh hơn, fast sẽ **bắt kịp slow** từ phía sau -- như chạy trên đường vòng, người nhanh luôn bắt kịp người chậm.

```
Bước 1:  slow=[1], fast=[1]
Bước 2:  slow=[2], fast=[3]
Bước 3:  slow=[3], fast=[5]
Bước 4:  slow=[4], fast=[4]  ← GẶP NHAU! Có cycle!
                                (fast đi [5]→[3]→[4], slow đi [3]→[4])
```

Tại sao **guaranteed gặp nhau**? Khi cả hai đã vào vòng, mỗi bước fast tiến gần slow thêm 1 node. Khoảng cách giảm dần: d, d-1, d-2, ... 1, 0. Không bao giờ "nhảy qua" nhau.

**Lưu ý Rust thú vị:** với `Box<Node>` (ownership duy nhất), bạn **không thể tạo cycle** trong safe Rust! Vì mỗi node chỉ có 1 chủ -- không thể có 2 pointer trỏ tới cùng 1 node. Đây là một trong những điểm tuyệt vời của ownership system: Rust **ngăn chặn cycle** ở cấp độ type system.

Cycle detection thường gặp trong các ngôn ngữ dùng garbage collection (Java, Python) hoặc khi dùng raw pointer / `Rc` trong Rust.

```rust
// Pseudocode minh họa (không thể tạo cycle với Box trong safe Rust)
fn has_cycle(head: &Node) -> bool {
    let mut slow = head;
    let mut fast = head;
    while fast.next != None && fast.next.next != None {
        slow = slow.next;         // đi 1 bước
        fast = fast.next.next;    // đi 2 bước
        if slow == fast {
            return true;          // gặp nhau → có cycle
        }
    }
    false  // fast tới None → không có cycle
}
```

### Pattern 2: Merge Two Sorted Lists

Hai hàng người đã **xếp theo chiều cao** từ thấp tới cao. Bạn muốn gộp thành một hàng, vẫn giữ thứ tự. Cách làm: **so sánh người đầu** của 2 hàng, chọn người thấp hơn bước vào trước.

```
List 1:  [1] → [3] → [5] → None
List 2:  [2] → [4] → [6] → None

So sánh 1 vs 2 → chọn 1    Result: [1]
So sánh 3 vs 2 → chọn 2    Result: [1] → [2]
So sánh 3 vs 4 → chọn 3    Result: [1] → [2] → [3]
So sánh 5 vs 4 → chọn 4    Result: [1] → [2] → [3] → [4]
So sánh 5 vs 6 → chọn 5    Result: [1] → [2] → [3] → [4] → [5]
List 1 hết → lấy hết list 2 Result: [1] → [2] → [3] → [4] → [5] → [6]
```

**Big-O:** O(n+m) thời gian -- mỗi node chỉ được xét đúng 1 lần. O(1) bộ nhớ thêm (chỉ dùng lại các node có sẵn).

```rust
pub fn merge_sorted_lists<T: Ord>(
    mut list1: SinglyLinkedList<T>,
    mut list2: SinglyLinkedList<T>,
) -> SinglyLinkedList<T> {
    let mut result = SinglyLinkedList::new();
    loop {
        // So sánh phần tử đầu của 2 list
        let take_from_1 = match (list1.head.as_ref(), list2.head.as_ref()) {
            (None, None) => break,           // cả 2 hết → xong
            (Some(_), None) => true,          // list2 hết → lấy từ list1
            (None, Some(_)) => false,         // list1 hết → lấy từ list2
            (Some(a), Some(b)) => a.val <= b.val,  // lấy node nhỏ hơn
        };
        if take_from_1 {
            if let Some(val) = list1.pop_front() {
                result.push_front(val);       // push_front = O(1)
            }
        } else {
            if let Some(val) = list2.pop_front() {
                result.push_front(val);
            }
        }
    }
    result.reverse();  // đảo ngược vì push_front cho kết quả ngược
    result
}
```

**Trick:** vì `push_back` là O(n) (phải đi hết list), ta dùng `push_front` (O(1)) rồi `reverse` ở cuối. Kết quả vẫn đúng mà tổng complexity chỉ O(n+m).

## Code Rust

Code đầy đủ nằm trong `src/linked_list.rs`.

### Struct Node và List

```rust
struct SinglyNode<T> {
    val: T,
    next: Option<Box<SinglyNode<T>>>,
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<SinglyNode<T>>>,
    len: usize,
}
```

`Box<SinglyNode<T>>` -- node được cấp phát trên heap, chỉ có **một chủ sở hữu** duy nhất.

`Option` -- có thể là `Some(node)` hoặc `None`. Không bao giờ có null pointer.

`len` -- lưu số lượng node, để `len()` là O(1) thay vì phải đếm.

### push_front -- O(1)

```rust
pub fn push_front(&mut self, val: T) {
    // Create new node, take old head as its next
    let new_node = Box::new(SinglyNode {
        val,
        next: self.head.take(),
    });
    // New node becomes head
    self.head = Some(new_node);
    self.len += 1;
}
```

`self.head.take()` -- lấy giá trị cũ ra, thay bằng `None`. Giống như "rút head cũ ra để gắn vào node mới". Đây là cách Rust an toàn để di chuyển ownership mà không vi phạm borrow checker.

### pop_front -- O(1)

```rust
pub fn pop_front(&mut self) -> Option<T> {
    // Take the head node out
    self.head.take().map(|node| {
        // The next node becomes the new head
        self.head = node.next;
        self.len -= 1;
        // Return the value
        node.val
    })
}
```

Nếu list rỗng, `take()` trả về `None`. `map` không chạy, hàm trả về `None`. An toàn!

### push_back -- O(n)

```rust
pub fn push_back(&mut self, val: T) {
    let new_node = Box::new(SinglyNode { val, next: None });
    let mut current = &mut self.head;
    while let Some(node) = current {
        current = &mut node.next;
    }
    *current = Some(new_node);
    self.len += 1;
}
```

Phải đi từ head tới cuối -- O(n). Nếu list rỗng thì node mới là head luôn.

### reverse -- O(n)

```rust
pub fn reverse(&mut self) {
    let mut prev = None;
    let mut current = self.head.take();
    while let Some(mut node) = current {
        // Save next before we overwrite it
        let next = node.next.take();
        // Reverse the pointer
        node.next = prev;
        // Move forward
        prev = Some(node);
        current = next;
    }
    self.head = prev;
}
```

Đi qua mỗi node đúng 1 lần. Đổi hướng `next` pointer. O(n) thời gian, O(1) bộ nhớ thêm.

### nth -- Lấy phần tử thứ n (O(n))

Hai thao tác `nth` và `find` hay gặp trong bài tập. Cả hai đều phải duyệt list vì không có random access.

```rust
pub fn nth(&self, n: usize) -> Option<&T> {
    self.iter().nth(n)
}
```

Đơn giản vì ta đã có iterator! `.nth(n)` tự động đi n bước.

Nhấn mạnh: **O(n)** vì phải đi từ đầu. Đây là "cái giá" của linked list. Array thì `array[n]` là O(1).

### find -- Tìm phần tử (O(n))

```rust
impl<T: PartialEq> SinglyLinkedList<T> {
    pub fn find(&self, target: &T) -> bool {
        self.iter().any(|val| val == target)
    }
}
```

Duyệt qua từng node, so sánh. Nếu tìm thấy thì dừng sớm.

So sánh: array sorted có binary search O(log n). Linked list thì **không thể** binary search vì không có random access -- không thể "nhảy" tới giữa list, phải đi bộ từ đầu.

### Iterator và Lifetime

```rust
pub struct SinglyIter<'a, T> {
    current: Option<&'a SinglyNode<T>>,
}

impl<'a, T> Iterator for SinglyIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.val
        })
    }
}
```

Nếu bạn mới gặp `'a` lần đầu, đây là giải thích:

**Lifetime `'a` là gì?** Iterator **mượn** list để duyệt. `'a` là "cam kết" rằng iterator sẽ không tồn tại lâu hơn list. Nếu list bị xóa mà iterator vẫn còn, Rust bắt lỗi lúc compile.

```
  SinglyLinkedList (chủ)     SinglyIter (người mượn)
  ┌─────────────────┐        ┌───────────────────┐
  │ head → [1]→[2]→ │ ←───── │ current: &[1]     │
  │       [3]→None   │  'a    │ "tôi chỉ đọc,    │
  └─────────────────┘        │  không sở hữu"    │
                              └───────────────────┘
  List phải sống ít nhất
  bằng lifetime 'a
```

Tưởng tượng iterator như **ngón tay chỉ**: di chuyển từ node này sang node kia, nhìn giá trị nhưng không sở hữu gì cả. `'a` đảm bảo "ngón tay" không trỏ vào thứ đã bị xóa.

**`as_deref()` là gì?** Bước biến đổi từng bước:

```
node.next             : Option<Box<SinglyNode<T>>>   ← có Box, sở hữu node
node.next.as_deref()  : Option<&SinglyNode<T>>       ← bỏ Box, chỉ mượn reference

as_deref() làm: Some(Box<X>) → Some(&X)
                None          → None
```

Ta có `Option<Box<Node>>` -- Box **sở hữu** node trên heap. Ta chỉ muốn `Option<&Node>` -- **mượn** để đọc. `as_deref()` làm đúng việc đó: bỏ Box đi, giữ lại reference.

### Custom Drop

```rust
impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}
```

Vòng lặp `while let` lấy từng node ra rồi hủy. Chỉ dùng 1 stack frame -- an toàn cho list hàng triệu node.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ thêm | Pattern |
|----------|-----------|-------------|---------|
| `push_front` | O(1) | O(1) | Single step |
| `pop_front` | O(1) | O(1) | Single step |
| `push_back` | O(n) | O(1) | Single traversal |
| `reverse` | O(n) | O(1) | Three pointers |
| `len` / `is_empty` | O(1) | O(1) | Stored value |
| `iter` (duyệt hết) | O(n) | O(1) | Single traversal |
| `nth` | O(n) | O(1) | Single traversal |
| `find` | O(n) | O(1) | Single traversal |
| `find_middle` | O(n) | O(1) | Two pointers (slow/fast) |
| `has_cycle` | O(n) | O(1) | Two pointers (slow/fast) |
| `merge_sorted` | O(n+m) | O(1) | Two pointers (compare) |
| **Tổng bộ nhớ** | -- | **O(n)** | -- |

**Hiểu đơn giản:**
- O(1) = làm ngay, không cần đi đâu cả. Giống lấy toa đầu tiên.
- O(n) = phải đi qua n toa. List có 1000 node thì đi qua 1000 node.

## Luyện nhận diện Pattern

5 bài toán. Đọc đề, suy nghĩ approach trước, rồi mở đáp án.

---

**Bài 1:** Cho linked list, tìm node thứ k **từ cuối** (không biết trước list dài bao nhiêu). Ví dụ: list `[1→2→3→4→5]`, k=2 → trả về node `[4]`.

*Gợi ý: dùng hai con trỏ cách nhau k bước.*

<details>
<summary>Đáp án</summary>

**Approach:** Two pointers cách nhau k bước.

**Tại sao:** Cho pointer A chạy trước k bước. Rồi cho cả A và B cùng chạy. Khi A tới cuối, B đang ở vị trí cách cuối đúng k bước. Giống như hai người đi bộ cách nhau k mét -- khi người đầu tới đích, người sau cách đích k mét.

**Độ phức tạp:** Time O(n), Space O(1)

```rust
fn kth_from_end(list: &SinglyLinkedList<T>, k: usize) -> Option<&T> {
    let mut fast = list.head.as_deref()?;
    // fast chạy trước k bước
    for _ in 0..k {
        fast = fast.next.as_deref()?;
    }
    let mut slow = list.head.as_deref()?;
    // cả hai cùng chạy cho tới khi fast hết đường
    while let Some(next) = fast.next.as_deref() {
        fast = next;
        slow = slow.next.as_deref().unwrap();
    }
    Some(&slow.val)
}
```
</details>

---

**Bài 2:** Kiểm tra linked list có phải **palindrome** không (đọc xuôi ngược giống nhau). Ví dụ: `[1→2→3→2→1]` → true.

*Gợi ý: tìm giữa, reverse nửa sau, so sánh.*

<details>
<summary>Đáp án</summary>

**Approach:** Tìm giữa (slow/fast) + reverse nửa sau + so sánh từng cặp.

**Tại sao:** Nếu nửa đầu giống nửa sau (đảo ngược), thì list là palindrome. Dùng slow/fast tìm giữa trong O(n), reverse nửa sau trong O(n), so sánh trong O(n). Tổng vẫn O(n).

**Độ phức tạp:** Time O(n), Space O(1)

```rust
fn is_palindrome(list: &mut SinglyLinkedList<T>) -> bool {
    // Bước 1: Tìm giữa bằng slow/fast
    // Bước 2: Reverse nửa sau
    // Bước 3: So sánh nửa đầu với nửa sau
    // Bước 4: (Optional) Reverse lại để khôi phục list
    todo!()
}
```
</details>

---

**Bài 3:** Xóa node thứ n **từ cuối** trong **1 lượt duyệt**. Ví dụ: list `[1→2→3→4→5]`, n=2 → xóa `[4]` → `[1→2→3→5]`.

*Gợi ý: giống bài 1, nhưng cần giữ node TRƯỚC node cần xóa.*

<details>
<summary>Đáp án</summary>

**Approach:** Two pointers cách nhau n bước. Khi fast tới cuối, slow ở ngay **trước** node cần xóa.

**Tại sao:** Giống bài tìm node thứ k từ cuối, nhưng ta cần pointer tới node **trước** nó để nối lại. Cho fast chạy trước n+1 bước (thay vì n), thì khi fast hết đường, slow ở đúng vị trí để `slow.next = slow.next.next`.

**Độ phức tạp:** Time O(n), Space O(1)

```rust
fn remove_nth_from_end(list: &mut SinglyLinkedList<T>, n: usize) {
    // Cho fast chạy trước n+1 bước
    // Cả hai cùng chạy
    // Khi fast hết → slow.next = slow.next.next
    todo!()
}
```
</details>

---

**Bài 4:** Kiểm tra 2 linked list có **giao nhau** không (chia sẻ phần đuôi). Ví dụ:

```
List A: [1] → [2] ↘
                     [6] → [7] → None
List B:       [3] ↗
```

*Gợi ý: đếm độ dài, cắt cho bằng nhau, rồi so sánh.*

<details>
<summary>Đáp án</summary>

**Approach:** Đếm chiều dài cả hai list. Cho list dài hơn chạy trước |lenA - lenB| bước. Rồi cả hai cùng chạy, so sánh pointer cho tới khi gặp nhau.

**Tại sao:** Nếu hai list giao nhau, phần đuôi chung dài bằng nhau. Chỉ phần đầu khác nhau. Cắt phần thừa ở list dài hơn → hai list còn lại dài bằng nhau → so sánh từng cặp node.

**Độ phức tạp:** Time O(n+m), Space O(1)

```rust
fn find_intersection(list_a: &List, list_b: &List) -> Option<&Node> {
    // Bước 1: Đếm len_a, len_b
    // Bước 2: Cho list dài hơn chạy trước |len_a - len_b| bước
    // Bước 3: Cả hai cùng chạy, so sánh pointer
    todo!()
}
```
</details>

---

**Bài 5:** Đảo ngược linked list **từng nhóm k node**. Ví dụ: list `[1→2→3→4→5]`, k=2 → `[2→1→4→3→5]`.

*Gợi ý: reverse k node, nối lại, lặp lại.*

<details>
<summary>Đáp án</summary>

**Approach:** Chia list thành nhóm k node. Reverse từng nhóm. Nối các nhóm lại. Nhóm cuối nếu ít hơn k node thì giữ nguyên.

**Tại sao:** Bài này kết hợp reverse (đã biết) với kỹ thuật chia nhóm. Key insight: sau khi reverse nhóm đầu, node đầu tiên (giờ là cuối nhóm) cần trỏ tới kết quả reverse của nhóm tiếp theo → có thể dùng recursion.

**Độ phức tạp:** Time O(n), Space O(n/k) nếu dùng recursion, O(1) nếu iterative

```rust
fn reverse_k_group(head: Option<Box<Node>>, k: usize) -> Option<Box<Node>> {
    // Bước 1: Đếm xem có đủ k node không
    // Bước 2: Reverse k node đầu
    // Bước 3: Đệ quy reverse phần còn lại
    // Bước 4: Nối lại
    todo!()
}
```
</details>

## Ví dụ

```rust
use rust_ds2a::linked_list::SinglyLinkedList;

let mut list = SinglyLinkedList::new();

// Thêm phần tử
list.push_back(1);
list.push_back(2);
list.push_back(3);
list.push_front(0);
// list: 0 -> 1 -> 2 -> 3

// Duyệt list
for val in list.iter() {
    println!("{val}");
}
// In ra: 0, 1, 2, 3

// Lấy phần tử đầu
assert_eq!(list.pop_front(), Some(0));
// list: 1 -> 2 -> 3

// Tìm phần tử
assert!(list.find(&2));
assert!(!list.find(&99));

// Lấy phần tử thứ n
assert_eq!(list.nth(0), Some(&1));
assert_eq!(list.nth(2), Some(&3));

// Tìm phần tử giữa
assert_eq!(list.find_middle(), Some(&2));

// Đảo ngược
list.reverse();
// list: 3 -> 2 -> 1

assert_eq!(list.len(), 3);
assert!(!list.is_empty());
```

### Merge hai list đã sắp xếp

```rust
use rust_ds2a::linked_list::{SinglyLinkedList, merge_sorted_lists};

let mut odds = SinglyLinkedList::new();
for &v in &[1, 3, 5] { odds.push_back(v); }

let mut evens = SinglyLinkedList::new();
for &v in &[2, 4, 6] { evens.push_back(v); }

let merged = merge_sorted_lists(odds, evens);
// merged: 1 -> 2 -> 3 -> 4 -> 5 -> 6
```

### Khi nào dùng singly linked list?

- Khi bạn thêm/xóa ở **đầu** danh sách rất nhiều.
- Khi không cần truy cập ngẫu nhiên (không cần nhảy tới phần tử thứ k).
- Khi muốn hiểu ownership trong Rust (bài tập rất tốt!).

### Khi nào KHÔNG nên dùng?

- Khi cần truy cập nhanh theo vị trí -- dùng array/Vec.
- Khi cần thêm/xóa ở cuối nhanh -- dùng doubly linked list hoặc Vec.
- Khi performance quan trọng hơn Big-O lý thuyết -- Vec thường thắng nhờ cache locality.

## Những cái bẫy hay gặp

---

❌ **Dùng `push_back` trong vòng lặp**

```rust
// ❌ Tạo list bằng push_back -- O(n) mỗi lần → tổng O(n²)!
let mut list = SinglyLinkedList::new();
for i in 0..10000 {
    list.push_back(i);  // Mỗi lần phải đi bộ từ đầu tới cuối
}
```

```rust
// ✅ push_front rồi reverse -- O(1) mỗi lần + O(n) reverse = O(n) tổng
let mut list = SinglyLinkedList::new();
for i in (0..10000).rev() {
    list.push_front(i);
}
// Hoặc: push_front tất cả rồi reverse()
```

💡 **Tại sao:** Mỗi `push_back` phải đi bộ từ đầu tới cuối. 10,000 node → đi bộ trung bình 5,000 bước mỗi lần. Tổng cộng ~50 triệu bước. `push_front` thì luôn chỉ 1 bước.

---

❌ **Không hiểu tại sao không thể có 2 `&mut` cùng lúc**

```rust
// ❌ Rust sẽ KHÔNG cho phép điều này compile
fn swap_adjacent(list: &mut SinglyLinkedList<i32>) {
    let a = &mut list[0];  // mutable borrow thứ 1
    let b = &mut list[1];  // mutable borrow thứ 2 -- LỖI!
    std::mem::swap(a, b);
}
```

```rust
// ✅ Dùng .take() để tạm "rút" node ra, xử lý xong rồi gắn lại
// Rust đảm bảo tại mỗi thời điểm chỉ có 1 người sở hữu/chỉnh sửa
```

💡 **Tại sao:** Rust safety guarantee: nếu 2 nơi cùng sửa 1 dữ liệu, sẽ có data race. Rust bắt lỗi này lúc compile. Với linked list, pattern phổ biến là dùng `.take()` để tạm lấy node ra khỏi list, xử lý xong rồi gắn lại.

---

❌ **Dùng linked list khi không cần**

```rust
// ❌ "Tôi cần danh sách, dùng linked list đi!"
let mut tasks = SinglyLinkedList::new();
tasks.push_back("task 1");
tasks.push_back("task 2");
// ... rồi chỉ duyệt qua, không bao giờ thêm/xóa ở đầu
```

```rust
// ✅ Vec đơn giản hơn, nhanh hơn trong hầu hết trường hợp
let mut tasks = vec!["task 1", "task 2"];
```

💡 **Tại sao:** Trong thực tế, `Vec` thường **nhanh hơn** linked list ngay cả khi Big-O nói linked list tốt hơn. Lý do: **cache locality** -- các phần tử Vec nằm liền nhau trong bộ nhớ, CPU đọc nhanh hơn nhiều so với nhảy lung tung trên heap. Chỉ dùng linked list khi thêm/xóa ở đầu là thao tác chính.

---

❌ **Nhầm ownership khi move node**

```rust
// ❌ Không thể "copy" node như Java/Python
let node = Box::new(SinglyNode { val: 42, next: None });
let copy = node;  // Đây KHÔNG phải copy -- đây là MOVE!
// println!("{:?}", node);  // LỖI! node đã bị move rồi
```

```rust
// ✅ Trong Rust, mỗi node chỉ có 1 chủ. Muốn chuyển thì phải "move".
// Sau khi move, biến cũ không dùng được nữa. Đây là điểm khác biệt lớn
// nhất so với Java/Python/C++ (nơi bạn thoải mái copy reference).
```

💡 **Tại sao:** Trong Java/Python, nhiều biến có thể trỏ tới cùng 1 object. Trong Rust, mỗi `Box<Node>` chỉ có đúng 1 chủ. Muốn chia sẻ thì phải dùng `Rc` (reference counting) -- như ở doubly linked list.

---

[← Strings](../01-fundamentals/03-strings.md) | [Doubly Linked List →](./02-doubly-linked-list.md)
