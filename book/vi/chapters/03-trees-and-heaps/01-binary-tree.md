# Binary Tree

## Đây là gì?

Bạn biết **cây gia phả** (family tree) không? Ông bà ở trên cùng, bố mẹ ở giữa, con cháu ở dưới. Mỗi người có thể có con hoặc không. **Binary tree** (cây nhị phân) cũng giống vậy -- nhưng có một quy tắc: mỗi "người" (node) chỉ được có **tối đa 2 con**, gọi là con trái (left child) và con phải (right child).

Tại sao lại cần cấu trúc này? Vì rất nhiều thứ trong máy tính có dạng phân nhánh:
- **File system**: thư mục chứa thư mục con
- **HTML**: thẻ cha chứa thẻ con
- **Biểu thức toán**: `(1 + 2) * 3` có cấu trúc cây

Một vài thuật ngữ cần biết:
- **Root** (gốc): node trên cùng, giống ông tổ trong gia phả
- **Leaf** (lá): node không có con, giống thế hệ trẻ nhất chưa có con
- **Depth** (độ sâu): khoảng cách từ root đến node đó (root có depth = 0)
- **Height** (chiều cao): khoảng cách xa nhất từ root đến leaf

Trong Rust, ta dùng `Option<Box<Node<T>>>` để biểu diễn con trái/con phải. `None` nghĩa là "không có con". Ta chèn theo **level-order** (theo từng tầng, từ trái sang phải) để cây luôn "đầy đều".

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

### Traversal -- cách "thăm" từng thành viên

Có 4 cách đi thăm tất cả node trong cây. Giống như 4 cách khác nhau để chào hỏi mọi người trong gia đình:

Với cây `[1, 2, 3, 4, 5, 6, 7]`:

```
            1
           / \
          2   3
         / \ / \
        4  5 6  7
```

**In-order** (Trái -> Gốc -> Phải): Đi sâu vào nhánh trái trước, rồi thăm gốc, rồi sang phải.
```
Kết quả: 4  2  5  1  6  3  7
         ↑trái↑ ↑gốc↑ ↑phải↑
```

**Pre-order** (Gốc -> Trái -> Phải): Thăm gốc trước, rồi đi xuống.
```
Kết quả: 1  2  4  5  3  6  7
```

**Post-order** (Trái -> Phải -> Gốc): Thăm con trước, gốc sau cùng.
```
Kết quả: 4  5  2  6  7  3  1
```

**Level-order** (theo từng tầng): Đi từ trên xuống, trái sang phải.
```
Kết quả: 1  2  3  4  5  6  7
```

Mẹo nhớ: tên gọi cho biết **gốc ở đâu**. Pre = gốc đầu tiên. In = gốc ở giữa. Post = gốc cuối cùng.

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

`Option<Box<Node<T>>>` nghĩa là: có thể có (`Some`) hoặc không có (`None`) một node được cấp phát trên heap.

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
