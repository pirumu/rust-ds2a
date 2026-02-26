# Doubly Linked List

## Đây là gì?

Ở chương trước, đoàn tàu chỉ có móc nối **một chiều** -- mỗi toa chỉ biết toa sau. Bây giờ hãy tưởng tượng **đoàn tàu 2 chiều**: mỗi toa có **2 móc nối** -- một nối tới toa **trước**, một nối tới toa **sau**.

```
         đầu tàu (head)                    cuối tàu (tail)
            |                                    |
            v                                    v
 None <-- [Toa A] <--> [Toa B] <--> [Toa C] --> None
```

Đó là **doubly linked list** (danh sách liên kết đôi).

Mỗi node giờ có 3 thứ:
1. **val** -- giá trị
2. **prev** -- con trỏ tới node trước (toa phía trước)
3. **next** -- con trỏ tới node sau (toa phía sau)

List giữ 2 con trỏ: **head** (đầu) và **tail** (cuối).

**Lợi ích so với singly linked list:**
- Thêm/xóa ở **cả 2 đầu** đều là O(1). Singly linked list chỉ nhanh ở đầu, còn ở cuối phải đi bộ O(n).
- Có thể đi **ngược** từ cuối về đầu.

**Cái giá:** Mỗi node tốn thêm bộ nhớ cho con trỏ `prev`. Và code phức tạp hơn -- mỗi lần thêm/xóa phải cập nhật pointer **cả 2 hướng**.

### Thử thách ownership trong Rust

Đây là phần thú vị. Với singly linked list, mỗi node chỉ có **1 chủ** (node trước nó). Hợp với Rust -- mỗi giá trị chỉ có 1 owner.

Nhưng doubly linked list thì sao? Nhìn node B:

```
  [A] --next--> [B] <--prev-- [C]
```

Node B bị trỏ vào bởi **2 node**: A (qua next) và C (qua prev). Nếu dùng `Box`, nghĩa là cả A và C đều **sở hữu** B. Rust không cho phép! `Box` = 1 chủ duy nhất.

```
// KHÔNG ĐƯỢC! A.next và C.prev cùng sở hữu B
//
// [A] --Box--> [B] <--Box-- [C]
//       ^                ^
//       |                |
//     "Tôi sở hữu B"   "Tôi cũng sở hữu B"
//
// Rust: "KHÔNG! Chỉ được 1 chủ!"
```

### Giải pháp: `Rc<RefCell<>>`

Rust có 2 công cụ giải quyết vấn đề này:

**`Rc<T>`** (Reference Counting) -- cho phép **nhiều chủ sở hữu**. Mỗi lần `clone()`, bộ đếm tăng lên 1. Khi bộ đếm về 0, dữ liệu bị xóa. Giống như một cuốn sách thư viện: nhiều người có thể mượn cùng lúc, khi không ai mượn nữa thì trả lại kệ.

**`RefCell<T>`** -- cho phép **thay đổi dữ liệu** dù đang bị chia sẻ. Bình thường Rust không cho sửa dữ liệu đang được borrow. `RefCell` chuyển việc kiểm tra từ lúc compile sang lúc **chạy chương trình** (runtime).

Kết hợp lại: **`Rc<RefCell<Node>>`** = nhiều chủ + có thể sửa = đúng thứ ta cần cho doubly linked list.

```
  [A] --Rc--> [B] <--Rc-- [C]
              (RefCell bên trong cho phép sửa)
```

Cái giá: chậm hơn `Box` một chút vì phải đếm reference và kiểm tra borrow lúc runtime. Nhưng hoàn toàn **safe** -- không cần `unsafe` code.

## Hoạt động như thế nào?

### Cấu trúc node

```
         head                                  tail
          |                                     |
          v                                     v
  None <--+-------+     +-------+     +-------+
          | val: A|<--->| val: B|<--->| val: C|--> None
          +-------+     +-------+     +-------+
```

Head không có prev (None). Tail không có next (None).

### push_front -- Thêm vào đầu (O(1))

Thêm toa mới vào đầu đoàn tàu.

```
Trước:   head --> [A] <--> [B] <--> [C] <-- tail

Bước 1:  Tạo node [X], X.next = head cũ [A]
Bước 2:  Gán A.prev = [X]
Bước 3:  head = [X]

Sau:     head --> [X] <--> [A] <--> [B] <--> [C] <-- tail
```

Nếu list rỗng thì node mới vừa là head vừa là tail.

### push_back -- Thêm vào cuối (O(1))

Ngược lại với push_front. Thêm vào cuối, cập nhật tail.

```
Trước:   head --> [A] <--> [B] <--> [C] <-- tail

Bước 1:  Tạo node [Y], Y.prev = tail cũ [C]
Bước 2:  Gán C.next = [Y]
Bước 3:  tail = [Y]

Sau:     head --> [A] <--> [B] <--> [C] <--> [Y] <-- tail
```

Ở singly linked list, push_back là O(n) vì phải đi bộ tới cuối. Ở đây ta có tail pointer, nên O(1)!

### pop_front -- Xóa ở đầu (O(1))

```
Trước:   head --> [A] <--> [B] <--> [C] <-- tail

Bước 1:  Lấy [A] ra
Bước 2:  B.prev = None
Bước 3:  head = [B]

Sau:     head --> [B] <--> [C] <-- tail    (trả về A)
```

### pop_back -- Xóa ở cuối (O(1))

```
Trước:   head --> [A] <--> [B] <--> [C] <-- tail

Bước 1:  Lấy [C] ra
Bước 2:  B.next = None
Bước 3:  tail = [B]

Sau:     head --> [A] <--> [B] <-- tail    (trả về C)
```

Singly linked list không thể pop_back nhanh được vì không biết node trước tail là gì. Doubly linked list thì biết nhờ có prev pointer!

## Code Rust

Code đầy đủ nằm trong `src/linked_list.rs`.

### Type alias và struct

```rust
use std::cell::RefCell;
use std::rc::Rc;

// Định nghĩa kiểu viết tắt cho gọn
type Link<T> = Option<Rc<RefCell<DoublyNode<T>>>>;

struct DoublyNode<T> {
    val: T,
    prev: Link<T>,  // con trỏ tới node trước
    next: Link<T>,  // con trỏ tới node sau
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}
```

`Link<T>` là viết tắt. Ý nghĩa: "có thể có node (`Some(Rc<RefCell<Node>>)`), hoặc không có gì (`None`)".

### push_front

```rust
pub fn push_front(&mut self, val: T) {
    // Create new node, next = old head
    let new_node = Rc::new(RefCell::new(DoublyNode {
        val,
        prev: None,
        next: self.head.clone(),
    }));
    match self.head.take() {
        Some(old_head) => {
            // Old head's prev = new node
            old_head.borrow_mut().prev = Some(new_node.clone());
            self.head = Some(new_node);
        }
        None => {
            // Empty list: new node is both head and tail
            self.tail = Some(new_node.clone());
            self.head = Some(new_node);
        }
    }
    self.len += 1;
}
```

`clone()` trên `Rc` rất rẻ -- chỉ tăng bộ đếm reference lên 1, **không copy dữ liệu**.

`borrow_mut()` -- "cho tôi mượn node này để sửa". Nếu ai đó đang mượn rồi, chương trình panic lúc runtime.

### pop_front

```rust
pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
        match old_head.borrow_mut().next.take() {
            Some(new_head) => {
                // New head has no prev
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            }
            None => {
                // List is now empty
                self.tail = None;
            }
        }
        self.len -= 1;
        // Unwrap the Rc -> RefCell -> Node -> val
        Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
    })
}
```

`Rc::try_unwrap` -- "lấy dữ liệu ra khỏi Rc, nhưng chỉ khi bộ đếm = 1". Ta đã xóa hết reference khác rồi, nên luôn thành công.

Chuỗi gọi: `Rc::try_unwrap` -> lấy `RefCell` -> `.into_inner()` -> lấy `Node` -> `.val` -> lấy giá trị. Hơi dài, nhưng mỗi bước đều có lý do.

### Custom Drop

`Rc` có thể tạo **vòng tròn** tham chiếu: A trỏ B, B trỏ lại A. Bộ đếm không bao giờ về 0, memory leak! Nên ta phải tự xóa:

```rust
impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        // Pop từng node ra cho tới khi hết
        while self.pop_front().is_some() {}
    }
}
```

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ thêm |
|----------|-----------|-------------|
| `push_front` | O(1) | O(1) |
| `push_back` | O(1) | O(1) |
| `pop_front` | O(1) | O(1) |
| `pop_back` | O(1) | O(1) |
| `len` / `is_empty` | O(1) | O(1) |
| Truy cập theo vị trí | O(n) | O(1) |
| **Tổng bộ nhớ** | -- | **O(n)** |

**So sánh với singly linked list:**

| Thao tác | Singly | Doubly |
|----------|--------|--------|
| push_front | O(1) | O(1) |
| push_back | **O(n)** | **O(1)** |
| pop_front | O(1) | O(1) |
| pop_back | **O(n)** | **O(1)** |

Doubly nhanh hơn ở cuối nhờ có tail pointer và prev pointer. Cái giá: mỗi node tốn thêm bộ nhớ, code phức tạp hơn.

## Ví dụ

```rust
use rust_ds2a::linked_list::DoublyLinkedList;

let mut list = DoublyLinkedList::new();

// Thêm phần tử
list.push_back(1);
list.push_back(2);
list.push_back(3);
list.push_front(0);
// list: 0 <-> 1 <-> 2 <-> 3

// Xóa ở đầu
assert_eq!(list.pop_front(), Some(0));
// list: 1 <-> 2 <-> 3

// Xóa ở cuối
assert_eq!(list.pop_back(), Some(3));
// list: 1 <-> 2

assert_eq!(list.len(), 2);
assert!(!list.is_empty());
```

### Khi nào dùng doubly linked list?

- Khi cần thêm/xóa nhanh ở **cả 2 đầu**.
- Khi cần duyệt **ngược** từ cuối về đầu.
- Khi xây dựng cấu trúc phức tạp hơn (LRU cache, text editor buffer...).

### Tóm tắt Rc<RefCell<>> cho ai hay quên

| Vấn đề | Giải pháp |
|--------|-----------|
| Nhiều node trỏ vào 1 node | `Rc` (shared ownership) |
| Cần sửa node đang bị chia sẻ | `RefCell` (interior mutability) |
| Kết hợp | `Rc<RefCell<Node>>` |
| Clone Rc | Chỉ tăng bộ đếm, rẻ |
| Nguy hiểm | Vòng tròn tham chiếu -> memory leak |
| Cách tránh | Tự viết `Drop` để phá vòng tròn |
