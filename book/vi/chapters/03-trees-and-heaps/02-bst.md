# Binary Search Tree (BST)

## Đây là gì?

Bạn đã bao giờ tra **từ điển giấy** chưa? Bạn không đọc từ đầu đến cuối. Bạn mở giữa cuốn sách, nhìn từ ở trang đó, rồi quyết định lật sang trái hay phải. Mỗi lần lật, bạn loại bỏ được **một nửa** số trang còn lại. Rất nhanh!

**Binary Search Tree** (cây tìm kiếm nhị phân, gọi tắt BST) hoạt động y hệt vậy, nhưng ở dạng cây. Quy tắc vàng:

> Với mỗi node: **tất cả giá trị bên trái < node < tất cả giá trị bên phải**

Nhờ quy tắc này, mỗi lần so sánh, ta loại bỏ được một nửa cây. Tìm kiếm, chèn, xóa đều nhanh.

**Tại sao không dùng mảng đã sắp xếp?** Vì mảng chèn/xóa phải dịch chuyển phần tử (O(n)). BST chèn/xóa chỉ cần đi theo nhánh cây (O(h), h = chiều cao).

BST là nền tảng cho nhiều cấu trúc nâng cao: AVL tree, Red-Black tree. Duyệt in-order cho kết quả **đã sắp xếp** -- đây là tính chất cực kỳ hữu ích.

## Hoạt động như thế nào?

### Tính chất BST

```
        5
       / \
      3   7
     / \ / \
    1  4 6  8

Node 5:  bên trái {1,3,4} đều < 5  |  bên phải {6,7,8} đều > 5
Node 3:  bên trái {1} < 3           |  bên phải {4} > 3
Node 7:  bên trái {6} < 7           |  bên phải {8} > 7
```

Mọi node đều thỏa mãn: trái < gốc < phải. Đây chính là "từ điển dạng cây".

### Tìm kiếm (search)

Giống tra từ điển: so sánh rồi rẽ trái hoặc phải.

```
Tìm số 6 trong cây:

        5           5 < 6 → rẽ phải
       / \
      3  [7]        7 > 6 → rẽ trái
         /
       [6]          6 == 6 → tìm thấy!
```

Mỗi bước loại bỏ một nửa cây. Giống binary search trên mảng.

### Chèn (insert)

Chèn cũng đi theo cùng logic: so sánh rồi rẽ, đến chỗ trống thì đặt vào.

```
Chèn 6 vào cây:           Kết quả:
      5                       5
     / \                     / \
    3   7                   3   7
                               /
    6 < 7 → rẽ trái          6  ← node mới
    chỗ trống → đặt vào
```

### Xóa (delete) -- 3 trường hợp

Xóa phức tạp hơn vì phải giữ tính chất BST. Có 3 trường hợp:

**Trường hợp 1: Node lá** (không có con) -- xóa thẳng, đơn giản.

```
Xóa 4:
      5              5
     / \            / \
    3   7    →     3   7
     \
      4  ← xóa
```

Giống xóa một người không có con trong gia phả. Không ảnh hưởng ai.

**Trường hợp 2: Node có 1 con** -- thay node bằng con duy nhất của nó.

```
Xóa 7:
      5              5
     / \            / \
    3   7    →     3   6
       /
      6

Con duy nhất (6) "lên thế chỗ" cha (7).
```

Giống khi một người mất, con duy nhất thừa kế vị trí.

**Trường hợp 3: Node có 2 con** -- đây là trường hợp khó nhất!

Ta tìm **người kế nhiệm** (in-order successor) = giá trị nhỏ nhất trong nhánh phải. Rồi hoán đổi giá trị, rồi xóa người kế nhiệm đó.

```
Xóa 5:
      5              6          Bước 1: Tìm successor của 5
     / \            / \                  = nhỏ nhất bên phải = 6
    3   7    →     3   7        Bước 2: Thay 5 bằng 6
       /                        Bước 3: Xóa node 6 cũ (trường hợp 1 hoặc 2)
      6
```

Tại sao chọn successor? Vì nó là giá trị nhỏ nhất lớn hơn node cần xóa, đảm bảo tính chất BST vẫn đúng.

## Code Rust

Code đầy đủ nằm trong `src/bst.rs`.

### Cấu trúc dữ liệu

```rust
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct BST<T: Ord> {
    root: Option<Box<Node<T>>>,
}
```

`T: Ord` nghĩa là kiểu T phải so sánh được (vì BST cần so sánh để quyết định trái/phải).

### Chèn (insert)

```rust
pub fn insert(&mut self, val: T) {
    fn insert_node<T: Ord>(node: &mut Option<Box<Node<T>>>, val: T) {
        match node {
            None => {
                // Chỗ trống → tạo node mới
                *node = Some(Box::new(Node {
                    value: val, left: None, right: None,
                }));
            }
            Some(n) => {
                if val < n.value { insert_node(&mut n.left, val); }  // nhỏ hơn → đi trái
                else if val > n.value { insert_node(&mut n.right, val); }  // lớn hơn → đi phải
                // bằng → bỏ qua (không cho trùng)
            }
        }
    }
    insert_node(&mut self.root, val);
}
```

### Xóa (delete)

```rust
pub fn delete(&mut self, val: &T) {
    // Đệ quy tìm node cần xóa, rồi xử lý 3 trường hợp:
    // - Không có con: set link thành None
    // - Một con: thay bằng con đó
    // - Hai con: tìm in-order successor, hoán đổi, xóa successor
}
```

### Duyệt in-order (trả về thứ tự sắp xếp)

```rust
pub fn inorder(&self) -> Vec<&T> {
    // Trái → Gốc → Phải
    // Kết quả luôn được sắp xếp tăng dần!
}
```

## Độ phức tạp

| Thao tác | Trung bình | Xấu nhất (cây bị lệch) |
|----------|------------|------------------------|
| `insert` | O(log n) | O(n) |
| `search` | O(log n) | O(n) |
| `delete` | O(log n) | O(n) |
| `min` / `max` | O(log n) | O(n) |
| `inorder` | O(n) | O(n) |
| **Bộ nhớ** | -- | **O(n)** |

### Tại sao có trường hợp xấu nhất O(n)?

Khi chèn theo thứ tự tăng dần: 1, 2, 3, 4, 5... cây bị "lệch" thành đường thẳng, giống linked list:

```
Cây cân bằng (tốt):     Cây bị lệch (tệ):
      4                  1
     / \                  \
    2   6                  2
   / \ / \                  \
  1  3 5  7                  3
                              \
Height = 3                     4
Tìm kiếm: O(log n)             \
                                 5
                              Height = 5
                              Tìm kiếm: O(n)
```

Đây là lý do ta cần cây tự cân bằng (AVL, Red-Black) ở các chương sau!

## Ví dụ

### Các thao tác cơ bản

```rust
use rust_ds2a::bst::BST;

let mut bst = BST::new();
for v in [5, 3, 7, 1, 4, 6, 8] {
    bst.insert(v);
}

assert!(bst.search(&5));          // tìm thấy
assert!(!bst.search(&99));        // không có
assert_eq!(bst.min(), Some(&1));  // nhỏ nhất = đi trái tận cùng
assert_eq!(bst.max(), Some(&8));  // lớn nhất = đi phải tận cùng

// Duyệt in-order cho kết quả đã sắp xếp!
let sorted: Vec<&i32> = bst.inorder();
assert_eq!(sorted, vec![&1, &3, &4, &5, &6, &7, &8]);
```

### Xóa node

```rust
use rust_ds2a::bst::BST;

let mut bst = BST::new();
for v in [5, 3, 7, 1, 4, 6, 8] {
    bst.insert(v);
}

bst.delete(&5);  // xóa root (trường hợp 2 con)
assert!(!bst.search(&5));
assert_eq!(bst.inorder(), vec![&1, &3, &4, &6, &7, &8]);
```
