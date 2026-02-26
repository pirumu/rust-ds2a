# Singly Linked List

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
    let new_node = Box::new(SinglyNode {
        val,
        next: None,
    });
    // Walk to the last node
    match self.head.as_mut() {
        None => self.head = Some(new_node),
        Some(mut current) => {
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_node);
        }
    }
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

### Iterator

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

`as_deref()` biến `&Option<Box<Node>>` thành `Option<&Node>`. Nhờ vậy ta duyệt list mà không cần lấy ownership.

### Custom Drop

Mặc định, Rust xóa linked list bằng đệ quy. List dài 1 triệu node? Stack overflow! Nên ta viết `Drop` thủ công, xóa từng node bằng vòng lặp:

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

## Dộ phức tạp

| Thao tác | Thời gian | Bộ nhớ thêm |
|----------|-----------|-------------|
| `push_front` | O(1) | O(1) |
| `pop_front` | O(1) | O(1) |
| `push_back` | O(n) | O(1) |
| `reverse` | O(n) | O(1) |
| `len` / `is_empty` | O(1) | O(1) |
| `iter` (duyệt hết) | O(n) | O(1) |
| Truy cập theo vị trí | O(n) | O(1) |
| **Tổng bộ nhớ** | -- | **O(n)** |

**Hiểu đơn giản:**
- O(1) = làm ngay, không cần đi đâu cả. Giống lấy toa đầu tiên.
- O(n) = phải đi qua n toa. List có 1000 node thì đi qua 1000 node.

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

// Đảo ngược
list.reverse();
// list: 3 -> 2 -> 1

assert_eq!(list.len(), 3);
assert!(!list.is_empty());
```

### Khi nào dùng singly linked list?

- Khi bạn thêm/xóa ở **đầu** danh sách rất nhiều.
- Khi không cần truy cập ngẫu nhiên (không cần nhảy tới phần tử thứ k).
- Khi muốn hiểu ownership trong Rust (bài tập rất tốt!).

### Khi nào KHÔNG nên dùng?

- Khi cần truy cập nhanh theo vị trí -- dùng array/Vec.
- Khi cần thêm/xóa ở cuối nhanh -- dùng doubly linked list hoặc Vec.
