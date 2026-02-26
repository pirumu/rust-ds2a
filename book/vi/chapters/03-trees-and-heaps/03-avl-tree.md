# AVL Tree

## Đây là gì?

Ở chương trước, ta thấy BST có thể bị "lệch" thành linked list khi chèn theo thứ tự. Giống như một cái cây ngoài đời bị gió thổi nghiêng về một phía -- nó vẫn là cây, nhưng mất cân bằng và dễ gãy.

**AVL tree** (đặt theo tên 2 nhà phát minh Adelson-Velsky và Landis) giải quyết vấn đề này. Nó là BST biết **tự cân bằng**. Sau mỗi lần chèn hoặc xóa, nếu cây bị lệch, nó sẽ tự "xoay" (rotation) để cân lại.

Quy tắc cân bằng rất đơn giản:

> Với mỗi node, chiều cao nhánh trái và nhánh phải chênh nhau **tối đa 1**.

Hiệu số này gọi là **balance factor** (hệ số cân bằng) = `height(trái) - height(phải)`. Nếu balance factor nằm trong [-1, 0, +1] thì OK. Nếu ra +2 hoặc -2 thì phải xoay.

**Khi nào dùng AVL?** Khi bạn cần **tìm kiếm nhanh** và không muốn lo về trường hợp xấu. AVL cân bằng hơn Red-Black tree, nên tìm kiếm nhanh hơn. Đổi lại, chèn/xóa có thể cần nhiều rotation hơn. Nếu đọc nhiều hơn ghi, AVL là lựa chọn tốt.

## Hoạt động như thế nào?

### Balance factor

```
        5  (bf = 1)          ← height(trái)=2, height(phải)=1
       / \
      3   7  (bf = 0)        ← height(trái)=0, height(phải)=0
     /
    1  (bf = 0)              ← leaf, không có con

Cây này cân bằng: mọi bf đều trong [-1, 0, +1]
```

Nhưng nếu ta chèn thêm 0:

```
          5  (bf = 2) ← MẤT CÂN BẰNG!
         / \
        3   7
       /
      1
     /
    0

Nhánh trái cao 3, nhánh phải cao 1 → bf = 2 → phải xoay!
```

### 4 phép xoay (rotation)

Có 4 trường hợp mất cân bằng, mỗi trường hợp dùng 1-2 phép xoay:

**Trường hợp LL** (Left-Left): Node mới nằm bên trái của con trái. Xoay phải 1 lần.

Tưởng tượng bạn nắm node giữa rồi "nhấc lên":

```
      3            2
     /            / \
    2      →     1   3
   /
  1

Node 2 "lên" làm gốc. Node 3 trở thành con phải của 2.
```

**Trường hợp RR** (Right-Right): Node mới nằm bên phải của con phải. Xoay trái 1 lần.

```
  1              2
   \            / \
    2    →     1   3
     \
      3

Node 2 "lên" làm gốc. Node 1 trở thành con trái của 2.
```

**Trường hợp LR** (Left-Right): Node mới nằm bên phải của con trái. Cần 2 bước: xoay trái con trái, rồi xoay phải gốc.

```
    3          3          2
   /          /          / \
  1    →     2    →     1   3
   \        /
    2      1

Bước 1: Xoay trái node 1 → biến thành LL
Bước 2: Xoay phải node 3 → cân bằng
```

**Trường hợp RL** (Right-Left): Node mới nằm bên trái của con phải. Cần 2 bước: xoay phải con phải, rồi xoay trái gốc.

```
  1          1            2
   \          \          / \
    3   →      2   →    1   3
   /            \
  2              3

Bước 1: Xoay phải node 3 → biến thành RR
Bước 2: Xoay trái node 1 → cân bằng
```

### Quy trình chèn

1. Chèn như BST bình thường (so sánh, rẽ trái/phải).
2. Đi ngược lên, cập nhật chiều cao từng node.
3. Ở mỗi node, tính balance factor.
4. Nếu mất cân bằng (bf = +2 hoặc -2), xoay cho đúng.

Giống như chèn một nhánh mới vào cây, rồi kiểm tra xem cây có bị nghiêng không. Nếu nghiêng thì "uốn" lại.

## Code Rust

Code đầy đủ nằm trong `src/avl_tree.rs`.

### Cấu trúc dữ liệu

```rust
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    height: i64,    // lưu chiều cao để tính balance factor nhanh
}

pub struct AVLTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}
```

Mỗi node lưu `height` để không phải tính lại mỗi lần. Tiết kiệm thời gian.

### Xoay phải (right rotation)

```rust
fn right_rotate<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    let mut new_root = root.left.take().unwrap();  // con trái sẽ lên làm gốc
    root.left = new_root.right.take();              // con phải của new_root → con trái của root cũ
    update_height(&mut root);                       // cập nhật chiều cao
    new_root.right = Some(root);                    // root cũ → con phải của new_root
    update_height(&mut new_root);
    new_root
}
```

### Tự cân bằng (rebalance)

```rust
fn rebalance<T: Ord>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    update_height(&mut node);
    let bf = balance_factor(&node);

    if bf > 1 {
        // Lệch trái
        if balance_factor(node.left.as_ref().unwrap()) < 0 {
            // LR: xoay trái con trái trước
            let left = node.left.take().unwrap();
            node.left = Some(left_rotate(left));
        }
        return right_rotate(node);  // LL hoặc LR (sau bước trên)
    }
    if bf < -1 {
        // Lệch phải
        if balance_factor(node.right.as_ref().unwrap()) > 0 {
            // RL: xoay phải con phải trước
            let right = node.right.take().unwrap();
            node.right = Some(right_rotate(right));
        }
        return left_rotate(node);  // RR hoặc RL (sau bước trên)
    }
    node  // đã cân bằng, không làm gì
}
```

### Chèn (insert)

```rust
fn insert_node<T: Ord>(node: Option<Box<Node<T>>>, val: T) -> Box<Node<T>> {
    let Some(mut n) = node else {
        return Box::new(Node::new(val));  // chỗ trống → tạo node mới
    };
    match val.cmp(&n.value) {
        Ordering::Less => n.left = Some(insert_node(n.left.take(), val)),    // nhỏ hơn → trái
        Ordering::Greater => n.right = Some(insert_node(n.right.take(), val)), // lớn hơn → phải
        Ordering::Equal => return n,  // trùng → bỏ qua
    }
    rebalance(n)  // ← đây là điểm khác biệt với BST thường!
}
```

Sau khi chèn, hàm `rebalance` tự động kiểm tra và xoay nếu cần. Đây là "phép màu" của AVL.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa |
|----------|-----------|--------|----------|
| `insert` | O(log n) | O(log n) stack | Luôn cân bằng, không bao giờ O(n) |
| `search` | O(log n) | O(log n) stack | Nhanh hơn BST thường |
| `inorder` | O(n) | O(n) | Phải thăm mọi node |
| `height` | O(1) | O(1) | Đã lưu sẵn trong node |

So sánh với BST thường:

```
BST thường (trường hợp xấu):    AVL tree (luôn luôn):
Chèn 1, 2, 3, 4, 5:            Chèn 1, 2, 3, 4, 5:

1                                    2
 \                                  / \
  2                                1   4
   \                                  / \
    3              →                 3   5
     \
      4            Height = 3 (log n)
       \           Tìm kiếm: O(log n)
        5

Height = 5 (n)
Tìm kiếm: O(n)
```

AVL đảm bảo chiều cao luôn O(log n), nên mọi thao tác đều logarithmic.

## Ví dụ

### Chèn tuần tự vẫn cân bằng

```rust
use rust_ds2a::avl_tree::AVLTree;

let mut tree = AVLTree::new();

// Chèn theo thứ tự tăng dần -- BST thường sẽ thành linked list
// Nhưng AVL tự xoay để giữ cân bằng!
for i in 1..=7 {
    tree.insert(i);
}

assert_eq!(tree.height(), 3);  // log2(7) ≈ 3, rất cân bằng
assert_eq!(tree.inorder(), vec![&1, &2, &3, &4, &5, &6, &7]);
```

### Xem rotation hoạt động

```rust
use rust_ds2a::avl_tree::AVLTree;

let mut tree = AVLTree::new();
tree.insert(3);
tree.insert(1);
tree.insert(2);  // ← LR case! Cây tự xoay

// Trước xoay: 3 → 1 → 2 (lệch)
// Sau xoay:   2 (gốc), 1 (trái), 3 (phải)

assert_eq!(tree.inorder(), vec![&1, &2, &3]);
assert_eq!(tree.height(), 2);  // cân bằng!
```
