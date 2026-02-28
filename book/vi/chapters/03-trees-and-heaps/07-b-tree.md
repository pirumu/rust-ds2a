# B-Tree

## Đây là gì?

Mở một cuốn sách dày ra. Phía sau có **mục lục** (index). Mục lục không liệt kê mọi dòng trong sách -- nó gom nhóm: "Chương 1: trang 1-50", "Chương 2: trang 51-100"... Bạn tìm chương cần đọc, rồi mở đúng nhóm trang.

**B-Tree** hoạt động y hệt mục lục sách. Khác với binary tree (mỗi node chứa 1 giá trị, có 2 con), B-Tree cho phép mỗi node chứa **nhiều giá trị** và có **nhiều con**. Cây rất "nông" (ít tầng) nhưng mỗi tầng "rộng" (nhiều giá trị).

**Ở đâu trong thực tế?**
- **Database** (MySQL, PostgreSQL): dùng B-Tree cho index
- **File system** (NTFS, ext4, HFS+, APFS): dùng B-Tree để tìm file
- **Rust standard library**: `BTreeMap` và `BTreeSet` -- cái mà bạn đã nghe nhắc suốt series này

> **B-Tree** trông phức tạp vì node chứa nhiều key và nhiều con -- khác hoàn toàn binary tree quen thuộc. Nhưng bình tĩnh: search B-Tree giống hệt search BST, chỉ thay "rẽ trái/phải" bằng "chọn 1 trong nhiều nhánh". Insert cũng giống BST, chỉ thêm 1 bước "chia node khi đầy". Nếu bạn hiểu BST (chương trước), bạn đã hiểu 70% B-Tree rồi.
>
> Đây là cây **mạnh nhất** trong gia đình tree cho data lớn. Sau chương này, bạn đã nắm gần hết spectrum -- từ cây đơn giản nhất (Binary Tree) đến cây tối ưu disk I/O (B-Tree). Còn 1 chương nữa: Trie -- cây chuyên biệt cho chuỗi. Và `BTreeMap` mà bạn dùng hàng ngày trong Rust? Giờ bạn sẽ hiểu nó hoạt động như thế nào.

---

## Tại sao cần B-Tree khi đã có AVL/Red-Black?

Câu hỏi này quan trọng. Nếu AVL và Red-Black đã tự cân bằng, tại sao phải học thêm B-Tree?

### Vấn đề của binary trees với data lớn

AVL và Red-Black tối ưu cho **RAM** -- data nằm hoàn toàn trong bộ nhớ, nhảy từ node này sang node khác rất nhanh.

Nhưng khi data không nằm hết trong RAM thì sao? Database với hàng triệu rows, filesystem với hàng triệu files -- data nằm trên **ổ cứng**. Mỗi lần truy cập node = 1 lần đọc disk. Và đọc disk **chậm kinh khủng** so với RAM:

```
Tốc độ truy cập:
  RAM:           ~100 nanosecond     (0.0001 ms)
  SSD:           ~100 microsecond    (0.1 ms)     -- chậm hơn RAM 1000x
  HDD:           ~10 millisecond     (10 ms)      -- chậm hơn RAM 100,000x
```

Binary tree 1 triệu key → height ≈ 20 → **20 lần đọc disk**. Trên HDD: 20 × 10ms = 200ms cho 1 search. Quá chậm!

### Ý tưởng B-Tree

Mỗi lần đọc disk, OS đọc nguyên **1 block** (thường 4KB) dù bạn chỉ cần vài byte. Binary tree mỗi node = 1 key → lãng phí cả block cho 1 key bé xíu.

Ý tưởng: nhét **nhiều key** vào 1 block → 1 lần đọc disk cho ta hàng trăm key → cây "nông" hơn → ít lần đọc disk hơn.

### Demo bằng số

```
1 triệu key:

Binary tree (AVL/RB):
  Height ≈ 20
  Disk reads per search: 20
  Time (HDD): 20 × 10ms = 200ms
  Time (SSD): 20 × 0.1ms = 2ms

B-Tree (t=100, mỗi node ~200 key):
  Height ≈ 3
  Disk reads per search: 3
  Time (HDD): 3 × 10ms = 30ms     (6.7x nhanh hơn!)
  Time (SSD): 3 × 0.1ms = 0.3ms   (6.7x nhanh hơn!)

1 tỷ key:
  Binary tree: height ≈ 30         → 30 disk reads
  B-Tree t=100: height ≈ 5         → 5 disk reads
```

### Ngay cả trong RAM, B-Tree vẫn tốt

B-Tree **cache-friendly**: mỗi node chứa nhiều key liên tiếp trong bộ nhớ → CPU cache line (64 bytes) chứa được nhiều key cùng lúc → ít cache miss.

Binary tree mỗi node là 1 con trỏ trên heap → nhảy khắp nơi → cache miss liên tục. Đó là lý do Rust chọn `BTreeMap` thay vì Red-Black cho sorted map trong standard library.

### Bảng tổng kết gia đình tree

|  | Binary Tree | BST | AVL/RB | Heap | **B-Tree** |
|---|-----------|-----|--------|------|--------|
| Keys/node | 1 | 1 | 1 | 1 (implicit) | **Nhiều** (t-1 đến 2t-1) |
| Children/node | 2 | 2 | 2 | 2 (implicit) | **Nhiều** (t đến 2t) |
| Self-balancing | ❌ | ❌ | ✅ | ✅ (implicit) | ✅ |
| Sorted | ❌ | ✅ | ✅ | ❌ (partial) | ✅ |
| Optimized for | Learning | Search | RAM | Min/Max | **Disk + Cache** |
| Rust std | -- | -- | -- | BinaryHeap | **BTreeMap/BTreeSet** |

---

## Tính chất B-Tree chi tiết

B-Tree có **minimum degree t** (bậc tối thiểu). Con số t quyết định mỗi node "rộng" cỡ nào.

### Formal definition

```
B-Tree bậc t (minimum degree):

1. Mỗi node chứa k key: t-1 ≤ k ≤ 2t-1
   (trừ root: 1 ≤ k ≤ 2t-1)

2. Mỗi internal node có k+1 children
   (node có 3 key → 4 children)

3. Key trong node được SẮP XẾP tăng dần
   [5 | 10 | 20] -- 5 < 10 < 20

4. Key phân chia children:
   child₀ < key₁ < child₁ < key₂ < child₂ < ...
   (giống mục lục sách: key là tiêu đề, children là phần chi tiết)

5. Mọi leaf ở CÙNG ĐỘ SÂU
   → cây luôn perfectly balanced!
   (không phải "gần cân bằng" như AVL/RB -- mà HOÀN TOÀN cân bằng)
```

### Ví dụ t cụ thể

```
Ví dụ t=2 (2-3-4 tree -- nhỏ nhất có thể):
  Min key/node: 1    Max key/node: 3
  Min children: 2    Max children: 4

Ví dụ t=3:
  Min key/node: 2    Max key/node: 5
  Min children: 3    Max children: 6

Ví dụ t=100 (database index):
  Min key/node: 99   Max key/node: 199
  Min children: 100  Max children: 200
```

### Diagram cho t=3

```
B-Tree t=3 (mỗi node 2-5 key, 3-6 con):

                    [20 | 40]
                   /    |    \
        [5|10|15]   [25|30|35]   [45|50|55|60]

Node root [20|40]: 2 key, 3 children
  child₀ (trái): tất cả < 20
  child₁ (giữa): 20 < tất cả < 40
  child₂ (phải): tất cả > 40

Node [5|10|15]: 3 key (leaf, t-1=2 ≤ 3 ≤ 2t-1=5 ✓)
Node [45|50|55|60]: 4 key (leaf, 2 ≤ 4 ≤ 5 ✓)
```

Giống mục lục sách nhiều tầng: "Phần 1: trang 1-19" → mở Phần 1 ra thấy "Chương 5, Chương 10, Chương 15" → mở Chương cần đọc.

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

---

## Hoạt động: Search (Tìm kiếm)

Search B-Tree giống hệt BST -- chỉ thay "rẽ trái/phải" bằng "chọn 1 trong nhiều nhánh".

### Thuật toán

Tại mỗi node:
1. Quét qua các key (hoặc binary search vì key đã sorted)
2. Nếu tìm thấy key → trả về
3. Nếu không → đi xuống child tương ứng
4. Nếu là leaf mà không thấy → không tồn tại

### Trace search -- tìm thấy

```
Tìm 30 trong B-Tree t=3:

                    [20 | 40]
                   /    |    \
        [5|10|15]   [25|30|35]   [45|50|55|60]

Bước 1: Ở root [20|40]
  30 > 20 → tiếp tục
  30 < 40 → đi xuống child₁ (giữa 20 và 40)

Bước 2: Ở node [25|30|35]
  30 > 25 → tiếp tục
  30 == 30 → TÌM THẤY!

Chỉ 2 bước cho 12 key.
```

### Trace search -- không tìm thấy

```
Tìm 33 trong cùng cây:

Bước 1: Ở root [20|40]
  33 > 20 → tiếp tục
  33 < 40 → đi xuống child₁

Bước 2: Ở node [25|30|35]
  33 > 25 → tiếp tục
  33 > 30 → tiếp tục
  33 < 35 → nếu có child giữa 30 và 35 thì đi xuống
  Nhưng đây là leaf → KHÔNG TÌM THẤY

Tổng: 2 node truy cập, ~5 so sánh key
Nếu đây là binary tree 12 key → height ≈ 4, cần 4 node truy cập
```

### Tối ưu: Binary search trong mỗi node

Trong mỗi node, key đã sorted → dùng **binary search** thay vì scan tuần tự:

```
Node có 5 key → binary search cần ≤ 3 so sánh (thay vì 5)
Node có 199 key (t=100) → binary search cần ~8 so sánh (thay vì 199!)

Đây là tối ưu quan trọng trong implementation thực tế.
```

### Tìm kiếm cơ bản

```
Tìm 15 trong:
         [10 | 20]
        /    |    \
    [5]   [15]   [25 | 30]

Bước 1: Ở root [10|20]: 10 < 15 < 20 → đi xuống con giữa
Bước 2: Ở node [15]: 15 == 15 → tìm thấy!

Chỉ cần 2 bước! Dù có hàng triệu key, B-Tree vẫn chỉ cần 3-4 bước.
```

---

## Hoạt động: Insert + Split (Chèn và Chia node)

Insert B-Tree có 2 ý tưởng chính:
1. Chèn vào **leaf** (giống BST)
2. Nếu node đầy → **split** (chia đôi) trước khi chèn

### Cơ chế split

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

### Proactive splitting

Quan trọng: ta chia node đầy **khi đi xuống**, trước khi đến leaf. Nên khi đến leaf luôn có chỗ trống. Gọi là **proactive splitting** (chia trước). Chỉ cần 1 pass top-down, không cần quay ngược lên.

### Full trace: chèn [10, 20, 30, 40, 5, 15, 25, 35] vào B-Tree t=2

```
t=2: mỗi node chứa 1-3 key, 2-4 children

─── Insert 10 ───
[10]

─── Insert 20 ───
[10|20]

─── Insert 30 ───
[10|20|30]   ← đầy! (3 = 2t-1)

─── Insert 40 ───
Root đầy → SPLIT ROOT trước khi insert

  Bước 1: Tạo root mới rỗng
  Bước 2: Split [10|20|30]:
    - Key giữa (20) lên root mới
    - Nửa trái [10], nửa phải [30]

        [20]
       /    \
    [10]    [30]

  Bước 3: 40 > 20 → đi phải → chèn vào [30]

        [20]
       /    \
    [10]    [30|40]

─── Insert 5 ───
5 < 20 → đi trái → chèn vào [10]

        [20]
       /    \
  [5|10]    [30|40]

─── Insert 15 ───
15 < 20 → muốn đi trái vào [5|10]
Nhưng [5|10] chưa đầy (2 key < 3) → chèn bình thường

        [20]
       /    \
 [5|10|15]  [30|40]

─── Insert 25 ───
25 > 20 → muốn đi phải vào [30|40]
[30|40] chưa đầy → chèn bình thường

        [20]
       /    \
 [5|10|15]  [25|30|40]

─── Insert 35 ───
35 > 20 → muốn đi phải vào [25|30|40]
Nhưng [25|30|40] ĐẦY (3 key) → SPLIT trước!

  Split [25|30|40]:
    - Key giữa (30) lên cha [20]
    - Nửa trái [25], nửa phải [40]

        [20|30]
       /   |   \
 [5|10|15] [25] [40]

  Giờ chèn 35: 35 > 30 → đi phải → [40] → chèn

        [20|30]
       /   |   \
 [5|10|15] [25] [35|40]

Nhưng khoan! [5|10|15] cũng đầy. Lần insert tiếp theo
đi qua nó sẽ phải split. Hãy giả sử insert thêm 3:

─── (Bonus) Insert 3 ───
3 < 20 → muốn đi trái vào [5|10|15]
[5|10|15] ĐẦY → SPLIT trước!

  Split [5|10|15]:
    - Key giữa (10) lên cha [20|30]
    - Nửa trái [5], nửa phải [15]

        [10|20|30]
       /  |   |   \
    [5]  [15] [25] [35|40]

  Chèn 3: 3 < 10 → đi trái → [5] → chèn

        [10|20|30]
       /  |   |   \
  [3|5] [15] [25] [35|40]
```

### Chia root (Split root)

Khi root đầy, tạo root mới rồi chia root cũ. Đây là cách **duy nhất** cây cao thêm -- và mọi leaf đồng loạt sâu thêm 1 → cây luôn perfectly balanced.

```
Root đầy: [10 | 20 | 30]

Tạo root mới:
          [20]              ← root mới
         /    \
    [10]      [30]          ← root cũ bị chia đôi
```

---

## Hoạt động: Delete (Xóa) -- Overview

Delete B-Tree phức tạp hơn insert vì phải giữ tính chất "mỗi node ≥ t-1 key". Nhưng ý tưởng cốt lõi giống nhau.

### 3 trường hợp chính

**Trường hợp 1: Key ở leaf, node đủ key (> t-1)**

Xóa thẳng. Đơn giản nhất.

```
Xóa 25 từ leaf [20|25|30] (t=2, cần ≥ 1 key):
→ [20|30]    ← vẫn đủ key, OK!
```

**Trường hợp 2: Key ở internal node**

Thay bằng predecessor (key lớn nhất bên trái) hoặc successor (key nhỏ nhất bên phải) -- giống hệt BST delete. Rồi xóa predecessor/successor ở leaf.

```
Xóa 20 (ở internal node):
         [20]
        /    \
  [10|15]    [25|30]

Thay 20 bằng predecessor (15):
         [15]
        /    \
   [10]     [25|30]
```

**Trường hợp 3: Node con có ít key (= t-1)**

Trước khi đi xuống, phải đảm bảo node con có đủ key. Hai cách:
- **Borrow** từ sibling (anh em cạnh bên có dư key)
- **Merge** với sibling (gộp 2 node nhỏ thành 1)

### Proactive merging

Giống proactive splitting khi insert, delete dùng **proactive merging** -- đảm bảo mỗi node có đủ key trước khi đi xuống. Chỉ cần 1 pass top-down.

> **Thực tế:** Nhiều implementation dùng **lazy deletion** (đánh dấu "deleted", compact khi cần) thay vì delete thật. Đơn giản hơn nhiều và đủ tốt cho hầu hết use case.

---

## B-Tree vs B+Tree

Đây là câu hỏi phỏng vấn cực phổ biến. Hầu hết database KHÔNG dùng B-Tree mà dùng **B+Tree** -- một biến thể quan trọng.

### Khác biệt chính

```
B-Tree:                          B+Tree:
  - Key ở CẢ internal + leaf      - Key ở CHỈ leaf
  - Data pointer ở mọi node       - Data pointer CHỈ ở leaf
  - Leaf không liên kết nhau       - Leaf liên kết thành linked list

B-Tree:                          B+Tree:
      [20]                            [20]
     /    \                          /    \
  [10]    [30]                    [10]→[20|30]
  ↓data   ↓data                       ↓data ↓data

                                  Leaf linked list: [10]→[20|30]
```

### So sánh chi tiết

| | B-Tree | B+Tree |
|---|--------|--------|
| Data ở đâu? | Mọi node | Chỉ leaf |
| Range scan | Phải traverse cây | Scan linked list ở leaf → **nhanh!** |
| Point lookup | Có thể dừng giữa cây | Luôn đi đến leaf |
| Duplicate key | Không | Internal chỉ chứa "separator" |
| Dùng ở đâu? | Rust `BTreeMap`, general | **Database index (MySQL InnoDB, PostgreSQL)** |

### Tại sao database chọn B+Tree?

Vì range scan rất phổ biến:

```sql
SELECT * FROM users WHERE age BETWEEN 20 AND 30
```

Query này cần scan **liên tiếp** tất cả age từ 20 đến 30. B+Tree leaf linked list tối ưu cho việc này -- cứ duyệt từ leaf chứa 20 theo linked list đến leaf chứa 30. Không cần quay lại traverse cây.

---

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

### Tìm kiếm (search)

```rust
fn search(&self, val: &T) -> bool {
    let mut i = 0;
    // Scan qua key để tìm đúng vị trí
    while i < self.keys.len() && *val > self.keys[i] {
        i += 1;
    }
    // Tìm thấy!
    if i < self.keys.len() && self.keys[i] == *val {
        return true;
    }
    // Nếu là leaf thì không có
    if self.leaf {
        return false;
    }
    // Đệ quy xuống child phù hợp
    self.children[i].search(val)
}
```

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
        // Nếu con đầy → chia trước (proactive splitting!)
        // Rồi đệ quy xuống con
    }
}
```

---

## Disk I/O -- Tại sao B-Tree thống trị database

Đây là lý do tồn tại chính của B-Tree. Hiểu phần này = hiểu tại sao database nhanh.

### Disk block = B-Tree node

HDD/SSD đọc data theo **block** (thường 4KB). Dù bạn chỉ cần 1 byte, OS vẫn đọc nguyên 1 block. B-Tree lợi dụng điều này: chọn node size = 1 block → **1 lần đọc disk = 1 node đầy đủ**.

### Ví dụ cụ thể

Key là integer 8 bytes, pointer là 8 bytes. 1 block 4KB chứa:

```
4096 / (8 key + 8 pointer) ≈ 256 key per node
→ B-Tree t ≈ 128
```

### Sức mạnh thật sự

```
1 tỷ key, B-Tree t=128:

Level 0 (root):    1 node, ~255 key            → 1 disk read
Level 1:           ~256 node, ~65K key          → 1 disk read
Level 2:           ~65K node, ~16M key          → 1 disk read
Level 3:           ~16M node, ~4 tỷ key         → 1 disk read

Tổng: 4 disk read cho 1 TỶ key!
Root thường cached trong RAM → thực tế chỉ 3 disk read.
```

So sánh: binary tree cho 1 tỷ key cần **30 disk read**. B-Tree giảm 10x!

### Cache hierarchy

Giống như CPU có L1 → L2 → L3 cache, B-Tree tận dụng disk → SSD → RAM hierarchy:

- **Root + level 1**: nhỏ, nằm sẵn trong RAM (buffer pool)
- **Level 2**: có thể cached trong RAM nếu đủ chỗ
- **Leaf (level 3+)**: chỉ phần này cần đọc disk

Kết quả: hầu hết query chỉ cần **1-2 disk read** dù data có hàng tỷ rows.

---

## BTreeMap trong Rust -- Bí mật bên trong

Suốt series này, bạn đã nghe nhắc đi nhắc lại `BTreeMap`. Giờ bạn biết nó là gì rồi -- một B-Tree!

### Thông số kỹ thuật

- Rust `BTreeMap` dùng **B-Tree** (không phải B+Tree)
- Mỗi node chứa tối đa **11 key** (B=6, tức 2B-1=11) trên hầu hết platform
- Con số 11 được chọn để node fit vào cache line/page → tối ưu cho RAM

### Tại sao Rust chọn B-Tree thay vì Red-Black/AVL?

1. **Cache-friendly**: 11 key liên tiếp trong 1 node → CPU prefetch hiệu quả
2. **Ít allocation**: mỗi node chứa nhiều key → ít node → ít `Box`/pointer
3. **Ít pointer chasing**: Red-Black mỗi node = 1 key + 2 pointer. B-Tree 11 key chỉ cần check 1 node thay vì đi qua 3-4 Red-Black node

### API -- những thứ bạn dùng hàng ngày

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("key", "value");

// Range query -- đặc sản của B-Tree!
// HashMap KHÔNG THỂ làm điều này
for (k, v) in map.range("a"..="m") {
    println!("{}: {}", k, v);
}

// Entry API -- tránh double lookup
map.entry("key")
    .and_modify(|v| *v = "new_value")
    .or_insert("default");
```

### So sánh performance thực tế

```
Benchmark insert 1M random integers:
  HashMap:   ~120ms  (O(1) amortized, nhưng hash tính toán)
  BTreeMap:  ~180ms  (O(log n), nhưng cache-friendly)

Benchmark sorted iteration 1M integers:
  HashMap:   ~350ms  (phải collect + sort)
  BTreeMap:  ~45ms   (đã sorted sẵn!)    ← 7.8x nhanh hơn

Benchmark range query [100..200] trong 1M integers:
  HashMap:   ~120ms  (scan all + filter)
  BTreeMap:  ~0.5ms  (đi thẳng đến range) ← 240x nhanh hơn!
```

**Khi nào dùng `BTreeMap` thay `HashMap`?** Khi bạn cần sorted order hoặc range query. Ngoài ra, `HashMap` thường nhanh hơn.

---

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Ý nghĩa thực tế |
|----------|-----------|--------|------------------|
| `search` | O(t × log_t(n)) | O(log_t(n)) | Rất ít tầng cần duyệt |
| `insert` | O(t × log_t(n)) | O(t × log_t(n)) | Có thể split node |
| `delete` | O(t × log_t(n)) | O(t × log_t(n)) | Có thể merge node |
| `inorder` | O(n) | O(n) | Thăm mọi key |

### Tại sao O(t × log_t(n))?

- **log_t(n)**: số tầng cây (mỗi node có t đến 2t con → cơ số log là t)
- **× t**: tại mỗi node, scan hoặc binary search qua tối đa 2t-1 key

Với binary search trong node: O(log(t) × log_t(n)) = O(log n). Cùng Big-O như BST/AVL/RB!

### Tại sao B-Tree nhanh cho ổ cứng?

```
Binary tree (1 triệu key):        B-Tree t=100 (1 triệu key):
Height ≈ 20                        Height ≈ 3
= 20 lần đọc ổ cứng               = 3 lần đọc ổ cứng

Mỗi lần đọc ổ cứng ≈ 10ms         Mỗi lần đọc ổ cứng ≈ 10ms
Tổng: 200ms                        Tổng: 30ms
```

B-Tree giảm số lần đọc ổ cứng vì mỗi node chứa nhiều key, và kích thước node được chọn vừa đúng 1 block ổ cứng (thường 4KB).

---

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

---

## Những cái bẫy hay gặp

Phần này giúp bạn tránh những hiểu lầm phổ biến khi học B-Tree -- đặc biệt trong phỏng vấn.

### 1. Nhầm B-Tree với Binary Tree

❌ "B-Tree là viết tắt của Binary Tree"

✅ "B" trong B-Tree **KHÔNG phải** "Binary". B-Tree mỗi node có nhiều key và nhiều con. Nguồn gốc tên "B" vẫn được tranh luận (balanced? Bayer? Boeing?)

💡 B-Tree với t=2 có thể có 2-4 con per node. Binary tree luôn có đúng 2 con.

### 2. Nhầm B-Tree với B+Tree

❌ "Database dùng B-Tree cho index"

✅ Hầu hết database (MySQL InnoDB, PostgreSQL) dùng **B+Tree** (data chỉ ở leaf, leaf liên kết). Rust `BTreeMap` dùng B-Tree (data ở mọi node).

💡 Khi phỏng vấn, nói rõ đang nói loại nào. Nếu hỏi "database index dùng gì?" → trả lời B+Tree.

### 3. Chọn t quá nhỏ hoặc quá lớn

❌ "t càng lớn càng tốt"

✅ t quá nhỏ (t=2) thì B-Tree giống 2-3-4 tree -- có thể dùng Red-Black thay. t quá lớn thì mỗi node search chậm. Optimal t phụ thuộc vào **block size** và **key size**.

💡 Quy tắc: chọn t sao cho 1 node vừa đúng 1 disk block (4KB/8KB).

### 4. Nghĩ B-Tree chỉ dành cho disk

❌ "B-Tree chỉ có ý nghĩa khi data trên ổ cứng"

✅ B-Tree **cache-friendly** ngay cả trong RAM. Đó là lý do Rust std chọn `BTreeMap` thay vì Red-Black/AVL cho in-memory sorted map.

💡 Cache line CPU = 64 bytes. Node B-Tree chứa nhiều key liên tiếp → tận dụng cache line tốt hơn binary tree.

### 5. Quên rằng mọi leaf cùng depth

❌ "B-Tree gần cân bằng như AVL"

✅ B-Tree LUÔN **perfectly balanced** -- mọi leaf ở cùng depth. Không phải "gần cân bằng" mà là **hoàn toàn cân bằng**.

💡 Cây chỉ cao thêm khi root split → mọi leaf đồng loạt sâu thêm 1. Không bao giờ có leaf lệch.

---

## Khi nào dùng / không nên dùng

| Tình huống | B-Tree? | Thay bằng gì? | Tại sao? |
|------------|---------|---------------|----------|
| Database index | ✅ (B+Tree) | -- | Sinh ra cho disk I/O |
| Filesystem metadata | ✅ | -- | Tối ưu block I/O |
| In-memory sorted map (Rust) | ✅ | -- | `BTreeMap` cache-friendly |
| Range query | ✅ | -- | Sorted + multi-key node |
| Chỉ cần key-value, không sorted | ❌ | `HashMap` | O(1) > O(log n) |
| Priority queue | ❌ | `BinaryHeap` | Heap đơn giản hơn cho min/max |
| Data nhỏ (< 1000 items) | ❌ | `Vec` + sort | Overhead không đáng |
| Cần custom balancing logic | ❌ | AVL/Red-Black | Dễ customize hơn |
| Concurrent write-heavy | ⚠️ | LSM Tree | B-Tree lock granularity khó |

---

## Luyện nhận diện Pattern

### Bài 1: Database query chạy chậm

Bạn có bảng `orders` với 10 triệu rows. Query sau chạy mất 3 giây:

```sql
SELECT * FROM orders WHERE created_at BETWEEN '2024-01-01' AND '2024-01-31'
```

Bạn sẽ làm gì?

**Gợi ý:** Có cần scan toàn bộ 10 triệu rows không?

<details>
<summary>Đáp án</summary>

Tạo B+Tree index trên cột `created_at`: `CREATE INDEX idx_created ON orders(created_at)`.

Không có index → database phải **full table scan** 10 triệu rows. Có B+Tree index → database đi thẳng đến leaf chứa '2024-01-01' rồi scan linked list đến '2024-01-31'. Chỉ đọc ~300K rows thay vì 10M.

Complexity: O(log n + k) với k = số rows trong range, thay vì O(n).

</details>

### Bài 2: Chọn cấu trúc dữ liệu

Bạn cần lưu 1 triệu user profiles, hỗ trợ:
1. Tìm user by ID nhanh
2. Liệt kê users sorted by name
3. Range query "tất cả user tên bắt đầu bằng 'Tr'"

Dùng `HashMap`, `BTreeMap`, hay cả hai?

**Gợi ý:** Mỗi requirement map tốt nhất với cấu trúc nào?

<details>
<summary>Đáp án</summary>

Dùng **cả hai**: `HashMap<UserId, User>` cho lookup by ID (O(1)), và `BTreeMap<String, UserId>` cho sorted listing và range query.

- Requirement 1: `HashMap` → O(1) lookup
- Requirement 2: `BTreeMap` → đã sorted sẵn, iterate O(n)
- Requirement 3: `BTreeMap::range("Tr".."Ts")` → O(log n + k)

Trade-off: tốn gấp đôi bộ nhớ, nhưng mỗi operation đều optimal.

</details>

### Bài 3: Tại sao node size = disk block?

Giải thích tại sao B-Tree database chọn node size vừa đúng 1 disk block (4KB/8KB), không lớn hơn, không nhỏ hơn.

**Gợi ý:** Nhỏ hơn thì lãng phí disk read, lớn hơn thì...

<details>
<summary>Đáp án</summary>

- **Nhỏ hơn 1 block**: OS vẫn đọc nguyên 1 block → lãng phí bandwidth. Node 1KB vẫn tốn 1 disk read giống node 4KB.
- **Lớn hơn 1 block**: cần 2+ disk read cho 1 node → chậm gấp đôi. Node 8KB trên disk 4KB block = 2 reads.
- **Vừa đúng 1 block**: 1 disk read = 1 node đầy đủ. Tối ưu nhất.

Đây là lý do t được chọn dựa trên key size và block size: `t ≈ block_size / (key_size + pointer_size) / 2`.

</details>

---

## B-Tree trong Rust ecosystem

### Standard library

- `std::collections::BTreeMap` / `BTreeSet` -- B=6, tối đa 11 key/node
- `BTreeMap::range()` -- killer feature: O(log n) để đến start + O(k) scan k results
- Dùng khi cần sorted order hoặc range query

### Crate nổi bật

- **`redb`** -- embedded key-value store, Rust-native, dùng B+Tree variant. An toàn, ACID-compliant.
- **`sled`** -- embedded database bằng Rust, dùng B+Tree variant. Lock-free, concurrent.

Cả hai đều là ứng dụng thực tế của B+Tree mà bạn vừa học.

---

## Tổng kết gia đình Tree

Đây là chương cuối trong gia đình tree. Hãy nhìn lại hành trình:

```
Binary Tree → BST → AVL → Red-Black → Heap → B-Tree

Mỗi bước thêm 1 ý tưởng:
  Binary Tree:  cấu trúc phân nhánh cơ bản
  BST:          + quy tắc trái < gốc < phải  → search O(log n)
  AVL:          + tự cân bằng (height)        → guaranteed O(log n)
  Red-Black:    + cân bằng bằng color         → ít rotation hơn AVL
  Heap:         + partial order + array        → O(1) min/max
  B-Tree:       + multi-key node              → tối ưu disk I/O & cache
```

### Chọn cái nào?

```
Chỉ cần min/max         → Heap (BinaryHeap)
Sorted + in-memory       → B-Tree (BTreeMap)
Key-value không sorted   → HashMap
Học self-balancing        → AVL (trực quan nhất)
Disk-based sorted data   → B+Tree (database)
```

Bạn đã đi từ cây đơn giản nhất đến cây mạnh nhất. Mỗi loại cây giải quyết một vấn đề cụ thể -- không có cây nào "tốt nhất" cho mọi trường hợp. Hiểu khi nào dùng cây nào, đó mới là điều quan trọng.

---

## Chương tiếp theo -- Trie

B-Tree tối ưu cho disk I/O, mỗi node chứa nhiều key để cây "nông" nhất có thể. Nhưng có một loại tree khác, chuyên biệt cho **chuỗi**: mỗi node là 1 ký tự, đường đi từ root đến node = 1 từ. Cấu trúc đó là **Trie** (cây tiền tố) -- "siêu năng lực" của nó là prefix search: autocomplete, spell check, IP routing. Bạn gõ "xin" trên điện thoại và ngay lập tức thấy "xin chào", "xin lỗi" -- đó là Trie đang hoạt động.

---

---

[← Priority Queue](./06-priority-queue.md) | [Trie →](./08-trie.md)
