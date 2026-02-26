# B-Tree

## Đây là gì?

Mở một cuốn sách dày ra. Phía sau có **mục lục** (index). Mục lục không liệt kê mọi dòng trong sách -- nó gom nhóm: "Chương 1: trang 1-50", "Chương 2: trang 51-100"... Bạn tìm chương cần đọc, rồi mở đúng nhóm trang.

**B-Tree** hoạt động y hệt mục lục sách. Khác với binary tree (mỗi node chứa 1 giá trị, có 2 con), B-Tree cho phép mỗi node chứa **nhiều giá trị** và có **nhiều con**. Cây rất "nông" (ít tầng) nhưng mỗi tầng "rộng" (nhiều giá trị).

**Tại sao cần B-Tree?** Vì ổ cứng và SSD rất chậm so với RAM. Mỗi lần đọc ổ cứng tốn rất nhiều thời gian. Binary tree cao 20 tầng = 20 lần đọc ổ cứng. B-Tree chỉ cao 3-4 tầng = 3-4 lần đọc. Nhanh hơn nhiều!

**Ở đâu trong thực tế?**
- **Database** (MySQL, PostgreSQL): dùng B-Tree cho index
- **File system** (NTFS, ext4, HFS+): dùng B-Tree để tìm file
- **Bất cứ đâu** cần tìm kiếm trong dữ liệu lớn trên ổ cứng

B-Tree có **minimum degree t** (bậc tối thiểu). Mỗi node (trừ root) chứa từ `t-1` đến `2t-1` key. Mỗi node có `k+1` con (k = số key). Mọi leaf ở cùng độ sâu.

## Hoạt động như thế nào?

### Node chứa nhiều key

Với B-Tree bậc t=2 (mỗi node chứa 1-3 key):

```
         [10 | 20]              ← root có 2 key, 3 con
        /    |    \
    [5]   [15]   [25 | 30]     ← leaf, mỗi node 1-2 key

Node [10 | 20]:
  - Con trái: chứa giá trị < 10
  - Con giữa: chứa giá trị giữa 10 và 20
  - Con phải: chứa giá trị > 20
```

Giống mục lục sách: "< 10 → xem trang trái", "10-20 → xem trang giữa", "> 20 → xem trang phải".

### Tìm kiếm (search)

Tại mỗi node, quét qua các key để tìm đúng "khoảng", rồi đi xuống con tương ứng.

```
Tìm 15 trong:
         [10 | 20]
        /    |    \
    [5]   [15]   [25 | 30]

Bước 1: Ở root [10|20]: 10 < 15 < 20 → đi xuống con giữa
Bước 2: Ở node [15]: 15 == 15 → tìm thấy!

Chỉ cần 2 bước! Dù có hàng triệu key, B-Tree vẫn chỉ cần 3-4 bước.
```

### Chèn và chia node (split)

Chèn vào node leaf. Nếu leaf đã đầy (2t-1 key), **chia** (split) trước khi chèn:

```
Chèn 17 vào node đầy [12|15|17] (t=2, tối đa 3 key):

Trước chia:                    Sau chia:
Cha: [10 | 20]                Cha: [10 | 15 | 20]
         |                          /    |    |    \
   [12 | 15 | 17]              [12]    [17]  ...
   (đầy! phải chia)

Cách chia:
1. Lấy key giữa (15) đẩy lên cha
2. Chia node thành 2: [12] và [17]
```

Giống khi một trang mục lục quá dài, bạn tách thành 2 trang và thêm 1 mục vào trang trên.

Quan trọng: ta chia node đầy **khi đi xuống**, trước khi đến leaf. Nên khi đến leaf luôn có chỗ trống. Gọi là **proactive splitting** (chia trước).

### Chia root

Khi root đầy, tạo root mới rồi chia root cũ. Đây là cách **duy nhất** cây cao thêm.

```
Root đầy: [10 | 20 | 30]

Tạo root mới:
          [20]              ← root mới
         /    \
    [10]      [30]          ← root cũ bị chia đôi
```

## Code Rust

Code đầy đủ nằm trong `src/b_tree.rs`.

### Cấu trúc dữ liệu

```rust
struct Node<T> {
    keys: Vec<T>,            // nhiều key trong 1 node
    children: Vec<Node<T>>,  // nhiều con
    leaf: bool,              // có phải leaf không?
}

pub struct BTree<T: Ord> {
    root: Node<T>,
    t: usize,  // bậc tối thiểu
}
```

Khác với binary tree (`left`/`right`), B-Tree dùng `Vec` vì số con không cố định.

### Chia node (split)

```rust
fn split_child_of(node: &mut Node<T>, i: usize, t: usize) {
    let mid = t - 1;
    let child = &mut node.children[i];

    let mut new_node = Node::new(child.leaf);
    new_node.keys = child.keys.split_off(mid + 1);  // nửa phải → node mới
    let median = child.keys.pop().unwrap();           // key giữa → đẩy lên cha

    if !child.leaf {
        new_node.children = child.children.split_off(mid + 1); // chia con
    }

    node.keys.insert(i, median);                     // đẩy median lên cha
    node.children.insert(i + 1, new_node);           // thêm node mới vào cha
}
```

### Chèn (insert)

```rust
fn insert_into_node(node: &mut Node<T>, val: T, t: usize) {
    if node.leaf {
        // Tìm vị trí đúng rồi chèn vào keys
    } else {
        // Tìm con phù hợp
        // Nếu con đầy → chia trước
        // Rồi đệ quy xuống con
    }
}
```

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa thực tế |
|----------|-----------|--------|------------------|
| `search` | O(t * log_t(n)) | O(log_t(n)) | Rất ít tầng cần duyệt |
| `insert` | O(t * log_t(n)) | O(t * log_t(n)) | Có thể chia node |
| `inorder` | O(n) | O(n) | Thăm mọi key |

### Tại sao B-Tree nhanh cho ổ cứng?

```
Binary tree (1 triệu key):        B-Tree t=100 (1 triệu key):
Height ≈ 20                        Height ≈ 3
= 20 lần đọc ổ cứng               = 3 lần đọc ổ cứng

Mỗi lần đọc ổ cứng ≈ 10ms         Mỗi lần đọc ổ cứng ≈ 10ms
Tổng: 200ms                        Tổng: 30ms
```

B-Tree giảm số lần đọc ổ cứng vì mỗi node chứa nhiều key, và kích thước node được chọn vừa đúng 1 block ổ cứng (thường 4KB).

## Ví dụ

### Sử dụng cơ bản

```rust
use rust_ds2a::b_tree::BTree;

let mut tree = BTree::new(2);  // bậc 2 (mỗi node 1-3 key)
for v in [10, 20, 5, 15, 25, 1, 8] {
    tree.insert(v);
}

assert!(tree.search(&15));    // tìm thấy
assert!(!tree.search(&99));   // không có
assert_eq!(tree.inorder(), vec![1, 5, 8, 10, 15, 20, 25]);
```

### Bậc lớn hơn (giống database thực tế)

```rust
use rust_ds2a::b_tree::BTree;

let mut tree = BTree::new(5);  // mỗi node chứa 4-9 key
for i in 1..=100 {
    tree.insert(i);
}

// 100 key nhưng cây chỉ cao vài tầng
for i in 1..=100 {
    assert!(tree.search(&i));
}
```
