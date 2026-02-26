# Binary Heap

## Đây là gì?

Hình dung **bảng xếp hạng** (leaderboard) trong game. Người có điểm cao nhất luôn đứng đầu. Khi có người mới vào, họ được xếp vào đúng vị trí. Khi người đứng đầu bị loại, người có điểm cao tiếp theo tự động lên thay.

**Binary heap** (đống nhị phân) hoạt động giống vậy. Đó là cây nhị phân hoàn chỉnh (complete binary tree), nhưng có quy tắc đặc biệt:

- **Max-heap**: cha luôn >= con. Node lớn nhất ở trên cùng (root).
- **Min-heap**: cha luôn <= con. Node nhỏ nhất ở trên cùng.

Điểm hay: cây này được lưu trong **mảng** (array), không cần pointer! Rất tiết kiệm bộ nhớ và thân thiện với cache.

**Dùng ở đâu?**
- **Priority queue**: phục vụ theo độ ưu tiên (chương sau)
- **Heap sort**: thuật toán sắp xếp
- **Top-K**: tìm K phần tử lớn nhất/nhỏ nhất

## Hoạt động như thế nào?

### Lưu cây trong mảng

Đây là "phép thuật" của heap. Một cây hoàn chỉnh ánh xạ vào mảng mà không lãng phí ô nào:

```
Mảng:  [50, 30, 40, 10, 20, 35, 25]
Index:   0   1   2   3   4   5   6

Cây tương ứng:
           50          index 0 (root)
          /  \
        30    40       index 1, 2
       / \   / \
     10  20 35  25     index 3, 4, 5, 6
```

Công thức chuyển đổi (rất quan trọng!):

```
Cha của node i:        (i - 1) / 2
Con trái của node i:    2 * i + 1
Con phải của node i:    2 * i + 2

Ví dụ: node index 1 (giá trị 30)
  - Cha: (1-1)/2 = 0 → node 50  ✓
  - Con trái: 2*1+1 = 3 → node 10  ✓
  - Con phải: 2*1+2 = 4 → node 20  ✓
```

Không cần pointer, chỉ cần phép tính đơn giản!

### Sift-up (đẩy lên) -- sau khi chèn

Khi chèn phần tử mới, đặt nó ở cuối mảng rồi "đẩy lên" cho đến khi đúng vị trí. Giống học sinh mới vào lớp, so điểm với người đứng trước, nếu cao hơn thì đổi chỗ.

```
Chèn 60 vào max-heap [50, 30, 40, 10, 20, 35, 25]:

[50, 30, 40, 10, 20, 35, 25, 60]
                                ^  60 ở cuối

60 > 10 (cha)? Có → đổi chỗ
[50, 30, 40, 60, 20, 35, 25, 10]
              ^

60 > 30 (cha)? Có → đổi chỗ
[50, 60, 40, 30, 20, 35, 25, 10]
      ^

60 > 50 (cha)? Có → đổi chỗ
[60, 50, 40, 30, 20, 35, 25, 10]
  ^
Done! 60 bây giờ là root (lớn nhất).
```

### Sift-down (đẩy xuống) -- sau khi xóa root

Khi lấy root ra (phần tử lớn nhất), ta đưa phần tử cuối lên thế chỗ rồi "đẩy xuống". So với 2 con, đổi chỗ với con lớn hơn.

```
Lấy 60 ra khỏi heap:

Bước 1: Đổi root với phần tử cuối rồi xóa
[10, 50, 40, 30, 20, 35, 25]  (60 đã ra)
  ^
10 ở root, nhưng 10 < con → phải đẩy xuống

Bước 2: 10 < max(50, 40) = 50 → đổi chỗ với 50
[50, 10, 40, 30, 20, 35, 25]
      ^

Bước 3: 10 < max(30, 20) = 30 → đổi chỗ với 30
[50, 30, 40, 10, 20, 35, 25]
              ^
Không còn con → done!
```

### Heapify -- xây heap từ mảng trong O(n)

Chèn từng phần tử: O(n log n). Nhưng có cách nhanh hơn!

Bắt đầu từ node cuối cùng có con (non-leaf), sift-down từng node từ dưới lên:

```
Input: [4, 1, 7, 3, 8, 2, 5]

           4
          / \
         1   7
        / \ / \
       3  8 2  5

Sift-down từ index 2 (node 7): 7 > cả 2 con → OK
Sift-down từ index 1 (node 1): đổi với 8
  [4, 8, 7, 3, 1, 2, 5]
Sift-down từ index 0 (node 4): đổi với 8
  [8, 4, 7, 3, 1, 2, 5]
  4 còn con 3, 1 → 4 > cả hai → OK

Kết quả: [8, 4, 7, 3, 1, 2, 5] -- max-heap hợp lệ!
```

Tại sao O(n) mà không phải O(n log n)? Vì phần lớn node ở gần leaf, sift-down chỉ 1-2 bước. Phân tích toán học chứng minh tổng là O(n).

## Code Rust

Code đầy đủ nằm trong `src/heap.rs`.

### Cấu trúc dữ liệu

```rust
pub enum HeapType { Max, Min }

pub struct BinaryHeap<T: Ord> {
    data: Vec<T>,            // mảng lưu cây
    heap_type: HeapType,     // max hay min?
}
```

Đơn giản! Chỉ 1 `Vec` và 1 flag.

### Chèn (push)

```rust
pub fn push(&mut self, val: T) {
    self.data.push(val);           // thêm vào cuối mảng
    let last = self.data.len() - 1;
    self.sift_up(last);            // đẩy lên đúng vị trí
}
```

### Lấy ra (pop)

```rust
pub fn pop(&mut self) -> Option<T> {
    if self.data.is_empty() { return None; }
    let last = self.data.len() - 1;
    self.data.swap(0, last);       // đổi root với cuối
    let top = self.data.pop();     // lấy phần tử cuối (root cũ) ra
    if !self.data.is_empty() {
        self.sift_down(0);         // đẩy phần tử mới ở root xuống
    }
    top
}
```

### Xem phần tử đầu (peek)

```rust
pub fn peek(&self) -> Option<&T> {
    self.data.first()  // root luôn ở index 0
}
```

O(1) -- không cần tìm kiếm!

### Heapify -- xây heap O(n)

```rust
pub fn heapify(vec: Vec<T>) -> Self {
    let mut heap = Self { data: vec, heap_type: HeapType::Max };
    let len = heap.data.len();
    for i in (0..len / 2).rev() {   // từ non-leaf cuối cùng, đi ngược lên
        heap.sift_down(i);
    }
    heap
}
```

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa |
|----------|-----------|--------|----------|
| `push` | O(log n) | O(1) | Sift-up tối đa log n tầng |
| `pop` | O(log n) | O(1) | Sift-down tối đa log n tầng |
| `peek` | O(1) | O(1) | Chỉ nhìn root |
| `heapify` | O(n) | O(1) | Nhanh hơn chèn từng cái! |
| `size` / `is_empty` | O(1) | O(1) | Chỉ đọc len |
| **Tổng bộ nhớ** | -- | **O(n)** | 1 mảng, không pointer |

So sánh: mảng sắp xếp cho peek O(1) nhưng push O(n). Heap cho cả push và pop đều O(log n). Đây là sự cân bằng tuyệt vời.

## Ví dụ

### Max-heap

```rust
use rust_ds2a::heap::BinaryHeap;

let mut heap = BinaryHeap::new();
for v in [3, 1, 5, 2, 4] {
    heap.push(v);
}

assert_eq!(heap.peek(), Some(&5));   // lớn nhất luôn ở đầu
assert_eq!(heap.pop(), Some(5));     // lấy ra 5
assert_eq!(heap.pop(), Some(4));     // tiếp theo: 4
assert_eq!(heap.pop(), Some(3));     // tiếp: 3
```

### Min-heap

```rust
use rust_ds2a::heap::{BinaryHeap, HeapType};

let mut heap = BinaryHeap::with_type(HeapType::Min);
for v in [3, 1, 5, 2, 4] {
    heap.push(v);
}

assert_eq!(heap.pop(), Some(1));     // nhỏ nhất ra trước
assert_eq!(heap.pop(), Some(2));
```

### Xây heap từ mảng

```rust
use rust_ds2a::heap::BinaryHeap;

// O(n) thay vì O(n log n)!
let heap = BinaryHeap::heapify(vec![4, 1, 7, 3, 8, 2, 5]);
assert_eq!(heap.peek(), Some(&8));   // 8 là lớn nhất
```
