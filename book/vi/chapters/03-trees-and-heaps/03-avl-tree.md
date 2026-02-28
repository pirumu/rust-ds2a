# AVL Tree

> 💡 **Đừng lo lắng:** Rotation nghe đáng sợ nhưng thực ra chỉ có 4 trường hợp, và 2 trong đó là bản mirror của nhau. Nếu bạn hiểu BST ở chương trước, AVL chỉ thêm 1 bước "xoay lại cho cân" sau khi insert. Đọc chậm, trace từng bước, bạn sẽ thấy nó không khó như tiếng đồn.

## Đây là gì?

AVL Tree là BST tự cân bằng -- sau mỗi lần insert hay delete, nó tự xoay (rotate) để không bị lệch. Trong thực tế, Rust std dùng B-Tree, Java dùng Red-Black -- nhưng hiểu AVL giúp bạn hiểu **tại sao** self-balancing trees hoạt động.

### Vấn đề mà AVL giải quyết

Ở chương trước, ta thấy BST có thể bị "lệch" thành linked list khi chèn theo thứ tự. Giống như một cái cây ngoài đời bị gió thổi nghiêng về một phía -- nó vẫn là cây, nhưng mất cân bằng và dễ gãy.

Cùng data `[1, 2, 3, 4, 5, 6, 7]`, nhìn sự khác biệt:

```
BST insert tuần tự:          AVL insert tuần tự:

1                                  4
 \                               /   \
  2                             2      6
   \                           / \    / \
    3                         1   3  5   7
     \
      4                   Height = 3 (log n)
       \                  Search: O(log n) LUÔN LUÔN
        5
         \
          6
           \
            7

Height = 7 (n)
Search: O(n)
```

Giá phải trả cho sự cân bằng này là gì? Mỗi lần insert, AVL phải kiểm tra balance factor và có thể xoay (1-2 phép xoay). Nhưng xoay chỉ O(1) -- giá rẻ so với lợi ích O(log n) cho **mọi** thao tác.

| | BST thường | AVL Tree |
|---|-----------|----------|
| Insert best case | O(log n) | O(log n) |
| Insert worst case | **O(n)** | **O(log n)** ← cải thiện |
| Search worst case | **O(n)** | **O(log n)** ← cải thiện |
| Overhead mỗi insert | Không | Kiểm tra bf + xoay (O(1)) |
| Code complexity | Đơn giản | Phức tạp hơn |
| Mỗi node lưu thêm | Không | height (1 field) |

**AVL tree** (đặt theo tên 2 nhà phát minh Adelson-Velsky và Landis, 1962) là BST biết **tự cân bằng**. Sau mỗi lần chèn hoặc xóa, nếu cây bị lệch, nó sẽ tự "xoay" (rotation) để cân lại.

Quy tắc cân bằng rất đơn giản:

> Với mỗi node, chiều cao nhánh trái và nhánh phải chênh nhau **tối đa 1**.

Nói cách khác: **AVL = BST + rebalance**. Mọi thứ BST làm được, AVL làm được. Nhưng AVL đảm bảo luôn nhanh.

## Balance Factor chi tiết

Tại sao phải hiểu balance factor (bf) kỹ? Vì bf quyết định **khi nào** cần xoay và **xoay kiểu gì**. Nếu tính bf sai, cây sẽ xoay sai hoặc không xoay khi cần.

**Balance factor** = `height(trái) - height(phải)`.

- bf ∈ {-1, 0, +1} → **cân bằng**, không cần xoay
- bf = +2 → **lệch trái**, cần xoay
- bf = -2 → **lệch phải**, cần xoay

### Tính bf cho mọi node trong cây

```
            10 (bf=?)
           /  \
         5     15 (bf=?)
        / \      \
       3   7     20 (bf=?)
      /
     1
```

Tính từ dưới lên (leaf trước, root sau):

```
  Node 1:  h(left)=0, h(right)=0 → bf = 0
  Node 3:  h(left)=1, h(right)=0 → bf = +1
  Node 7:  h(left)=0, h(right)=0 → bf = 0
  Node 5:  h(left)=2, h(right)=1 → bf = +1
  Node 20: h(left)=0, h(right)=0 → bf = 0
  Node 15: h(left)=0, h(right)=1 → bf = -1
  Node 10: h(left)=3, h(right)=2 → bf = +1

Tất cả bf ∈ {-1, 0, +1} → CÂN BẰNG ✓
```

### Ví dụ mất cân bằng

Insert node 0 vào cây trên:

```
            10 (bf = +2!) ← MẤT CÂN BẰNG
           /  \
         5     15
        / \      \
       3   7     20
      /
     1
    /
   0  ← node mới

Node 3: bf = +2 ← cũng mất cân bằng!
Node 10: bf = +2 ← cũng mất cân bằng!

Ta sửa node MẤT CÂN BẰNG GẦN NODE MỚI NHẤT → node 3
```

Khi insert, ta đi ngược lên từ node mới, cập nhật height và check bf. Node **đầu tiên** có |bf| > 1 là chỗ cần xoay. Xoay xong, các node phía trên thường tự cân bằng lại.

## 4 phép xoay (rotation)

Đây là phần core của AVL. Nếu bạn hiểu rotation, bạn hiểu AVL. Nếu chưa hiểu, đừng lo -- đọc chậm, vẽ ra giấy, trace từng bước.

Có 4 trường hợp mất cân bằng, mỗi trường hợp dùng 1-2 phép xoay:

```
Mẹo nhớ:
  LL → 1 xoay phải          (thẳng → 1 bước)
  RR → 1 xoay trái          (thẳng → 1 bước)
  LR → xoay trái + xoay phải (gấp khúc → 2 bước)
  RL → xoay phải + xoay trái (gấp khúc → 2 bước)
```

### Trường hợp LL (Left-Left) -- Xoay phải

Node mới nằm bên **trái** của con **trái**. Cây bị lệch trái thẳng → xoay phải 1 lần.

**Ví dụ đơn giản (3 node):**

```
      3            2
     /            / \
    2      →     1   3
   /
  1

Node 2 "lên" làm gốc. Node 3 trở thành con phải của 2.
```

**Ví dụ chi tiết với subtree:**

Đây mới là trường hợp thực tế -- khi xoay, subtree phải được "chuyển" đúng chỗ:

```
Trước xoay:
        30 (bf=+2)
       /  \
      20    D
     /  \
    10    C
   /  \
  A    B

A, B, C, D là subtree (có thể rỗng hoặc có nhiều node)

Bước xoay phải tại node 30:
  1. new_root = 20
  2. 20.right (subtree C) → chuyển thành 30.left
  3. 30 → chuyển thành 20.right

Sau xoay:
        20
       /  \
      10    30
     /  \  /  \
    A    B C    D

Kiểm tra BST property:
  A < 10 < B < 20 < C < 30 < D  ← vẫn đúng!
```

**Subtree C là điểm then chốt:**
- Trước xoay: C là con phải của 20 → C > 20
- Sau xoay: C là con trái của 30 → C < 30
- Cả hai đều đúng vì trong BST gốc: 20 < C < 30

### Trường hợp RR (Right-Right) -- Xoay trái

Mirror của LL. Node mới nằm bên **phải** của con **phải**. Xoay trái 1 lần.

```
Trước xoay:                    Sau xoay:
    10 (bf=-2)                     20
   /  \                           /  \
  A    20                        10    30
      /  \                      /  \  /  \
     B    30                   A    B C    D
         /  \
        C    D

Bước xoay trái tại node 10:
  1. new_root = 20
  2. 20.left (subtree B) → chuyển thành 10.right
  3. 10 → chuyển thành 20.left
```

### Trường hợp LR (Left-Right) -- 2 bước

Node mới nằm bên **phải** của con **trái**. Tạo hình "gấp khúc" (zig-zag). Phải "duỗi thẳng" thành LL trước, rồi mới xoay.

```
Trước:
      30 (bf=+2)
     /
    10 (bf=-1)
      \
       20

Bước 1: Left rotate tại node 10 (duỗi thẳng thành LL)
      30
     /
    20
   /
  10

Bước 2: Right rotate tại node 30
      20
     /  \
    10    30
```

Tại sao cần 2 bước? Vì "gấp khúc": 30 lệch trái, nhưng con trái (10) lại lệch phải. Phải "duỗi thẳng" thành LL trước, rồi mới xoay LL được.

### Trường hợp RL (Right-Left) -- 2 bước

Mirror của LR. Node mới nằm bên **trái** của con **phải**.

```
Trước:
  10 (bf=-2)
    \
     30 (bf=+1)
    /
   20

Bước 1: Right rotate tại node 30 (duỗi thẳng thành RR)
  10
    \
     20
       \
        30

Bước 2: Left rotate tại node 10
      20
     /  \
    10    30
```

### Tổng kết rotation

```
Tên gọi = hướng đi xuống từ node mất cân bằng:
  LL = đi trái, rồi trái  → xoay phải
  RR = đi phải, rồi phải  → xoay trái
  LR = đi trái, rồi phải  → xoay trái (duỗi) + xoay phải
  RL = đi phải, rồi trái  → xoay phải (duỗi) + xoay trái

Thẳng (LL, RR) = 1 bước
Gấp khúc (LR, RL) = 2 bước
```

## Full Insert Trace [1, 2, 3, 4, 5, 6, 7]

Đây là section quan trọng nhất của chương. Nếu bạn hiểu được cây biến đổi từng bước ở đây, bạn hiểu AVL. Hãy lấy giấy ra vẽ theo -- đọc không thôi sẽ khó hình dung.

**Insert 1:**
```
  1           (bf=0, OK)
```

**Insert 2:**
```
  1           (bf=-1, OK -- chưa mất cân bằng)
   \
    2
```

**Insert 3:**
```
  1 (bf=-2!)  → RR case → left rotate tại 1
   \
    2
     \
      3
```

Sau rotate:
```
    2         (bf=0, OK)
   / \
  1   3
```

**Insert 4:**
```
    2 (bf=-1)
   / \
  1   3 (bf=-1)
       \
        4         (OK, chưa mất cân bằng)
```

**Insert 5:**
```
    2 (bf=-2!)
   / \
  1   3 (bf=-2!)
       \
        4
         \
          5

Node gần nhất mất cân bằng: node 3 → RR → left rotate tại 3
```

Sau rotate:
```
    2 (bf=-1)
   / \
  1   4
     / \
    3   5       (OK)
```

**Insert 6:**
```
    2 (bf=-2!)
   / \
  1   4
     / \
    3   5
         \
          6

RR tại node 2 → left rotate
```

Sau rotate:
```
       4
     /   \
    2     5
   / \     \
  1   3     6    (tất cả bf OK)
```

**Insert 7:**
```
       4
     /   \
    2     5 (bf=-2!)
   / \     \
  1   3     6
             \
              7

RR tại node 5 → left rotate
```

Sau rotate:
```
       4
     /   \
    2     6
   / \   / \
  1   3 5   7    ← PERFECT BINARY TREE!

Height = 3 = ⌊log₂(7)⌋ + 1
```

Cùng data `[1, 2, 3, 4, 5, 6, 7]`, BST thường tạo cây lệch height = 7. AVL tạo cây hoàn hảo height = 3. Đó là sức mạnh của self-balancing. Cây bị gió thổi nghiêng bao nhiêu lần, AVL đều uốn lại được.

### Quy trình chèn tổng quát

1. Chèn như BST bình thường (so sánh, rẽ trái/phải)
2. Đi ngược lên, cập nhật chiều cao từng node
3. Ở mỗi node, tính balance factor
4. Nếu mất cân bằng (bf = +2 hoặc -2), xoay cho đúng

Giống như chèn một nhánh mới vào cây, rồi kiểm tra xem cây có bị nghiêng không. Nếu nghiêng thì "uốn" lại.

## Delete trong AVL

Tại sao cần biết delete? Vì nếu chỉ biết insert, bạn chỉ biết nửa câu chuyện. Tin vui: delete AVL = delete BST (3 trường hợp từ chương trước) + rebalance đi ngược lên. Chính xác **cùng rebalance function** dùng cho insert!

### Trace ví dụ delete

Bắt đầu với cây hoàn hảo từ insert trace:

```
        4
       / \
      2   6
     / \ / \
    1  3 5  7
```

**Xóa node 1** (leaf -- trường hợp đơn giản nhất):

```
        4
       / \
      2   6
       \ / \
       3 5  7

Check bf đi ngược lên:
  Node 2: bf = 0-1 = -1 → OK
  Node 4: bf = 2-2 = 0 → OK
  Không cần xoay!
```

**Xóa tiếp node 3:**

```
        4
       / \
      2   6
         / \
        5   7

  Node 2: bf = 0 → OK (leaf)
  Node 4: bf = 1-2 = -1 → OK
  Vẫn cân bằng!
```

**Xóa tiếp node 2:**

```
        4 (bf = 0-2 = -2!) ← MẤT CÂN BẰNG
          \
           6
          / \
         5   7

  RR case → left rotate tại 4:

        6
       / \
      4   7
       \
        5
```

### Delete có thể gây cascade rotation

Khác với insert (chỉ cần tối đa 2 rotation), delete có thể gây **cascade** -- xoay 1 node làm node cha mất cân bằng → phải xoay tiếp. Worst case cần O(log n) rotation khi delete. Nhưng tổng thời gian vẫn O(log n) vì mỗi rotation chỉ O(1).

Code recursive tự handle điều này vì rebalance được gọi ở **mỗi level** khi đệ quy quay lên (backtrack).

## Ownership trong Rotation -- Tại sao Rust khó hơn?

Nếu bạn đọc code rotation mà thấy `.take()` và `.unwrap()` xuất hiện khắp nơi, đây là lý do. Rotation trong Rust khó hơn C/Java vì **ownership** -- borrow checker không cho phép 2 mutable reference tới cùng 1 data.

### Right rotate -- từng bước ownership

```rust
fn right_rotate<T>(mut root: Box<Node<T>>) -> Box<Node<T>> {
    // root.left.take() làm 2 việc:
    // 1. Lấy giá trị ra (Some → inner value)
    // 2. Đặt root.left = None (tránh 2 owner cùng 1 data)
    let mut new_root = root.left.take().unwrap();

    // Bây giờ:
    //   root.left = None (đã take)
    //   new_root = node cũ ở root.left
    //   new_root.right = subtree C (sắp chuyển cho root)

    root.left = new_root.right.take();  // C chuyển sang root.left
    // Bây giờ: new_root.right = None

    update_height(&mut root);           // cập nhật height root trước
    new_root.right = Some(root);        // root cũ trở thành con phải
    update_height(&mut new_root);       // rồi mới update new_root
    new_root
}
```

Trong C/C++, rotation chỉ cần swap pointer. Trong Rust, borrow checker không cho 2 mutable reference cùng lúc. `.take()` giải quyết bằng cách tạm thời **move ownership ra**, sửa xong rồi gắn lại.

Nếu bạn thấy `.take()` và `.unwrap()` khó hiểu, hãy nhớ:
- `.take()` = "tạm lấy ra" (biến Option thành None, trả về giá trị bên trong)
- `.unwrap()` = "chắc chắn có" (vì ta biết nó không phải None)

Pattern này xuất hiện **rất nhiều** khi làm tree trong Rust. Quen rồi sẽ thấy tự nhiên.

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

Mỗi node lưu `height` để không phải tính lại mỗi lần. Tiết kiệm thời gian -- `height()` là O(1) thay vì O(n).

### Helper functions

```rust
/// Tính height: None → 0, Some(node) → node.height
fn height<T>(node: &Option<Box<Node<T>>>) -> i64 {
    node.as_ref().map_or(0, |n| n.height)
}

/// bf = height(trái) - height(phải)
fn balance_factor<T>(node: &Node<T>) -> i64 {
    height(&node.left) - height(&node.right)
}

/// Cập nhật height = 1 + max(h_left, h_right)
fn update_height<T>(node: &mut Node<T>) {
    node.height = 1 + height(&node.left).max(height(&node.right));
}
```

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
    rebalance(n)  // ← đây là điểm khác biệt DUY NHẤT với BST thường!
}
```

Sau khi chèn, hàm `rebalance` tự động kiểm tra và xoay nếu cần. Đây là "phép màu" của AVL -- chỉ 1 dòng `rebalance(n)` biến BST thường thành self-balancing tree.

## AVL vs Red-Black Tree vs B-Tree

Tại sao cần biết? Vì phỏng vấn rất hay hỏi "khi nào dùng cái nào?". Và trong thực tế, bạn sẽ gặp cả 3 loại này.

| Tiêu chí | AVL | Red-Black | B-Tree |
|----------|-----|-----------|--------|
| Cân bằng | Rất chặt (\|bf\| ≤ 1) | Lỏng hơn (black-height) | Luôn perfectly balanced |
| Search | Nhanh nhất (cây thấp nhất) | Chậm hơn AVL một chút | Tốt cho disk (ít I/O) |
| Insert/Delete | Nhiều rotation hơn | Ít rotation hơn (≤ 3) | Tối ưu cho batch write |
| Mỗi node lưu thêm | height (integer) | color (1 bit) | Nhiều key/children |
| Dùng ở đâu | Read-heavy systems | Linux kernel, Java TreeMap | Database index, filesystem, **Rust BTreeMap** |
| Khi nào chọn | Đọc >> ghi | Ghi nhiều, cần worst-case tốt | Data lớn, disk-based |

Tóm gọn:
- **AVL** tối ưu cho **đọc nhiều** vì cây luôn thấp nhất có thể
- **Red-Black** tối ưu cho **ghi nhiều** vì ít rotation hơn
- **B-Tree** tối ưu cho **data lớn** vì giảm số lần đọc disk

Tại sao Rust std chọn B-Tree thay vì AVL/Red-Black? Vì B-Tree **cache-friendly** hơn -- mỗi node chứa nhiều key liên tiếp trên memory, tận dụng cache line. Với modern CPU, cache miss tốn kém hơn vài phép so sánh extra.

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa |
|----------|-----------|--------|----------|
| `insert` | O(log n) | O(log n) stack | Luôn cân bằng, không bao giờ O(n) |
| `delete` | O(log n) | O(log n) stack | Có thể cascade rotation, nhưng tổng vẫn O(log n) |
| `search` | O(log n) | O(log n) stack | Nhanh hơn BST worst case |
| `inorder` | O(n) | O(n) | Phải thăm mọi node |
| `height` | O(1) | O(1) | Đã lưu sẵn trong node |

So sánh chi tiết:

| | BST worst | BST avg | AVL (luôn) | HashMap |
|---|----------|---------|------------|---------|
| Search | O(n) | O(log n) | O(log n) | O(1) avg |
| Insert | O(n) | O(log n) | O(log n) | O(1) avg |
| Delete | O(n) | O(log n) | O(log n) | O(1) avg |
| Sorted iteration | O(n) | O(n) | O(n) | O(n log n) sort |
| Min/Max | O(n) | O(log n) | O(log n) | O(n) |

AVL thắng BST ở worst case. HashMap thắng AVL ở lookup/insert. Nhưng AVL thắng HashMap khi cần **thứ tự** (sorted iteration, range query, min/max).

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

// Trước xoay: 3 → 1 → 2 (gấp khúc)
// Sau xoay:   2 (gốc), 1 (trái), 3 (phải)

assert_eq!(tree.inorder(), vec![&1, &2, &3]);
assert_eq!(tree.height(), 2);  // cân bằng!
```

### So sánh với BST trên data lớn

```rust
use rust_ds2a::avl_tree::AVLTree;

let mut tree = AVLTree::new();

// 1000 node chèn tuần tự -- BST thường sẽ có height = 1000
for i in 1..=1000 {
    tree.insert(i);
}

// AVL giữ height ≈ log2(1000) ≈ 10
assert!(tree.height() <= 15);  // thực tế khoảng 10-11
```

## Những cái bẫy hay gặp

### a) Quên cập nhật height sau khi xoay

❌ Xoay node mà không update height → bf tính sai → cây hỏng dần

✅ Update height theo thứ tự **bottom-up**: node con trước, node cha sau. Trong code rotate, `update_height(&mut root)` phải gọi **trước** khi gắn root vào new_root.

💡 Nhìn code `right_rotate`: update root trước, rồi mới gắn vào new_root, rồi update new_root. Thứ tự này không được đảo.

### b) Nhầm thứ tự rotation trong LR/RL

❌ LR case: xoay phải gốc trước, rồi xoay trái con → sai!

✅ LR = xoay trái **con trái** trước (duỗi thẳng), rồi xoay phải **gốc**.

💡 Mẹo: tên gọi = hướng đi xuống (Left-Right = đi trái rồi phải). Rotation thì làm **ngược lại từ dưới lên**: xoay trái ở dưới, xoay phải ở trên.

### c) Nhầm lẫn giữa insert và delete rebalance

❌ Nghĩ rằng delete cũng chỉ cần 1 lần rebalance như insert.

✅ Insert: tối đa 1 rebalance (tại node mất cân bằng gần nhất). Delete: có thể cần rebalance **nhiều node** đi lên root (cascade).

💡 Code recursive tự handle điều này vì `rebalance` được gọi ở mỗi level khi backtrack. Không cần xử lý gì đặc biệt.

### d) Dùng AVL khi không cần sorted order

❌ Cần insert + lookup nhanh → dùng AVL

✅ Nếu không cần sorted iteration, `HashMap` O(1) nhanh hơn AVL O(log n). AVL (và BST nói chung) chỉ thắng khi cần **thứ tự**.

💡 Hỏi: "Tôi có cần duyệt theo thứ tự, tìm min/max, hoặc range query không?" Nếu không → HashMap.

### e) Tự implement AVL trong production

❌ Viết AVL tree từ đầu cho project thật.

✅ Dùng `BTreeMap`/`BTreeSet` trong Rust std. Chúng đã được tối ưu kỹ, test kỹ, và cache-friendly hơn tree dùng `Box<Node>`.

💡 Tự implement AVL chỉ nên làm khi **học** hoặc khi có requirement đặc biệt mà std không đáp ứng.

## Khi nào dùng / không nên dùng

| Tình huống | AVL? | Thay bằng gì? | Tại sao? |
|------------|------|---------------|----------|
| Read-heavy sorted data, ít write | ✅ | -- | AVL tối ưu search (cây thấp nhất) |
| Cần guaranteed O(log n) mọi thao tác | ✅ | -- | Không có worst case O(n) |
| Write-heavy, insert/delete liên tục | ❌ | Red-Black Tree | Ít rotation hơn |
| Data lớn, disk-based | ❌ | B-Tree | Cache-friendly, ít disk I/O |
| Chỉ cần insert + lookup, không cần sorted | ❌ | HashMap | O(1) > O(log n) |
| Priority queue (chỉ cần min/max) | ❌ | BinaryHeap | Đơn giản và nhanh hơn |
| Production Rust code | ⚠️ | BTreeMap/BTreeSet | Std đã tối ưu sẵn |
| Học self-balancing concept | ✅ | -- | AVL trực quan nhất để hiểu rotation |

## Luyện nhận diện Pattern

**Bài 1: Xác định loại rotation**

Cho cây AVL đang cân bằng với các node `[10, 5, 15, 3, 7]`. Insert giá trị 6. Xác định rotation cần thực hiện (LL/RR/LR/RL) mà **không cần code** -- chỉ cần nhìn cây.

**Gợi ý:** Tìm node mất cân bằng gần node mới nhất, rồi nhìn hướng đi xuống 2 bước → tên case.

<details>
<summary>Đáp án</summary>

Cây trước insert 6:

```
        10
       /  \
      5    15
     / \
    3   7
```

Insert 6 → đi vào bên trái của 7:

```
        10 (bf=+2!)
       /  \
      5    15
     / \
    3   7
       /
      6
```

Node mất cân bằng gần nhất: 10. Đi xuống: **trái** (đến 5), rồi **phải** (đến 7) → **LR case**.

Bước 1: Left rotate tại 5. Bước 2: Right rotate tại 10.

Kết quả:

```
        7
       / \
      5   10
     /   /  \
    3   6   15  (hoặc variant tùy implementation)
```

Complexity: Xác định case O(log n) -- theo đường đi từ root đến node mới.

</details>

**Bài 2: Trace insert sequence**

Insert `[10, 20, 30, 15, 25]` vào AVL tree rỗng. Vẽ cây sau mỗi bước, bao gồm rotation.

**Gợi ý:** Trace từng bước giống section "Full Insert Trace" phía trên.

<details>
<summary>Đáp án</summary>

```
Insert 10:   10

Insert 20:   10
              \
               20       (bf=-1, OK)

Insert 30:   10 (bf=-2!) → RR → left rotate
              \
               20
                \
                 30

Sau rotate:    20
              /  \
             10   30

Insert 15:    20
             /  \
            10   30
              \
               15       (bf OK, 20 có bf=0)

Insert 25:    20
             /  \
            10   30 (bf=+1)
              \  /
              15 25     (bf OK!)
```

Cây cuối cùng:

```
       20
      /  \
    10    30
      \   /
      15 25
```

Không cần rotation cho insert 15 và 25! Vì cây vẫn cân bằng.

Complexity: 5 inserts, 1 rotation. Time O(n log n) cho n inserts.

</details>

**Bài 3: Đếm số rotation**

Insert N số ngẫu nhiên vào AVL tree. Trong worst case, tối đa bao nhiêu rotation xảy ra cho **1 lần insert**? Cho **toàn bộ N lần insert**?

**Gợi ý:** Mỗi insert đi xuống rồi đi lên, rebalance ở mỗi level. Nhưng insert có tính chất đặc biệt so với delete...

<details>
<summary>Đáp án</summary>

**1 lần insert:** Tối đa **2 rotation** (cho LR hoặc RL case). Tại sao không nhiều hơn? Vì sau 1 lần rebalance, subtree đã xoay sẽ có height bằng hoặc nhỏ hơn trước khi insert → các node phía trên không bị ảnh hưởng.

**N lần insert:** Tối đa **2N rotation** (mỗi insert tối đa 2). Nhưng thực tế ít hơn nhiều -- phần lớn insert không cần rotation.

So sánh với delete: 1 lần delete có thể cần **O(log n) rotation** (cascade). Nhưng tổng thời gian vẫn O(log n) vì mỗi rotation là O(1).

Complexity: Insert O(log n) time, tối đa 2 rotations. Delete O(log n) time, tối đa O(log n) rotations.

</details>

## AVL trong Rust ecosystem

- `std::collections::BTreeMap` / `BTreeSet` -- Rust chọn B-Tree thay vì AVL/Red-Black vì cache locality tốt hơn trên modern hardware. Mỗi node B-Tree chứa nhiều key liên tiếp trên memory, tận dụng CPU cache line.

- Crate `avl` trên crates.io -- nếu cần AVL cụ thể cho use case đặc biệt (ví dụ: cần đếm rotation cho benchmark).

- Tại sao hiểu AVL vẫn quan trọng dù không dùng trực tiếp? Rotation concept xuất hiện trong Red-Black tree, splay tree, treap -- tất cả đều dùng variant của rotation để cân bằng. Hiểu 1 loại → hiểu tất cả.

## Chương tiếp theo -- Red-Black Tree

AVL tree cân bằng **nghiêm ngặt** -- balance factor chỉ cho phép -1, 0, +1. Kết quả: search rất nhanh, nhưng insert/delete có thể cần nhiều rotation.

Chương tiếp theo sẽ giới thiệu **Red-Black Tree** -- một cách cân bằng "lỏng hơn" bằng cách tô màu node đỏ/đen. Red-Black cho phép cây lệch hơn AVL một chút, đổi lại insert/delete cần ít rotation hơn. Đây là cây mà Java `TreeMap`, C++ `std::map` chọn dùng -- và hiểu nó sẽ giúp bạn hiểu tại sao Rust lại chọn B-Tree thay thế.

---

[← Binary Search Tree](./02-bst.md) | [Red-Black Tree →](./04-red-black-tree.md)
