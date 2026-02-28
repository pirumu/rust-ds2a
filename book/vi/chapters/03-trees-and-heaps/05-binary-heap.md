# Binary Heap

> 💡 **Đừng lo lắng:** Nếu bạn vừa sống sót qua AVL và Red-Black, tin vui: Heap đơn giản hơn rất nhiều. Không rotation, không tô màu -- chỉ là một mảng `Vec` quen thuộc với 1 quy tắc: cha luôn lớn hơn (hoặc nhỏ hơn) con. Coi chương này như nhịp nghỉ.

## Đây là gì?

Heap không dùng `Box<Node>` hay pointer -- nó lưu trong **Vec**, đúng cái `Vec` quen thuộc từ chương 1. Nếu bạn nhớ "flat representation" ở chương Binary Tree (lưu cây trong mảng), Heap chính là ứng dụng thực tế hoàn hảo của nó.

Hình dung **bảng xếp hạng** (leaderboard) trong game. Người có điểm cao nhất luôn đứng đầu. Khi có người mới vào, họ được xếp vào đúng vị trí. Khi người đứng đầu bị loại, người có điểm cao tiếp theo tự động lên thay.

**Binary heap** (đống nhị phân) hoạt động giống vậy. Đó là cây nhị phân hoàn chỉnh (complete binary tree), nhưng có quy tắc đặc biệt:

- **Max-heap**: cha luôn >= con. Node lớn nhất ở trên cùng (root).
- **Min-heap**: cha luôn <= con. Node nhỏ nhất ở trên cùng.

Điểm hay: cây này được lưu trong **mảng** (array), không cần pointer! Rất tiết kiệm bộ nhớ và thân thiện với cache.

## Heap vs BST/AVL/Red-Black -- khác nhau ở đâu?

3 chương vừa rồi, bạn học BST → AVL → Red-Black. Cả 3 đều duy trì **total order** (trái < gốc < phải). Rất mạnh -- bạn có thể search, sorted traversal, range query -- nhưng tốn công duy trì (rotation, đổi màu).

Bây giờ đặt câu hỏi: **"Nếu bạn chỉ cần lấy phần tử lớn nhất (hoặc nhỏ nhất) mà không cần toàn bộ sorted order thì sao?"** Dùng cả 1 cây AVL chỉ để lấy max là quá "sang". Giống như thuê cả dàn nhạc giao hưởng chỉ để nghe 1 nốt nhạc.

| | BST/AVL/Red-Black | Heap |
|---|-------------------|------|
| Thứ tự | Total order (trái < gốc < phải) | Partial order (cha >= con) |
| Lấy max/min | O(log n) | **O(1)** -- luôn ở root |
| Search bất kỳ | O(log n) | O(n) -- phải duyệt hết |
| Insert | O(log n) | O(log n) |
| Delete max/min | O(log n) | O(log n) |
| Sorted traversal | O(n) in-order | Không hỗ trợ |
| Lưu trữ | Pointer-based (`Box<Node>`) | **Array-based (Vec)** |
| Cache-friendly | Không (pointer chasing) | **Rất tốt** (contiguous) |

Heap biết ít hơn (chỉ biết ai lớn nhất), nhưng duy trì rẻ hơn → nhanh hơn cho bài toán chỉ cần max/min.

Quay lại ẩn dụ game: BST family giống **bảng xếp hạng đầy đủ** -- bạn biết ai hạng 1, hạng 2, ... hạng N. Heap giống **chỉ hiện top 1 trên màn hình** -- bạn biết ai đang dẫn đầu, nhưng không biết thứ tự còn lại. Đổi lại, cập nhật bảng xếp hạng "top 1" rẻ hơn nhiều.

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

### Tại sao Vec tốt hơn Box\<Node\> cho Heap?

Ở các chương trước, BST/AVL/Red-Black đều dùng `Box<Node>` -- mỗi node là 1 allocation riêng, nằm rải rác trên memory. Heap dùng `Vec` -- tất cả nằm liên tiếp:

```
Box<Node> tree:                 Vec heap:
┌──────────┐                    ┌──┬──┬──┬──┬──┬──┬──┐
│ val: 50  │                    │50│30│40│10│20│35│25│
│ left: ─────→ [30]             └──┴──┴──┴──┴──┴──┴──┘
│ right: ────→ [40]              Liên tiếp trên memory
└──────────┘
  Rải rác trên heap memory
```

Vec ưu điểm:
- Cache-friendly (spatial locality) -- CPU load cả block memory vào cache
- 0 pointer overhead (tiết kiệm 16 bytes/node trên 64-bit)
- 0 allocation per node (chỉ 1 Vec duy nhất)
- Truy cập bất kỳ node = O(1) bằng index
- Borrow checker happy (không cần `Box`/`Option` lồng nhau)

Nhớ chương Binary Tree nói về Vec representation? *"Chỉ hiệu quả cho complete binary tree vì không lãng phí slot."* Heap **luôn** là complete binary tree (phần tử được thêm từ trái sang phải, trên xuống dưới) → hoàn hảo cho Vec!

Và nhớ chương Stack/Queue? Chúng cũng dùng Vec. Heap tiếp tục truyền thống "Vec là king" trong Rust.

### Sift-up (đẩy lên) -- sau khi chèn

Khi chèn phần tử mới, đặt nó ở cuối mảng rồi "đẩy lên" cho đến khi đúng vị trí. Giống player mới vào bảng xếp hạng -- bắt đầu từ cuối, rồi "thách đấu" lần lượt người phía trên. Nếu thắng thì đổi chỗ, thua thì dừng.

**Max-heap trace:**

```
Chèn 60 vào max-heap [50, 30, 40, 10, 20, 35, 25]:

[50, 30, 40, 10, 20, 35, 25, 60]
                                ^  60 ở cuối (index 7)

Bước 1: cha = (7-1)/2 = 3 → giá trị 10
  60 > 10? Có → đổi chỗ
  [50, 30, 40, 60, 20, 35, 25, 10]
                ^

Bước 2: cha = (3-1)/2 = 1 → giá trị 30
  60 > 30? Có → đổi chỗ
  [50, 60, 40, 30, 20, 35, 25, 10]
        ^

Bước 3: cha = (1-1)/2 = 0 → giá trị 50
  60 > 50? Có → đổi chỗ
  [60, 50, 40, 30, 20, 35, 25, 10]
    ^

Đã ở root → done! 60 bây giờ là lớn nhất.
```

**Min-heap trace -- chỉ khác dấu so sánh:**

```
Insert 0 vào min-heap [1, 3, 2, 7, 4, 5, 6]:

[1, 3, 2, 7, 4, 5, 6, 0]
                       ^ 0 ở cuối (index 7)

0 < 7 (cha)? Có → swap     [1, 3, 2, 0, 4, 5, 6, 7]
0 < 3 (cha)? Có → swap     [1, 0, 2, 3, 4, 5, 6, 7]
0 < 1 (cha)? Có → swap     [0, 1, 2, 3, 4, 5, 6, 7]

Done! 0 là nhỏ nhất, lên root.
```

```
Max-heap: swap khi con > cha    (con mạnh hơn → đẩy cha xuống)
Min-heap: swap khi con < cha    (con nhỏ hơn → đẩy cha xuống)
Chỉ khác DẤU SO SÁNH. Mọi thứ khác giống hệt.
```

### Sift-down (đẩy xuống) -- sau khi xóa root

Khi lấy root ra (phần tử lớn nhất/nhỏ nhất), ta đưa phần tử cuối lên thế chỗ rồi "đẩy xuống". So với 2 con, đổi chỗ với con lớn hơn (max-heap) hoặc nhỏ hơn (min-heap).

```
Lấy 60 ra khỏi max-heap:

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

#### Tại sao swap với con LỚN NHẤT (max-heap)?

Nhiều bạn thắc mắc: "Sao không swap với con nào cũng được?" Hãy xem điều gì xảy ra:

```
Max-heap, sift-down node 10:
        10
       /  \
      50   40

Nếu swap với 40 (con NHỎ hơn):
        40
       /  \
      50   10
       ↑
      50 > 40 → VI PHẠM heap property!

Nếu swap với 50 (con LỚN nhất):
        50
       /  \
      10   40
      ↑
      40 < 50 ✓, 10 < 50 ✓ → đúng!
```

Luôn swap với con **LỚN NHẤT** (max-heap) hoặc **NHỎ NHẤT** (min-heap) để đảm bảo node mới lớn hơn (hoặc nhỏ hơn) cả 2 con.

Nghĩ theo bảng xếp hạng game: khi player top 1 bị loại, player cuối bảng lên thay. Bây giờ 2 runner-up thách đấu -- bạn phải cho player **mạnh hơn** lên, không thì player yếu hơn sẽ bị player mạnh hơn "đè".

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

Sift-down từ index 2 (node 7): 7 > cả 2 con (2, 5) → OK
Sift-down từ index 1 (node 1): 1 < 8 → đổi với 8
  [4, 8, 7, 3, 1, 2, 5]
Sift-down từ index 0 (node 4): 4 < 8 → đổi với 8
  [8, 4, 7, 3, 1, 2, 5]
  4 còn con 3, 1 → 4 > cả hai → OK

Kết quả: [8, 4, 7, 3, 1, 2, 5] -- max-heap hợp lệ!
```

**Tại sao O(n) mà không phải O(n log n)?**

Trực giác: phần lớn node ở gần leaf. Trong cây n node:
- ~n/2 node là leaf → sift-down 0 bước
- ~n/4 node ở tầng trên leaf → sift-down tối đa 1 bước
- ~n/8 node ở tầng tiếp → sift-down tối đa 2 bước
- ...
- 1 node root → sift-down tối đa log n bước

Tổng = n/4 * 1 + n/8 * 2 + n/16 * 3 + ... ≈ **n** (chuỗi hội tụ).

Node càng gần root thì sift-down nhiều bước, nhưng số node đó cực ít. Node nhiều nhất (leaf) thì sift-down 0 bước. Vì vậy tổng công việc chỉ O(n).

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

Đơn giản! Chỉ 1 `Vec` và 1 flag. So sánh với AVL tree cần `height`, Red-Black cần `color` + pointer trái/phải/cha... Heap thực sự là "nhịp nghỉ".

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

## BinaryHeap trong Rust std

Bạn đã tự implement heap ở trên. Trong production, Rust cung cấp sẵn `std::collections::BinaryHeap`:

```rust
use std::collections::BinaryHeap;

// ⚠️ Rust BinaryHeap là MAX-HEAP mặc định!
let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);
assert_eq!(heap.peek(), Some(&5));  // max = 5
assert_eq!(heap.pop(), Some(5));    // lấy ra max
```

### Min-heap dùng Reverse

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(5));
assert_eq!(min_heap.peek(), Some(&Reverse(1)));  // min = 1
```

`Reverse(T)` đảo ngược thứ tự so sánh: `Reverse(1) > Reverse(5)` vì 1 < 5. Max-heap + đảo thứ tự = min-heap. Pattern này xuất hiện **cực nhiều** trong competitive programming và production Rust. Nhớ kỹ!

### Các API hữu ích

```rust
use std::collections::BinaryHeap;

// Heapify từ Vec -- O(n), không phải O(n log n)!
let heap = BinaryHeap::from(vec![4, 1, 7, 3, 8]);
assert_eq!(heap.peek(), Some(&8));

// peek_mut() -- sửa root in-place, tự sift-down khi drop
let mut heap = BinaryHeap::from(vec![5, 3, 1]);
if let Some(mut top) = heap.peek_mut() {
    *top = 0;  // đổi root từ 5 thành 0
}
// heap tự sift-down 0 xuống đúng vị trí
assert_eq!(heap.peek(), Some(&3));  // 3 lên root

// into_sorted_vec() -- heapsort built-in!
let heap = BinaryHeap::from(vec![4, 1, 7, 3]);
let sorted = heap.into_sorted_vec();
assert_eq!(sorted, vec![1, 3, 4, 7]);

// drain_sorted() -- lazy sorted iterator (nightly)
// iter() -- iterate KHÔNG theo thứ tự (chỉ duyệt mảng)
```

`peek_mut()` đặc biệt hữu ích cho Dijkstra -- thay vì pop rồi push giá trị mới (2 * O(log n)), bạn sửa trực tiếp root (1 * O(log n)).

## Heapsort

Heap sort chỉ được nhắc qua ở trên. Giờ trace chi tiết:

**Ý tưởng**: xây max-heap, rồi lặp lấy root (max) ra đặt cuối mảng. Mỗi lần lấy, heap shrink 1, phần sorted ở cuối grow 1.

```
Heapsort [4, 1, 7, 3, 8, 2, 5]:

Bước 1: Heapify → max-heap [8, 4, 7, 3, 1, 2, 5]

Bước 2: Lặp -- swap root với cuối, shrink heap, sift-down
  Swap 8↔5: [5, 4, 7, 3, 1, 2 | 8]    sift-down → [7, 4, 5, 3, 1, 2 | 8]
  Swap 7↔2: [2, 4, 5, 3, 1 | 7, 8]    sift-down → [5, 4, 2, 3, 1 | 7, 8]
  Swap 5↔1: [1, 4, 2, 3 | 5, 7, 8]    sift-down → [4, 3, 2, 1 | 5, 7, 8]
  Swap 4↔1: [1, 3, 2 | 4, 5, 7, 8]    sift-down → [3, 1, 2 | 4, 5, 7, 8]
  Swap 3↔2: [2, 1 | 3, 4, 5, 7, 8]    sift-down → [2, 1 | 3, 4, 5, 7, 8]
  Swap 2↔1: [1 | 2, 3, 4, 5, 7, 8]

Kết quả: [1, 2, 3, 4, 5, 7, 8] ✓
```

Phần bên trái `|` là heap (đang shrink), phần bên phải là sorted (đang grow).

**Time**: O(n log n) -- heapify O(n) + n lần pop O(log n)
**Space**: O(1) -- in-place! Không cần mảng phụ

### So sánh sorting algorithms

| | Heapsort | Quicksort | Mergesort |
|---|---------|-----------|-----------|
| Time (avg) | O(n log n) | O(n log n) | O(n log n) |
| Time (worst) | **O(n log n)** | O(n²) | O(n log n) |
| Space | **O(1)** in-place | O(log n) stack | O(n) |
| Stable? | Không | Không | **Có** |
| Cache-friendly? | Không (nhảy xa) | **Có** (sequential) | Có |
| Thực tế | Ít dùng standalone | Phổ biến nhất | Dùng cho stable sort |

Heapsort đảm bảo **O(n log n) worst case + O(1) space** -- không algorithm nào khác làm được cả hai. Nhưng thực tế quicksort nhanh hơn vì cache-friendly hơn (truy cập tuần tự, không nhảy lung tung như sift-down).

Rust `sort_unstable()` dùng pattern-defeating quicksort. `sort()` (stable) dùng merge sort variant. Heapsort ít dùng standalone, nhưng concept sift-down là nền tảng cho priority queue.

## Heap trong thực tế

### a) Priority Queue -- ứng dụng số 1

**Phòng cấp cứu**: bệnh nhân nặng nhất được khám trước, không phải người đến trước. Queue thông thường (FIFO) không đủ -- cần "queue có ưu tiên".

```
Thường queue (FIFO):      Priority queue (Heap):
  Vào: A → B → C → D        Vào: A(3) → B(7) → C(1) → D(5)
  Ra:  A → B → C → D        Ra:  B(7) → D(5) → A(3) → C(1)
                              (số lớn = ưu tiên cao)
```

Thêm ví dụ thực tế:
- **OS scheduler**: process ưu tiên cao chạy trước (Linux CFS dùng Red-Black tree, nhưng nhiều OS khác dùng heap)
- **Game event system**: event quan trọng xử lý trước (damage calculation trước animation)
- **Print queue**: tài liệu "urgent" in trước

### b) Dijkstra's Shortest Path

Tìm đường đi ngắn nhất trong graph -- thuật toán Dijkstra cần liên tục lấy node **gần nhất** chưa thăm:

```
Graph:
  A --1-- B --3-- D
  |       |       |
  4       1       1
  |       |       |
  C --5-- E --2-- F

Min-heap theo khoảng cách từ A:

Bước 1: Heap = [(A,0)]. Pop A(0). Update B(1), C(4).
Bước 2: Heap = [(B,1), (C,4)]. Pop B(1). Update D(4), E(2).
Bước 3: Heap = [(E,2), (C,4), (D,4)]. Pop E(2). Update F(4).
Bước 4: Heap = [(C,4), (D,4), (F,4)]. Pop C(4).
Bước 5: Pop D(4). Bước 6: Pop F(4).

Kết quả: A→B = 1, A→C = 4, A→D = 4, A→E = 2, A→F = 4
```

Không có Heap: O(V²). Có Heap: O((V+E) log V). Với graph lớn, sự khác biệt là hàng triệu operations.

### c) Top-K Problems

**Bài toán**: tìm 10 sản phẩm bán chạy nhất trong 1 triệu sản phẩm.

Cách naive: sort toàn bộ → O(n log n). Nhưng ta chỉ cần top 10!

**Trick: dùng min-heap size K.**

```
Tìm top-3 trong [5, 2, 8, 1, 9, 3, 7]:

Min-heap (size 3):
  Push 5 → [5]
  Push 2 → [2, 5]
  Push 8 → [2, 5, 8]           ← heap đầy (size = K = 3)
  Push 1 → 1 < 2 (root)? Bỏ.   Heap vẫn [2, 5, 8]
  Push 9 → 9 > 2 (root)? Pop 2, push 9 → [5, 8, 9]
  Push 3 → 3 < 5 (root)? Bỏ.   Heap vẫn [5, 8, 9]
  Push 7 → 7 > 5 (root)? Pop 5, push 7 → [7, 8, 9]

Kết quả: heap chứa top-3 = {7, 8, 9} ✓
```

Tại sao **min-heap** cho top-K **lớn nhất**? Vì root là phần tử **nhỏ nhất** trong K ứng viên hiện tại -- "người yếu nhất trong nhóm". Nếu ứng viên mới mạnh hơn "người yếu nhất", thì loại người yếu nhất, cho ứng viên mới vào.

Time: O(n log K). Khi K << n (10 << 1,000,000), nhanh hơn sort rất nhiều.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn top_k(data: &[i32], k: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for &val in data {
        heap.push(Reverse(val));
        if heap.len() > k {
            heap.pop();  // loại nhỏ nhất
        }
    }
    heap.into_sorted_vec().into_iter().map(|r| r.0).collect()
}
```

### d) Merge K Sorted Lists

K danh sách sorted, merge thành 1 sorted list:

```
List 1: [1, 4, 7]
List 2: [2, 5, 8]
List 3: [3, 6, 9]

Min-heap chứa đầu mỗi list: [(1,L1), (2,L2), (3,L3)]

Pop (1,L1) → output 1. Push next from L1 (4). Heap: [(2,L2), (3,L3), (4,L1)]
Pop (2,L2) → output 2. Push next from L2 (5). Heap: [(3,L3), (4,L1), (5,L2)]
Pop (3,L3) → output 3. Push next from L3 (6). Heap: [(4,L1), (5,L2), (6,L3)]
...

Output: [1, 2, 3, 4, 5, 6, 7, 8, 9]
```

Time: O(N log K) -- N tổng phần tử, K số list. Heap chỉ chứa K phần tử (đầu mỗi list), nên mỗi pop/push là O(log K).

Ứng dụng thực tế: merge messages từ nhiều Kafka partition theo timestamp order. Mỗi partition đã sorted theo timestamp → min-heap chứa head message mỗi partition → pop message cũ nhất.

## Median Finding -- 2 Heap trick

Đây là bài toán nâng cao hay nhất dùng Heap:

**Bài toán**: stream số liên tục, tìm median tại mọi thời điểm.

**Ý tưởng**: dùng 2 heap
- `max_heap`: chứa **nửa NHỎ** (lấy max của nửa nhỏ = "giữa bên trái")
- `min_heap`: chứa **nửa LỚN** (lấy min của nửa lớn = "giữa bên phải")

```
Hình dung:
  max_heap ← [nhỏ...giữa_trái] | [giữa_phải...lớn] → min_heap
              root = giữa_trái    root = giữa_phải

Median = trung bình 2 root (nếu size bằng nhau)
       = root heap lớn hơn (nếu size lệch 1)
```

**Trace:**

```
Stream [5, 2, 8, 1]:

Insert 5: max_heap=[5], min_heap=[]
  → median = 5

Insert 2: max_heap=[2], min_heap=[5]
  → median = (2+5)/2 = 3.5

Insert 8: max_heap=[2], min_heap=[5,8]
  → min_heap lớn hơn → pop 5 sang max_heap
  → max_heap=[2,5], min_heap=[8]
  → median = 5

Insert 1: max_heap=[1,2,5], min_heap=[8]
  → max_heap lớn hơn 1 → pop 5 sang min_heap
  → max_heap=[1,2], min_heap=[5,8]
  → median = (2+5)/2 = 3.5

Mỗi insert: O(log n). Query median: O(1).
```

Không có heap, tìm median mỗi lần cần sort lại → O(n log n). Với 2 heap → O(log n) mỗi insert. Elegant!

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa |
|----------|-----------|--------|----------|
| `push` | O(log n) | O(1) | Sift-up tối đa log n tầng |
| `pop` | O(log n) | O(1) | Sift-down tối đa log n tầng |
| `peek` | **O(1)** | O(1) | Chỉ nhìn root |
| `heapify` | **O(n)** | O(1) | Nhanh hơn chèn từng cái! |
| `size` / `is_empty` | O(1) | O(1) | Chỉ đọc len |
| **Tổng bộ nhớ** | -- | **O(n)** | 1 mảng, không pointer |

So sánh nhanh:

| Cách tiếp cận | push | peek max | pop max |
|---------------|------|----------|---------|
| Mảng unsorted | O(1) | O(n) | O(n) |
| Mảng sorted | O(n) | O(1) | O(1) |
| **Heap** | **O(log n)** | **O(1)** | **O(log n)** |

Heap là sự cân bằng tuyệt vời: push và pop đều O(log n), peek O(1). Mảng sorted cho peek O(1) nhưng push O(n). Mảng unsorted cho push O(1) nhưng peek O(n).

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

### Heapsort bằng MinHeap

```rust
use rust_ds2a::heap::MinHeap;

fn heapsort<T: Ord>(data: Vec<T>) -> Vec<T> {
    let mut heap = MinHeap::from_vec(data);
    let mut sorted = Vec::with_capacity(heap.size());
    while let Some(val) = heap.delete_min() {
        sorted.push(val);
    }
    sorted
}

let result = heapsort(vec![5, 3, 8, 1, 2, 7, 4]);
assert_eq!(result, vec![1, 2, 3, 4, 5, 7, 8]);
```

## Những cái bẫy hay gặp

### a) Nghĩ Heap có sorted order

- Sai: "Heap sorted nên `[50, 30, 40, 10, 20]` thì 30 < 40"
- Đúng: Heap chỉ đảm bảo **cha >= con**. `[50, 10, 40, 3, 8]` là max-heap hợp lệ dù 10 đứng trước 40
- Nhớ: Heap = partial order. Chỉ root là chắc chắn max/min. Phần còn lại **không có thứ tự** với nhau

### b) Quên BinaryHeap Rust là max-heap

- Sai: `BinaryHeap::new()` rồi `pop()` -- nghĩ lấy ra min
- Đúng: Rust `BinaryHeap` **mặc định là max-heap**. `pop()` lấy ra **max**
- Nhớ: Muốn min-heap? Luôn dùng `Reverse(T)`. Đừng quên unwrap: `Reverse(val).0`

### c) Insert từng phần tử khi có Vec sẵn

- Sai: `for x in vec { heap.push(x); }` → O(n log n)
- Đúng: `BinaryHeap::from(vec)` = heapify O(n), nhanh gần 2x
- Nhớ: Có sẵn data? Dùng `from()`. Chỉ `push()` khi data đến dần dần (streaming)

### d) Search by value trong heap

- Sai: "Tìm phần tử 42 trong heap" → O(n), phải duyệt hết
- Đúng: Heap **không** hỗ trợ search hiệu quả. Nếu cần search → dùng HashMap + Heap combo hoặc lazy deletion
- Nhớ: Heap chỉ giỏi 1 việc: lấy max/min. Đừng bắt nó làm việc khác

### e) Heap cho sorted iteration

- Sai: Dùng heap để iterate sorted → mỗi lần `pop()` phá heap
- Đúng: Dùng `BTreeSet` nếu cần iterate sorted nhiều lần mà không phá cấu trúc
- Nhớ: `into_sorted_vec()` consume heap. `iter()` trả về **unsorted**!

## Khi nào dùng / không nên dùng

| Tình huống | Heap? | Thay bằng gì? | Tại sao? |
|------------|-------|---------------|----------|
| Priority queue | ✅ | -- | Sinh ra cho việc này |
| Top-K elements | ✅ | -- | Min-heap size K pattern |
| Dijkstra / Prim | ✅ | -- | Lấy min liên tục |
| Timer queue | ✅ | -- | Event sớm nhất ở root |
| Merge K sorted lists | ✅ | -- | O(N log K) |
| Heapsort (O(1) space) | ✅ | -- | Duy nhất O(n log n) + O(1) |
| Sorted iteration | ❌ | BTreeSet | Heap không giữ sorted |
| Search by value | ❌ | HashMap/BTreeMap | Heap search O(n) |
| Sorted insert + lookup | ❌ | BST/AVL | Heap không total order |
| Chỉ cần min 1 lần | ❌ | `iter().min()` | Đừng build heap cho 1 query |
| Median finding | ⚠️ | 2 Heaps pattern | Nâng cao nhưng elegant |

## Luyện nhận diện Pattern

### Bài 1: Last Stone Weight (LeetCode #1046)

Mảng đá `[2, 7, 4, 1, 8, 1]`. Mỗi lần lấy 2 viên nặng nhất va chạm: bằng nhau thì cả 2 vỡ, khác thì viên nhỏ vỡ, viên lớn còn lại (weight = hiệu). Viên cuối nặng bao nhiêu?

<details>
<summary>Gợi ý</summary>

Cần liên tục lấy **2 max**. Max-heap!

```
Heap: [8, 7, 4, 2, 1, 1]
Pop 8, 7 → 8-7=1 → push 1 → [4, 2, 1, 1, 1]
Pop 4, 2 → 4-2=2 → push 2 → [2, 1, 1, 1]
Pop 2, 1 → 2-1=1 → push 1 → [1, 1, 1]
Pop 1, 1 → vỡ hết → [1]
Kết quả: 1
```

</details>

<details>
<summary>Code Rust</summary>

```rust
use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        if a != b {
            heap.push(a - b);
        }
    }
    heap.pop().unwrap_or(0)
}
```

</details>

### Bài 2: Kth Largest Element (LeetCode #215)

Tìm phần tử lớn thứ K trong mảng unsorted. Ví dụ `[3,2,1,5,6,4]`, k=2 → 5.

<details>
<summary>Gợi ý</summary>

**Cách 1**: Min-heap size K. Sau khi duyệt hết, root = phần tử lớn thứ K.

**Cách 2**: Heapify max-heap + pop K lần.

Cách 1 tốt hơn khi K << n (dùng ít memory hơn).

</details>

<details>
<summary>Code Rust</summary>

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn find_kth_largest(nums: Vec<i32>, k: usize) -> i32 {
    let mut heap = BinaryHeap::new();
    for num in nums {
        heap.push(Reverse(num));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.peek().unwrap().0
}
```

</details>

### Bài 3: Task Scheduler (LeetCode #621)

Tasks `['A','A','A','B','B','B']`, cooldown n=2. Tìm minimum time để hoàn thành tất cả task (cùng loại phải cách nhau >= n slot).

<details>
<summary>Gợi ý</summary>

Task nào có count cao nhất nên ưu tiên trước. Dùng max-heap theo count. Mỗi round: pop tối đa n+1 task, giảm count, push lại nếu count > 0.

```
Tasks: A=3, B=3. n=2. Mỗi round cần n+1=3 slot.

Round 1: A B idle → A=2, B=2
Round 2: A B idle → A=1, B=1
Round 3: A B      → A=0, B=0

Total = 8 slots
```

</details>

## Heap trong Rust ecosystem

- `std::collections::BinaryHeap` -- max-heap, dùng `Reverse` cho min. Bên trong là Vec + sift-up/sift-down y hệt code ta viết
- `BinaryHeap::peek_mut()` -- modify root in-place, tự sift-down khi `PeekMut` drop. Rất hữu ích cho decrease-key trong Dijkstra
- `BinaryHeap::into_sorted_vec()` -- heapsort built-in, O(n log n)
- `BinaryHeap::from(vec)` -- heapify O(n). Luôn dùng thay vì push từng phần tử
- **Tokio timer wheel** -- variant của heap cho timer management. Mỗi timeout là 1 entry, cần lấy timeout sắp expire sớm nhất

Ứng dụng thực tế: merge messages từ nhiều Kafka partition theo timestamp → min-heap chứa head message mỗi partition, pop message cũ nhất. Buffer management: heap track buffer nào "gần hết hạn" cần flush trước.

## Tiếp theo: Priority Queue

Heap là "động cơ" -- mạnh nhưng API chưa nói rõ mục đích. Chương tiếp theo bọc heap lại thành **Priority Queue** với interface rõ ràng hơn: `enqueue` / `dequeue` thay vì `push` / `pop`. Quan trọng hơn, bạn sẽ thấy heap được dùng trong những bài toán thực tế lớn: Dijkstra tìm đường ngắn nhất, Huffman coding nén dữ liệu, event-driven systems -- tất cả đều chạy trên cùng cái heap bạn vừa học.

---

[← Red-Black Tree](./04-red-black-tree.md) | [Priority Queue →](./06-priority-queue.md)
