# Binary Search Tree (BST)

## Đây là gì?

> **Trước khi bắt đầu:** Chương này có phần xóa node (delete) với 3 trường hợp -- nhìn thì đáng sợ, nhưng thực ra 2 trường hợp đầu cực kỳ đơn giản (xóa thẳng hoặc cho con lên thế). Chỉ trường hợp thứ 3 cần suy nghĩ, và ý tưởng cũng chỉ là "tìm người thay thế phù hợp nhất". Nếu bạn đã hiểu Binary Tree ở chương trước, BST chỉ thêm **1 quy tắc** duy nhất. Quy tắc đó biến cây "ngớ ngẩn" (search O(n)) thành cây "thông minh" (search O(log n)). BST là nền tảng trực tiếp của `BTreeMap` trong Rust std, database index (B-Tree), và hầu hết mọi hệ thống cần tìm kiếm nhanh.

### Từ Binary Tree sang BST -- chỉ thêm 1 quy tắc

Chương trước, Binary Tree chỉ **chứa** dữ liệu -- không có quy tắc sắp xếp. Muốn tìm 1 giá trị? Duyệt hết O(n).

Nếu mỗi lần tìm kiếm ta có thể loại bỏ **một nửa** số node còn lại thì sao? 1 triệu node → chỉ cần ~20 bước. Đó là sức mạnh của O(log n).

| Thao tác | Binary Tree (chương trước) | BST (chương này) |
|----------|---------------------------|-------------------|
| Insert | O(n) -- BFS tìm chỗ trống | O(log n) -- so sánh rẽ trái/phải |
| Search | O(n) -- duyệt hết | O(log n) -- loại nửa mỗi bước |
| Delete | Không dạy (phức tạp, ít dùng) | O(log n) -- 3 trường hợp |
| In-order | Thứ tự tùy cách chèn | **Luôn tăng dần!** |
| Min/Max | O(n) -- duyệt hết | O(log n) -- đi trái/phải tận cùng |

Chỉ 1 quy tắc (trái < gốc < phải) mà mọi thao tác nhanh hơn hàng nghìn lần. Đó là lý do BST quan trọng đến vậy.

### Tra từ điển -- ẩn dụ xuyên suốt

Bạn đã bao giờ tra **từ điển giấy** chưa? Bạn không đọc từ đầu đến cuối. Bạn mở giữa cuốn sách, nhìn từ ở trang đó, rồi quyết định lật sang trái hay phải. Mỗi lần lật, bạn loại bỏ được **một nửa** số trang còn lại. Rất nhanh!

**Binary Search Tree** (cây tìm kiếm nhị phân, gọi tắt BST) hoạt động y hệt vậy, nhưng ở dạng cây. Quy tắc vàng:

> Với mỗi node: **tất cả giá trị bên trái < node < tất cả giá trị bên phải**

Nhờ quy tắc này, mỗi lần so sánh, ta loại bỏ được một nửa cây -- giống mỗi lần lật trang từ điển, bạn loại được nửa cuốn sách. Tìm kiếm, chèn, xóa đều nhanh.

BST là nền tảng cho nhiều cấu trúc nâng cao: AVL tree, Red-Black tree, B-Tree. Duyệt in-order cho kết quả **đã sắp xếp** -- đây là tính chất cực kỳ hữu ích.

## BST vs Sorted Array -- khi nào dùng gì?

Đây là câu hỏi phỏng vấn cực phổ biến. "Tại sao không dùng mảng đã sắp xếp?" -- vì mảng chèn/xóa phải dịch chuyển phần tử O(n), còn BST chỉ cần đi theo nhánh cây O(log n). Nhưng câu chuyện không đơn giản thế:

| Thao tác | Sorted Array | BST (cân bằng) | Ai thắng? |
|----------|-------------|-----------------|-----------|
| Search | O(log n) binary search | O(log n) | Hòa |
| Insert | O(n) dịch phần tử | O(log n) | BST |
| Delete | O(n) dịch phần tử | O(log n) | BST |
| Min/Max | O(1) đầu/cuối mảng | O(log n) | Array |
| Access by index | O(1) | O(n) | Array |
| Sorted iteration | O(n) | O(n) in-order | Hòa |
| Memory | Contiguous, cache-friendly | Fragmented, pointer chasing | Array |

**Kết luận:** Sorted array thắng khi data **ít thay đổi** (read-heavy). BST thắng khi data **thay đổi liên tục** (insert/delete thường xuyên). Trong thực tế, nhiều hệ thống dùng cả hai: database dùng B-Tree (BST variant) cho index, nhưng dùng sorted array cho data đã biết trước (static lookup table).

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

Giống tra từ điển: mở giữa sách (root), so sánh, rồi lật trái hoặc phải. Mỗi lần lật loại nửa cuốn sách.

**Search thành công:**

```
Tìm 4 trong cây [5, 3, 7, 1, 4, 6, 8]:

        5
       / \
      3   7
     / \ / \
    1  4 6  8

Bước  So sánh           Quyết định         Nodes còn lại
1     4 vs 5            4 < 5 → rẽ trái    loại {5,6,7,8} = 4 nodes
2     4 vs 3            4 > 3 → rẽ phải    loại {1} = 1 node
3     4 vs 4            4 == 4 → TÌM THẤY!

Tổng: 3 bước cho 7 nodes. log₂(7) ≈ 2.8 → đúng O(log n)!
```

**Search thất bại:**

```
Tìm 4.5 trong cùng cây:

Bước  So sánh           Quyết định
1     4.5 vs 5          4.5 < 5 → rẽ trái
2     4.5 vs 3          4.5 > 3 → rẽ phải
3     4.5 vs 4          4.5 > 4 → rẽ phải
4     node phải = None  → KHÔNG TÌM THẤY

Đến None nghĩa là đã đi hết nhánh mà không thấy.
Cũng chỉ 4 bước -- search thất bại cũng nhanh O(log n)!
```

```
Tìm số 6 trong cây:

        5           5 < 6 → rẽ phải
       / \
      3  [7]        7 > 6 → rẽ trái
         /
       [6]          6 == 6 → tìm thấy!
```

Mỗi bước loại bỏ một nửa cây. Giống binary search trên mảng, giống lật trang từ điển.

### Chèn (insert)

Chèn cũng đi theo cùng logic: so sánh rồi rẽ, đến chỗ trống thì đặt vào. Giống khi bạn muốn thêm 1 từ mới vào từ điển -- bạn tra đến đúng vị trí rồi viết vào.

```
Chèn 6 vào cây:           Kết quả:
      5                       5
     / \                     / \
    3   7                   3   7
                               /
    6 < 7 → rẽ trái          6  ← node mới
    chỗ trống → đặt vào
```

## Xóa node -- 3 trường hợp

Xóa phức tạp hơn vì phải giữ tính chất BST. Nhưng 2 trường hợp đầu rất đơn giản, chỉ trường hợp 3 cần suy nghĩ kỹ.

**Trường hợp 1: Node lá** (không có con) -- xóa thẳng, đơn giản nhất.

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

Ta tìm **người kế nhiệm** (in-order successor) = giá trị nhỏ nhất trong nhánh phải. Rồi copy giá trị đó vào vị trí cần xóa, rồi xóa node successor cũ.

```
Xóa 5:
      5              6          Bước 1: Tìm successor của 5
     / \            / \                  = nhỏ nhất bên phải = 6
    3   7    →     3   7        Bước 2: Thay 5 bằng 6
       /                        Bước 3: Xóa node 6 cũ (trường hợp 1 hoặc 2)
      6
```

Tại sao chọn successor? Vì nó là giá trị nhỏ nhất lớn hơn node cần xóa, đảm bảo tính chất BST vẫn đúng.

### Trace chi tiết trường hợp 3

Phần này hay khiến người học bị stuck. Hãy đi thật chậm qua từng bước:

```
Xóa node 3 trong cây:
        5
       / \
      3   7
     / \ / \
    1  4 6  8

Bước 1: Tìm node 3 → có 2 con (1 và 4)

Bước 2: Tìm in-order successor
         = node nhỏ nhất trong subtree PHẢI của 3
         = node 4 (đi phải 1 bước, rồi trái tận cùng --
           nhưng 4 không có con trái nên chính nó là nhỏ nhất)

Bước 3: Copy giá trị successor vào node cần xóa
         Node 3 → đổi value thành 4

Bước 4: Xóa node successor cũ (node 4 ở vị trí cũ)
         Node 4 cũ là leaf → trường hợp 1 → xóa thẳng

Kết quả:
        5
       / \
      4   7
     /   / \
    1   6   8

Kiểm tra BST property:
  Node 5: trái {1,4} < 5, phải {6,7,8} > 5 ✓
  Node 4: trái {1} < 4 ✓
  Node 7: trái {6} < 7, phải {8} > 7 ✓
```

**Câu hỏi tư duy:** Tại sao chọn in-order **successor** (nhỏ nhất bên phải) mà không phải in-order **predecessor** (lớn nhất bên trái)? Thực ra **cả hai đều được!** Cả successor và predecessor đều đảm bảo BST property sau khi thay thế. Convention chọn successor phổ biến hơn, nhưng nhiều implementation chọn predecessor hoặc thay đổi luân phiên để cây không bị lệch về 1 phía.

## In-order = Sorted -- tại sao?

Đây là tính chất quan trọng nhất của BST mà nhiều người bỏ qua. Hiểu tính chất này sẽ giúp bạn giải rất nhiều bài phỏng vấn.

### Chứng minh trực quan

```
In-order traversal: trái → gốc → phải

        5
       / \
      3   7
     / \
    1   4

inorder(5)
  inorder(3)
    inorder(1)
      inorder(None) → return
      VISIT 1          ← nhỏ nhất bên trái
      inorder(None) → return
    VISIT 3              ← gốc trái
    inorder(4)
      inorder(None) → return
      VISIT 4
      inorder(None) → return
  VISIT 5                ← root
  inorder(7)
    inorder(None) → return
    VISIT 7
    inorder(None) → return

Kết quả: [1, 3, 4, 5, 7]  ← ĐÃ SẮP XẾP!
```

### Tại sao luôn sorted?

Vì BST guarantee trái < gốc < phải. In-order đi trái trước (nhỏ hơn), rồi gốc (giữa), rồi phải (lớn hơn). Đệ quy áp dụng cho mọi subtree → toàn bộ kết quả tăng dần.

### Ứng dụng: Tree Sort

Muốn kết quả sorted mà không cần sort? Chèn vào BST rồi in-order traversal. Đây chính là **Tree Sort** -- O(n log n) average case, tương đương quicksort/mergesort.

## Min, Max, Floor, Ceiling

Bốn thao tác này tận dụng trực tiếp cấu trúc BST. Min/Max bạn đã thấy, nhưng Floor/Ceiling ít người biết mà cực kỳ hữu ích trong thực tế.

```
        5
       / \
      3   7
     / \ / \
    1  4 6  8

Min: đi trái tận cùng → 1       (O(h))
Max: đi phải tận cùng → 8       (O(h))
```

**Floor(x):** giá trị LỚN NHẤT ≤ x trong cây. Giống tra từ điển: tìm từ gần nhất mà không vượt quá từ bạn cần.

```
Floor(4.5): giá trị LỚN NHẤT ≤ 4.5 → 4
  Bắt đầu từ root 5: 4.5 < 5 → rẽ trái, candidate = None
  Node 3: 4.5 > 3 → rẽ phải, candidate = 3
  Node 4: 4.5 > 4 → rẽ phải, candidate = 4
  None → return candidate = 4
```

**Ceiling(x):** giá trị NHỎ NHẤT ≥ x trong cây.

```
Ceiling(4.5): giá trị NHỎ NHẤT ≥ 4.5 → 5
  Bắt đầu từ root 5: 4.5 < 5 → rẽ trái, candidate = 5
  Node 3: 4.5 > 3 → rẽ phải, candidate = 5
  Node 4: 4.5 > 4 → rẽ phải, candidate = 5
  None → return candidate = 5
```

Floor/Ceiling rất hữu ích trong thực tế. Ví dụ: tìm giá khách sạn gần nhất với budget -- `floor(budget)` cho giá cao nhất không vượt budget.

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

## Cây bị lệch -- vấn đề nghiêm trọng nhất của BST

Tại sao phải quan tâm? Vì một sai lầm đơn giản khi insert có thể biến BST O(log n) thành linked list O(n). Đây là điểm yếu chí mạng mà bạn phải biết.

### Thứ tự insert quyết định hình dạng cây

**Insert [1, 2, 3, 4, 5] theo thứ tự tăng dần → cây lệch thành linked list:**

```
1
 \
  2
   \
    3
     \
      4
       \
        5

Height = 5, search = O(n) -- tệ hại!
```

**Insert [4, 2, 6, 1, 3, 5, 7] → cây cân bằng hoàn hảo:**

```
        4
       / \
      2   6
     / \ / \
    1  3 5  7

Height = 3, search = O(log n) -- tuyệt vời!
```

Cùng 7 giá trị, khác thứ tự insert → performance khác nhau hoàn toàn.

### Impact trên dữ liệu lớn

```
Cây 1 triệu node:

Cây cân bằng:     height ≈ 20    → search ~20 bước
Cây bị lệch:      height = 1M    → search ~1,000,000 bước

Chênh lệch: 50,000 lần!
```

### Giải pháp

Đây là lý do cây tự cân bằng tồn tại. AVL tree và Red-Black tree tự động xoay (rotate) sau mỗi insert/delete để giữ chiều cao luôn O(log n). Rust std `BTreeMap` dùng B-Tree (luôn cân bằng). Chương sau sẽ nói chi tiết.

**Mẹo thực tế:** Nếu biết trước data, shuffle random trước khi insert → cây gần cân bằng (expected height O(log n)). Đây là randomized BST -- đơn giản mà hiệu quả.

## Độ phức tạp

| Thao tác | Trung bình | Xấu nhất (cây bị lệch) |
|----------|------------|------------------------|
| `insert` | O(log n) | O(n) |
| `search` | O(log n) | O(n) |
| `delete` | O(log n) | O(n) |
| `min` / `max` | O(log n) | O(n) |
| `inorder` | O(n) | O(n) |
| **Bộ nhớ** | -- | **O(n)** |

Tất cả thao tác chính phụ thuộc vào chiều cao cây h. Cây cân bằng h = log n, cây bị lệch h = n.

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

### Demo cây bị lệch vs cân bằng

```rust
use rust_ds2a::bst::BST;

// Cây bị lệch: insert theo thứ tự tăng dần
let mut skewed = BST::new();
for v in [1, 2, 3, 4, 5] {
    skewed.insert(v);
}
// Cây thành linked list: 1 → 2 → 3 → 4 → 5

// Cây cân bằng: insert theo thứ tự "giữa trước"
let mut balanced = BST::new();
for v in [4, 2, 6, 1, 3, 5, 7] {
    balanced.insert(v);
}
// Cây cân bằng hoàn hảo, height = 3

// Cả hai cho cùng kết quả in-order
assert_eq!(skewed.inorder(), vec![&1, &2, &3, &4, &5]);
assert_eq!(balanced.inorder(), vec![&1, &2, &3, &4, &5, &6, &7]);
```

## BST trong thực tế

Ba use case quan trọng giúp bạn hiểu tại sao BST (và các biến thể) xuất hiện khắp nơi.

### a) Database Index (B-Tree)

Khi bạn `CREATE INDEX ON users(email)`, database tạo B-Tree (họ hàng BST, mỗi node chứa nhiều key) để tìm kiếm nhanh.

```
SELECT * FROM users WHERE email = 'zan@example.com'

Không có index: scan 1 triệu rows → O(n) = 1,000,000 so sánh
Có B-Tree index: đi theo cây      → O(log n) ≈ 20 so sánh
```

`BTreeMap` trong Rust std chính là B-Tree implementation.

### b) Autocomplete / Range Query

BST cho phép tìm tất cả giá trị trong range [a, b] rất nhanh. Ví dụ: tìm tất cả user có tên bắt đầu bằng "Tr" → range query trên BST of strings: tìm tất cả giá trị trong ["Tr", "Ts").

```rust
use std::collections::BTreeSet;

let names: BTreeSet<&str> = ["An", "Bình", "Trung", "Trinh", "Trà", "Vân"]
    .into_iter().collect();

// Range query: tất cả tên bắt đầu bằng "Tr"
let tr_names: Vec<&&str> = names.range("Tr".."Ts").collect();
// → ["Trà", "Trinh", "Trung"]
```

### c) In-memory Sorted Set (BTreeSet)

Khi cần collection vừa insert/delete nhanh, vừa duyệt sorted. Ví dụ: leaderboard game -- thêm/xóa player liên tục, nhưng luôn cần hiển thị top 10.

`BTreeSet` trong Rust dùng B-Tree internally → mọi thao tác O(log n) + sorted iteration O(n).

## Những cái bẫy hay gặp

Phần này giúp bạn tránh những sai lầm phổ biến khi làm việc với BST.

### a) Insert data đã sorted → cây lệch thành linked list

❌ `bst.insert(1); bst.insert(2); bst.insert(3);` → cây thành đường thẳng, mọi thao tác O(n)

✅ Shuffle data trước khi insert, hoặc dùng self-balancing tree (AVL, Red-Black, B-Tree)

💡 Đây là pitfall #1 của BST. Luôn cảnh giác khi data có thể đến theo thứ tự sorted.

### b) Nhầm BST với Binary Tree

❌ Dùng BFS insert như Binary Tree thường → mất tính chất sorted

✅ BST insert so sánh giá trị rồi rẽ trái/phải. Binary Tree thường insert vào chỗ trống bất kỳ.

💡 BST có tính chất trái < gốc < phải. Binary Tree thường thì không. Insert logic hoàn toàn khác nhau.

### c) Chỉ check con trực tiếp khi validate BST

❌ Check `node.left < node && node.right > node` rồi tưởng đúng

✅ Phải check TOÀN BỘ subtree trái < node < TOÀN BỘ subtree phải

💡 Ví dụ sai:

```
      5
     / \
    3   7
       /
      2    ← 2 < 7? ✓ Nhưng 2 nằm bên PHẢI của 5 → vi phạm BST!

Nếu chỉ check: 2 < 7? ✓ → tưởng đúng
Check đúng cách: 2 phải > 5 (vì nằm trong right subtree của 5) → SAI
```

### d) Quên handle trường hợp duplicate

❌ Không quyết định trước: duplicate đi trái, phải, hay bỏ qua?

✅ Quyết định convention trước khi implement: bỏ qua (BST chuẩn), đi trái, thêm count trong node

💡 BST chuẩn không cho trùng. Code trong `src/bst.rs` bỏ qua duplicate (`val == n.value → do nothing`).

### e) Dùng BST khi không cần sorted order

❌ Dùng BST O(log n) cho insert/search/delete khi không cần sorted iteration

✅ Nếu chỉ cần tìm kiếm nhanh mà không cần thứ tự → `HashMap` O(1) nhanh hơn

💡 BST chỉ thắng HashMap khi cần **thứ tự**: sorted iteration, range query, min/max, floor/ceiling.

## Khi nào dùng / không nên dùng BST?

| Tình huống | BST? | Thay bằng gì? | Tại sao? |
|------------|------|---------------|----------|
| Tìm kiếm + insert/delete thường xuyên, cần sorted | ✅ | -- | BST sinh ra cho việc này |
| Database index | ✅ (B-Tree variant) | -- | Range query + sorted access |
| Leaderboard / sorted set | ✅ | BTreeSet | Insert + sorted traversal |
| Chỉ cần tìm kiếm nhanh, không cần sorted | ❌ | HashMap | O(1) > O(log n) |
| Data tĩnh, không thay đổi | ❌ | Sorted array + binary search | Cache-friendly hơn |
| Priority queue (chỉ cần min/max) | ❌ | BinaryHeap | Heap đơn giản và nhanh hơn |
| Prefix search (autocomplete) | ⚠️ | Trie | BST được nhưng Trie tối ưu hơn |
| Dữ liệu insert theo thứ tự sorted | ❌ | Self-balancing BST | BST thường bị lệch |

## Luyện nhận diện Pattern

Ba bài kinh điển giúp bạn luyện sử dụng tính chất BST trong phỏng vấn.

**Bài 1: Validate BST** -- cho 1 binary tree, kiểm tra nó có phải BST hợp lệ không.

**Gợi ý:** In-order traversal trên BST có tính chất gì đặc biệt? Nếu kết quả in-order không tăng dần thì...

<details>
<summary>Đáp án</summary>

Cách giải: In-order traversal trên BST **luôn** cho kết quả tăng dần. Duyệt in-order, nếu bất kỳ phần tử nào ≤ phần tử trước đó → không phải BST hợp lệ. Hoặc dùng approach truyền min/max bound khi đệ quy.

Complexity: Time O(n), Space O(h) -- h là chiều cao cây (stack đệ quy)

</details>

**Bài 2: Kth Smallest Element** -- cho BST và số k, tìm phần tử nhỏ thứ k.

**Gợi ý:** In-order traversal cho kết quả sorted -- phần tử thứ k trong kết quả sorted là gì?

<details>
<summary>Đáp án</summary>

Cách giải: In-order traversal, đếm đến phần tử thứ k rồi trả về. Không cần duyệt hết cây -- dừng ngay khi đếm đủ k.

Complexity: Time O(h + k), Space O(h)

</details>

**Bài 3: Lowest Common Ancestor (LCA) trong BST** -- cho BST và 2 node p, q, tìm tổ tiên chung gần nhất.

**Gợi ý:** Dùng tính chất BST -- nếu cả p và q đều < node hiện tại thì LCA ở đâu? Nếu 1 bên trái 1 bên phải thì sao?

<details>
<summary>Đáp án</summary>

Cách giải: Từ root đi xuống. Nếu cả p, q < node → LCA nằm bên trái. Nếu cả p, q > node → LCA nằm bên phải. Nếu p và q ở 2 bên khác nhau (hoặc 1 trong 2 bằng node) → node hiện tại chính là LCA.

Complexity: Time O(h), Space O(1) nếu dùng iterative

</details>

## BST trong Rust ecosystem

Vài điều hữu ích khi làm việc với Rust:

- **`std::collections::BTreeMap` / `BTreeSet`** -- B-Tree (BST generalized, mỗi node nhiều key, tối ưu cho cache + disk). Đây là sorted map/set mặc định trong Rust.

- **`HashMap` vs `BTreeMap`**: HashMap O(1) nhưng unordered. BTreeMap O(log n) nhưng sorted. Chọn theo nhu cầu -- cần thứ tự? BTreeMap. Không cần? HashMap nhanh hơn.

- **Liên hệ thực tế:** Kafka consumer group quản lý partition assignment có thể dùng sorted structure để track range. Topic-partition offset tracking cũng dùng sorted order (offset tăng dần) -- BTreeMap là lựa chọn tự nhiên cho kiểu data này.

## Tiếp theo: AVL Tree

BST có 1 điểm yếu chí mạng: nếu data insert theo thứ tự sorted, cây bị lệch thành linked list, mọi thao tác thành O(n). Làm sao để cây **tự cân bằng**?

Chương tiếp theo sẽ giới thiệu **AVL Tree** -- BST đầu tiên biết tự cân bằng. AVL theo dõi "balance factor" (chênh lệch height giữa cây trái và phải) và dùng **rotation** để sửa khi cây lệch. Kết quả: guaranteed O(log n) cho mọi thao tác, bất kể thứ tự insert.

---

---

[← Binary Tree](./01-binary-tree.md) | [AVL Tree →](./03-avl-tree.md)
