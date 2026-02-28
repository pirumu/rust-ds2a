# Stack

> 💡 **Đừng lo lắng:** Stack nghe tên lạ, nhưng thật ra bộ não bạn cũng hoạt động kiểu LIFO mỗi ngày -- bạn nhớ việc vừa làm xong rõ hơn việc từ sáng sớm. Khi đang nấu ăn mà chuông cửa kêu, bạn dừng nấu → mở cửa → quay lại nấu. Đó là stack. Hiểu xong chương này, bạn sẽ thấy một mảnh ghép lớn trong cách máy tính thực sự hoạt động -- từ cách hàm gọi nhau đến cách debugger hoạt động.

## Đây là gì?

Bạn đã từng xếp đĩa ở nhà hàng chưa? Đĩa sạch được chồng lên trên cùng. Khi cần lấy đĩa, bạn lấy đĩa trên cùng -- không ai rút đĩa ở giữa chồng cả. Đĩa cuối cùng được đặt lên sẽ là đĩa đầu tiên được lấy ra.

Đó chính là **stack** -- cấu trúc dữ liệu hoạt động theo nguyên tắc **LIFO** (Last-In, First-Out: vào sau, ra trước).

### Tại sao cần biết stack?

Stack xuất hiện ở khắp nơi, bạn chỉ không nhận ra thôi:

- **Nút Undo trong Word** -- Mỗi thao tác bạn làm được đẩy vào stack. Bấm Ctrl+Z? Lấy thao tác gần nhất ra và hoàn tác.
- **Nút Back trong trình duyệt** -- Mỗi trang web bạn mở được đẩy vào stack. Bấm Back? Lấy trang gần nhất ra.
- **Kiểm tra ngoặc** -- Trình biên dịch (compiler) dùng stack để kiểm tra `({[]})` có khớp ngoặc không.

Nhưng use case quan trọng nhất mà hầu hết người mới không biết:

#### Function Call Stack -- Stack đang chạy ngay trong code của bạn

Mỗi khi bạn gọi một hàm, máy tính **push** địa chỉ "quay về" vào một stack gọi là **call stack**. Khi hàm chạy xong, máy tính **pop** địa chỉ đó ra để biết quay về đâu.

```
main() gọi foo(), foo() gọi bar():

    Call Stack:
    ┌──────────┐
    │  bar()   │ ← đang chạy
    │  foo()   │ ← chờ bar() xong
    │  main()  │ ← chờ foo() xong
    └──────────┘

    bar() xong → pop → quay về foo()
    foo() xong → pop → quay về main()
```

Đây là lý do:
- **StackOverflow** xảy ra khi đệ quy vô tận -- hàm cứ gọi chính nó, push mãi không pop, call stack đầy tràn.
- Khi chương trình crash, debugger cho bạn xem **stack trace** -- đó chính là nội dung call stack lúc đó. Bạn sẽ thấy hàm nào gọi hàm nào, theo đúng thứ tự LIFO.

Nói cách khác: **mỗi lần bạn viết một hàm gọi hàm khác, bạn đang dùng stack mà không biết.**

## Mental Model: Stack = Bộ nhớ tạm có kỷ luật

Trước khi đi vào chi tiết, hãy nhớ một hình ảnh xuyên suốt:

**Stack giống như trí nhớ ngắn hạn bị ép theo kỷ luật.** Bạn chỉ được nhớ thứ vừa đặt vào gần nhất, và phải giải quyết xong thứ đó trước khi quay lại thứ trước đó. Không được nhảy cóc.

Quay lại ví dụ chồng đĩa: bạn không được rút đĩa ở giữa. Phải lấy đĩa trên cùng trước. Quy tắc đơn giản này tạo ra sự **dự đoán được** -- bạn luôn biết thứ tiếp theo phải xử lý là gì.

Mỗi khi gặp bài toán mà bạn cần **nhớ thứ gì đó rồi quay lại xử lý sau, theo thứ tự ngược** -- nghĩ đến stack.

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

#### Trace thành công: `"({[]})"`

```
Ký tự  Hành động            Stack
  (    push '('             ['(']
  {    push '{'             ['(', '{']
  [    push '['             ['(', '{', '[']
  ]    pop → '[' khớp ']'   ['(', '{']
  }    pop → '{' khớp '}'   ['(']
  )    pop → '(' khớp ')'   []

Stack rỗng → ngoặc cân bằng! ✓
```

#### Trace thất bại: `"({[})"`

```
Ký tự  Hành động                   Stack             Kết quả
  (    push '('                    ['(']
  {    push '{'                    ['(', '{']
  [    push '['                    ['(', '{', '[']
  }    pop → lấy '[' ra            ['(', '{']         ✗ DỪNG!
       '[' ≠ '}' → không khớp!
```

Stack "nhớ" rằng ngoặc mở gần nhất là `[`, nhưng ký tự `}` lại muốn đóng `{`. Đây là mâu thuẫn -- giống như bạn mở cửa phòng A, rồi lại cầm chìa khóa phòng B để đóng. Stack phát hiện ngay lỗi này vì nó luôn biết thứ gần nhất chưa được đóng là gì.

### Tính toán biểu thức hậu tố (Reverse Polish Notation)

Bình thường bạn viết `(3 + 4) * 2`. Nhưng máy tính có cách viết khác gọi là **hậu tố (postfix)**: `3 4 + 2 *`. Nghĩa là: "lấy 3 và 4, cộng lại, rồi nhân với 2".

Cái hay: **không cần ngoặc.** Thứ tự tính toán được xác định hoàn toàn bằng vị trí của toán tử. Máy tính HP ngày xưa dùng RPN vì lý do này -- đơn giản hơn cho máy xử lý.

Stack tính RPN như thế nào? Quy tắc đơn giản:
- Gặp **số** → push vào stack (chồng đĩa lên)
- Gặp **toán tử** → pop 2 số ra, tính, push kết quả lại (lấy 2 đĩa, gộp thành 1 đĩa mới)

#### Trace: `3 4 + 2 *` = (3+4) × 2 = 14

```
Token  Hành động            Stack
  3    push 3               [3]
  4    push 4               [3, 4]
  +    pop 4, pop 3         []
       3 + 4 = 7
       push 7               [7]
  2    push 2               [7, 2]
  *    pop 2, pop 7         []
       7 * 2 = 14
       push 14              [14]

Kết quả: pop → 14 ✓
```

#### Code Rust

```rust
use rust_ds2a::stack::{Stack, evaluate_rpn};

// 3 4 + 2 * = (3+4)*2 = 14
let tokens = vec!["3", "4", "+", "2", "*"];
assert_eq!(evaluate_rpn(&tokens), Some(14));

// 10 3 - = 7
assert_eq!(evaluate_rpn(&["10", "3", "-"]), Some(7));

// Biểu thức không hợp lệ → None
assert_eq!(evaluate_rpn(&["3", "+", "+"]), None);
```

Code đầy đủ của `evaluate_rpn` trong `src/stack.rs`:

```rust
pub fn evaluate_rpn(tokens: &[&str]) -> Option<i64> {
    let mut stack = Stack::new();

    for &token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop()?;  // toán hạng phải (đĩa trên)
                let a = stack.pop()?;  // toán hạng trái (đĩa dưới)
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 { return None; }  // chia cho 0
                        a / b
                    }
                    _ => unreachable!(),
                };
                stack.push(result);    // đặt kết quả lại lên đỉnh
            }
            num_str => {
                let num: i64 = num_str.parse().ok()?;  // chuyển chuỗi → số
                stack.push(num);
            }
        }
    }

    // Biểu thức hợp lệ: stack còn đúng 1 phần tử
    if stack.size() == 1 { stack.pop() } else { None }
}
```

Lưu ý thứ tự pop: `b` được pop trước (đĩa trên), `a` pop sau (đĩa dưới). Với phép trừ và chia, thứ tự quan trọng: `10 3 -` là `a - b` = `10 - 3` = 7, không phải `3 - 10`.

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

## Những cái bẫy hay gặp

❌ **Gọi `pop()` mà không kiểm tra stack rỗng**
```rust
let val = stack.pop().unwrap(); // panic nếu stack rỗng!
```
✅ **Luôn xử lý trường hợp `None`**
```rust
match stack.pop() {
    Some(val) => println!("Lấy được: {}", val),
    None => println!("Stack rỗng rồi!"),
}
```
💡 Trong Rust, `pop()` trả về `Option<T>`. Dùng `unwrap()` trên stack rỗng = chương trình crash. Dùng `match` hoặc `if let` để an toàn.

---

❌ **Nhầm Stack với Queue**
```rust
// Bạn muốn xử lý theo thứ tự đến trước, nhưng lại dùng stack
stack.push(task_1);  // vào trước
stack.push(task_2);
stack.pop();         // lấy task_2 ra trước ← sai!
```
✅ **LIFO dùng Stack, FIFO dùng Queue**
```rust
// Cần xử lý theo thứ tự đến? Dùng Queue (VecDeque)
queue.push_back(task_1);
queue.push_back(task_2);
queue.pop_front();   // lấy task_1 ra trước ← đúng!
```
💡 Stack = vào sau ra trước (LIFO). Queue = vào trước ra trước (FIFO). Nhớ: stack là chồng đĩa, queue là hàng chờ mua trà sữa.

---

❌ **Dùng `Vec` trực tiếp thay vì Stack**
```rust
let mut data = Vec::new();
data.push(1);
data.insert(0, 99);  // oops, insert ở đầu -- vi phạm LIFO
data[1] = 42;        // oops, truy cập index -- vi phạm LIFO
```
✅ **Dùng Stack để tự giới hạn bản thân**
```rust
let mut stack = Stack::new();
stack.push(1);
// stack.insert(0, 99);  // không compile! Stack không có insert
// stack[1] = 42;        // không compile! Stack không có index
```
💡 Stack cố tình không cho bạn làm nhiều thứ. Đó là feature, không phải bug. Ít API = ít khả năng dùng sai.

---

❌ **Nhầm `peek()` với `pop()` trong Rust**
```rust
let top = stack.peek();   // &T -- reference, chỉ nhìn
stack.push(100);          // OK, peek không lấy gì ra
// Nhưng...
let top = stack.pop();    // T -- owned value, đã lấy ra khỏi stack
// phần tử đó biến mất rồi!
```
✅ **Nhớ: `peek()` = mượn nhìn, `pop()` = lấy luôn**
```rust
// Muốn xem không lấy ra:
if let Some(top) = stack.peek() {
    println!("Đỉnh stack: {}", top);
}
// Muốn lấy ra:
if let Some(val) = stack.pop() {
    println!("Đã lấy: {}", val);
}
```
💡 `peek()` trả về `Option<&T>` (reference -- mượn nhìn). `pop()` trả về `Option<T>` (owned -- lấy luôn). Trong Rust, đây là sự khác biệt giữa "nhìn qua cửa sổ" và "mở cửa bước ra".

## Khi nào dùng / không nên dùng

| Tình huống | Stack? | Thay bằng gì? |
|------------|--------|---------------|
| Cần undo/redo | ✅ | -- |
| Cần process theo thứ tự đến | ❌ | Queue (`VecDeque`) |
| Cần truy cập phần tử bất kỳ | ❌ | `Vec` / Array |
| Function call management | ✅ | -- |
| BFS (duyệt đồ thị theo chiều rộng) | ❌ | Queue |
| DFS (duyệt đồ thị theo chiều sâu) | ✅ | -- |
| Kiểm tra ngoặc / cú pháp | ✅ | -- |
| Tính biểu thức hậu tố | ✅ | -- |

**Quy tắc ngón tay cái:** Nếu bạn cần xử lý thứ **gần nhất trước** → Stack. Nếu cần xử lý thứ **lâu nhất trước** → Queue.

## Luyện nhận diện Pattern

**Bài 1:** Bạn có một mảng số nguyên. Với mỗi phần tử, tìm **số lớn hơn gần nhất ở phía bên phải** (Next Greater Element). Ví dụ: `[4, 5, 2, 10]` → `[5, 10, 10, -1]`.

*Gợi ý:* Duyệt từ phải sang trái. Có cần "nhớ" những số đã thấy theo thứ tự không?

<details>
<summary>Đáp án</summary>

**Có dùng Stack không?** Có.

**Tại sao:** Duyệt từ phải sang trái, duy trì một stack chứa các "ứng viên" cho next greater element. Với mỗi phần tử, pop hết các số nhỏ hơn hoặc bằng nó (vì chúng không bao giờ là next greater cho ai nữa). Số ở đỉnh stack chính là next greater element.

**Complexity:** Time O(n), Space O(n). Mỗi phần tử được push/pop tối đa 1 lần.
</details>

---

**Bài 2:** Implement **MinStack** -- một stack mà ngoài push/pop/peek thông thường, còn hỗ trợ `get_min()` trả về giá trị nhỏ nhất trong stack, tất cả đều O(1).

*Gợi ý:* Dùng 1 stack hay 2 stack?

<details>
<summary>Đáp án</summary>

**Có dùng Stack không?** Có -- dùng 2 stack.

**Tại sao:** Stack chính chứa dữ liệu. Stack phụ chứa giá trị min tương ứng tại mỗi thời điểm. Khi push, so sánh giá trị mới với đỉnh stack phụ để quyết định min mới. Khi pop, pop cả hai stack. `get_min()` chỉ cần peek stack phụ.

**Complexity:** Time O(1) cho mọi thao tác, Space O(n).
</details>

---

**Bài 3:** Duyệt cây nhị phân theo thứ tự **inorder** (trái → gốc → phải) mà **không dùng đệ quy**.

*Gợi ý:* Đệ quy ngầm dùng call stack. Bạn có thể tự quản lý stack thay cho nó không?

<details>
<summary>Đáp án</summary>

**Có dùng Stack không?** Có.

**Tại sao:** Đệ quy sử dụng call stack ngầm. Khi viết iterative, bạn dùng stack tường minh để mô phỏng đúng hành vi đó: đi trái hết cỡ (push), xử lý node (pop), rồi rẽ phải. Stack nhớ các node cha mà bạn cần quay lại -- đúng kiểu LIFO.

**Complexity:** Time O(n) -- duyệt mỗi node đúng 1 lần. Space O(h) -- h là chiều cao cây.
</details>

---

---

[← Doubly Linked List](./02-doubly-linked-list.md) | [Queue →](./04-queue.md)
