# Deque (Double-Ended Queue)

## Đây là gì?

Tưởng tượng một cửa ra vào hai đầu -- giống hành lang tàu điện ngầm. Người có thể vào và ra từ **cả hai phía**. Không bị giới hạn một hướng.

Đó chính là **deque** (đọc là "deck", viết tắt của double-ended queue) -- hàng đợi hai đầu. Bạn có thể thêm/xóa phần tử ở **cả đầu lẫn cuối**.

Deque là "anh cả" của stack và queue:
- Chỉ dùng một đầu? Nó là **stack** (LIFO).
- Thêm ở cuối, xóa ở đầu? Nó là **queue** (FIFO).
- Dùng cả hai đầu? Nó là **deque** -- linh hoạt nhất.

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

Khi mảng đầy, `VecDeque` tự động mở rộng gấp đôi và sao chép (amortized O(1)).

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
