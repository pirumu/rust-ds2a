# Deque (Double-Ended Queue)

## Đây là gì?

> **Deque** nghe tên có vẻ phức tạp, nhưng bạn đừng lo. Nếu bạn đã hiểu Stack (LIFO) và Queue (FIFO) ở hai chương trước, thì Deque chỉ là **gộp cả hai lại** -- một Queue mà mở thêm một đầu. Hiểu Deque là bạn đã hiểu nền tảng của sliding window algorithm -- một pattern xuất hiện cực nhiều trong phỏng vấn và trong hệ thống thực tế (scheduler, buffer management). Bạn hoàn toàn làm được.

### Nhìn lại Stack và Queue

Ở hai chương trước, bạn đã quen:

- **Stack** = LIFO (chồng đĩa): chỉ thêm/xóa ở **một đầu** (đỉnh).
- **Queue** = FIFO (hàng trà sữa): thêm ở cuối, xóa ở đầu -- **một hướng** duy nhất.

Nhưng nếu bạn cần **vừa** thêm/xóa ở đầu, **vừa** thêm/xóa ở cuối thì sao? Stack không cho xóa đáy. Queue không cho thêm ở đầu. Bạn cần một cấu trúc linh hoạt hơn.

### So sánh 3 cấu trúc

| Thao tác | Stack | Queue | Deque |
|----------|-------|-------|-------|
| Thêm đầu | ❌ | ❌ | ✅ |
| Thêm cuối | ✅ (push) | ✅ (enqueue) | ✅ |
| Xóa đầu | ❌ | ✅ (dequeue) | ✅ |
| Xóa cuối | ✅ (pop) | ❌ | ✅ |

**Deque = Stack ∪ Queue.** Mọi thứ Stack làm được, Deque làm được. Mọi thứ Queue làm được, Deque làm được. Và hơn thế nữa.

### Bộ bài trên tay

Tưởng tượng một **bộ bài** cầm trên tay. Bạn có thể rút lá trên cùng, hoặc rút lá dưới cùng. Bạn cũng có thể đặt thêm lá vào trên cùng hoặc nhét vào dưới cùng. Cả hai đầu đều dùng được.

Đó chính là **deque** (đọc là "deck", viết tắt của double-ended queue) -- hàng đợi hai đầu. Bạn có thể thêm/xóa phần tử ở **cả đầu lẫn cuối**.

Deque là "anh cả" của stack và queue:
- Chỉ dùng một đầu? Nó là **stack** (LIFO).
- Thêm ở cuối, xóa ở đầu? Nó là **queue** (FIFO).
- Dùng cả hai đầu? Nó là **deque** -- linh hoạt nhất nhưng cũng dễ dùng sai nhất.

### Khi nào cần deque?

- **Sliding window** -- Bài toán tìm giá trị lớn nhất trong cửa sổ trượt. Thêm phần tử mới ở cuối, xóa phần tử cũ ở đầu, đồng thời xóa phần tử nhỏ hơn ở cuối.
- **Palindrome checking** -- Lấy ký tự từ cả hai đầu để so sánh.
- **Work-stealing scheduler** -- Thread lấy việc từ đầu deque của mình, nhưng "ăn trộm" việc từ cuối deque của thread khác.

## Hoạt động như thế nào?

### 4 thao tác cốt lõi

| Thao tác | Nghĩa là gì? | Phía nào? |
|----------|---------------|-----------|
| `push_front(x)` | Thêm x vào đầu | Đầu |
| `push_back(x)` | Thêm x vào cuối | Cuối |
| `pop_front()` | Lấy phần tử đầu ra | Đầu |
| `pop_back()` | Lấy phần tử cuối ra | Cuối |

```
push_front(1):    [1]
push_back(2):     [1, 2]
push_back(3):     [1, 2, 3]
push_front(0):    [0, 1, 2, 3]
pop_front() → 0:  [1, 2, 3]
pop_back() → 3:   [1, 2]
```

### Ring buffer -- bí mật bên trong

Giống queue, deque dùng **ring buffer** (bộ đệm vòng). Bộ nhớ vật lý là một mảng thẳng, nhưng ta xem nó như vòng tròn:

```
Nhìn logic:   [A, B, C, D]

Bộ nhớ vật lý (dung lượng 8):

  index:    0   1   2   3   4   5   6   7
          +---+---+---+---+---+---+---+---+
          | C | D | _ | _ | _ | _ | A | B |
          +---+---+---+---+---+---+---+---+
                    ^               ^
                   tail            head

push_front(Z) -- thêm Z vào đầu:
          +---+---+---+---+---+---+---+---+
          | C | D | _ | _ | _ | Z | A | B |
          +---+---+---+---+---+---+---+---+
                    ^           ^
                   tail        head (lùi 1 ô)

push_back(E) -- thêm E vào cuối:
          +---+---+---+---+---+---+---+---+
          | C | D | E | _ | _ | Z | A | B |
          +---+---+---+---+---+---+---+---+
                        ^       ^
                       tail    head
```

Điểm quan trọng: **không có phần tử nào bị dịch chuyển**. `head` lùi trái khi push_front, `tail` tiến phải khi push_back. Cả hai đều O(1).

### Nghĩ như vòng tròn

Diagram mảng thẳng ở trên đúng, nhưng đôi khi khó hình dung khi index "quấn" qua mép mảng. Hãy nghĩ nó như vòng tròn thực sự:

```
Thay vì nghĩ như mảng thẳng:
  [C, D, _, _, _, Z, A, B]
                  ^
                 head

Hãy nghĩ như vòng tròn:

         0(C)
     7(B)     1(D)
   6(A)    ●    2(_)
     5(Z)     3(_)
         4(_)

  A ở index 6, B ở index 7, C ở index 0, D ở index 1, Z ở index 5.
  head = 5 (vị trí Z), tail = 2 (ô trống tiếp theo sau D)
```

Trên vòng tròn, mọi thứ rõ ràng hơn:

- **push_front(W):** head lùi 1 bước (5→4), đặt W ở index 4. Head đi **ngược chiều kim đồng hồ**.
- **push_back(E):** đặt E ở tail (index 2), tail tiến 1 bước (2→3). Tail đi **thuận chiều kim đồng hồ**.
- **Khi nào đầy?** head và tail chạy ngược chiều nhau -- khi chúng gặp nhau, mảng đầy, cần resize.

Khi mảng đầy, `VecDeque` tự động mở rộng gấp đôi và sao chép (amortized O(1)).

## VecDeque trong Rust

Phần này quan trọng vì bạn sẽ dùng `VecDeque` rất nhiều trong thực tế. Hiểu nó giúp bạn biết tại sao Deque nhanh, và tránh sai lầm phổ biến.

`VecDeque` là ring buffer implement sẵn trong `std::collections` -- chính xác cái mà diagram vòng tròn ở trên mô tả.

### VecDeque vs Vec

```rust
// ❌ Vec: push_front là O(n) -- dịch hết mọi phần tử
let mut v = Vec::new();
v.insert(0, item);  // mọi phần tử bị dịch phải 1 ô

// ✅ VecDeque: push_front là O(1) -- chỉ lùi head pointer
let mut d = VecDeque::new();
d.push_front(item);  // head lùi 1 ô, không ai bị dịch
```

Minh họa sự khác biệt:

```
Vec::insert(0, X) -- phải dịch TẤT CẢ phần tử:
  Trước: [A, B, C, D, _]
              ←←←←         dịch phải
  Sau:   [X, A, B, C, D]
  → O(n) vì n phần tử bị dịch

VecDeque::push_front(X) -- chỉ lùi head:
  Trước: [_, _, A, B, C, D, _, _]
                ^
               head
  Sau:   [_, X, A, B, C, D, _, _]
              ^
             head (lùi 1)
  → O(1) vì không ai bị dịch
```

### Tại sao không tự implement ring buffer?

`VecDeque` đã xử lý rất nhiều chi tiết phức tạp:
- **Resize** khi đầy (gấp đôi capacity)
- **Wrap-around index** (head < tail hoặc head > tail đều hoạt động đúng)
- **Memory alignment** và **drop safety** (tự gọi destructor cho mọi phần tử)

Rust-specific: `VecDeque<T>` dùng `usize` index thay vì pointer -- borrow checker thân thiện hơn, không cần `unsafe` cho basic operations.

## Code Rust

Code đầy đủ nằm trong `src/deque.rs`.

```rust
use std::collections::VecDeque;

pub struct Deque<T> {
    data: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Tạo deque rỗng
    pub fn new() -> Self {
        Self { data: VecDeque::new() }
    }

    /// Thêm vào đầu
    pub fn push_front(&mut self, val: T) {
        self.data.push_front(val);
    }

    /// Thêm vào cuối
    pub fn push_back(&mut self, val: T) {
        self.data.push_back(val);
    }

    /// Lấy phần tử đầu ra
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Lấy phần tử cuối ra
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Nhìn phần tử đầu (không lấy ra)
    pub fn peek_front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Nhìn phần tử cuối (không lấy ra)
    pub fn peek_back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Deque có rỗng không?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Deque có bao nhiêu phần tử?
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
```

Giống Stack và Queue, code ngắn gọn vì `VecDeque` xử lý ring buffer bên trong. `Deque` wrapper cho ta interface rõ ràng, dễ đọc.

## Độ phức tạp

| Thao tác | Thời gian (trung bình) | Bộ nhớ |
|----------|------------------------|--------|
| `push_front` | O(1) | O(1) |
| `push_back` | O(1) | O(1) |
| `pop_front` | O(1) | O(1) |
| `pop_back` | O(1) | O(1) |
| `peek_front` | O(1) | O(1) |
| `peek_back` | O(1) | O(1) |
| `is_empty` | O(1) | O(1) |
| `size` | O(1) | O(1) |
| **Tổng bộ nhớ** | -- | **O(n)** |

**Nôm na:** Tất cả thao tác đều siêu nhanh. Deque mạnh hơn stack và queue, nhưng không chậm hơn chút nào.

## Ví dụ

### Sử dụng cơ bản

```rust
use rust_ds2a::deque::Deque;

let mut d = Deque::new();

d.push_back(1);
d.push_back(2);
d.push_front(0);
// deque: [0, 1, 2]

assert_eq!(d.peek_front(), Some(&0));  // đầu deque
assert_eq!(d.peek_back(), Some(&2));   // cuối deque

assert_eq!(d.pop_front(), Some(0));    // lấy đầu ra
assert_eq!(d.pop_back(), Some(2));     // lấy cuối ra
// deque: [1]

assert_eq!(d.size(), 1);
```

### Deque làm stack

```rust
use rust_ds2a::deque::Deque;

let mut stack = Deque::new();
stack.push_back(1);
stack.push_back(2);
stack.push_back(3);

// Chỉ dùng 1 đầu (cuối) → hoạt động như stack (LIFO)
assert_eq!(stack.pop_back(), Some(3));
assert_eq!(stack.pop_back(), Some(2));
```

### Deque làm queue

```rust
use rust_ds2a::deque::Deque;

let mut queue = Deque::new();
queue.push_back(1);
queue.push_back(2);
queue.push_back(3);

// Thêm ở cuối, lấy ở đầu → hoạt động như queue (FIFO)
assert_eq!(queue.pop_front(), Some(1));
assert_eq!(queue.pop_front(), Some(2));
```

### Kiểm tra palindrome bằng deque

Ý tưởng cực hay: bỏ từng ký tự vào deque, rồi lấy đồng thời từ 2 đầu để so sánh. Nếu luôn giống nhau → palindrome.

```rust
use rust_ds2a::deque::Deque;

fn is_palindrome(s: &str) -> bool {
    let mut d = Deque::new();
    for ch in s.chars().filter(|c| c.is_alphanumeric()) {
        d.push_back(ch.to_ascii_lowercase());
    }
    while d.size() > 1 {
        if d.pop_front() != d.pop_back() {
            return false;  // 2 đầu khác nhau → không phải palindrome
        }
    }
    true  // còn 0 hoặc 1 ký tự → palindrome
}

assert!(is_palindrome("racecar"));
assert!(!is_palindrome("hello"));
```

```
Duyệt "racecar":

Deque: [r, a, c, e, c, a, r]

So sánh 2 đầu:
  pop_front → r  vs  pop_back → r  ✓
  pop_front → a  vs  pop_back → a  ✓
  pop_front → c  vs  pop_back → c  ✓
  Còn 1 ký tự (e) → dừng

Kết quả: palindrome! ✓
```

Giờ thử với "hello" -- một từ KHÔNG phải palindrome:

```
Duyệt "hello":

Deque: [h, e, l, l, o]

So sánh 2 đầu:
  pop_front → h  vs  pop_back → o  ✗ (khác nhau!)
  → return false ngay, không cần kiểm tra tiếp

Nếu dùng brute force (reverse string rồi so sánh): phải duyệt hết.
Deque approach: dừng sớm khi phát hiện sai → tiết kiệm thời gian.
```

**Câu hỏi tư duy:** Palindrome check dùng Deque có thực sự tốt hơn dùng 2 pointer (đầu + cuối string) không?

Về complexity thì giống nhau O(n), nhưng 2 pointer tốt hơn vì không cần bộ nhớ phụ O(n) cho deque. Vậy tại sao dạy cách deque? Vì nó minh họa **tư duy dùng deque** -- kỹ năng này áp dụng cho bài toán khó hơn (sliding window) mà 2 pointer không giải được.

## Deque trong thực tế

Biết cấu trúc dữ liệu suông chưa đủ -- quan trọng là biết **ở đâu trong thực tế** nó phát huy sức mạnh. Dưới đây là 3 use case mà Deque là lựa chọn tối ưu.

### a) Sliding Window Maximum -- pattern phỏng vấn kinh điển

**Bài toán:** Cho mảng `[1, 3, -1, -3, 5, 3, 6, 7]` và window size `k=3`. Tìm giá trị lớn nhất trong mỗi cửa sổ trượt khi di chuyển từ trái sang phải.

```
Window 1: [1, 3, -1]       → max = 3
Window 2: [3, -1, -3]      → max = 3
Window 3: [-1, -3, 5]      → max = 5
Window 4: [-3, 5, 3]       → max = 5
Window 5: [5, 3, 6]        → max = 6
Window 6: [3, 6, 7]        → max = 7

Kết quả: [3, 3, 5, 5, 6, 7]
```

**Brute force** duyệt lại k phần tử cho mỗi window → O(nk). Với Deque, ta đạt **O(n)**.

**Ý tưởng:** Deque chứa **index**, giữ giá trị **giảm dần** từ front đến back. Front luôn là max của window hiện tại.

Tại sao giảm dần? Nếu phần tử A nhỏ hơn phần tử B và A đứng **trước** B, thì A sẽ **không bao giờ** là max (vì B sẽ luôn còn trong window khi A còn). Loại A sớm, khỏi tốn công.

**Trace từng bước:**

```
Mảng: [1, 3, -1, -3, 5, 3, 6, 7],  k=3
Deque chứa index, giữ giá trị giảm dần

Bước  i  nums[i]  Action                                Deque (index→value)       Max
─────────────────────────────────────────────────────────────────────────────────────────
  1   0    1      push_back(0)                           [0→1]                      -
  2   1    3      3>1 → pop_back(0), push_back(1)        [1→3]                      -
  3   2   -1      push_back(2)                           [1→3, 2→-1]                3
  4   3   -3      push_back(3)                           [1→3, 2→-1, 3→-3]          3
  5   4    5      5>-3 → pop, 5>-1 → pop, 5>3 → pop     [4→5]                      5
                  index 1 ra khỏi window → đã bị pop
  6   5    3      push_back(5)                           [4→5, 5→3]                 5
  7   6    6      6>3 → pop, 6>5 → pop, push_back(6)    [6→6]                      6
  8   7    7      7>6 → pop, push_back(7)                [7→7]                      7

Kết quả: [3, 3, 5, 5, 6, 7] ✓
```

**Code Rust** (có trong `src/deque.rs`):

```rust
use std::collections::VecDeque;

pub fn sliding_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }
    let k = k.min(nums.len());
    let mut result = Vec::with_capacity(nums.len() - k + 1);
    let mut dq: VecDeque<usize> = VecDeque::new();  // chứa index

    for i in 0..nums.len() {
        // Xóa index đã ra khỏi window
        if let Some(&front) = dq.front() {
            if front + k <= i {
                dq.pop_front();
            }
        }

        // Xóa các index có giá trị nhỏ hơn nums[i]
        // (chúng sẽ không bao giờ là max khi nums[i] còn trong window)
        while let Some(&back) = dq.back() {
            if nums[back] <= nums[i] {
                dq.pop_back();
            } else {
                break;
            }
        }

        dq.push_back(i);  // thêm index hiện tại

        // Đủ k phần tử → ghi nhận max (front của deque)
        if i >= k - 1 {
            result.push(nums[dq[0]]);
        }
    }

    result
}
```

### b) Work-Stealing Scheduler (tokio / crossbeam)

**Bài toán:** Trong runtime như tokio, mỗi worker thread có một deque riêng chứa task. Mục tiêu: phân phối task đều giữa các thread mà không cần lock nặng.

```
 Worker Thread 1 (Owner)          Worker Thread 2 (Thief)
┌─────────────────────┐          ┌─────────────────────┐
│    push_back(task)   │          │   "Hết việc rồi!"   │
│         ↓            │          │         ↓            │
│  ┌───┬───┬───┬───┐  │          │    pop_front(task)   │
│  │ D │ C │ B │ A │  │ ←────────│    từ deque của      │
│  └───┴───┴───┴───┘  │  steal   │    Thread 1          │
│         ↑            │          │                      │
│    pop_back(task)    │          │                      │
│   (xử lý task mới   │          │                      │
│    nhất -- LIFO)     │          │                      │
└─────────────────────┘          └─────────────────────┘
```

**Tại sao LIFO cho owner?** Task mới nhất còn "nóng" trong CPU cache → xử lý nhanh hơn.

**Tại sao FIFO cho thief?** Lấy task cũ nhất (đầu kia) → ít conflict với owner (owner đang làm việc ở đuôi). Task cũ thường lớn hơn (ví dụ: task gốc chưa chia nhỏ) → thief lấy 1 task nhưng có nhiều việc để làm.

Trong Rust ecosystem: `crossbeam::deque::Worker` / `Stealer` chính là pattern này. tokio runtime dùng cơ chế tương tự.

### c) Browser History -- Bounded Deque

**Bài toán:** Browser giữ tối đa N trang trong history. Khi đầy, trang cũ nhất bị xóa tự động.

```
Browser history (max = 4):

Truy cập google.com:     [google]
Truy cập github.com:     [google, github]
Truy cập docs.rs:        [google, github, docs]
Truy cập crates.io:      [google, github, docs, crates]    ← đầy!
Truy cập rust-lang.org:  [github, docs, crates, rust-lang] ← google bị xóa (pop_front)
Nút Back:                [github, docs, crates]             ← rust-lang ra (pop_back)
```

Đây là **bounded deque** -- khi `size > max`, tự động `pop_front`.

**Code Rust** (có trong `src/deque.rs`):

```rust
use std::collections::VecDeque;

pub struct BoundedDeque<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> BoundedDeque<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Thêm vào cuối. Nếu đầy, phần tử đầu tiên bị xóa.
    pub fn push_back(&mut self, val: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front();  // xóa phần tử cũ nhất
        }
        self.data.push_back(val);
    }

    /// Nút "Back" -- lấy phần tử cuối ra
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
```

## Những cái bẫy hay gặp

Phần này giúp bạn tránh những sai lầm mà hầu như ai mới học deque cũng mắc phải.

---

❌ **Vấn đề:** Dùng `Vec` + `insert(0, x)` thay vì `VecDeque::push_front(x)`

✅ **Đúng:** Dùng `VecDeque` khi cần thêm/xóa ở đầu

💡 **Tại sao:** `Vec::insert(0, x)` là O(n) -- phải dịch TẤT CẢ phần tử sang phải. `VecDeque::push_front(x)` là O(1) -- chỉ lùi head pointer, không ai bị dịch.

```
Vec::insert(0, X):        VecDeque::push_front(X):
  [A, B, C, D]              [_, A, B, C, D]
   →→→→→→→→  dịch hết!        ^
  [X, A, B, C, D]            head lùi 1 ô
  → O(n)                    [X, A, B, C, D]
                             → O(1)
```

---

❌ **Vấn đề:** Nhầm lẫn push_front/push_back với pop_front/pop_back

✅ **Đúng:** Nhớ bảng "ai thêm ở đâu, ai xóa ở đâu"

💡 **Tại sao:** Tên hàm nói rõ: `push` = thêm, `pop` = xóa, `front` = đầu, `back` = cuối.

```
          ĐẦU                      CUỐI
          ←──                      ──→
push_front(x) →  [  ...deque...  ]  ← push_back(x)
pop_front()  ←   [  ...deque...  ]  → pop_back()
```

---

❌ **Vấn đề:** `unwrap()` bừa trên `pop_front()` / `pop_back()`

✅ **Đúng:** Dùng `if let` hoặc `match` để xử lý `None`

💡 **Tại sao:** Rust trả `Option<T>` nên compiler nhắc, nhưng nếu bạn `.unwrap()` khi deque rỗng thì **panic**. Luôn handle trường hợp rỗng.

```rust
// ❌ Panic nếu deque rỗng
let val = deque.pop_front().unwrap();

// ✅ An toàn
if let Some(val) = deque.pop_front() {
    println!("Got: {val}");
} else {
    println!("Deque rỗng!");
}
```

---

❌ **Vấn đề:** Dùng Deque khi chỉ cần Stack hoặc Queue

✅ **Đúng:** Dùng đúng cấu trúc cho đúng nhu cầu

💡 **Tại sao:** YAGNI (You Ain't Gonna Need It). Deque linh hoạt hơn nhưng **intent không rõ ràng**. Người đọc code không biết bạn dùng nó làm stack hay queue. Nếu chỉ cần LIFO → `Vec`. Nếu chỉ cần FIFO → `VecDeque` với `push_back`/`pop_front`. Code rõ ý hơn khi dùng đúng cấu trúc.

## Khi nào dùng / không nên dùng

Biết khi nào **không** dùng deque cũng quan trọng không kém biết khi nào dùng. Bảng dưới đây giúp bạn chọn đúng công cụ.

| Tình huống | Deque? | Thay bằng gì? | Tại sao? |
|------------|--------|---------------|----------|
| Sliding window problems | ✅ | -- | Cần thêm/xóa cả hai đầu |
| Work-stealing scheduler | ✅ | -- | Owner LIFO + thief FIFO |
| Browser history có giới hạn | ✅ | -- | Bounded: pop_front khi đầy |
| Task queue đơn giản (FIFO) | ❌ | Queue (VecDeque) | Chỉ cần 1 hướng |
| Undo/Redo | ❌ | 2 Stacks | Không cần queue behavior |
| Priority-based processing | ❌ | BinaryHeap | Cần sắp xếp theo ưu tiên |
| Random access phần tử | ❌ | Vec | Deque O(1) ở 2 đầu, không ở giữa |
| Cache/LRU | ⚠️ | LinkedHashMap | Deque được nhưng lookup O(n) |

## Luyện nhận diện Pattern

Kỹ năng quan trọng nhất không phải code, mà là **nhận ra bài toán nào cần deque**. 3 bài tập dưới đây rèn đúng kỹ năng đó.

---

**Bài 1: Sliding Window Minimum**

Cho mảng số nguyên và window size k, tìm phần tử **nhỏ nhất** trong mỗi window khi trượt từ trái sang phải.

**Gợi ý:** Bạn đã giải Sliding Window Maximum ở trên. Bài này khác gì?

<details>
<summary>Đáp án</summary>

Có dùng Deque không? **Có**

Tại sao: Giống hệt Sliding Window Maximum, nhưng deque giữ thứ tự **tăng dần** thay vì giảm dần. Phần tử lớn hơn phần tử mới → loại bỏ (thay vì phần tử nhỏ hơn). Front của deque luôn là min.

Complexity: Time O(n), Space O(k)

</details>

---

**Bài 2: Maximum of all subarrays of size k**

Cho mảng `[8, 5, 10, 7, 9, 4, 15, 12, 90, 13]` và k=4, tìm max mỗi subarray.

**Gợi ý:** Đây có phải bài mới không, hay bạn đã thấy nó ở đâu rồi?

<details>
<summary>Đáp án</summary>

Có dùng Deque không? **Có**

Tại sao: Đây **chính là** Sliding Window Maximum! "Subarray of size k" = "window of size k". Đừng để cách đặt câu hỏi khác nhau đánh lừa. Áp dụng y hệt thuật toán ở trên.

Complexity: Time O(n), Space O(k)

</details>

---

**Bài 3: Design a Queue using 2 Stacks**

Implement `enqueue()` và `dequeue()` chỉ dùng 2 stack, không dùng deque hay queue.

**Gợi ý:** Một stack để nhận, một stack để phát. Khi nào cần "đảo" thứ tự?

<details>
<summary>Đáp án</summary>

Có dùng Deque không? **Không** -- đây là bài luyện tư duy stack, không cần deque.

Tại sao: Stack A nhận input (`push`). Khi cần `dequeue`, nếu Stack B rỗng → đổ hết A sang B (đảo thứ tự) → `pop` từ B. Amortized O(1) cho mỗi operation vì mỗi phần tử chỉ bị di chuyển tối đa 2 lần.

Complexity: Time O(1) amortized, Space O(n)

</details>

## Deque trong Rust ecosystem

Nắm được pattern rồi, giờ xem Deque xuất hiện ở đâu trong thực tế Rust:

- **`std::collections::VecDeque`** -- cái bạn vừa học, dùng hàng ngày.
- **`crossbeam::deque::Worker` / `Stealer`** -- work-stealing deque dùng trong thread pool. tokio runtime dùng pattern tương tự để phân phối task giữa các worker thread.
- **`tokio::sync::mpsc`** -- không phải deque, nhưng concept tương tự: producer push vào 1 đầu, consumer lấy từ đầu kia.
- **Buffer management** trong các Kafka client (như KaCrab) cũng dùng ring buffer concept -- data vào từ network (`push_back`), consumer lấy từ đầu (`pop_front`).

## Chương tiếp theo

Bạn đã nắm được các cấu trúc linear trong Part 2: Singly Linked List, Doubly Linked List, Stack, Queue, và Deque. Tất cả đều thao tác ở **đầu** hoặc **cuối** -- rất nhanh.

Nhưng thế giới không chỉ có đường thẳng. Part 3 sẽ giới thiệu **Binary Tree** -- cấu trúc mà mỗi phần tử có thể có 2 "con", tạo thành hình cây. Tree cho phép tìm kiếm, thêm, xóa trong O(log n) -- nhanh hơn nhiều so với O(n) của mảng. Trade-off? Cấu trúc phức tạp hơn, cần tư duy đệ quy.

Đó là bài học quan trọng trong CS: **không có cấu trúc nào hoàn hảo -- chỉ có cấu trúc phù hợp.**

---

---

[← Queue](./04-queue.md) | [Binary Tree →](../03-trees-and-heaps/01-binary-tree.md)
