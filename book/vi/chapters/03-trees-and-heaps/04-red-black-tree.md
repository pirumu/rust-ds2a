# Red-Black Tree

## Đây là gì?

> **Trước khi bắt đầu -- hít thở sâu.**
> Red-Black tree có tiếng là "boss cuối" của DSA -- nhiều sách dành 40-50 trang cho nó. Nhưng bản LLRB mà chương này dạy **đơn giản hơn hẳn**: chỉ 3 quy tắc fixup, mỗi quy tắc là 1 dòng `if`. Nếu bạn đã hiểu rotation từ chương AVL, bạn đã có 70% kiến thức cần thiết. Phần mới chỉ là: thay vì check balance factor, ta check **màu sắc**. Mục tiêu của chương này không phải để bạn tự implement Red-Black tree (gần như không ai làm vậy trong production) -- mà là hiểu **tại sao** nó hoạt động và **khi nào** chọn nó thay vì AVL. Đây là kiến thức phỏng vấn cực kỳ hữu ích.

### Cùng vấn đề, khác giải pháp

Chương trước, AVL giải quyết BST bị lệch bằng cách theo dõi height và xoay khi |bf| > 1. Hiệu quả, nhưng có thể cần nhiều rotation khi delete.

Có cách nào giữ cây cân bằng mà rotation **ít hơn** không? Red-Black tree trả lời: **CÓ** -- bằng cách nới lỏng tiêu chuẩn cân bằng. Thay vì bắt 2 nhánh chênh tối đa 1 (AVL), Red-Black cho phép đường dài nhất gấp đôi đường ngắn nhất.

**Red-Black tree** (cây đỏ-đen) là một cách khác để tự cân bằng. Thay vì theo dõi chiều cao, nó **tô màu** mỗi node -- đỏ hoặc đen -- rồi dùng một bộ quy tắc màu sắc để giữ cây gần cân bằng. Kết quả: ít rotation hơn AVL, nên **chèn và xóa nhanh hơn** (ít overhead hơn).

| | AVL (chương trước) | Red-Black (chương này) |
|---|-------------------|----------------------|
| Theo dõi gì? | Height (chiều cao) | Color (màu sắc) |
| Tiêu chuẩn cân bằng | \|bf\| ≤ 1 (rất chặt) | Cùng black-height (lỏng hơn) |
| Khi mất cân bằng | Xoay (1-2 lần/insert) | Đổi màu + xoay (ít hơn) |
| Chiều cao tối đa | ~1.44 log n | ~2 log n |
| Trade-off | Search nhanh hơn | Insert/delete nhanh hơn |

Cả hai đều O(log n). AVL tối ưu **đọc**, Red-Black tối ưu **ghi**. Đó là lý do Java `TreeMap`, C++ `std::map`, Linux kernel đều chọn Red-Black -- vì phần lớn workload ghi nhiều hơn đọc.

**Ở đâu trong thực tế?**
- C++ `std::map` -- dùng Red-Black tree
- Java `TreeMap` -- dùng Red-Black tree
- Linux kernel CFS scheduler -- dùng Red-Black tree
- Khi cần chèn/xóa nhiều, Red-Black tree tốt hơn AVL

Implementation của chúng ta dùng biến thể **Left-Leaning Red-Black tree (LLRB)** của Robert Sedgewick. LLRB đơn giản hơn nhiều: thay vì hàng chục trường hợp, chỉ cần 3 quy tắc fixup.

---

## 2-3 Tree Connection -- tại sao color rules hoạt động

Đây là insight quan trọng nhất của chương -- nếu bạn hiểu phần này, mọi thứ phía sau sẽ tự nhiên. Hầu hết tài liệu bỏ qua phần này, nhưng đây chính là lý do LLRB đơn giản đến vậy.

### 2-3 Tree là gì?

2-3 tree là một cây mà mỗi node chứa **1 hoặc 2 key**, có **2 hoặc 3** children. Điểm đặc biệt: 2-3 tree **luôn perfectly balanced** -- mọi leaf cùng depth.

```
2-3 tree:
        [5, 10]
       /   |   \
     [3]  [7]  [15]

Node [5,10] chứa 2 key → 3 children
Node [3], [7], [15] chứa 1 key → 2 children
Mọi leaf cùng depth = 1 → PERFECTLY BALANCED
```

2-3 tree đẹp nhưng khó implement: mỗi node có thể chứa 1 hoặc 2 key, có 2 hoặc 3 children → code phức tạp. Có cách nào biểu diễn 2-3 tree bằng binary tree (mỗi node chỉ 1 key, tối đa 2 children)?

### Red-Black = 2-3 tree "mở ra"

Ý tưởng: node 2-key được tách thành **parent + red child**:

```
2-3 tree:              Red-Black tree:
    [5, 10]       →         B:10
                           (R)/  \
                           5     15
                          / \
                         3   7

Node [5,10] → 10 (black) + 5 (red, con trái)
Link đỏ = "2 node này thuộc cùng 1 node logic trong 2-3 tree"
```

Hãy nghĩ link đỏ như keo dán: **node đỏ "dính" vào cha thành 1 node logic**. Node đen thì đứng độc lập.

Một ví dụ lớn hơn:

```
2-3 tree:                    Red-Black tree:
       [7]                        B:7
      /   \                      /   \
   [3,5]  [10,15]            B:5     B:15
   / | \   / | \           (R)/  \  (R)/  \
 [1][4][6][8][12][20]       3    6  10   20
                           / \       \
                          1   4       12

Node [3,5] → 5 (black) + 3 (red con trái)
Node [10,15] → 15 (black) + 10 (red con trái)
```

### Tại sao color rules đảm bảo cân bằng?

Vì 2-3 tree **luôn balanced**, và Red-Black tree chỉ là 2-3 tree biến hình:

- **Quy tắc "cùng black-height"** = "cùng depth trong 2-3 tree gốc". Mỗi node đen tương đương 1 tầng trong 2-3 tree. Node đỏ không tính vì nó "dính" vào cha.
- **Quy tắc "không 2 đỏ liên tiếp"** = "node 2-3 tree chỉ chứa tối đa 2 key". 2 đỏ liên tiếp = 3 key gom lại = node chứa 3 key = vi phạm 2-3 tree.
- **LLRB: link đỏ chỉ bên trái** = cách chọn nhất quán: key nhỏ hơn luôn là con đỏ bên trái.

**Kết luận:** Nếu bạn hiểu 2-3 tree (đơn giản, luôn balanced), thì Red-Black tree chỉ là cách **biểu diễn** 2-3 tree bằng binary tree + color. Đây là insight mà Sedgewick nhấn mạnh -- và là lý do LLRB đơn giản đến vậy.

---

## Hoạt động như thế nào?

### 5 tính chất của Red-Black tree

Một Red-Black tree hợp lệ phải thỏa mãn 5 quy tắc. Mỗi quy tắc đều có lý do tồn tại:

| # | Quy tắc | Giải thích dễ hiểu |
|---|---------|---------------------|
| 1 | Mỗi node là **đỏ** hoặc **đen** | Chỉ 2 màu, không có màu khác |
| 2 | **Root** luôn đen | Ông tổ luôn mặc áo đen |
| 3 | Mọi **leaf** (NIL) là đen | Các "chỗ trống" được coi là đen |
| 4 | Node đỏ phải có con **đen** | Không được 2 node đỏ liên tiếp |
| 5 | Mọi đường từ node đến leaf có cùng số **node đen** | "Chiều cao đen" đồng đều |

#### Tại sao mỗi quy tắc tồn tại?

**Quy tắc 4: Không 2 node đỏ liên tiếp**

```
Tại sao?  Node đỏ = "dính" vào cha thành 1 node logic.
          2 đỏ liên tiếp = node logic chứa 3 key = vi phạm 2-3 tree.

Hậu quả nếu vi phạm:
          Đường đi có thể dài vô hạn → mất O(log n).
```

**Quy tắc 5: Cùng số node đen trên mọi đường root→leaf (black-height)**

```
Tại sao?  Black-height = depth trong 2-3 tree gốc.
          2-3 tree luôn balanced → cùng depth → cùng black-height.

Hậu quả nếu vi phạm:
          Cây lệch, search không còn O(log n).
```

**Kết hợp quy tắc 4 + 5 -- đây là phép màu:**

```
- Đường ngắn nhất: toàn node đen
  → length = black-height

- Đường dài nhất: xen kẽ đỏ-đen (vì không cho 2 đỏ liên tiếp)
  → length = 2 × black-height

- Vậy: đường dài nhất ≤ 2 × đường ngắn nhất

- Height ≤ 2 log(n+1) → mọi thao tác O(log n) ✓
```

---

### 3 quy tắc fixup (LLRB)

Sau khi chèn (node mới luôn **đỏ**), có thể vi phạm quy tắc. LLRB sửa bằng 3 thao tác đơn giản. Thứ tự **rất quan trọng**: (1) rotate left, (2) rotate right, (3) flip colors. Mỗi bước có thể tạo điều kiện cho bước tiếp theo.

**Quy tắc 1: Link đỏ nghiêng phải → Rotate Left**

Trong LLRB, link đỏ chỉ được nghiêng **trái**. Nếu con phải đỏ mà con trái không đỏ, xoay trái:

```
Trước:                    Sau rotate_left:
    B:a                       B:b
   /   \(R)                 (R)/ \
  X     b                   a    Z
       / \                 / \
      Y   Z              X   Y

Subtree Y chuyển từ con trái của b → con phải của a
Màu: b lấy màu của a (black), a trở thành red

Khi nào trigger? is_red(right) && !is_red(left)
```

**Quy tắc 2: Hai link đỏ liên tiếp bên trái → Rotate Right**

```
Trước:                    Sau rotate_right:
      B:c                     B:b
    (R)/  \                 (R)/ \(R)
    b      Z               a    c
  (R)/ \                  / \  / \
  a    Y                 W  X Y   Z
 / \
W   X

Subtree Y chuyển từ con phải của b → con trái của c
Màu: b lấy màu c (black), c trở thành red

Khi nào trigger? is_red(left) && is_red(left.left)
```

**Quy tắc 3: Cả 2 con đều đỏ → Flip Colors**

```
Trước:                    Sau flip:
    B:c                     R:c
  (R)/ \(R)               (B)/ \(B)
   a    b                  a    b

Đỏ "đẩy lên" cha. a và b trở thành đen.
Nếu c là root → ép lại thành đen (quy tắc 2).

Tương đương 2-3 tree: node [a,b,c] có 3 key → tách thành
  c lên cha, a và b thành 2 node riêng.

Khi nào trigger? is_red(left) && is_red(right)
```

---

## Full Insert Trace -- chèn [1, 2, 3, 4, 5, 6, 7]

Đây là section quan trọng nhất. Hãy theo dõi từng bước, chú ý **màu sắc** và **quy tắc nào được trigger**.

### Insert 1

```
Cây rỗng → tạo node mới (đỏ) → ép root thành đen.

Kết quả:
  B:1
```

### Insert 2

```
Chèn 2 vào bên phải (vì 2 > 1):

  Trước fixup:
    B:1
       \(R)
        2    ← node mới (đỏ)

  Check fixup (tại node 1):
    ✓ Quy tắc 1? right red, left NOT red → YES! Rotate left
    → Sau rotate left:
         B:2
       (R)/
        1

    ✓ Quy tắc 2? left red, left.left? → 1 không có con trái → NO
    ✓ Quy tắc 3? left red AND right red? → right = None → NO

  → Root ép thành đen (2 đã đen rồi)

  Kết quả:
       B:2
     (R)/
      1
```

### Insert 3

```
Chèn 3 vào bên phải (vì 3 > 2):

  Trước fixup:
       B:2
     (R)/  \(R)
      1     3    ← node mới (đỏ)

  Check fixup (tại node 2):
    ✓ Quy tắc 1? right red, left NOT red? → left IS red → NO
    ✓ Quy tắc 2? left red, left.left? → 1 không có con trái → NO
    ✓ Quy tắc 3? left red AND right red? → YES! Flip colors

    → Sau flip:
         R:2
       (B)/  \(B)
        1     3

  → Root ép thành đen:
         B:2
       (B)/  \(B)
        1     3

  Kết quả:
       B:2
      /   \
    B:1   B:3

  Cây hoàn hảo! 3 node, height = 1.
```

### Insert 4

```
Chèn 4: đi phải từ 2, phải từ 3 → con phải của 3.

  Trước fixup (tại node 3):
       B:2
      /   \
    B:1   B:3
             \(R)
              4    ← node mới

  Check fixup (tại node 3):
    ✓ Quy tắc 1? right red (4), left NOT red → YES! Rotate left

    → Sau rotate left tại 3:
       B:2
      /   \
    B:1   B:4
        (R)/
         3

    ✓ Quy tắc 2? left red (3), left.left? → 3 không có con trái → NO
    ✓ Quy tắc 3? left red (3) AND right? → right = None → NO

  Check fixup (tại node 2, đi lên):
    ✓ Quy tắc 1? right NOT red (4 đen) → NO
    ✓ Quy tắc 2? left NOT red (1 đen) → NO
    ✓ Quy tắc 3? left NOT red → NO

  Kết quả:
       B:2
      /   \
    B:1   B:4
        (R)/
         3
```

### Insert 5

```
Chèn 5: đi phải từ 2, phải từ 4 → con phải của 4.

  Trước fixup (tại node 4):
       B:2
      /   \
    B:1   B:4
        (R)/  \(R)
         3     5    ← node mới

  Check fixup (tại node 4):
    ✓ Quy tắc 1? right red (5), left red (3) → left IS red → NO
    ✓ Quy tắc 2? left red (3), left.left? → 3 không có con trái → NO
    ✓ Quy tắc 3? left red (3) AND right red (5) → YES! Flip colors

    → Sau flip tại 4:
       B:2
      /   \
    B:1   R:4
         /   \
       B:3   B:5

  Check fixup (tại node 2, đi lên):
    ✓ Quy tắc 1? right red (4)! left NOT red (1 đen) → YES! Rotate left

    → Sau rotate left tại 2:
         B:4
       (R)/  \
        2    B:5
       / \
     B:1 B:3

    ✓ Quy tắc 2? left red (2), left.left? → 1 đen → NO
    ✓ Quy tắc 3? left red (2) AND right? → right (5) đen → NO

  → Root ép thành đen (4 đã đen)

  Kết quả:
         B:4
       (R)/  \
        2    B:5
       / \
     B:1 B:3
```

### Insert 6

```
Chèn 6: đi phải từ 4, phải từ 5 → con phải của 5.

  Trước fixup (tại node 5):
         B:4
       (R)/  \
        2    B:5
       / \      \(R)
     B:1 B:3     6    ← node mới

  Check fixup (tại node 5):
    ✓ Quy tắc 1? right red (6), left NOT red → YES! Rotate left

    → Sau rotate left tại 5:
         B:4
       (R)/  \
        2    B:6
       / \  (R)/
     B:1 B:3  5

    ✓ Quy tắc 2? left red (5), left.left? → 5 không có con trái → NO
    ✓ Quy tắc 3? left red (5) AND right? → right = None → NO

  Check fixup (tại node 4, đi lên):
    ✓ Quy tắc 1? right NOT red (6 đen) → NO
    ✓ Quy tắc 2? left red (2), left.left? → 1 đen → NO
    ✓ Quy tắc 3? left red (2) AND right NOT red → NO

  Kết quả:
         B:4
       (R)/  \
        2    B:6
       / \  (R)/
     B:1 B:3  5
```

### Insert 7

```
Chèn 7: đi phải từ 4, phải từ 6 → con phải của 6.

  Trước fixup (tại node 6):
         B:4
       (R)/  \
        2    B:6
       / \  (R)/  \(R)
     B:1 B:3  5    7    ← node mới

  Check fixup (tại node 6):
    ✓ Quy tắc 1? right red (7), left red (5) → left IS red → NO
    ✓ Quy tắc 2? left red (5), left.left? → 5 không có con trái → NO
    ✓ Quy tắc 3? left red (5) AND right red (7) → YES! Flip colors

    → Sau flip tại 6:
         B:4
       (R)/  \
        2    R:6
       / \   /  \
     B:1 B:3 B:5 B:7

  Check fixup (tại node 4, đi lên):
    ✓ Quy tắc 1? right red (6)! left red (2) → left IS red → NO
    ✓ Quy tắc 2? left red (2), left.left? → 1 đen → NO
    ✓ Quy tắc 3? left red (2) AND right red (6) → YES! Flip colors

    → Sau flip tại 4:
         R:4
       (B)/  \(B)
        2     6
       / \   /  \
     B:1 B:3 B:5 B:7

  → Root ép thành đen:
         B:4
       (B)/  \(B)
        2     6
       / \   /  \
     B:1 B:3 B:5 B:7

  Kết quả:
         B:4
        /   \
      B:2   B:6
      / \   / \
    B:1 B:3 B:5 B:7

  CÂY HOÀN HẢO! 7 node, height = 2, mọi node đều đen.
```

**Tóm tắt:** Chèn 1..7 theo thứ tự tăng dần -- worst case cho BST thường (linked list height=6). Nhưng LLRB tự cân bằng thành cây hoàn hảo height=2. Chỉ nhờ 3 quy tắc fixup đơn giản!

---

## LLRB vs Standard Red-Black

Khi bạn đọc sách hoặc blog khác về Red-Black tree, bạn có thể thấy code rất khác với chương này. Đó là vì có 2 variant chính:

| | LLRB (chương này) | Standard Red-Black |
|---|-------------------|-------------------|
| Link đỏ | Chỉ bên **trái** | Cả trái lẫn phải |
| Insert fixup | 3 quy tắc, ~15 dòng code | 6+ trường hợp, ~50 dòng |
| Delete | Đơn giản hơn (nhưng vẫn phức tạp) | Rất phức tạp (uncle node, sibling) |
| Tương đương | 2-3 tree | 2-3-4 tree |
| Ai dùng | Sedgewick's textbook, teaching | Java, C++, Linux kernel |
| Performance | Hơi chậm hơn (nhiều rotation hơn do ép left-leaning) | Nhanh hơn chút (ít constraint) |

**Khi phỏng vấn** hỏi Red-Black tree, họ thường hỏi **tính chất** và **khi nào dùng**, không hỏi implement chi tiết. Hiểu LLRB là đủ để trả lời. Nếu họ hỏi sâu hơn, nói bạn hiểu variant LLRB của Sedgewick -- đó là câu trả lời hoàn toàn hợp lệ.

---

## Delete -- tổng quan

Delete Red-Black tree là thao tác phức tạp nhất trong tất cả data structures chương này. Ngay cả LLRB variant cũng cần ~30 dòng code cho delete. Nên chúng ta chỉ tìm hiểu ý tưởng high-level.

### Ý tưởng chính: "đẩy đỏ xuống"

Khi xóa một node, vấn đề là: nếu xóa node **đen**, black-height bị lệch → vi phạm quy tắc 5. Nhưng xóa node **đỏ** thì không ảnh hưởng black-height!

Vậy chiến lược là:

1. **Đẩy đỏ xuống** -- khi đi xuống tìm node cần xóa, biến đổi cây sao cho node đích luôn đỏ
2. **Xóa** -- xóa node đỏ, black-height không đổi
3. **Fixup đi ngược lên** -- giống insert, apply 3 quy tắc LLRB từ dưới lên

**Trong thực tế**, bạn dùng `BTreeMap::remove()` -- không ai tự implement delete Red-Black tree. Nhưng biết ý tưởng "đẩy đỏ xuống" giúp hiểu tại sao delete phức tạp hơn insert rất nhiều.

---

## Trực quan hóa -- đọc Red-Black tree

Kỹ năng đọc và verify Red-Black tree quan trọng hơn implement. Đây là kỹ năng bạn cần khi debug hoặc khi phỏng vấn cho bạn một cây và hỏi "hợp lệ không?"

### Ví dụ 1: Cây hợp lệ (Red-Black chuẩn, nhưng không LLRB)

```
Cho cây sau, kiểm tra có hợp lệ không:

        B:10
       /    \
     R:5    B:15
    /   \      \
  B:3   B:7   R:20
```

**Checklist:**

```
✓ Quy tắc 1: mọi node đỏ hoặc đen? → ✓
✓ Quy tắc 2: root đen? → ✓ (10 đen)
✓ Quy tắc 3: leaf (NIL) đen? → ✓ (convention)
✓ Quy tắc 4: node đỏ có con đen?
  - R:5 → con trái B:3 ✓, con phải B:7 ✓
  - R:20 → con trái NIL (đen) ✓, con phải NIL (đen) ✓
✓ Quy tắc 5: cùng black-height?
  - 10→5→3→NIL:   đen 10, (đỏ 5 skip), đen 3, đen NIL = 3
  - 10→5→7→NIL:   đen 10, (đỏ 5 skip), đen 7, đen NIL = 3
  - 10→15→NIL:    đen 10, đen 15, đen NIL = 3
  - 10→15→20→NIL: đen 10, đen 15, (đỏ 20 skip), đen NIL = 3

Black-height = 3 cho TẤT CẢ đường → HỢP LỆ ✓
```

**LLRB check thêm:** link đỏ chỉ bên trái?

```
- R:5 là con TRÁI của 10 → ✓
- R:20 là con PHẢI của 15 → ✗ vi phạm LLRB!

→ Hợp lệ Red-Black chuẩn, nhưng KHÔNG hợp lệ LLRB.
```

### Ví dụ 2: Cây KHÔNG hợp lệ

```
        B:10
       /    \
     R:5    R:15     ← 2 con đều đỏ? OK cho Red-Black chuẩn
    /
  R:3                ← 2 đỏ liên tiếp (5 đỏ, 3 đỏ)!

✗ Vi phạm quy tắc 4: R:5 có con R:3 → 2 đỏ liên tiếp → KHÔNG HỢP LỆ
```

---

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

---

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ |
|----------|-----------|--------|
| `insert` | O(log n) | O(log n) stack |
| `search` | O(log n) | O(log n) stack |
| `inorder` | O(n) | O(n) |

Chiều cao Red-Black tree tối đa 2 * log(n+1). Nên mọi thao tác O(log n).

### So sánh chi tiết AVL vs Red-Black

```
                    AVL             Red-Black
Cân bằng:         Chặt hơn         Lỏng hơn
                   |bf| ≤ 1         Đường dài ≤ 2× đường ngắn
Chiều cao:        ~1.44 log n       ~2 log n
Tìm kiếm:         Nhanh hơn chút   Chậm hơn chút (cây cao hơn)
Chèn/xóa:         Nhiều rotation    Ít rotation hơn
Insert cost:       1-2 rotation     Chủ yếu đổi màu, ít xoay
Delete cost:       Tới O(log n)     Amortized ít hơn
                   rotation
Phù hợp khi:      Đọc nhiều         Ghi nhiều
Dùng trong:        Database index    std::map, TreeMap, kernel
```

---

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

---

## Những cái bẫy hay gặp

5 sai lầm phổ biến khi học Red-Black tree:

### a) Nhầm LLRB với Standard Red-Black

❌ Đọc sách/blog thấy case xử lý khác với code mình → hoảng, nghĩ code sai.

✅ LLRB chỉ cho link đỏ bên trái, standard cho cả hai bên. Luôn check source dùng variant nào trước khi so sánh.

💡 Nếu thấy "uncle node" hoặc "sibling" trong giải thích, đó là standard Red-Black, không phải LLRB.

### b) Quên ép root thành black sau insert

❌ Node mới luôn đỏ, `flip_colors` có thể đẩy đỏ lên root. Quên ép → vi phạm quy tắc 2.

✅ Luôn thêm `root.color = Black` ở dòng cuối cùng của `insert()`.

💡 Đây là lý do `insert()` tách riêng khỏi `insert_node()` -- `insert()` chịu trách nhiệm ép root đen.

### c) Nhầm thứ tự 3 quy tắc fixup

❌ Đổi thứ tự 3 quy tắc → cây hỏng. Ví dụ: flip trước rotate → miss case.

✅ Thứ tự cố định: (1) rotate left, (2) rotate right, (3) flip colors. Không bao giờ đổi.

💡 Mỗi bước tạo điều kiện cho bước tiếp: rotate left có thể tạo 2 đỏ liên tiếp → trigger rotate right → tạo cả 2 con đỏ → trigger flip.

### d) Nghĩ Red-Black cân bằng hoàn hảo như AVL

❌ Red-Black cho phép đường dài **gấp 2** đường ngắn. Search chậm hơn AVL constant factor.

✅ Red-Black trade search speed cho insert/delete speed. Đó là design choice, không phải bug.

💡 Trên thực tế, constant factor nhỏ đến mức benchmark phải chạy hàng triệu operations mới thấy khác biệt.

### e) Tự implement trong production

❌ Tốn thời gian implement và debug, dễ có bug tinh vi.

✅ Java, C++, Rust đều có sẵn. `BTreeMap` trong Rust thậm chí tốt hơn Red-Black cho hầu hết use case.

💡 Chỉ implement khi đang học hoặc khi interview yêu cầu. Trong production, dùng standard library.

---

## Khi nào dùng / không nên dùng

| Tình huống | Red-Black? | Thay bằng gì? | Tại sao? |
|------------|-----------|---------------|----------|
| Write-heavy sorted data | ✅ | -- | Ít rotation hơn AVL |
| Cần guaranteed O(log n) | ✅ | -- | Worst case vẫn O(log n) |
| Read-heavy, ít write | ❌ | AVL | AVL cây thấp hơn → search nhanh hơn |
| Data lớn, disk-based | ❌ | B-Tree | Cache-friendly, ít disk I/O |
| Chỉ cần key-value, không cần sorted | ❌ | HashMap | O(1) > O(log n) |
| Production Rust code | ⚠️ | BTreeMap | Rust std đã tối ưu B-Tree |
| Cần hiểu self-balancing concept | ✅ | -- | LLRB = version đơn giản nhất |
| OS kernel, memory allocator | ✅ | -- | Predictable O(log n), ít allocation |

---

## Luyện nhận diện Pattern

### Bài 1: Verify Red-Black tree

Cho cây sau, kiểm tra có hợp lệ không? Tìm quy tắc bị vi phạm (nếu có).

```
        B:8
       /    \
     R:4    B:12
    /   \   /
  B:2  B:6 R:10
  /
R:1
```

<details>
<summary>Gợi ý</summary>

Check 5 quy tắc theo thứ tự. Đếm black-height mỗi đường root→NIL.

</details>

<details>
<summary>Đáp án</summary>

```
✓ Quy tắc 1: mọi node đỏ/đen → ✓
✓ Quy tắc 2: root B:8 đen → ✓
✓ Quy tắc 3: NIL đen → ✓
✓ Quy tắc 4: node đỏ có con đen?
  - R:4 → B:2 ✓, B:6 ✓
  - R:10 → NIL ✓, NIL ✓
  - R:1 → NIL ✓, NIL ✓
  ✗ NHƯNG: R:4 là con của B:8 ✓, R:1 là con của B:2 ✓ ... OK

✓ Quy tắc 5: black-height?
  - 8→4→2→1→NIL:    B:8, (R:4 skip), B:2, (R:1 skip), NIL = 3
  - 8→4→2→NIL(phải): B:8, (R:4 skip), B:2, NIL = 3
  - 8→4→6→NIL:      B:8, (R:4 skip), B:6, NIL = 3
  - 8→12→10→NIL:    B:8, B:12, (R:10 skip), NIL = 3
  - 8→12→NIL(phải): B:8, B:12, NIL = 3

Black-height = 3 cho tất cả → HỢP LỆ ✓

LLRB check: R:10 là con TRÁI của 12 ✓, R:4 con TRÁI của 8 ✓,
            R:1 con TRÁI của 2 ✓ → Hợp lệ LLRB ✓
```

</details>

### Bài 2: Insert sequence

Cho cây LLRB hiện tại:

```
     B:5
    /   \
  B:3   B:7
```

Chèn giá trị **6**. Vẽ cây sau khi insert + fixup.

<details>
<summary>Gợi ý</summary>

Insert 6 như BST (node đỏ), rồi apply 3 quy tắc fixup từ dưới lên.

</details>

<details>
<summary>Đáp án</summary>

```
Bước 1: Insert 6 (đỏ) vào bên trái của 7:
     B:5
    /   \
  B:3   B:7
       (R)/
        6

Bước 2: Fixup tại node 7:
  ✓ Quy tắc 1? right NOT red → NO
  ✓ Quy tắc 2? left red (6), left.left? → 6 không có con trái → NO
  ✓ Quy tắc 3? left red, right NOT red → NO
  → Không thay đổi

Bước 3: Fixup tại node 5:
  ✓ Quy tắc 1? right NOT red (7 đen) → NO
  ✓ Quy tắc 2? left NOT red (3 đen) → NO
  ✓ Quy tắc 3? left NOT red → NO
  → Không thay đổi

Kết quả:
     B:5
    /   \
  B:3   B:7
       (R)/
        6
```

</details>

### Bài 3: AVL hay Red-Black?

Cho 3 scenario thực tế, chọn cây phù hợp và giải thích tại sao:

**a)** Hệ thống DNS cache -- lookup domain rất nhiều (hàng triệu/giây), update hiếm khi xảy ra.

**b)** Linux process scheduler -- insert/remove process liên tục, mỗi giây hàng nghìn lần.

**c)** In-memory database index -- read và write gần bằng nhau.

<details>
<summary>Đáp án</summary>

**a) DNS cache → AVL**. Đọc cực nhiều, ghi gần như không. AVL cây thấp hơn → mỗi lookup nhanh hơn. Overhead rotation khi update không quan trọng vì update rất hiếm.

**b) Process scheduler → Red-Black**. Insert/remove liên tục → cần minimize rotation overhead. Red-Black ít rotation hơn AVL khi insert/delete. Đây chính xác là lý do Linux kernel chọn Red-Black cho CFS scheduler.

**c) Database index → Red-Black hoặc B-Tree**. Read/write gần bằng nhau → Red-Black hợp lý vì balance giữa read và write. Nhưng nếu data lớn, B-Tree tốt hơn vì cache-friendly. Trong Rust, dùng `BTreeMap`.

</details>

---

## Red-Black trong Rust ecosystem

Rust standard library chọn **B-Tree** thay vì Red-Black/AVL → `BTreeMap`, `BTreeSet`.

**Tại sao?** B-Tree cache-friendly hơn. Mỗi node B-Tree chứa nhiều key liên tiếp trên memory. Trên modern CPU, cache miss tốn kém hơn vài phép so sánh extra. Nên dù B-Tree so sánh nhiều hơn per-node, tổng thể nhanh hơn vì ít cache miss.

**Crate `im`** (immutable collections) dùng Red-Black tree variant cho persistent data structures -- khi bạn cần giữ version cũ của cây sau mỗi thay đổi.

**Thực tế:** Nếu cần sorted map/set trong Rust, dùng `BTreeMap`/`BTreeSet`. Hiểu Red-Black tree giúp bạn hiểu **tại sao** Rust chọn B-Tree thay thế -- và trả lời câu hỏi phỏng vấn "tại sao không dùng Red-Black tree?"

---

## Tiếp theo: Binary Heap

BST, AVL, Red-Black -- tất cả đều cho search/insert/delete O(log n). Nhưng nếu bạn chỉ cần phần tử **nhỏ nhất** (hoặc lớn nhất) thì sao? Dùng cả 1 cây balanced BST chỉ để lấy min là quá "sang".

Chương tiếp theo sẽ giới thiệu **Binary Heap** -- cấu trúc đơn giản hơn nhiều: chỉ đảm bảo parent ≤ children, lấy min trong O(1), insert/delete trong O(log n). Và điều hay nhất? Heap lưu trong **Vec** -- không cần `Box<Node>` hay pointer nào cả. Binary Heap là nền tảng của Priority Queue -- dùng trong Dijkstra, OS scheduler, và event-driven systems khắp nơi.

---

---

[← AVL Tree](./03-avl-tree.md) | [Binary Heap →](./05-binary-heap.md)
