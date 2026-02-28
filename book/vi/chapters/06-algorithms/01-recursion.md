# Recursion

> **Bạn đang lo lắng?** Đừng. Recursion nghe thì ma thuật, nhưng thực ra chỉ cần hiểu đúng 2 thứ: **base case** (khi nào dừng) và **recursive case** (gọi lại chính mình với bài toán nhỏ hơn). Nếu bạn hiểu được vòng lặp `for`, bạn hiểu được recursion. Chỉ khác ở chỗ: thay vì lặp, hàm tự gọi chính nó. Vậy thôi. Đọc xong chương này bạn sẽ thấy nó không đáng sợ như mọi người hay nói.

---

## Đây là gì?

Tưởng tượng một con búp bê Matryoshka (búp bê Nga lồng nhau). Bạn mở con búp bê lớn ra, bên trong có con búp bê nhỏ hơn. Mở tiếp, lại có con nhỏ hơn nữa. Cứ thế cho đến khi gặp **con búp bê nhỏ nhất** — không mở được nữa.

Đây chính là **Recursion** (đệ quy) — kỹ thuật mà một hàm **gọi chính nó** để giải bài toán nhỏ hơn. Mỗi lời giải đệ quy đều có:

1. **Base case** (trường hợp cơ sở) — con búp bê nhỏ nhất, giải được ngay không cần gọi tiếp
2. **Recursive case** (trường hợp đệ quy) — mở búp bê ra, gọi lại chính mình với bài toán nhỏ hơn

Đệ quy là nền tảng của Divide and Conquer, Backtracking, và nhiều thuật toán trên cây/đồ thị. Hiểu đệ quy trước khi học những chủ đề đó là rất quan trọng.

### Cầu nối: Stack chính là Call Stack

Nhớ **Stack** ở [chương 2](../02-linear-structures/03-stack.md) không? Chồng đĩa, push lên, pop ra, LIFO. Hóa ra, **call stack của máy tính cũng hoạt động y hệt vậy:**

- Mỗi lần hàm gọi chính nó = **push** một stack frame mới lên call stack
- Mỗi lần hàm return = **pop** stack frame đó ra

Bạn đã học Stack là cấu trúc dữ liệu. Giờ bạn thấy nó đang chạy ngầm bên dưới mỗi lời gọi đệ quy. Hiểu Stack → hiểu recursion dễ hơn rất nhiều.

```
factorial(4) gọi factorial(3) gọi factorial(2) gọi factorial(1):

    Call Stack (y hệt chồng đĩa ở chương Stack!):
    ┌────────────────┐
    │ factorial(1)   │ ← đỉnh (base case, sắp return = pop)
    │ factorial(2)   │
    │ factorial(3)   │
    │ factorial(4)   │ ← đáy (lời gọi ban đầu)
    └────────────────┘

    Mỗi return = pop một frame ra, trả kết quả cho frame bên dưới.
```

---

## Hoạt động như thế nào?

### Minh họa Call Stack

Khi hàm gọi chính nó, mỗi lần gọi tạo ra một **stack frame** riêng trên call stack. Các frame được gỡ ra khi base case trả về.

```
factorial(4)
  -> 4 * factorial(3)
       -> 3 * factorial(2)
            -> 2 * factorial(1)
                 -> return 1          (base case — búp bê nhỏ nhất!)
            -> return 2 * 1 = 2
       -> return 3 * 2 = 6
  -> return 4 * 6 = 24

Call stack tại điểm sâu nhất:

  | factorial(1) |  <-- đỉnh (base case, sắp return)
  | factorial(2) |
  | factorial(3) |
  | factorial(4) |  <-- đáy (lời gọi ban đầu)
  +==============+
```

Giống búp bê Matryoshka: mở ra 4 lớp, rồi đóng lại từ trong ra ngoài.

### Xác định Base Case

Base case phải:
- **Đạt được** — mỗi lần gọi đệ quy phải tiến gần hơn đến base case
- **Không gọi tiếp** — dừng đệ quy tại đây
- **Trả về giá trị có nghĩa** — không phải giá trị rác

| Bài toán | Base case | Recursive case |
|---------|-----------|---------------|
| Factorial | n <= 1 -> 1 | n * factorial(n-1) |
| Fibonacci | n == 0 -> 0, n == 1 -> 1 | fib(n-1) + fib(n-2) |
| Binary Search | lo >= hi -> không tìm thấy | tìm nửa trái hoặc phải |
| Duyệt cây | node là None -> return | duyệt trái, gốc, phải |
| Merge Sort | len <= 1 -> return | sort 2 nửa, gộp |

> **Quên base case = thảm họa!** Không có base case, hàm sẽ gọi chính nó mãi mãi cho đến khi **stack overflow** (tràn stack). Giống mở búp bê mà không bao giờ gặp con nhỏ nhất.

### Tower of Hanoi — Tháp Hà Nội

Di chuyển n đĩa từ cọc A sang cọc C, dùng cọc B làm trung gian. Quy tắc: mỗi lần chỉ di chuyển 1 đĩa, không bao giờ đặt đĩa lớn lên đĩa nhỏ.

**Insight đệ quy:** Để di chuyển n đĩa từ A sang C:
1. Di chuyển n-1 đĩa trên cùng từ A sang B (dùng C làm trung gian)
2. Di chuyển đĩa lớn nhất từ A sang C
3. Di chuyển n-1 đĩa từ B sang C (dùng A làm trung gian)

```
n=3 đĩa:  A=[3,2,1]  B=[]  C=[]

Bước 1: Di chuyển đĩa 1: A -> C     A=[3,2]    B=[]      C=[1]
Bước 2: Di chuyển đĩa 2: A -> B     A=[3]      B=[2]     C=[1]
Bước 3: Di chuyển đĩa 1: C -> B     A=[3]      B=[2,1]   C=[]
Bước 4: Di chuyển đĩa 3: A -> C     A=[]       B=[2,1]   C=[3]
Bước 5: Di chuyển đĩa 1: B -> A     A=[1]      B=[2]     C=[3]
Bước 6: Di chuyển đĩa 2: B -> C     A=[1]      B=[]      C=[3,2]
Bước 7: Di chuyển đĩa 1: A -> C     A=[]       B=[]      C=[3,2,1]

Tổng số bước = 2^n - 1 = 7
```

**Cây đệ quy của Hanoi(3, A, C, B):**

```
hanoi(3, A, C, B)
  |-- hanoi(2, A, B, C)
  |     |-- hanoi(1, A, C, B)   ->  di chuyển đĩa 1: A->C
  |     |-- di chuyển đĩa 2: A->B
  |     |-- hanoi(1, C, B, A)   ->  di chuyển đĩa 1: C->B
  |-- di chuyển đĩa 3: A->C
  |-- hanoi(2, B, C, A)
        |-- hanoi(1, B, A, C)   ->  di chuyển đĩa 1: B->A
        |-- di chuyển đĩa 2: B->C
        |-- hanoi(1, A, C, B)   ->  di chuyển đĩa 1: A->C
```

---

## Recursion vs Iteration — Khi nào dùng cái nào?

Đây là câu hỏi mà mọi người mới đều thắc mắc. Không phải lúc nào recursion cũng tốt, cũng không phải lúc nào iteration cũng hay hơn.

| Khía cạnh | Recursion | Iteration (vòng lặp) |
|-----------|-----------|----------------------|
| **Đọc code** | Thường ngắn gọn, thanh lịch hơn | Có thể dài dòng, nhưng rõ ràng |
| **Stack** | O(depth) stack frame — tốn bộ nhớ | O(1) thường — tiết kiệm |
| **Hiệu suất** | Overhead gọi hàm mỗi lần | Thường nhanh hơn |
| **Tail call** | Rust KHÔNG đảm bảo TCO | Dùng `loop` thay thế |
| **Debug** | Stack trace dài, khó theo dõi | Dễ đặt breakpoint hơn |

### Khi nào recursion tốt hơn?

| Bài toán | Tại sao recursion tự nhiên hơn |
|----------|-------------------------------|
| **Duyệt cây** (inorder, preorder, postorder) | Cây có cấu trúc đệ quy tự nhiên — mỗi node là gốc của cây con |
| **Divide & Conquer** (merge sort, quick sort) | Chia bài toán → giải phần nhỏ → gộp lại. Recursion mô tả chính xác quá trình này |
| **Backtracking** (N-Queens, Sudoku) | Thử → sai → quay lui. Recursion tự động "nhớ" trạng thái trước đó qua call stack |
| **Đồ thị: DFS** | Đi sâu hết cỡ rồi quay lại — bản chất là LIFO, recursion rất tự nhiên |

### Khi nào iteration tốt hơn?

| Bài toán | Tại sao iteration hợp lý hơn |
|----------|-------------------------------|
| **Vòng lặp đơn giản** (tính tổng, duyệt mảng) | Không cần "nhớ" trạng thái → loop đơn giản hơn |
| **Fibonacci, Factorial** | Recursion gây overhead không cần thiết. Loop nhanh hơn nhiều |
| **Khi depth có thể rất lớn** | 100K recursive calls → stack overflow. Loop thì không |
| **Performance-critical code** | Mỗi function call có overhead (push/pop stack frame) |

**Quy tắc ngón tay cái:** Nếu bài toán có **cấu trúc đệ quy tự nhiên** (cây, chia để trị, backtracking) → dùng recursion. Nếu chỉ là **lặp tuần tự** → dùng loop.

Trong Rust, nên **ưu tiên vòng lặp** khi độ sâu đệ quy có thể lớn (tránh stack overflow). Nhiều thuật toán đệ quy có thể chuyển sang dùng stack tường minh — đúng như bạn đã thấy ở [bài Stack chương 2](../02-linear-structures/03-stack.md) (bài 3: duyệt cây không dùng đệ quy).

---

## Nguy hiểm: Stack Overflow

Đây là cái bẫy lớn nhất của recursion mà bạn **phải** biết.

### Chuyện gì xảy ra khi recursion quá sâu?

Mỗi lần gọi đệ quy, Rust push một stack frame lên call stack. Stack mặc định của Rust chỉ có khoảng **8 MB**. Mỗi stack frame thường tốn vài chục đến vài trăm bytes. Làm phép tính nhanh:

```
Stack mặc định:  ~8 MB = 8,388,608 bytes
Stack frame trung bình: ~64-256 bytes
Số lần gọi tối đa: ~30,000 - 130,000 lần

→ factorial(100_000) hoặc fib_recursive(100_000) sẽ CRASH!
```

```
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

### Tail Recursion KHÔNG được optimize trong Rust

Trong một số ngôn ngữ (Haskell, Scheme), compiler có thể biến tail recursion thành loop tự động (Tail Call Optimization — TCO). **Rust KHÔNG đảm bảo điều này.** Nghĩa là ngay cả code dạng tail-recursive vẫn có thể stack overflow:

```rust
// Dạng tail-recursive — nhưng Rust vẫn KHÔNG optimize!
fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n <= 1 { return acc; }
    factorial_tail(n - 1, n * acc)  // tail position, nhưng Rust vẫn push stack frame
}

// Giải pháp: chuyển sang loop
fn factorial_loop(n: u64) -> u64 {
    let mut result = 1u64;
    for i in 2..=n {
        result *= i;
    }
    result
}
```

### Giải pháp khi cần recursion sâu

1. **Chuyển sang loop** — luôn là lựa chọn an toàn nhất
2. **Dùng stack tường minh** — thay call stack bằng `Vec<T>` do bạn quản lý (xem [Stack chương 2](../02-linear-structures/03-stack.md))
3. **Tăng kích thước stack** — dùng `std::thread::Builder::new().stack_size(64 * 1024 * 1024)` để tạo thread với stack 64 MB
4. **Crate `stacker`** — tự động mở rộng stack khi cần (xem phần Rust Ecosystem bên dưới)

---

## Memoization — Từ Recursion đến DP

Nhìn cây đệ quy của Fibonacci:

```
fib(5)
├── fib(4)
│   ├── fib(3)
│   │   ├── fib(2)  ← tính lần 1
│   │   └── fib(1)
│   └── fib(2)      ← tính lần 2 (TRÙNG!)
└── fib(3)           ← tính lần 2 (TRÙNG!)
    ├── fib(2)       ← tính lần 3 (TRÙNG!!)
    └── fib(1)
```

`fib(2)` bị tính **3 lần**, `fib(3)` bị tính **2 lần**. Với `fib(50)`, số lần tính lặp lại là hàng tỷ. Đây là lý do `fib_recursive` có O(2^n) — cực chậm.

**Giải pháp: Memoization** — nhớ kết quả đã tính, không tính lại.

```rust
use std::collections::HashMap;

fn fib_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&val) = cache.get(&n) {
        return val;  // đã tính rồi, trả về luôn!
    }
    let result = fib_memo(n - 1, cache) + fib_memo(n - 2, cache);
    cache.insert(n, result);  // nhớ lại cho lần sau
    result
}

// Dùng:
let mut cache = HashMap::new();
assert_eq!(fib_memo(10, &mut cache), 55);
// fib_memo(50, &mut cache) chạy được! (fib_recursive(50) thì treo máy)
```

**Recursion + Memoization = Dynamic Programming (DP) dạng top-down.** Đây chính là cầu nối từ chương này sang [chương Dynamic Programming](13-dynamic-programming.md). Khi đến chương DP, bạn sẽ thấy memoization là một trong hai cách tiếp cận chính (top-down memo vs bottom-up tabulation).

---

## Code Rust

Đệ quy là kỹ thuật, không phải một thuật toán cụ thể. Các implementation trong sách này sử dụng đệ quy bao gồm:

- **Merge Sort** — `sorting::merge_sort` (chia, đệ quy, gộp)
- **Quick Sort** — `sorting::quick_sort` (partition, đệ quy)
- **Binary Search** — `searching::binary_search` (bản iterative, nhưng bản chất là đệ quy)
- **N-Queens** — `backtracking::n_queens` (đặt từng hàng, quay lui)
- **Sudoku** — `backtracking::solve_sudoku` (thử số, đệ quy, quay lui)

Đây là ví dụ riêng — Tower of Hanoi:

```rust
/// Tower of Hanoi — di chuyển n đĩa từ cọc `from` sang cọc `to`.
fn hanoi(n: u32, from: &str, to: &str, aux: &str) {
    if n == 0 {
        return;  // Base case: không có đĩa nào để di chuyển
    }
    // Bước 1: di chuyển n-1 đĩa trên cùng sang cọc phụ
    hanoi(n - 1, from, aux, to);
    // Bước 2: di chuyển đĩa lớn nhất sang đích
    println!("Di chuyển đĩa {n} từ {from} sang {to}");
    // Bước 3: di chuyển n-1 đĩa từ cọc phụ sang đích
    hanoi(n - 1, aux, to, from);
}

// hanoi(3, "A", "C", "B") in ra 7 bước.
```

Và Fibonacci (so sánh đệ quy thường vs vòng lặp):

```rust
// Đệ quy thường: O(2^n) thời gian, O(n) stack — CỰC CHẬM với n lớn!
fn fib_recursive(n: u64) -> u64 {
    if n <= 1 { return n; }         // Base case
    fib_recursive(n - 1) + fib_recursive(n - 2)  // 2 lời gọi đệ quy
}

// Vòng lặp (tabulation): O(n) thời gian, O(1) bộ nhớ — NÊN DÙNG.
fn fib_iterative(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
```

**Ghi chú về Rust:**

- Rust **không đảm bảo** tail call optimization (TCO). Nghĩa là đệ quy đuôi (tail recursion) vẫn có thể gây stack overflow. Nếu cần, hãy chuyển sang vòng lặp.
- Stack mặc định của Rust là khoảng 8 MB. Với đệ quy sâu (ví dụ: `factorial(1_000_000)`), sẽ bị crash. Giải pháp: dùng vòng lặp hoặc tăng kích thước stack.
- `println!("Di chuyển đĩa {n}")` dùng cú pháp format string mới của Rust — gọn hơn `println!("Di chuyển đĩa {}", n)`.

---

## Độ phức tạp

| Bài toán | Thời gian | Bộ nhớ (stack) |
|---------|----------|---------------|
| Factorial | O(n) | O(n) |
| Fibonacci (đệ quy thường) | O(2^n) | O(n) |
| Fibonacci (memoization) | O(n) | O(n) |
| Fibonacci (tabulation/loop) | O(n) | O(1) |
| Tower of Hanoi | O(2^n) | O(n) |
| Merge Sort | O(n log n) | O(n) |
| Binary Search | O(log n) | O(log n) hoặc O(1) iterative |

**Giải thích thực tế:**

- Fibonacci đệ quy thường là ví dụ kinh điển về "đệ quy tệ" — tính lại cùng một giá trị hàng triệu lần. Giải pháp: dùng DP (memoization hoặc tabulation).
- Tower of Hanoi có O(2^n) là **không thể tối ưu hơn** — đây là số bước tối thiểu bắt buộc.
- Mỗi lời gọi đệ quy tốn bộ nhớ stack. 1 triệu lời gọi lồng nhau sẽ crash. Luôn cân nhắc độ sâu đệ quy trước khi viết.

---

## Ví dụ

```rust
use rust_ds2a::dynamic_programming::fibonacci;
use rust_ds2a::sorting::merge_sort;
use rust_ds2a::backtracking::permutations;

// Fibonacci qua tabulation (đệ quy biến thành vòng lặp)
assert_eq!(fibonacci(10), 55);

// Merge Sort dùng đệ quy bên trong
let mut v = vec![5, 1, 4, 2, 8];
merge_sort(&mut v);
assert_eq!(v, vec![1, 2, 4, 5, 8]);

// Hoán vị dùng đệ quy + backtracking
let perms = permutations(&['a', 'b', 'c']);
assert_eq!(perms.len(), 6);
```

---

## Những cái bẫy hay gặp

### Bẫy 1: Quên base case — Infinite Recursion

❌ **Sai:**
```rust
fn countdown(n: i32) {
    println!("{}", n);
    countdown(n - 1);  // không bao giờ dừng!
}
```
✅ **Đúng:**
```rust
fn countdown(n: i32) {
    if n < 0 { return; }  // base case!
    println!("{}", n);
    countdown(n - 1);
}
```
💡 **Tại sao:** Không có base case = hàm gọi chính nó mãi mãi. Giống mở búp bê Matryoshka mà không bao giờ gặp con nhỏ nhất. Kết quả: stack overflow, chương trình crash. **Luôn viết base case TRƯỚC khi viết recursive case.**

---

### Bẫy 2: Return value sai — Quên trả kết quả đệ quy

❌ **Sai:**
```rust
fn sum(n: u64) -> u64 {
    if n == 0 { return 0; }
    sum(n - 1);  // gọi đệ quy nhưng KHÔNG trả về kết quả!
    n            // trả về n thay vì n + sum(n-1)
}
```
✅ **Đúng:**
```rust
fn sum(n: u64) -> u64 {
    if n == 0 { return 0; }
    n + sum(n - 1)  // PHẢI cộng kết quả đệ quy vào
}
```
💡 **Tại sao:** Recursion hoạt động bằng cách **kết hợp** kết quả của bài toán con. Nếu bạn gọi đệ quy mà không dùng kết quả trả về, thì coi như gọi vô ích. Trong Rust, compiler sẽ cảnh báo "unused result" — hãy chú ý warning này.

---

### Bẫy 3: Stack Overflow — Đệ quy quá sâu

❌ **Sai:**
```rust
fn factorial(n: u64) -> u64 {
    if n <= 1 { return 1; }
    n * factorial(n - 1)
}
// factorial(1_000_000) → CRASH! Stack overflow.
```
✅ **Đúng:**
```rust
fn factorial(n: u64) -> u64 {
    let mut result = 1u64;
    for i in 2..=n {
        result = result.saturating_mul(i);  // tránh overflow số
    }
    result
}
```
💡 **Tại sao:** Stack mặc định 8 MB. ~100K recursive calls = tràn. Với bài toán có depth lớn (n > 10,000), **luôn dùng loop**. Chỉ dùng recursion khi depth nhỏ và có bound rõ ràng (ví dụ: cây cân bằng có depth O(log n) — 1 triệu node chỉ sâu ~20 tầng).

---

### Bẫy 4: Tính lại giá trị trùng lặp

❌ **Sai:**
```rust
fn fib(n: u64) -> u64 {
    if n <= 1 { return n; }
    fib(n - 1) + fib(n - 2)  // fib(40) tốn ~1 tỷ phép tính!
}
```
✅ **Đúng:**
```rust
fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&v) = memo.get(&n) { return v; }
    let result = fib(n - 1, memo) + fib(n - 2, memo);
    memo.insert(n, result);
    result
}
```
💡 **Tại sao:** Khi recursion tạo ra **overlapping subproblems** (bài toán con trùng lặp), bạn đang tính cùng một thứ nhiều lần. Thêm memoization biến O(2^n) thành O(n). Đây chính là ý tưởng cốt lõi của [Dynamic Programming](13-dynamic-programming.md).

---

## Khi nào dùng / không nên dùng

| Tình huống | Recursion? | Thay bằng gì? | Tại sao |
|------------|-----------|---------------|---------|
| Duyệt cây (inorder, preorder...) | ✅ Dùng | — | Cây có cấu trúc đệ quy tự nhiên |
| Divide & Conquer (merge sort, quick sort) | ✅ Dùng | — | Chia → giải → gộp = đệ quy thuần túy |
| Backtracking (N-Queens, Sudoku) | ✅ Dùng | — | Call stack tự nhớ trạng thái, quay lui dễ |
| DFS trên đồ thị | ✅ / ⚠️ | Stack tường minh nếu graph lớn | Recursion đẹp, nhưng graph 100K node → overflow |
| Tính tổng, duyệt mảng | ❌ Không | `for` loop | Không cần "nhớ" trạng thái → loop đơn giản hơn |
| Fibonacci, Factorial | ❌ Không | Loop hoặc DP | Recursion thường gây overhead không cần thiết |
| Depth có thể > 10,000 | ❌ Không | Loop + stack tường minh | Stack overflow risk quá cao |
| Performance-critical hot path | ❌ Không | Loop | Mỗi function call có overhead push/pop |

**Quy tắc ngón tay cái:** Hỏi 2 câu:
1. Bài toán có **cấu trúc đệ quy tự nhiên** không? (cây, chia để trị, quay lui)
2. Độ sâu đệ quy có **bounded và nhỏ** không? (< 10,000, hoặc O(log n))

Cả hai "có" → dùng recursion. Một trong hai "không" → cân nhắc loop.

---

## Rust Ecosystem

### Ownership và Recursion

Rust có ownership system nghiêm ngặt, và điều này ảnh hưởng đến cách bạn viết recursion:

```rust
// Đệ quy trên linked list — cần pattern matching trên Option<Box<Node>>
fn list_length(node: &Option<Box<ListNode>>) -> usize {
    match node {
        None => 0,                              // base case
        Some(n) => 1 + list_length(&n.next),    // borrow recursive
    }
}
```

Vì Rust không có garbage collector, mỗi recursive call phải rõ ràng về ownership: bạn đang **borrow** (`&`) hay **move**? Hầu hết recursion trên cấu trúc dữ liệu nên dùng `&` (borrow) để tránh move ownership.

### Crates hữu ích

- **`stacker`** — Tự động mở rộng stack khi cần. Hữu ích khi bạn không kiểm soát được depth (ví dụ: parse JSON lồng nhau rất sâu). Dùng `stacker::maybe_grow()` để wrap recursive calls.
- **`rayon`** — Parallel recursion. Divide & Conquer trên nhiều core. `rayon::join(|| sort_left(), || sort_right())` chạy 2 nhánh đệ quy song song.
- **`tracing`** — Debug recursive functions bằng cách trace mỗi lần gọi, xem depth, arguments, return value.

---

## Luyện tập

### Bài 1: Climbing Stairs (LeetCode #70)

Bạn leo cầu thang có `n` bậc. Mỗi bước leo được 1 hoặc 2 bậc. Có bao nhiêu cách leo lên đỉnh?

```
n = 3 → 3 cách: [1+1+1], [1+2], [2+1]
n = 4 → 5 cách
n = 5 → 8 cách
```

*Gợi ý:* Nhìn kỹ dãy 1, 2, 3, 5, 8... Quen không? Đây chính là Fibonacci! `ways(n) = ways(n-1) + ways(n-2)`. Dùng memoization hoặc loop.

<details>
<summary>Hướng dẫn</summary>

```rust
fn climb_stairs(n: u32) -> u32 {
    if n <= 2 { return n; }
    let (mut a, mut b) = (1u32, 2u32);
    for _ in 3..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
assert_eq!(climb_stairs(3), 3);
assert_eq!(climb_stairs(5), 8);
```

**Tại sao loop?** Vì depth = n, có thể rất lớn. Recursion thuần sẽ O(2^n) hoặc stack overflow. Loop cho O(n) time, O(1) space.
</details>

---

### Bài 2: Power of Two (LeetCode #231)

Kiểm tra xem số `n` có phải lũy thừa của 2 không. (1, 2, 4, 8, 16, 32...)

```
n = 16 → true  (2^4)
n = 18 → false
n = 1  → true  (2^0)
```

*Gợi ý:* Base case: `n == 1` → true. Recursive case: `n` chẵn và `is_power_of_two(n / 2)`.

<details>
<summary>Hướng dẫn</summary>

```rust
// Bản recursion (để luyện tập)
fn is_power_of_two(n: i32) -> bool {
    if n <= 0 { return false; }
    if n == 1 { return true; }         // base case: 2^0 = 1
    n % 2 == 0 && is_power_of_two(n / 2)  // chia đôi, kiểm tra tiếp
}

// Bản bit trick (thực tế dùng cái này — xem chương Bit Manipulation)
fn is_power_of_two_fast(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}
```

Depth tối đa = log2(n) = 31 cho i32 → recursion an toàn. Nhưng bản bit trick O(1) là đẹp hơn nhiều — bạn sẽ hiểu khi đến [chương Bit Manipulation](../07-patterns/01-bit-manipulation.md).
</details>

---

### Bài 3: Reverse Linked List Recursively (LeetCode #206)

Đảo ngược một singly linked list bằng đệ quy.

```
1 -> 2 -> 3 -> 4 -> None
                ↓
4 -> 3 -> 2 -> 1 -> None
```

*Gợi ý:* Base case: list rỗng hoặc chỉ có 1 node. Recursive case: đảo ngược phần còn lại, rồi nối node hiện tại vào cuối.

<details>
<summary>Hướng dẫn</summary>

Ý tưởng: đệ quy đến cuối list, rồi "nối ngược" khi unwind:

```
Đi sâu (push):  1 → 2 → 3 → 4 (base case: 4 là cuối)
Quay lại (pop): 4→3, 3→2, 2→1, 1→None

Tại mỗi bước unwind:
  node.next.next = node   (node tiếp theo trỏ ngược lại mình)
  node.next = None         (cắt link cũ)
```

Trong Rust, việc này khó hơn vì ownership — bạn cần dùng `Option<Box<ListNode>>` và `take()` để move ownership an toàn. Đây là bài tập tuyệt vời để hiểu ownership + recursion. Xem thêm ở [chương Singly Linked List](../02-linear-structures/01-singly-linked-list.md).

**Lưu ý:** Depth = n (chiều dài list). Nếu list có 100K nodes → stack overflow. Trong thực tế, nên dùng bản iterative cho linked list reversal.
</details>

---

## Tổng kết nhanh

```
Recursion = hàm gọi chính nó
          = base case + recursive case
          = push lên call stack + pop khi return

Khi nào dùng:  cây, chia để trị, backtracking, DFS
Khi nào không:  depth lớn, loop đơn giản, cần hiệu suất

Recursion + cache  = Memoization = DP top-down
Recursion + stack  = có thể chuyển thành iteration
Rust + recursion   = cẩn thận stack 8MB, không có TCO
```

---

---

[← Union-Find](../05-graphs/10-union-find.md) | [Basic Sorting →](./02-basic-sorting.md)
