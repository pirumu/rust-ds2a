# Stack

## Đây là gì?

Bạn đã từng xếp đĩa ở nhà hàng chưa? Đĩa sạch được chồng lên trên cùng. Khi cần lấy đĩa, bạn lấy đĩa trên cùng -- không ai rút đĩa ở giữa chồng cả. Đĩa cuối cùng được đặt lên sẽ là đĩa đầu tiên được lấy ra.

Đó chính là **stack** -- cấu trúc dữ liệu hoạt động theo nguyên tắc **LIFO** (Last-In, First-Out: vào sau, ra trước).

### Tại sao cần biết stack?

Stack xuất hiện ở khắp nơi, bạn chỉ không nhận ra thôi:

- **Nút Undo trong Word** -- Mỗi thao tác bạn làm được đẩy vào stack. Bấm Ctrl+Z? Lấy thao tác gần nhất ra và hoàn tác.
- **Nút Back trong trình duyệt** -- Mỗi trang web bạn mở được đẩy vào stack. Bấm Back? Lấy trang gần nhất ra.
- **Function call stack** -- Khi hàm A gọi hàm B, gọi hàm C, máy tính dùng stack để nhớ đường về. C xong → quay lại B → quay lại A.
- **Kiểm tra ngoặc** -- Trình biên dịch (compiler) dùng stack để kiểm tra `({[]})` có khớp ngoặc không.

## Hoạt động như thế nào?

Stack chỉ có 3 thao tác cơ bản. Đơn giản vậy thôi.

| Thao tác | Nghĩa là gì? |
|----------|---------------|
| **push** | Đặt đĩa lên trên cùng |
| **pop** | Lấy đĩa trên cùng ra |
| **peek** | Nhìn đĩa trên cùng (không lấy ra) |

```
push(1)    push(2)    push(3)    pop() → 3    pop() → 2
+-----+    +-----+    +-----+    +-----+      +-----+
|     |    |     |    |  3  | ← top            |     |
|     |    |  2  |    |  2  |    |  2  | ← top |     |
|  1  |    |  1  |    |  1  |    |  1  |       |  1  | ← top
+-----+    +-----+    +-----+    +-----+      +-----+
```

### Dùng Vec làm "nền"

Trong Rust, `Vec` (vector -- mảng động) đã hỗ trợ sẵn push/pop ở cuối mảng. Phần tử cuối của `Vec` chính là đỉnh stack.

```
Vec bên trong:  [1, 2, 3]
                         ^
                         đỉnh stack (top)
```

Push vào cuối: O(1) amortized (thỉnh thoảng phải mở rộng bộ nhớ, nhưng trung bình vẫn nhanh).
Pop từ cuối: luôn O(1).

Tại sao không dùng thẳng `Vec`? Vì `Vec` cho phép quá nhiều thứ -- insert ở giữa, truy cập theo index. Stack cố tình **giới hạn** chỉ còn push/pop/peek để đảm bảo bạn dùng đúng cách. Ít lựa chọn hơn = ít bug hơn.

## Code Rust

Code đầy đủ nằm trong `src/stack.rs`.

```rust
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Tạo stack rỗng
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Đặt phần tử lên đỉnh stack
    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    /// Lấy phần tử trên cùng ra (trả về None nếu stack rỗng)
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Nhìn phần tử trên cùng (không lấy ra)
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Stack có rỗng không?
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Stack có bao nhiêu phần tử?
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
```

Code rất ngắn phải không? Vì `Vec` đã làm hết việc nặng rồi. Giá trị của `Stack` nằm ở **interface bị giới hạn** -- không cho index, không cho insert ở giữa. Nguyên tắc LIFO được bảo vệ.

## Độ phức tạp

| Thao tác | Thời gian (trung bình) | Bộ nhớ |
|----------|------------------------|--------|
| `push` | O(1) | O(1) |
| `pop` | O(1) | O(1) |
| `peek` | O(1) | O(1) |
| `is_empty` | O(1) | O(1) |
| `size` | O(1) | O(1) |
| **Tổng bộ nhớ** | -- | **O(n)** |

Tất cả thao tác đều O(1) -- nhanh không thể nhanh hơn. `push` là O(1) amortized vì `Vec` đôi khi phải cấp phát lại bộ nhớ (gấp đôi dung lượng), nhưng chi phí này được trải đều.

**Nôm na:** Bất kể stack có 10 hay 10 triệu phần tử, push/pop/peek đều nhanh như nhau.

## Ví dụ

### Sử dụng cơ bản

```rust
use rust_ds2a::stack::Stack;

let mut stack = Stack::new();

stack.push(10);
stack.push(20);
stack.push(30);

assert_eq!(stack.peek(), Some(&30));  // đỉnh stack là 30
assert_eq!(stack.pop(), Some(30));    // lấy 30 ra
assert_eq!(stack.pop(), Some(20));    // giờ đỉnh là 20, lấy ra
assert_eq!(stack.size(), 1);          // còn 1 phần tử (số 10)
```

### Bài toán kiểm tra ngoặc

Đây là ứng dụng kinh điển nhất của stack. Ý tưởng: gặp ngoặc mở thì push, gặp ngoặc đóng thì pop ra kiểm tra có khớp không.

```rust
use rust_ds2a::stack::Stack;

fn is_balanced(s: &str) -> bool {
    let mut stack = Stack::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),             // ngoặc mở → push
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}  // bỏ qua ký tự khác
        }
    }
    stack.is_empty()  // cuối cùng stack phải rỗng
}

assert!(is_balanced("({[]})"));   // ✓ tất cả ngoặc khớp
assert!(!is_balanced("({[})"));   // ✗ ngoặc vuông đóng trước ngoặc nhọn
```

```
Duyệt "({[]})":

Ký tự  Hành động            Stack
  (    push '('             ['(']
  {    push '{'             ['(', '{']
  [    push '['             ['(', '{', '[']
  ]    pop → '[' khớp ']'   ['(', '{']
  }    pop → '{' khớp '}'   ['(']
  )    pop → '(' khớp ')'   []

Stack rỗng → ngoặc cân bằng! ✓
```
