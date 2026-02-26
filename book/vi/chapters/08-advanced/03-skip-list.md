# Skip List

## Đây là gì?

Bạn đã học Linked List ở Phần 2 và Binary Search Tree (BST) ở Phần 3. Skip List là một cách khác để có O(log n) search — mà không cần cân bằng cây.

### Thang máy thường vs. thang máy tốc hành

Tưởng tượng một tòa nhà 100 tầng:

- **Thang máy thường** dừng ở **mọi tầng**: 1, 2, 3, 4, ... 100. Muốn đến tầng 87? Phải đi qua 86 tầng. Chậm.
- **Thang máy tốc hành** chỉ dừng ở một số tầng: 1, 10, 20, 30, ... 100. Muốn đến tầng 87? Đi tốc hành đến tầng 80, rồi chuyển sang thang thường đi thêm 7 tầng.

Linked List bình thường giống **thang máy thường** — muốn tìm phần tử, phải đi từ đầu đến cuối, O(n).

Skip List thêm nhiều "tầng express" phía trên — giống **thang máy tốc hành**. Nhờ vậy, search chỉ mất O(log n).

## Tại sao cần Skip List?

### Vấn đề với BST

BST cho O(log n) search — nhưng chỉ khi cây **cân bằng**. Nếu insert 1, 2, 3, 4, 5 theo thứ tự, BST thành linked list và search thành O(n).

Giải pháp? AVL tree, Red-Black tree — nhưng code phức tạp kinh khủng. Rotation, recoloring, cả đống edge case.

### Skip List: cân bằng bằng xác suất

Skip List dùng cách tiếp cận hoàn toàn khác: **xác suất (randomization)**. Thay vì rotation phức tạp, mỗi khi insert node mới, ta **tung đồng xu** để quyết định node đó cao bao nhiêu tầng.

- Tung được "sấp"? Dừng, node chỉ ở tầng 0.
- Tung được "ngửa"? Lên thêm 1 tầng, tung tiếp.

Kết quả: **trung bình**, khoảng 50% node ở tầng 0, 25% ở tầng 1, 12.5% ở tầng 2... Tự nhiên tạo ra cấu trúc giống cây cân bằng!

## Cấu trúc nhiều tầng

Đây là một Skip List chứa [3, 6, 7, 9, 12, 19, 21, 25]:

```
Level 3:  HEAD ────────────────────────> 9 ─────────────────────────> END
           │                             │
Level 2:  HEAD ──────────> 6 ──────────> 9 ──────────> 19 ─────────> END
           │               │             │              │
Level 1:  HEAD ──> 3 ──> 6 ──────────> 9 ──> 12 ──> 19 ──> 21 ───> END
           │       │      │             │      │      │      │
Level 0:  HEAD ──> 3 ──> 6 ──> 7 ──> 9 ──> 12 ──> 19 ──> 21 ──> 25 > END
```

Nhận xét:
- **Level 0** (tầng trệt): chứa **tất cả** phần tử — giống linked list thường.
- **Level 1**: chứa một phần — "express lane" đầu tiên.
- **Level 2, 3**: càng lên cao, càng ít phần tử — nhảy xa hơn.

Node `9` may mắn — tung đồng xu được ngửa 3 lần, nên nó xuất hiện ở cả 4 tầng. Node `7` chỉ ở tầng 0.

## Search — tìm kiếm

Tìm giá trị `12` trong Skip List ở trên:

```
Bắt đầu ở Level 3, HEAD:
  HEAD ──────────────────> 9    (9 < 12, đi tiếp)
  9 ──────────────────────> END  (hết, xuống level)

Xuống Level 2, ở node 9:
  9 ────────────> 19   (19 > 12, dừng! Xuống level)

Xuống Level 1, ở node 9:
  9 ──> 12    (12 == 12, TÌM THẤY! ✓)
```

Chỉ đi qua **3 bước** thay vì 5 bước nếu duyệt từ đầu! Với danh sách lớn, lợi ích càng rõ rệt.

### Thuật toán search

```
1. Bắt đầu ở HEAD, tầng cao nhất.
2. Tại mỗi node, nhìn sang phải (forward):
   - Nếu giá trị bên phải < target → đi sang phải.
   - Nếu giá trị bên phải >= target → xuống 1 tầng.
3. Khi xuống đến tầng 0, kiểm tra node bên phải.
   - Nếu == target → tìm thấy!
   - Nếu != target → không có trong list.
```

## Insert — thêm phần tử

Thêm giá trị `15` vào Skip List:

### Bước 1: Tung đồng xu

Tung xu để quyết định chiều cao của node mới:
- Lần 1: ngửa → lên level 1
- Lần 2: ngửa → lên level 2
- Lần 3: sấp → dừng

Node `15` sẽ có chiều cao = 2 (xuất hiện ở level 0, 1, 2).

### Bước 2: Tìm vị trí insert (giống search)

Đi từ trên xuống, ghi lại node cuối cùng ở mỗi tầng trước vị trí insert. Gọi đây là mảng `update`.

```
Level 2:  ... 9 ──────────> 19 ...
                  ^ update[2] = 9

Level 1:  ... 9 ──> 12 ──> 19 ...
                     ^ update[1] = 12

Level 0:  ... 12 ──> 19 ...
               ^ update[0] = 12
```

### Bước 3: Nối node mới vào mỗi tầng

```
Level 2:  ... 9 ──> [15] ──> 19 ...
Level 1:  ... 12 ──> [15] ──> 19 ...
Level 0:  ... 12 ──> [15] ──> 19 ...
```

Giống insert vào linked list — nhưng làm ở nhiều tầng cùng lúc!

## Delete — xóa phần tử

Xóa giá trị `15`:

1. **Tìm node** (giống search), ghi lại `update` — node trước nó ở mỗi tầng.
2. **Bỏ liên kết** ở mỗi tầng: `update[lvl].next = node.next` (giống delete linked list).
3. **Giảm level** nếu tầng cao nhất giờ trống.

```
Trước:
  Level 2:  ... 9 ──> 15 ──> 19 ...
  Level 1:  ... 12 ──> 15 ──> 19 ...
  Level 0:  ... 12 ──> 15 ──> 19 ...

Sau khi xóa 15:
  Level 2:  ... 9 ──────────> 19 ...
  Level 1:  ... 12 ──────────> 19 ...
  Level 0:  ... 12 ──────────> 19 ...
```

## Code Rust

```rust
use rust_ds2a::skip_list::SkipList;

let mut sl = SkipList::new();

// Insert
sl.insert(10);
sl.insert(20);
sl.insert(5);
sl.insert(15);

// Search
assert!(sl.search(10));   // true — có trong list
assert!(!sl.search(99));  // false — không có

// Delete
assert!(sl.delete(10));   // true — xóa thành công
assert!(!sl.search(10));  // false — đã bị xóa
assert!(!sl.delete(10));  // false — không còn để xóa

// Duplicates OK
sl.insert(5);
sl.insert(5);
assert_eq!(sl.len(), 5);  // [5, 5, 5, 15, 20]
```

## Bảng độ phức tạp

| Operation | Average    | Worst case | Space     |
|-----------|-----------|------------|-----------|
| Search    | O(log n)  | O(n)*      | O(1)      |
| Insert    | O(log n)  | O(n)*      | O(log n)  |
| Delete    | O(log n)  | O(n)*      | O(log n)  |
| Space     | —         | —          | O(n log n)|

*Worst case O(n) xảy ra khi tung đồng xu cực kỳ xui — tất cả node cùng chiều cao. Xác suất cực kỳ thấp.*

## So sánh với BST cân bằng

| Tiêu chí            | AVL / Red-Black Tree | Skip List          |
|---------------------|---------------------|--------------------|
| Search              | O(log n) chắc chắn  | O(log n) trung bình |
| Code phức tạp       | Rất phức tạp (rotation) | Đơn giản hơn nhiều |
| Concurrent access   | Khó lock            | Dễ lock từng tầng   |
| Memory              | Ít hơn              | Nhiều hơn (con trỏ nhiều tầng) |
| Dùng thực tế        | Phổ biến            | Redis sorted set, LevelDB |

## Tổng kết

Skip List là một ý tưởng rất đẹp: **thay vì cố gắng cân bằng hoàn hảo (AVL, Red-Black), ta dùng xác suất để đạt kết quả "đủ tốt" với code đơn giản hơn nhiều.**

Bạn gặp Skip List trong thực tế ở:
- **Redis** — Sorted Set dùng Skip List
- **LevelDB / RocksDB** — memtable dùng Skip List
- **Lucene** — posting list trong search engine

Nếu bạn hiểu linked list và biết tung đồng xu, bạn đã hiểu Skip List!
