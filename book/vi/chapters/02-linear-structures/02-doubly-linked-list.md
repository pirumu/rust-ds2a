# Doubly Linked List

> 💡 **Đừng lo lắng:** Nếu bạn đã sống sót qua singly linked list ở chương trước thì chương này chỉ thêm 1 con trỏ `prev` nữa thôi. Ý tưởng y hệt, chỉ là toa tàu giờ có móc nối 2 chiều. `Rc<RefCell<>>` nhìn lạ nhưng đọc chậm là hiểu.

## Đây là gì?

> Chương này có `Rc<RefCell<>>` -- combo mà hầu hết người học Rust đều thấy đáng sợ lần đầu. Nhưng thực ra nó chỉ là **2 lớp wrapper**: `Rc` = "nhiều người cùng sở hữu", `RefCell` = "mượn lúc runtime thay vì compile time". Nếu bạn hiểu singly linked list ở chương trước, logic ở đây **y hệt** -- chỉ thêm 1 con trỏ `prev` nữa thôi. Đọc chậm, trace từng dòng code, bạn sẽ thấy nó không khó như nó trông.

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

Đây là phần thú vị. Với singly linked list, mỗi node chỉ có **1 chủ** (node trước nó). Hợp với Rust -- mỗi giá trị chỉ có 1 owner. Ở chương trước ta dùng `Box` vì mỗi node chỉ bị sở hữu bởi 1 node khác -- hoàn hảo cho ownership model của Rust.

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

Bây giờ cần `Rc` vì mỗi node bị trỏ bởi **nhiều** node khác -- shared ownership là bắt buộc.

## Rc\<RefCell\<>> -- Cái giá của 2 chiều

Rust có 2 công cụ giải quyết vấn đề ownership:

**`Rc<T>`** (Reference Counting) -- cho phép **nhiều chủ sở hữu**. Mỗi lần `clone()`, bộ đếm tăng lên 1. Khi bộ đếm về 0, dữ liệu bị xóa. Giống như một cuốn sách thư viện: nhiều người có thể mượn cùng lúc, khi không ai mượn nữa thì trả lại kệ.

**`RefCell<T>`** -- cho phép **thay đổi dữ liệu** dù đang bị chia sẻ. Bình thường Rust không cho sửa dữ liệu đang được borrow. `RefCell` chuyển việc kiểm tra từ lúc compile sang lúc **chạy chương trình** (runtime).

Kết hợp lại: **`Rc<RefCell<Node>>`** = nhiều chủ + có thể sửa = đúng thứ ta cần cho doubly linked list.

```
  [A] --Rc--> [B] <--Rc-- [C]
              (RefCell bên trong cho phép sửa)
```

Cái giá: chậm hơn `Box` một chút vì phải đếm reference và kiểm tra borrow lúc runtime. Nhưng hoàn toàn **safe** -- không cần `unsafe` code.

### Vòng tròn tham chiếu -- Memory Leak

Đây là cạm bẫy nguy hiểm nhất của `Rc`. Khi 2 node trỏ vào nhau, bộ đếm reference **không bao giờ về 0**:

```
                Rc count: 2         Rc count: 2
                +---------+         +---------+
                | Node A  |--next-->| Node B  |
                |         |<--prev--|         |
                +---------+         +---------+
```

**Tại sao count = 2?** Mỗi node được trỏ bởi 2 nguồn:
- Node A: list.head trỏ (1) + B.prev trỏ (1) = count 2
- Node B: list.tail trỏ (1) + A.next trỏ (1) = count 2

**Hãy theo dõi từng bước khi ta xây list `[A, B]`:**

```
Bước 1: push_back(A)
  head ──Rc──> [A] <──Rc── tail
  A count: 2 (head + tail cùng trỏ)

Bước 2: push_back(B)
  head ──Rc──> [A] ──next(Rc)──> [B] <──Rc── tail
                ^                  |
                └──prev(Rc)────────┘

  A count: 2 (head + B.prev)
  B count: 2 (tail + A.next)
```

**Bây giờ, khi list bị drop (không có custom Drop):**

```
Bước 1: Drop list → head và tail bị xóa
  A count: 2 → 1  (mất head, còn B.prev)
  B count: 2 → 1  (mất tail, còn A.next)

Bước 2: Cả A và B đều có count = 1
  → Không ai về 0
  → Không ai bị xóa
  → MEMORY LEAK! 💀
```

```
  +---------+         +---------+
  | Node A  |--next-->| Node B  |
  |  (1)    |<--prev--|  (1)    |     Mãi mãi trôi nổi
  +---------+         +---------+     trong bộ nhớ...
       ↑                              không ai giải phóng.
       Không ai trỏ từ bên ngoài nữa!
```

**Custom Drop giải quyết thế nào?**

Thay vì để Rust tự drop, ta viết `Drop` thủ công: gọi `pop_front()` liên tục. Mỗi lần pop, ta **ngắt connection** (set prev = None, lấy next ra), reference count giảm dần về 0:

```
List: [A] <--> [B] <--> [C]

pop_front [A]:
  - Ngắt A.next (B mất 1 ref)
  - Ngắt B.prev (A mất 1 ref)
  - A count → 0 → DROP! ✓

pop_front [B]:
  - Ngắt B.next (C mất 1 ref)
  - Ngắt C.prev (B mất 1 ref)
  - B count → 0 → DROP! ✓

pop_front [C]:
  - C count → 0 → DROP! ✓
```

## Weak\<T> -- Giải pháp chuẩn cho Cycle

Custom Drop giải quyết vấn đề, nhưng có cách **tốt hơn**: `Weak<T>`.

**Ví dụ đời thường:**
- `Rc` là **sở hữu** -- bạn có cuốn sách, sách tồn tại chừng nào bạn còn giữ.
- `Weak` là **biết đường** -- bạn biết cuốn sách ở thư viện nào, nhưng không "giữ" nó. Nếu thư viện đóng cửa (sách bị xóa), bạn không thể đọc nữa.

`Weak` tăng `weak_count` nhưng **không tăng `strong_count`**. Khi `strong_count = 0`, data bị drop -- kể cả khi `weak_count > 0`.

### So sánh Rc vs Weak

| Thuộc tính | `Rc` | `Weak` |
|---|---|---|
| Ngăn drop? | Có -- data sống chừng nào còn Rc | Không -- data bị drop khi strong_count = 0 |
| Truy cập | Trực tiếp `.borrow()` | Phải `.upgrade()` trước, trả về `Option` |
| Dùng khi | Owner -- "tôi cần data này sống" | Back-reference -- "tôi chỉ nhìn lại" |
| Với cycle | Gây memory leak | An toàn -- không tăng strong_count |

### Refactor: dùng Weak cho prev

Ý tưởng: `next` là "đi tới phía trước" -- ownership thật sự (Rc). `prev` chỉ là "nhìn lại phía sau" -- không cần sở hữu (Weak).

```rust
use std::rc::Weak;

struct DoublyNode<T> {
    val: T,
    prev: Option<Weak<RefCell<DoublyNode<T>>>>,  // Weak! Không tăng count
    next: Option<Rc<RefCell<DoublyNode<T>>>>,     // Rc -- sở hữu thật
}
```

```
  head ──Rc──> [A] ──next(Rc)──> [B] ──next(Rc)──> [C]
                ^                  ^                  ^
                └──prev(Weak)──────┘──prev(Weak)──────┘

  A strong_count: 1 (chỉ head)
  B strong_count: 1 (chỉ A.next)
  C strong_count: 1 (chỉ B.next + tail)
```

Không có cycle! Khi drop list, head bị xóa → A strong_count về 0 → A bị drop → B strong_count về 0 → B bị drop → ... Tự động, không cần custom Drop.

### `.upgrade()` -- cách dùng Weak

```rust
// weak_ref là Weak<RefCell<DoublyNode<T>>>
match weak_ref.upgrade() {
    Some(rc) => {
        // Node vẫn còn sống! rc là Rc, dùng bình thường
        let val = rc.borrow().val;
    }
    None => {
        // Node đã bị drop rồi -- an toàn, không crash
    }
}
```

`upgrade()` trả về `Option<Rc<T>>`:
- `Some(rc)` nếu node vẫn còn tồn tại
- `None` nếu node đã bị drop

Đây là safety check tự động -- không bao giờ truy cập vào data đã bị giải phóng.

### So sánh 2 approach

| | Approach 1: Rc cả 2 chiều | Approach 2: Rc + Weak |
|---|---|---|
| prev pointer | `Rc` (strong) | `Weak` |
| Cycle? | Có -- phải custom Drop | Không -- safe tự động |
| Custom Drop? | **Bắt buộc** | Không cần |
| Code complexity | Đơn giản hơn | Phức tạp hơn (upgrade) |
| Dùng khi | Học DSA -- dễ hiểu | Production code -- an toàn |

**Kết luận:** Trong tài liệu này ta dùng Approach 1 (Rc cả 2 chiều) vì dễ hiểu hơn cho người mới. Nhưng khi viết production code, hãy dùng Weak cho back-pointer -- đó là best practice trong Rust.

## Những cái bẫy hay gặp với Rc\<RefCell\<>>

### Bẫy 1: `borrow_mut()` panic lúc runtime

❌ Gọi `borrow_mut()` khi đang có `borrow()` active:
```rust
let node = Rc::new(RefCell::new(DoublyNode { val: 1, prev: None, next: None }));
let borrowed = node.borrow();       // immutable borrow active
node.borrow_mut().val = 2;          // 💥 PANIC! Đang có borrow() rồi
```

✅ Đảm bảo borrow scope kết thúc trước khi `borrow_mut`:
```rust
{
    let borrowed = node.borrow();    // scope bắt đầu
    println!("{}", borrowed.val);
}                                    // scope kết thúc, borrow trả lại

node.borrow_mut().val = 2;          // ✓ OK, không ai đang borrow
```

💡 Đây là điểm yếu của `RefCell`: lỗi lúc **runtime**, không phải compile time. Quy tắc: dùng `borrow()` trong scope **nhỏ nhất** có thể.

### Bẫy 2: Clone Rc không phải clone data

❌ Nghĩ `.clone()` tạo node mới độc lập:
```rust
let node = Rc::new(RefCell::new(DoublyNode { val: 1, prev: None, next: None }));
let node2 = node.clone();     // node2 trỏ vào CÙNG node!

node2.borrow_mut().val = 999;
println!("{}", node.borrow().val);  // In ra 999! Cả 2 chung data
```

✅ Hiểu `clone()` chỉ tăng reference count, vẫn trỏ vào **cùng** node:
```
  node ───Rc───> [val: 1]     clone()     node ───Rc───> [val: 1] <───Rc─── node2
                                           count: 1  →  count: 2
```

💡 Nếu muốn data mới thực sự, phải tạo node mới hoàn toàn. `Rc::clone()` rẻ (chỉ tăng số đếm), nhưng **không** tạo bản sao data.

### Bẫy 3: Quên ngắt cycle trước khi drop

❌ Drop list mà không clear references → memory leak:
```rust
{
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    // Nếu không có custom Drop...
}
// Nodes vẫn tồn tại trong memory! Không ai giải phóng!
```

✅ Luôn implement custom `Drop` hoặc dùng `Weak` cho back-pointer:
```rust
impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}  // Ngắt từng connection
    }
}
```

💡 Cách kiểm tra: dùng `Rc::strong_count()` để xem reference count. Nếu count không về 0 sau khi drop, có leak.

### Bẫy 4: Dùng Rc\<RefCell\<>> khi không cần

❌ Dùng `Rc<RefCell<>>` cho mọi thứ vì "an toàn":
```rust
// ĐỪNG! Singly linked list không cần Rc<RefCell<>>
struct BadSinglyNode<T> {
    val: T,
    next: Option<Rc<RefCell<BadSinglyNode<T>>>>,  // Quá mức cần thiết!
}
```

✅ Chỉ dùng khi **thực sự** cần shared mutable ownership:
```rust
// Singly: mỗi node chỉ có 1 chủ → Box là đủ
struct GoodSinglyNode<T> {
    val: T,
    next: Option<Box<GoodSinglyNode<T>>>,  // Đơn giản, nhanh
}
```

💡 `Rc<RefCell<>>` chậm hơn `Box` vì runtime check + reference counting. Với single ownership, `Box` **luôn** là lựa chọn tốt hơn.

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

#### Bóc vỏ hành: `Rc::try_unwrap(old_head).ok().unwrap().into_inner().val`

Dòng này trông đáng sợ, nhưng nó chỉ là **bóc từng lớp wrapper ra** để lấy giá trị bên trong:

```
  Rc<RefCell<DoublyNode<T>>>          ← Lớp ngoài cùng
   │
   └─ Rc::try_unwrap(old_head)       → Result<RefCell<DoublyNode<T>>, _>
       │
       └─ .ok()                      → Option<RefCell<DoublyNode<T>>>
           │
           └─ .unwrap()              → RefCell<DoublyNode<T>>
               │
               └─ .into_inner()      → DoublyNode<T>
                   │
                   └─ .val           → T  ← Giá trị ta cần!
```

**Tại sao `try_unwrap` thay vì unwrap trực tiếp?**

`Rc` có thể có nhiều owner. `try_unwrap` chỉ thành công khi **strong_count = 1** (chỉ còn mình ta giữ). Nếu còn reference khác, nó trả về `Err` -- đây là safety net.

Trước khi gọi `try_unwrap`, ta đã:
1. Ngắt `next` pointer của old_head (bước `next.take()`)
2. Set `prev = None` ở new_head (ngắt back-reference)

Nên lúc này old_head chỉ còn **1 reference** duy nhất -- `try_unwrap` chắc chắn thành công.

**Tại sao `.ok().unwrap()` -- 2 bước thay vì 1?**

- `try_unwrap` trả về `Result<T, Rc<T>>`
- `.ok()` chuyển `Result` → `Option` (bỏ error info)
- `.unwrap()` lấy giá trị ra từ `Option`

Pattern `.ok().unwrap()` hay dùng khi "ta chắc chắn sẽ thành công" -- gọn hơn match.

### pop_back

Đây là lợi thế lớn nhất của doubly linked list so với singly! Xóa ở cuối chỉ O(1).

```
Trước: head --> [A] <--> [B] <--> [C] <-- tail

Bước 1:  Lấy tail [C] ra (tail.take())
Bước 2:  Lấy prev của C = [B] (old_tail.prev.take())
Bước 3:  Ngắt B.next = None (không trỏ tới C nữa)
Bước 4:  tail = [B]
Bước 5:  Bóc vỏ Rc → RefCell → Node → val

Sau:   head --> [A] <--> [B] <-- tail    (trả về C)
```

```rust
pub fn pop_back(&mut self) -> Option<T> {
    self.tail.take().map(|old_tail| {
        match old_tail.borrow_mut().prev.take() {
            Some(new_tail) => {
                // New tail has no next
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            }
            None => {
                // List is now empty
                self.head = None;
            }
        }
        self.len -= 1;
        Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
    })
}
```

**So sánh với pop_front:** Logic gần như giống nhau, chỉ đổi chiều -- `tail` thay `head`, `prev` thay `next`. Đây là sự đối xứng đẹp đẽ của doubly linked list.

**Tại sao singly linked list không làm được?** Vì không có `prev` pointer. Muốn biết node trước tail là gì, phải đi bộ từ head -- O(n). Doubly linked list biết ngay nhờ `prev` -- O(1).

### Iterator -- Duyệt danh sách

Ở chương singly linked list, ta đã có `iter()` để duyệt list. Doubly linked list cũng cần, và còn có thêm `iter_back()` -- duyệt ngược!

#### iter() -- Duyệt từ đầu đến cuối

```rust
pub struct DoublyIter<T> {
    current: Link<T>,  // Option<Rc<RefCell<DoublyNode<T>>>>
}

impl<T: Clone> Iterator for DoublyIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let borrowed = node.borrow();
            self.current = borrowed.next.clone();
            borrowed.val.clone()
        })
    }
}
```

**Khác gì với iterator của singly linked list?**

Singly dùng `&'a T` (reference) -- vì `Box` cho phép borrow trực tiếp. Doubly dùng `T` (clone giá trị) -- vì `RefCell` không cho phép giữ reference ra ngoài scope `borrow()`. Ta phải clone giá trị ra.

```
Iterator đi từ đầu → cuối:
  head → [A] → [B] → [C] → None
          ↑
        current bắt đầu ở đây, rồi nhảy sang next
```

#### iter_back() -- Duyệt ngược từ cuối về đầu

Đây là **lợi thế độc quyền** của doubly linked list!

```rust
pub struct DoublyIterBack<T> {
    current: Link<T>,
}

impl<T: Clone> Iterator for DoublyIterBack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let borrowed = node.borrow();
            self.current = borrowed.prev.clone();
            borrowed.val.clone()
        })
    }
}
```

```
Iterator đi từ cuối → đầu:
  None ← [A] ← [B] ← [C] ← tail
                               ↑
                        current bắt đầu ở đây, nhảy sang prev
```

**Ví dụ sử dụng:**

```rust
let mut list = DoublyLinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_back(3);

// Duyệt xuôi: 1, 2, 3
for val in list.iter() {
    print!("{val} ");
}

// Duyệt ngược: 3, 2, 1
for val in list.iter_back() {
    print!("{val} ");
}
```

**Use case thực tế:** Duyệt lịch sử browser. `iter()` = đi từ trang đầu tiên đến trang hiện tại. `iter_back()` = nhấn nút "Back" liên tục.

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

## LRU Cache -- Use Case thực tế

"LRU Cache" được nhắc đến ở phần "khi nào dùng", nhưng nó quan trọng đến mức cần giải thích kỹ hơn.

### LRU Cache là gì?

Bộ nhớ máy tính có giới hạn. Khi bạn mở nhiều tab Chrome, máy không thể giữ hết nội dung tất cả các trang trong RAM. Giải pháp: cache các trang **vừa xem gần đây**. Khi bộ nhớ đầy, xóa trang **ít dùng nhất gần đây** (Least Recently Used).

Ví dụ: cache chứa tối đa 3 trang.

```
Trạng thái ban đầu (cache trống):
  []

Truy cập A: [A]
Truy cập B: [B, A]         ← B mới nhất, ở đầu
Truy cập C: [C, B, A]      ← đầy rồi!
Truy cập D: [D, C, B]      ← A bị đuổi (ít dùng nhất)
Truy cập B: [B, D, C]      ← B được dùng lại, nhảy lên đầu
```

### Cấu trúc: Doubly Linked List + HashMap

```
HashMap: key → node pointer          (O(1) tìm node)
  ┌─────────────────────┐
  │ "A" → ptr to node A │
  │ "B" → ptr to node B │
  │ "C" → ptr to node C │
  └─────────────────────┘

Doubly Linked List:                   (O(1) di chuyển node)
  [MRU] <--> [...] <--> [LRU]
   head                  tail
  (mới nhất)           (cũ nhất)
```

**Tại sao cần cả 2?**
- **HashMap** giải quyết: "tìm node nào đó" → O(1)
- **Doubly Linked List** giải quyết: "di chuyển node lên đầu" và "xóa node cuối" → O(1)

Nếu chỉ dùng list: tìm node là O(n). Nếu chỉ dùng HashMap: không biết thứ tự "mới/cũ".

### Pseudocode

```
get(key):
  1. Tìm key trong HashMap → node
  2. Nếu không có → return None
  3. Di chuyển node lên đầu list (vừa dùng = mới nhất)
  4. Return node.val

put(key, val):
  1. Nếu key đã có trong HashMap:
     - Cập nhật val
     - Di chuyển node lên đầu list
  2. Nếu chưa có:
     - Tạo node mới, thêm vào đầu list
     - Thêm vào HashMap
     - Nếu vượt capacity:
       - Xóa node cuối list (LRU)
       - Xóa key tương ứng khỏi HashMap
```

**Big-O:** Cả `get` và `put` đều **O(1)**. HashMap lookup O(1) + linked list move O(1) = O(1). Đây là lý do LRU Cache nổi tiếng -- và là bài phỏng vấn kinh điển.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ thêm |
|----------|-----------|-------------|
| `push_front` | O(1) | O(1) |
| `push_back` | O(1) | O(1) |
| `pop_front` | O(1) | O(1) |
| `pop_back` | O(1) | O(1) |
| `len` / `is_empty` | O(1) | O(1) |
| `iter` (duyệt hết) | O(n) | O(1) |
| `iter_back` (duyệt ngược) | O(n) | O(1) |
| Truy cập theo vị trí | O(n) | O(1) |
| **Tổng bộ nhớ** | -- | **O(n)** |

**So sánh với singly linked list:**

| Thao tác | Singly | Doubly |
|----------|--------|--------|
| push_front | O(1) | O(1) |
| push_back | **O(n)** | **O(1)** |
| pop_front | O(1) | O(1) |
| pop_back | **O(n)** | **O(1)** |
| Duyệt ngược | **Không thể** | **O(n)** |
| Bộ nhớ/node | 1 pointer | 2 pointers |

Doubly nhanh hơn ở cuối nhờ có tail pointer và prev pointer. Cái giá: mỗi node tốn thêm bộ nhớ, code phức tạp hơn.

**Lưu ý về overhead:** `Rc<RefCell<>>` chậm hơn `Box` vì:
- `Rc`: đếm reference mỗi lần clone/drop
- `RefCell`: kiểm tra borrow rules lúc runtime
- Mỗi node thêm 2 counter (strong_count + weak_count) = 16 bytes overhead

Với singly linked list dùng `Box`, overhead gần như bằng 0.

## Luyện nhận diện Pattern

**Bài 1:** Bạn đang xây dựng trình duyệt web. User nhấn "Back" để quay lại trang trước, nhấn "Forward" để đi tới trang sau. Khi vào trang mới, lịch sử "Forward" bị xóa. Dùng cấu trúc gì?

*Gợi ý:* Cần di chuyển cả 2 chiều từ trang hiện tại.

<details>
<summary>Đáp án</summary>

**Cấu trúc:** Doubly linked list với con trỏ `current` chỉ vào trang hiện tại.

**Tại sao:** Back = di chuyển `current` sang `prev`. Forward = di chuyển sang `next`. Khi vào trang mới, xóa tất cả node sau `current` rồi thêm trang mới. Doubly linked list cho phép di chuyển 2 chiều O(1).

**Complexity:** Time O(1) cho back/forward, O(k) cho "vào trang mới" (k = số trang forward bị xóa). Space O(n) với n là số trang trong lịch sử.
</details>

**Bài 2:** Bạn đang viết text editor. Cần hỗ trợ Undo (Ctrl+Z) và Redo (Ctrl+Y). Mỗi thao tác chỉnh sửa là một "action". Khi Undo rồi gõ gì đó mới, lịch sử Redo bị xóa. Dùng cấu trúc gì?

*Gợi ý:* Tương tự browser history nhưng với các editing action.

<details>
<summary>Đáp án</summary>

**Cấu trúc:** Doubly linked list chứa các editing action, con trỏ `current` chỉ action hiện tại.

**Tại sao:** Undo = di chuyển `current` sang `prev` và đảo ngược action. Redo = di chuyển sang `next` và áp dụng lại action. Khi thực hiện action mới, xóa tất cả node sau `current`. Pattern giống hệt browser history.

**Complexity:** Time O(1) cho undo/redo. Space O(n) với n là số action trong lịch sử.
</details>

**Bài 3:** Implement LRU Cache với capacity = k. Hỗ trợ `get(key)` và `put(key, value)`, cả 2 phải O(1). Dùng cấu trúc gì?

*Gợi ý:* Cần tìm nhanh theo key VÀ biết thứ tự "mới dùng → cũ nhất".

<details>
<summary>Đáp án</summary>

**Cấu trúc:** Doubly linked list + HashMap.

**Tại sao:** HashMap cho O(1) lookup theo key. Doubly linked list giữ thứ tự LRU -- head = mới nhất, tail = cũ nhất. Khi `get` hoặc `put`, di chuyển node lên head (O(1) nhờ doubly linked list). Khi đầy, xóa tail (O(1)). Singly linked list không thể xóa node ở giữa nhanh được vì không có prev pointer.

**Complexity:** Time O(1) cho cả get và put. Space O(k) với k là capacity.
</details>

**Bài 4:** Cho một doubly linked list nhiều tầng: mỗi node ngoài `prev` và `next`, còn có `child` trỏ xuống một doubly linked list con. Flatten toàn bộ thành 1 doubly linked list duy nhất. Giải thuật gì?

*Gợi ý:* Khi gặp node có child, "chen" child list vào giữa.

<details>
<summary>Đáp án</summary>

**Cấu trúc:** Duyệt doubly linked list, khi gặp node có `child`:
1. Lưu `next` hiện tại
2. Nối `child` vào sau node hiện tại
3. Tìm cuối child list
4. Nối cuối child list với `next` đã lưu

**Tại sao:** Có thể dùng iterative (stack) hoặc recursive (DFS). Iterative an toàn hơn cho list dài. Mỗi node chỉ được duyệt 1 lần nên vẫn O(n).

**Complexity:** Time O(n) với n là tổng số node ở tất cả tầng. Space O(1) nếu iterative (chỉ cần vài pointer), O(d) nếu recursive (d = depth).
</details>

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

// Duyệt xuôi
let forward: Vec<_> = list.iter().collect();
assert_eq!(forward, vec![1, 2]);

// Duyệt ngược
let backward: Vec<_> = list.iter_back().collect();
assert_eq!(backward, vec![2, 1]);

assert_eq!(list.len(), 2);
assert!(!list.is_empty());
```

### Khi nào dùng doubly linked list?

- Khi cần thêm/xóa nhanh ở **cả 2 đầu** -- deque operations.
- Khi cần duyệt **ngược** từ cuối về đầu.
- Khi xây dựng cấu trúc phức tạp hơn: **LRU cache**, text editor buffer, browser history.
- Khi cần xóa node ở **giữa** list mà đã có pointer tới node đó -- O(1) nhờ prev/next.

### Khi nào KHÔNG nên dùng?

- Khi chỉ cần thêm/xóa ở đầu -- singly linked list đơn giản và nhẹ hơn.
- Khi cần truy cập nhanh theo vị trí -- dùng `Vec`.
- Khi data nhỏ và ít thay đổi -- `Vec` nhanh hơn nhờ cache locality.
- Khi không cần shared ownership -- `Box` (singly) nhanh và đơn giản hơn `Rc<RefCell<>>`.

### Tóm tắt Rc\<RefCell\<>> cho ai hay quên

| Vấn đề | Giải pháp |
|--------|-----------|
| Nhiều node trỏ vào 1 node | `Rc` (shared ownership) |
| Cần sửa node đang bị chia sẻ | `RefCell` (interior mutability) |
| Kết hợp | `Rc<RefCell<Node>>` |
| Clone Rc | Chỉ tăng bộ đếm, rẻ |
| Nguy hiểm | Vòng tròn tham chiếu → memory leak |
| Cách tránh (học tập) | Tự viết `Drop` để phá vòng tròn |
| Cách tránh (production) | Dùng `Weak<T>` cho back-pointer |

---

---

[← Singly Linked List](./01-singly-linked-list.md) | [Stack →](./03-stack.md)
