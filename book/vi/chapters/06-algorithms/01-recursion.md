# Recursion

## Đây là gì?

Tưởng tượng một con búp bê Matryoshka (búp bê Nga lồng nhau). Bạn mở con búp bê lớn ra, bên trong có con búp bê nhỏ hơn. Mở tiếp, lại có con nhỏ hơn nữa. Cứ thế cho đến khi gặp **con búp bê nhỏ nhất** — không mở được nữa.

Đây chính là **Recursion** (đệ quy) — kỹ thuật mà một hàm **gọi chính nó** để giải bài toán nhỏ hơn. Mỗi lời giải đệ quy đều có:

1. **Base case** (trường hợp cơ sở) — con búp bê nhỏ nhất, giải được ngay không cần gọi tiếp
2. **Recursive case** (trường hợp đệ quy) — mở búp bê ra, gọi lại chính mình với bài toán nhỏ hơn

Đệ quy là nền tảng của Divide and Conquer, Backtracking, và nhiều thuật toán trên cây/đồ thị. Hiểu đệ quy trước khi học những chủ đề đó là rất quan trọng.

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

### Đệ quy vs Vòng lặp

| Khía cạnh | Đệ quy | Vòng lặp |
|-----------|--------|----------|
| Dễ đọc | Thường thanh lịch hơn | Có thể dài dòng |
| Stack | O(depth) stack frame | O(1) thường |
| Hiệu suất | Overhead gọi hàm | Thường nhanh hơn |
| Tail call | Rust KHÔNG đảm bảo TCO | Dùng loop thay thế |

Trong Rust, nên **ưu tiên vòng lặp** khi độ sâu đệ quy có thể lớn (tránh stack overflow). Nhiều thuật toán đệ quy có thể chuyển sang dùng stack tường minh.

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
| Fibonacci (DP) | O(n) | O(1) |
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
