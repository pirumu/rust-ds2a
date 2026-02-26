# Red-Black Tree

## Đây là gì?

AVL tree tự cân bằng bằng cách xoay mỗi khi cây lệch. Nhưng đôi khi nó xoay hơi "nhiều" -- mỗi lần chèn có thể cần đến 2 rotation, mỗi lần xóa có thể cần nhiều hơn.

**Red-Black tree** (cây đỏ-đen) là một cách khác để tự cân bằng. Thay vì theo dõi chiều cao, nó **tô màu** mỗi node -- đỏ hoặc đen -- rồi dùng một bộ quy tắc màu sắc để giữ cây gần cân bằng. Kết quả: ít rotation hơn AVL, nên **chèn và xóa nhanh hơn** (ít overhead hơn).

Đổi lại, Red-Black tree không cân bằng hoàn hảo như AVL. Đường đi dài nhất có thể gấp 2 lần đường đi ngắn nhất. Nhưng vẫn đủ tốt: mọi thao tác O(log n).

**Ở đâu trong thực tế?**
- C++ `std::map` -- dùng Red-Black tree
- Java `TreeMap` -- dùng Red-Black tree
- Linux kernel CFS scheduler -- dùng Red-Black tree
- Khi cần chèn/xóa nhiều, Red-Black tree tốt hơn AVL

Implementation của chúng ta dùng biến thể **Left-Leaning Red-Black tree (LLRB)** của Robert Sedgewick. LLRB đơn giản hơn nhiều: thay vì hàng chục trường hợp, chỉ cần 3 quy tắc fixup.

## Hoạt động như thế nào?

### 5 tính chất của Red-Black tree

Một Red-Black tree hợp lệ phải thỏa mãn 5 quy tắc:

| # | Quy tắc | Giải thích dễ hiểu |
|---|---------|---------------------|
| 1 | Mỗi node là **đỏ** hoặc **đen** | Chỉ 2 màu, không có màu khác |
| 2 | **Root** luôn đen | Ông tổ luôn mặc áo đen |
| 3 | Mọi **leaf** (NIL) là đen | Các "chỗ trống" được coi là đen |
| 4 | Node đỏ phải có con **đen** | Không được 2 node đỏ liên tiếp |
| 5 | Mọi đường từ node đến leaf có cùng số **node đen** | "Chiều cao đen" đồng đều |

Quy tắc 4 và 5 là quan trọng nhất. Quy tắc 4 ngăn cây bị dài quá. Quy tắc 5 đảm bảo cây "đều" về mặt node đen.

### Cách hiểu trực quan

Hãy nghĩ link đỏ = "dính" 2 node thành 1 node logic lớn hơn. Cây Red-Black thực ra là cây 2-3 tree được "mở ra".

```
     [B:10]
    /      \
  R:5     [B:15]
  / \
[B:3] [B:7]

B = Black (đen), R = Red (đỏ)

Node đỏ 5 "dính" với cha 10 → logic: {5, 10} là 1 node lớn
```

### 3 quy tắc fixup (LLRB)

Sau khi chèn (node mới luôn đỏ), có thể vi phạm quy tắc. LLRB sửa bằng 3 thao tác đơn giản:

**Quy tắc 1: Link đỏ nghiêng phải** -- xoay trái.

Trong LLRB, link đỏ chỉ được nghiêng trái. Nếu con phải đỏ mà con trái không đỏ, xoay trái:

```
  a              b
   \(đỏ)  →   (đỏ)/
    b          a
```

**Quy tắc 2: Hai link đỏ liên tiếp bên trái** -- xoay phải.

```
      c            b
    (đỏ)/   →     / \
    b           a    c
  (đỏ)/
  a
```

**Quy tắc 3: Cả 2 con đều đỏ** -- đổi màu (flip colors).

```
    B:c            R:c
   / \    →       / \
 R:a  R:b      B:a  B:b
```

Đẩy màu đỏ lên cha. Sau cùng, root luôn được ép thành đen (quy tắc 2).

### Ví dụ chèn từng bước

Chèn 1, 2, 3 vào cây rỗng:

```
Chèn 1:        Chèn 2:              Fixup (xoay trái):
  B:1            B:1                      B:2
                   \(đỏ)               (đỏ)/
                    2                   1

Chèn 3:              Fixup (đổi màu):
     B:2                   B:2
   (đỏ)/ \(đỏ)           / \
   1       3            B:1  B:3
                        (root giữ đen)
```

Sau 3 lần chèn, cây hoàn toàn cân bằng! Không cần chèn theo thứ tự đặc biệt.

## Code Rust

Code đầy đủ nằm trong `src/red_black_tree.rs`.

### Cấu trúc dữ liệu

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color { Red, Black }

struct Node<T> {
    value: T,
    color: Color,              // mỗi node có màu
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct RedBlackTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}
```

### Fixup -- 3 quy tắc LLRB

```rust
fn fixup<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    // Quy tắc 1: con phải đỏ, con trái không đỏ → xoay trái
    if is_red(&node.right) && !is_red(&node.left) {
        node = rotate_left(node);
    }
    // Quy tắc 2: con trái đỏ VÀ cháu trái cũng đỏ → xoay phải
    if is_red(&node.left) && is_red(&node.left.as_ref().unwrap().left) {
        node = rotate_right(node);
    }
    // Quy tắc 3: cả 2 con đỏ → đổi màu
    if is_red(&node.left) && is_red(&node.right) {
        flip_colors(&mut node);
    }
    node
}
```

Chỉ 3 dòng `if`. Đơn giản hơn nhiều so với Red-Black tree chuẩn!

### Chèn (insert)

```rust
pub fn insert(&mut self, val: T) {
    let root = self.root.take();
    let mut new_root = insert_node(root, val);
    new_root.color = Color::Black;    // root luôn đen (quy tắc 2)
    self.root = Some(new_root);
}
```

Node mới luôn đỏ. Sau khi chèn, `fixup` tự sửa. Cuối cùng ép root thành đen.

### Kiểm tra tính hợp lệ

```rust
pub fn is_valid(&self) -> bool {
    // Kiểm tra 3 điều:
    // 1. Root phải đen
    // 2. Không có 2 node đỏ liên tiếp
    // 3. Mọi đường root→leaf có cùng số node đen
}
```

Hàm này hữu ích để test -- đảm bảo mọi thao tác giữ đúng 5 tính chất.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ |
|----------|-----------|--------|
| `insert` | O(log n) | O(log n) stack |
| `search` | O(log n) | O(log n) stack |
| `inorder` | O(n) | O(n) |

Chiều cao Red-Black tree tối đa 2 * log(n+1). Nên mọi thao tác O(log n).

### So sánh AVL vs Red-Black

```
                    AVL             Red-Black
Cân bằng:         Chặt hơn         Lỏng hơn
Tìm kiếm:         Nhanh hơn chút   Chậm hơn chút
Chèn/xóa:         Nhiều rotation    Ít rotation
Phù hợp khi:      Đọc nhiều         Ghi nhiều
Dùng trong:        Database index    std::map, TreeMap, kernel
```

## Ví dụ

### Sử dụng cơ bản

```rust
use rust_ds2a::red_black_tree::RedBlackTree;

let mut tree = RedBlackTree::new();
for v in [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9] {
    tree.insert(v);
}

assert!(tree.search(&7));
assert!(!tree.search(&99));
assert!(tree.is_valid());  // mọi tính chất đều đúng
```

### Chèn tuần tự vẫn cân bằng

```rust
use rust_ds2a::red_black_tree::RedBlackTree;

let mut tree = RedBlackTree::new();
for i in 1..=20 {
    tree.insert(i);
}

// Chèn 1, 2, 3, ..., 20 theo thứ tự
// BST thường → linked list (height = 20)
// Red-Black tree → vẫn cân bằng!
assert!(tree.is_valid());
let sorted: Vec<&i32> = tree.inorder();
assert_eq!(sorted.len(), 20);
assert_eq!(*sorted[0], 1);
assert_eq!(*sorted[19], 20);
```
