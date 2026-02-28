# Binary Tree

## Đây là gì?

> **Nếu bạn cảm thấy choáng ngợp khi đọc chương này -- đó là hoàn toàn bình thường.** Binary Tree là bước nhảy lớn nhất trong series cho đến giờ. Ba chương trước (Stack, Queue, Deque) đều là cấu trúc tuyến tính -- mọi thứ xếp thành hàng, dễ hình dung. Chương này khác: dữ liệu **phân nhánh**. Nhưng thực ra bạn đã thấy tree mỗi ngày mà không nhận ra: thư mục trong máy tính, mục lục sách, sơ đồ tổ chức công ty, hay cái DOM tree khi bạn inspect element trên trình duyệt. Hiểu Binary Tree là nền tảng để sau này hiểu BST (tìm kiếm nhanh), Heap (priority queue), Trie (autocomplete), và cả cách database index hoạt động (B-Tree). Bạn đang xây nền móng cho rất nhiều thứ hay ho phía trước.

Bạn biết **cây gia phả** (family tree) không? Ông bà ở trên cùng, bố mẹ ở giữa, con cháu ở dưới. Mỗi người có thể có con hoặc không. **Binary tree** (cây nhị phân) cũng giống vậy -- nhưng có một quy tắc: mỗi "người" (node) chỉ được có **tối đa 2 con**, gọi là con trái (left child) và con phải (right child).

Tại sao lại cần cấu trúc này? Vì rất nhiều thứ trong máy tính có dạng phân nhánh:
- **File system**: thư mục chứa thư mục con
- **HTML**: thẻ cha chứa thẻ con
- **Biểu thức toán**: `(1 + 2) * 3` có cấu trúc cây

### Từ cấu trúc tuyến tính sang phi tuyến

Đây là bước chuyển quan trọng nhất trong series. Hãy nhìn lại những gì bạn đã học:

Stack, Queue, Deque đều là cấu trúc **linear** (tuyến tính) -- phần tử xếp thành 1 hàng, mỗi phần tử có tối đa 1 "người đứng trước" và 1 "người đứng sau":

```
Linear:   [A] → [B] → [C] → [D] → [E]
           Một hàng thẳng, chỉ có 1 đường đi
```

Giới hạn của linear:
- Tìm kiếm luôn **O(n)** -- phải duyệt từng phần tử
- Không thể biểu diễn quan hệ **phân cấp** (hierarchy) -- ai là "cha" của ai?

Tree phá vỡ giới hạn này: 1 node có thể có **nhiều con**. Dữ liệu không còn "xếp hàng" mà "phân nhánh":

```
Tree:         [A]
             /   \
           [B]   [C]
          / \      \
        [D] [E]   [F]
        Phân nhánh, nhiều đường đi
```

| Đặc điểm | Linear (Stack/Queue/Deque) | Tree |
|-----------|---------------------------|------|
| Cấu trúc | Một hàng thẳng | Phân nhánh |
| Mỗi phần tử có | 1 predecessor, 1 successor | 1 parent, nhiều children |
| Tìm kiếm | O(n) luôn | O(log n) nếu cây cân bằng |
| Biểu diễn hierarchy | Không được | Tự nhiên |
| Ví dụ | Hàng đợi, lịch sử undo | File system, DOM, database index |

Binary Tree là trường hợp đặc biệt: mỗi node có **tối đa 2 con**. Tại sao giới hạn ở 2? Vì nó đủ mạnh để biểu diễn hầu hết mọi thứ, nhưng đủ đơn giản để phân tích và implement.

## Các thuật ngữ cần biết

Trước khi đi sâu, bạn cần biết "ngôn ngữ" của tree. Mỗi thuật ngữ dưới đây sẽ xuất hiện liên tục trong các chương sau, nên hãy dành thời gian hiểu kỹ. Tất cả đều dựa trên ẩn dụ **cây gia phả**:

```
            1           ← root (gốc, depth=0)
           / \
          2   3         ← internal nodes (depth=1)
         / \   \
        4   5   6       ← node 4,5 là leaf; node 6 cũng là leaf
```

- **Root** (gốc): node trên cùng, giống **ông tổ** trong gia phả -- không có cha. Trong cây trên, root là node `1`.
- **Leaf** (lá): node không có con, giống **thế hệ trẻ nhất** chưa có con. Trong cây trên: node `4`, `5`, `6`.
- **Internal node** (node nội): node có ít nhất 1 con -- là "cha mẹ". Trong cây trên: node `2`, `3`.
- **Parent / Child** (cha / con): `1` là parent của `2` và `3`; `2` là parent của `4` và `5`.
- **Sibling** (anh em): các node cùng cha. `2` và `3` là sibling (cùng cha là `1`).
- **Subtree** (cây con): 1 node cùng tất cả con cháu của nó tạo thành 1 subtree. Node `2` và con cháu `4`, `5` tạo thành subtree bên trái.
- **Depth** (độ sâu): khoảng cách từ root đến node đó (đếm số cạnh). Root có depth = 0.
  - Depth của node `5`: 2 (đếm cạnh: `1→2→5`)
- **Height** (chiều cao): số tầng trong cây.
  - Height của cây trên: 3 (tầng 0, 1, 2)

> **Lưu ý:** Depth đếm từ **trên xuống** (root = 0). Height đếm **tổng số tầng**. Nhiều sách định nghĩa hơi khác nhau (có sách tính height bắt đầu từ 0) -- đừng lo nếu thấy lệch 1, miễn là hiểu concept.

## Các loại Binary Tree

Biết phân loại giúp bạn hiểu khi nào nên dùng loại nào. Đừng cố nhớ hết -- chỉ cần nắm bức tranh tổng:

```
                    Binary Tree (cây nhị phân)
                           │
          ┌────────────────┼────────────────┐
          │                │                │
     Full Binary      Complete Binary   Perfect Binary
    (mỗi node có     (đầy từ trên      (tất cả leaf
     0 hoặc 2 con)   xuống, trái→phải)  cùng tầng, mọi
                                         node đều có 2 con)
          │
          ▼
   Binary Search Tree (BST)
   (trái < gốc < phải)     ← Chương tiếp theo!
          │
          ▼
   AVL / Red-Black Tree
   (BST tự cân bằng)       ← Nâng cao
```

**Full Binary Tree** -- mỗi node có 0 hoặc 2 con (không ai chỉ có 1 con):
```
        1
       / \
      2   3
     / \
    4   5
```

**Complete Binary Tree** -- đầy từ trên xuống, trái sang phải (chỉ tầng cuối có thể thiếu, và thiếu bên phải):
```
        1
       / \
      2   3
     / \  /
    4  5 6
```

**Perfect Binary Tree** -- tất cả leaf cùng tầng, mọi internal node đều có 2 con:
```
        1
       / \
      2   3
     / \ / \
    4  5 6  7
```

Chương này dạy generic Binary Tree. Chương sau sẽ thêm 1 quy tắc đơn giản (trái < gốc < phải) để biến nó thành BST -- và insert/search đột nhiên nhanh hơn rất nhiều.

## Hoạt động như thế nào?

### Cấu trúc cây

Hãy tưởng tượng một gia phả 3 đời:

```
            1           <- root  (tầng 0, ông tổ)
           / \
          2   3         <- tầng 1 (bố mẹ)
         / \ / \
        4  5 6  7       <- tầng 2 (con cháu, đều là leaf)

Height = 3 (3 tầng)
```

Node `1` là root. Node `4, 5, 6, 7` là leaf (không có con).

### Chèn theo level-order

Khi chèn giá trị mới, ta tìm chỗ trống đầu tiên từ trên xuống, từ trái sang phải. Giống như xếp chỗ ngồi trong rạp: lấp đầy hàng trước rồi mới xuống hàng sau.

```
Chèn 1:       Chèn 2:       Chèn 3:       Chèn 4:
    1               1               1               1
                   /               / \             / \
                  2               2   3           2   3
                                                 /
                                                4
```

Cách này đảm bảo cây luôn **complete** (đầy đều) -- không có "lỗ hổng" giữa các node.

**Tại sao dùng BFS để insert?** Vì ta muốn cây complete -- lấp đầy từ trên xuống, trái sang phải. BFS (duyệt theo tầng) duyệt đúng thứ tự này, nên node đầu tiên có slot trống chính là vị trí cần insert.

**Trace insert từng bước** -- cho cây `[1, 2, 3, _, 5]` khi `insert(4)`:

```
Trước insert(4):
        1
       / \
      2   3
       \
        5

BFS tìm chỗ trống:
  Queue: [1]
  Dequeue 1 → left=2 (có), right=3 (có) → enqueue(2, 3)
  Queue: [2, 3]
  Dequeue 2 → left=None! ← TÌM THẤY! Insert 4 vào đây.

Sau insert(4):
        1
       / \
      2   3
     / \
    4   5
```

> **Lưu ý:** Insert này là O(n) worst case vì BFS có thể phải duyệt hết. Đây là Binary Tree **thường** (không sorted). Chương BST sau sẽ có insert O(log n) vì biết đi trái hay phải dựa trên giá trị.

## Tại sao `Option<Box<Node<T>>>`?

Đây là phần Rust-specific mà nhiều người mới hay bị stuck. Nếu bạn cũng vậy, bạn không hề đơn độc -- đây là 1 trong những điểm khó nhất khi implement tree trong Rust.

### Tại sao cần `Box<T>`?

Tree là recursive structure -- `Node` chứa `Node` con. Compiler cần biết size của mỗi type lúc compile. Nhưng nếu `Node` chứa `Node`, thì `Node` chứa `Node` chứa `Node`... size = vô hạn!

```rust
// ❌ Không compile -- size vô hạn
struct Node {
    value: i32,
    left: Option<Node>,   // Node chứa Node chứa Node chứa...
}

// ✅ Compile được -- Box<Node> có size cố định (8 bytes pointer)
struct Node {
    value: i32,
    left: Option<Box<Node>>,  // 8 bytes pointer → size xác định
}
```

`Box` đưa child lên **heap**, chỉ giữ pointer (8 bytes) trên struct. Compiler happy vì biết chính xác size.

### Tại sao cần `Option<T>`?

Vì child có thể **không tồn tại**. `None` = không có con. `Some(box_node)` = có con. Đây là cách Rust thay thế **null pointer** -- an toàn hơn, compiler bắt lỗi nếu bạn quên check.

### Memory layout

```
Stack:                    Heap:
┌──────────────┐
│ root: Some ──────────> ┌──────────────┐
└──────────────┘         │ value: 1     │
                         │ left: Some ──────> ┌──────────────┐
                         │ right: Some ─────> │ value: 3     │
                         └──────────────┘     │ left: None   │
                              │               │ right: None  │
                              ▼               └──────────────┘
                         ┌──────────────┐
                         │ value: 2     │
                         │ left: None   │
                         │ right: None  │
                         └──────────────┘
```

> **Liên hệ:** Nhớ `Vec` trong Stack/Queue/Deque chứa data liên tiếp trên heap (cache-friendly). Tree thì mỗi node ở vị trí **riêng** trên heap -- **không** cache-friendly. Đây là trade-off: tree cho ta cấu trúc phân nhánh nhưng mất đi spatial locality.

## Code Rust

Code đầy đủ nằm trong `src/binary_tree.rs`.

### Cấu trúc dữ liệu

```rust
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,    // con trái (có thể None)
    right: Option<Box<Node<T>>>,   // con phải (có thể None)
}

pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,    // cây có thể rỗng
}
```

### Chèn (insert)

Dùng BFS (duyệt theo tầng) để tìm chỗ trống đầu tiên:

```rust
pub fn insert(&mut self, val: T) {
    let new_node = Box::new(Node { value: val, left: None, right: None });
    if self.root.is_none() {
        self.root = Some(new_node);
        return;
    }
    // BFS tìm node đầu tiên còn chỗ trống cho con
    // ... (xem src/binary_tree.rs)
}
```

### Duyệt cây (traversal)

Dùng đệ quy (recursion). Ví dụ in-order:

```rust
pub fn inorder(&self) -> Vec<T> {
    let mut result = Vec::new();
    fn walk<T: Clone>(node: &Option<Box<Node<T>>>, out: &mut Vec<T>) {
        if let Some(n) = node {
            walk(&n.left, out);       // 1. đi trái
            out.push(n.value.clone()); // 2. lấy giá trị gốc
            walk(&n.right, out);      // 3. đi phải
        }
    }
    walk(&self.root, &mut result);
    result
}
```

### Chiều cao (height)

```rust
pub fn height(&self) -> usize {
    fn h<T>(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => 0,                              // không có node = cao 0
            Some(n) => 1 + h(&n.left).max(h(&n.right)), // 1 + max(trái, phải)
        }
    }
    h(&self.root)
}
```

Logic: chiều cao = 1 (bản thân) + chiều cao của nhánh con cao hơn.

## Traversal chi tiết

Traversal (duyệt cây) là kỹ năng quan trọng nhất khi làm việc với tree. Có 4 cách, mỗi cách cho kết quả khác nhau. Với người mới, đệ quy rất trừu tượng -- nên mình sẽ trace từng bước call stack.

Với cây `[1, 2, 3, 4, 5]`:

```
            1
           / \
          2   3
         / \
        4   5
```

### In-order (Trái -> Gốc -> Phải) -- trace call stack chi tiết

```
inorder(1)
  ├─ inorder(2)            ← đi trái
  │   ├─ inorder(4)        ← đi trái
  │   │   ├─ inorder(None) ← trái của 4 = None, return
  │   │   ├─ VISIT 4 ✓    ← thêm 4 vào kết quả
  │   │   └─ inorder(None) ← phải của 4 = None, return
  │   ├─ VISIT 2 ✓        ← thêm 2 vào kết quả
  │   └─ inorder(5)        ← đi phải
  │       ├─ inorder(None) ← trái của 5 = None, return
  │       ├─ VISIT 5 ✓    ← thêm 5 vào kết quả
  │       └─ inorder(None) ← phải của 5 = None, return
  ├─ VISIT 1 ✓            ← thêm 1 vào kết quả
  └─ inorder(3)            ← đi phải
      ├─ inorder(None)     ← trái của 3 = None, return
      ├─ VISIT 3 ✓        ← thêm 3 vào kết quả
      └─ inorder(None)     ← phải của 3 = None, return

Kết quả theo thứ tự VISIT: [4, 2, 5, 1, 3]
```

Nhìn vào trace, bạn thấy pattern: đệ quy đi sâu hết nhánh trái trước, rồi mới quay lên (backtrack). Đây chính là **DFS** (Depth-First Search) -- tên gọi chung cho in/pre/post-order. Level-order thì dùng **BFS** (Breadth-First Search) -- duyệt theo chiều rộng, cần Queue thay vì recursion.

### Pre-order (Gốc -> Trái -> Phải)

Giống in-order, nhưng **VISIT xảy ra đầu tiên** (trước khi đi trái):
```
Kết quả: [1, 2, 4, 5, 3]
          ↑gốc trước, rồi mới đi sâu
```

Pre-order hữu ích khi bạn muốn **copy cây** -- vì gốc luôn được xử lý trước con.

### Post-order (Trái -> Phải -> Gốc)

VISIT xảy ra **cuối cùng** (sau khi đã thăm hết con):
```
Kết quả: [4, 5, 2, 3, 1]
                       ↑gốc cuối cùng
```

Post-order hữu ích khi bạn muốn **xóa cây** hoặc **tính toán từ dưới lên** -- vì con luôn được xử lý trước cha.

### Level-order (theo từng tầng) -- trace Queue chi tiết

Level-order không dùng đệ quy mà dùng **Queue** -- chính cái Queue bạn vừa học chương trước!

```
Queue trace cho level_order:

Bước  Action              Queue sau action    Result
1     enqueue(1)          [1]                 []
2     dequeue()→1         []                  [1]
      enqueue(2), enq(3)  [2, 3]
3     dequeue()→2         [3]                 [1, 2]
      enqueue(4), enq(5)  [3, 4, 5]
4     dequeue()→3         [4, 5]             [1, 2, 3]
      (3 không có con)
5     dequeue()→4         [5]                [1, 2, 3, 4]
6     dequeue()→5         []                 [1, 2, 3, 4, 5]

Kết quả: [1, 2, 3, 4, 5]
```

> Nhận ra không? Level-order dùng **Queue** -- chương trước bạn vừa học! BFS trên tree chính là ứng dụng thực tế của Queue. Còn DFS (in/pre/post-order) dùng **call stack** -- bản chất là Stack!

**Mẹo nhớ:** tên gọi cho biết **gốc ở đâu**. Pre = gốc đầu tiên. In = gốc ở giữa. Post = gốc cuối cùng.

## Flat representation -- Vec-based tree

Ngoài `Option<Box<Node<T>>>`, bạn có thể biểu diễn cây bằng... `Vec`! Nghe lạ nhưng rất phổ biến.

```
Index:    0   1   2   3   4   5   6
Value:  [ 1,  2,  3,  4,  5,  6,  7 ]

Quy tắc:
  - Root ở index 0
  - Con trái của node i:  index 2*i + 1
  - Con phải của node i:  index 2*i + 2
  - Parent của node i:    index (i-1) / 2
```

Ví dụ: node ở index 2 (value = 3):
- Con trái: index `2*2+1 = 5` → value 6
- Con phải: index `2*2+2 = 6` → value 7
- Parent: index `(2-1)/2 = 0` → value 1

```
Cây tương ứng:
        1           index 0
       / \
      2   3         index 1, 2
     / \ / \
    4  5 6  7       index 3, 4, 5, 6
```

**Trade-off:**

| | Vec representation | Box\<Node\> representation |
|--|---|---|
| Cache | Cache-friendly (liên tiếp trên heap) | Fragmented (mỗi node riêng) |
| Truy cập | Random access O(1) bằng index | Phải traverse từ root |
| Flexibility | Chỉ hiệu quả cho complete tree | Cây hình dạng bất kỳ |
| Memory | Cây lệch → nhiều slot trống lãng phí | Chỉ dùng bộ nhớ cho node thật |

> **Liên hệ:** `BinaryHeap` trong Rust standard library dùng Vec representation! Vì heap luôn là complete binary tree → hoàn hảo cho Vec. Chương Heap sau sẽ nói chi tiết.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa thực tế |
|----------|-----------|--------|------------------|
| `insert` | O(n) | O(n) | Phải duyệt tìm chỗ trống |
| `inorder` | O(n) | O(n) | Thăm mọi node |
| `preorder` | O(n) | O(n) | Thăm mọi node |
| `postorder` | O(n) | O(n) | Thăm mọi node |
| `level_order` | O(n) | O(n) | Thăm mọi node |
| `height` | O(n) | O(h) | Đệ quy sâu h tầng |
| `count` | O(n) | O(h) | Đệ quy sâu h tầng |

**n** = tổng số node, **h** = chiều cao cây.

Insert là O(n) vì BFS phải quét qua các node để tìm chỗ trống. Tất cả traversal đều O(n) vì phải thăm mọi node. Height và count dùng đệ quy nên tốn O(h) bộ nhớ cho call stack.

## Ví dụ

### Tạo cây và duyệt

```rust
use rust_ds2a::binary_tree::BinaryTree;

let mut tree = BinaryTree::new();
for i in 1..=7 {
    tree.insert(i);
}

// Cây sẽ có dạng:
//         1
//        / \
//       2   3
//      / \ / \
//     4  5 6  7

assert_eq!(tree.level_order(), vec![1, 2, 3, 4, 5, 6, 7]);
assert_eq!(tree.inorder(), vec![4, 2, 5, 1, 6, 3, 7]);
assert_eq!(tree.height(), 3);
assert_eq!(tree.count(), 7);
```

### Cây chỉ có 1 node

```rust
use rust_ds2a::binary_tree::BinaryTree;

let mut tree = BinaryTree::new();
tree.insert(42);

assert_eq!(tree.height(), 1);     // chỉ có root
assert_eq!(tree.count(), 1);
assert_eq!(tree.preorder(), vec![42]);
```

## Binary Tree trong thực tế

Biết "khi nào gặp tree ngoài đời" giúp bạn nhận ra pattern nhanh hơn khi giải bài hoặc đi làm.

### a) Expression Tree (Cây biểu thức)

Biểu thức `(1 + 2) * 3` biểu diễn dưới dạng cây:

```
        *
       / \
      +   3
     / \
    1   2
```

Điều thú vị: traversal khác nhau cho ra **notation** khác nhau:
- **In-order:** `1 + 2 * 3` (nhưng thiếu ngoặc! cần thêm ngoặc khi in)
- **Post-order:** `1 2 + 3 *` (Reverse Polish Notation -- cách máy tính evaluate)
- **Pre-order:** `* + 1 2 3` (Polish Notation)

Post-order chính là cách stack-based calculator hoạt động -- liên hệ lại chương Stack!

### b) DOM Tree

Mỗi trang web là 1 cây:

```html
<html>
  <body>
    <h1>Hello</h1>
    <p>World</p>
  </body>
</html>
```

```
        html
         |
        body
       /    \
      h1     p
      |      |
   "Hello" "World"
```

Browser parse HTML thành DOM tree, rồi dùng tree traversal để render, tìm element (`querySelector`), hay tính layout. Khi bạn inspect element trên Chrome DevTools -- bạn đang nhìn tree!

### c) Decision Tree

```
          Trời mưa?
          /       \
        Có        Không
        |           |
     Mang ô     Đi chơi
```

Mỗi internal node là 1 câu hỏi, mỗi leaf là 1 quyết định. Random Forest trong ML là tập hợp nhiều decision tree -- nhưng đó là chuyện xa, hiện tại chỉ cần hiểu cấu trúc.

## Những cái bẫy hay gặp

Ai cũng từng mắc những lỗi này -- biết trước để tránh:

---

❌ **Vấn đề:** Nhầm height và depth

✅ **Đúng:** Height = tổng số tầng (của cả cây). Depth = số cạnh từ root đến 1 node cụ thể.

💡 **Tại sao:** Depth đếm "xuống" cho 1 node. Height đếm "toàn bộ" cây. Trong cây height=3, node ở tầng cuối có depth=2.

```
        1          depth=0
       / \
      2   3        depth=1
     / \
    4   5          depth=2
                   ↑ depth của node 5 = 2
Height của cây = 3 (3 tầng)
```

---

❌ **Vấn đề:** Quên base case trong đệ quy → stack overflow

✅ **Đúng:** Luôn check `None` (hoặc `Option`) trước khi đệ quy xuống con.

💡 **Tại sao:** Rust giúp phần nào vì `Option` bắt phải pattern match. Nhưng nếu dùng `.unwrap()` bừa bãi thì vẫn panic. Luôn dùng `if let Some(n) = node` hoặc `match`.

---

❌ **Vấn đề:** Nghĩ Binary Tree = BST (Binary Search Tree)

✅ **Đúng:** Binary Tree thường **không** có thứ tự, search là O(n). BST mới có quy tắc trái < gốc < phải và search O(log n).

💡 **Tại sao:** Đây là 2 cấu trúc khác nhau. Binary Tree chỉ nói "mỗi node tối đa 2 con". BST thêm quy tắc sắp xếp.

---

❌ **Vấn đề:** Clone khi không cần thiết

✅ **Đúng:** Nếu T lớn (String, struct), trả `Vec<&T>` thay vì `Vec<T>`, hoặc dùng callback/visitor pattern.

💡 **Tại sao:** Traversal trong code trên cần `T: Clone` để push vào Vec. Clone 1 triệu String thì rất tốn kém.

---

❌ **Vấn đề:** Dùng `Box<Node>` cho cây triệu node → chậm

✅ **Đúng:** Production code thường dùng **arena allocation** hoặc lưu cây trong Vec (flat representation).

💡 **Tại sao:** Mỗi node là 1 heap allocation riêng → fragmented memory, cache miss. Nhưng cho học tập, `Box<Node>` hoàn toàn OK.

## Khi nào dùng / không nên dùng

| Tình huống | Binary Tree? | Thay bằng gì? | Tại sao? |
|------------|:---:|---------------|----------|
| Biểu diễn hierarchy (file system, DOM) | ✅ | -- | Cấu trúc phân nhánh tự nhiên |
| Parse biểu thức toán | ✅ | -- | Expression tree |
| Tìm kiếm nhanh trong sorted data | ❌ | BST / BTreeMap | Binary Tree thường không sorted |
| Lấy min/max nhanh | ❌ | Heap (BinaryHeap) | Heap có property đặc biệt |
| Dữ liệu flat, truy cập nhanh theo index | ❌ | Vec / Array | Tree overhead không cần thiết |
| Autocomplete / prefix search | ❌ | Trie | Trie tối ưu cho string prefix |
| Task queue (FIFO) | ❌ | Queue / VecDeque | Không cần phân nhánh |
| Graph với cycle | ❌ | Adjacency list | Tree không có cycle |

## Luyện nhận diện Pattern

3 bài tập để bạn quen với tư duy tree. Đừng vội xem đáp án -- thử nghĩ 2-3 phút trước.

**Bài 1: Tính tổng tất cả node trong cây**

Cho binary tree chứa số nguyên, tính tổng tất cả giá trị.

**Gợi ý:** Traversal nào cũng được -- nhưng dùng đệ quy post-order sẽ tự nhiên nhất, tại sao?

<details>
<summary>Đáp án</summary>

**Cách giải:** Post-order tự nhiên vì bạn tính tổng nhánh trái, tổng nhánh phải, rồi cộng thêm giá trị gốc. Giống cách tính dân số cả gia tộc: đếm từ thế hệ trẻ nhất lên.

**Traversal:** Post-order (hoặc bất kỳ -- tổng không phụ thuộc thứ tự)

**Complexity:** Time O(n), Space O(h) -- h là chiều cao cây (do call stack)

```rust
fn sum(node: &Option<Box<Node<i32>>>) -> i32 {
    match node {
        None => 0,
        Some(n) => sum(&n.left) + sum(&n.right) + n.value,
    }
}
```

</details>

**Bài 2: Kiểm tra 2 cây có giống nhau không**

Cho 2 binary tree, kiểm tra chúng có cùng cấu trúc VÀ cùng giá trị tại mỗi node không.

**Gợi ý:** So sánh đệ quy -- khi nào 2 subtree được coi là "giống nhau"?

<details>
<summary>Đáp án</summary>

**Cách giải:** 2 cây giống nhau khi: (1) cả 2 đều None, hoặc (2) cả 2 đều Some, giá trị bằng nhau, VÀ cây trái giống nhau, VÀ cây phải giống nhau. Đây là bài kinh điển về đệ quy trên tree.

**Traversal:** Pre-order (check gốc trước, rồi so sánh con)

**Complexity:** Time O(n), Space O(h)

```rust
fn is_same(a: &Option<Box<Node<i32>>>, b: &Option<Box<Node<i32>>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            a.value == b.value
                && is_same(&a.left, &b.left)
                && is_same(&a.right, &b.right)
        }
        _ => false,
    }
}
```

</details>

**Bài 3: Tìm node sâu nhất (deepest node)**

Cho binary tree, tìm giá trị của node ở sâu nhất. Nếu nhiều node cùng depth, lấy node cuối cùng (phải nhất).

**Gợi ý:** Traversal nào cho ta node "cuối cùng ở tầng sâu nhất"? Nghĩ về level-order...

<details>
<summary>Đáp án</summary>

**Cách giải:** Level-order duyệt từ trên xuống, trái sang phải. Node cuối cùng trong level-order chính là node sâu nhất, phải nhất. Chỉ cần duyệt hết và lấy phần tử cuối!

**Traversal:** Level-order (BFS)

**Complexity:** Time O(n), Space O(n)

```rust
fn deepest(tree: &BinaryTree<i32>) -> Option<i32> {
    let order = tree.level_order();
    order.last().copied()
}
```

</details>

## Binary Tree trong Rust ecosystem

- `std::collections::BinaryHeap` -- priority queue dùng binary tree lưu trong Vec (chương Heap sau sẽ nói chi tiết)
- `std::collections::BTreeMap` / `BTreeSet` -- B-Tree, generalization của binary tree, mỗi node có nhiều key, tối ưu cho disk access
- `serde_json::Value` -- khi parse JSON, kết quả là tree structure mà bạn traverse bằng pattern matching
- Khi bạn viết `enum` lồng nhau trong Rust (ví dụ: AST cho compiler), bạn đang tạo tree structure

## Chương tiếp theo -- Binary Search Tree (BST)

Binary Tree chương này chỉ **chứa** dữ liệu -- không có quy tắc sắp xếp. Muốn tìm 1 giá trị? Phải duyệt hết O(n). Chương tiếp theo sẽ thêm **1 quy tắc duy nhất**: mọi giá trị bên trái < gốc < mọi giá trị bên phải. Quy tắc đơn giản này biến tìm kiếm từ O(n) thành O(log n) -- nhanh hơn **hàng nghìn lần** khi cây có 1 triệu node. Đó là sức mạnh của Binary Search Tree (BST).

---

---

[← Deque](../02-linear-structures/05-deque.md) | [Binary Search Tree →](./02-bst.md)
