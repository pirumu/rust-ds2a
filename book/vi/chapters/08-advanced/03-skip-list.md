# Skip List

## Bạn đã biết đủ rồi!

> Linked List (chương 2) + multiple levels + random coin flip = **Skip List**

Nếu bạn hiểu 3 thứ này, bạn hiểu Skip List:

1. **Linked List**: duyệt từ đầu đến cuối, insert/delete bằng cách nối con trỏ.
2. **Nhiều tầng**: xếp chồng nhiều linked list lên nhau, tầng cao nhảy xa hơn.
3. **Tung đồng xu**: mỗi node insert vào, tung xu để quyết định nó cao mấy tầng.

Không có rotation, không có recoloring, không có gì phức tạp. Thật đấy.

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

## Randomized level generation — sâu hơn

Cách tung đồng xu ở trên chính là **geometric distribution** (phân phối hình học) với p = 0.5.

### Nó hoạt động thế nào?

```
random_level():
    level = 0
    while coin_flip() == HEADS and level < MAX_LEVEL:
        level += 1
    return level
```

Xác suất node ở mỗi tầng:

```
Level 0: 100% node đều có         (mọi node đều ở tầng trệt)
Level 1: 50%  node có              (1/2)
Level 2: 25%  node có              (1/4)
Level 3: 12.5% node có             (1/8)
...
Level k: (1/2)^k node có
```

### Expected height của Skip List

Với n phần tử, expected height (chiều cao trung bình) là:

```
E[height] = log₂(n)
```

Ví dụ: 1000 phần tử -> expected height ~ 10 tầng. 1 triệu phần tử -> ~ 20 tầng.

Tại sao? Tầng cao nhất là tầng mà **ít nhất 1 node** chạm tới. Xác suất 1 node đạt level k là (1/2)^k. Với n node, expected max level ~ log₂(n).

### Tại sao lại dùng p = 0.5?

Paper gốc của William Pugh (1990) phân tích nhiều giá trị p:

- **p = 0.5**: cân bằng tốt giữa speed và memory. Mỗi node trung bình có 2 con trỏ.
- **p = 0.25**: tiết kiệm memory hơn (trung bình 1.33 con trỏ/node), nhưng search chậm hơn một chút.
- Redis dùng **p = 0.25** và MAX_LEVEL = 32.

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

### Probabilistic analysis — tại sao O(log n)?

Để hiểu trực giác, hãy nghĩ **ngược** — đi từ node tìm thấy trở lại HEAD:

1. Tại mỗi bước, ta hoặc đi **lên** (nếu node có tầng cao hơn) hoặc đi **sang trái**.
2. Xác suất đi lên = 1/2 (vì mỗi node có 50% cơ hội lên tầng tiếp).
3. Trung bình, ta đi lên log₂(n) lần (vì có log₂(n) tầng).
4. Tại mỗi tầng, trung bình đi sang trái 1/p = 2 bước.

Tổng expected steps:

```
E[search time] = (1/p) × log₁/ₚ(n)

Với p = 0.5:  2 × log₂(n)  = O(log n)
Với p = 0.25: 1.33 × log₄(n) = O(log n) (hệ số nhỏ hơn nhưng log base lớn hơn)
```

Kết luận: **dù p là bao nhiêu, expected search time luôn là O(log n).**

## Insert — thêm phần tử

Thêm giá trị `15` vào Skip List:

### Bước 1: Tung đồng xu

Tung xu để quyết định chiều cao của node mới:
- Lần 1: ngửa -> lên level 1
- Lần 2: ngửa -> lên level 2
- Lần 3: sấp -> dừng

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

## Redis Sorted Set — tại sao dùng Skip List?

Đây là câu hỏi phỏng vấn kinh điển: **Redis dùng Skip List thay vì Red-Black Tree cho ZSET (sorted set). Tại sao?**

Antirez (tác giả Redis) từng giải thích trực tiếp. Lý do chính:

### 1. Code đơn giản hơn nhiều

Red-Black Tree cần rotation, recoloring, xử lý hàng chục case. Skip List chỉ cần tung xu + nối linked list. Dễ viết, dễ debug, dễ maintain.

### 2. Range query nhanh

ZSET cần thao tác `ZRANGEBYSCORE` — lấy tất cả phần tử trong khoảng [min, max]. Với Skip List, tìm min rồi duyệt level 0 là xong. Với Red-Black Tree, phải duyệt inorder phức tạp hơn.

### 3. Concurrent-friendly

Skip List dễ lock từng tầng hoặc dùng lock-free algorithm hơn nhiều so với tree. Khi cần concurrent access (nhiều thread đọc/ghi cùng lúc), Skip List thắng rõ rệt.

### 4. Memory tương đương

Với p = 0.25 (Redis dùng), mỗi node trung bình chỉ có 1.33 con trỏ — gần như bằng Red-Black Tree (2 con trỏ left/right + 1 bit color).

```
Redis ZSET thực tế:
  ZADD leaderboard 100 "alice"    -- insert score=100, member="alice"
  ZADD leaderboard 200 "bob"
  ZADD leaderboard 150 "charlie"

  ZRANGEBYSCORE leaderboard 100 180
  --> ["alice", "charlie"]          -- range query dùng Skip List!
```

## Khi nào dùng Skip List?

| Tình huống | Dùng Skip List? | Tại sao |
|---|---|---|
| Cần sorted data + insert/delete nhanh | **Co** | O(log n) cho mọi thao tác |
| Cần range query (lấy phần tử trong khoảng) | **Co** | Duyệt level 0 sau khi tìm điểm bắt đầu |
| Cần concurrent access (multi-thread) | **Co** | Dễ lock-free hơn tree |
| Code cần đơn giản, dễ debug | **Co** | Không rotation, không recoloring |
| Cần worst-case guarantee O(log n) | **Khong** | Dùng AVL/Red-Black Tree |
| Memory cực kỳ hạn chế | **Khong** | Skip List tốn thêm con trỏ ở mỗi tầng |
| Data ít (< 100 phần tử) | **Khong** | Sorted array + binary search đủ rồi |
| Cần persistent/immutable structure | **Khong** | Tree dễ share subtree hơn |

## Pitfalls — bẫy hay gặp

### 1. Random seed quality

❌ **Sai**: Dùng fixed seed hoặc seed kém chất lượng.

```rust
// Seed cố định = mọi lần chạy tạo cùng cấu trúc
// Attacker có thể exploit!
rng_state: 12345,
```

✅ **Dung**: Dùng seed từ nguồn entropy tốt (thời gian, OS random).

```rust
// Trong production, dùng thread_rng() hoặc tương tự
rng_state: std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_nanos() as u64,
```

💡 **Tai sao**: Nếu attacker biết seed, họ có thể craft input khiến mọi node cùng level -> O(n) search. Đây là dạng **algorithmic complexity attack**.

### 2. Không giới hạn max level

❌ **Sai**: Cho phép level tăng vô hạn.

```rust
fn random_level(&mut self) -> usize {
    let mut lvl = 0;
    while self.coin_flip() { // không có giới hạn!
        lvl += 1;
    }
    lvl
}
```

✅ **Dung**: Luôn cap max level.

```rust
const MAX_LEVEL: usize = 16; // đủ cho ~65,000 phần tử
// Hoặc MAX_LEVEL = 32 cho ~4 tỷ phần tử (Redis dùng 32)

fn random_level(&mut self) -> usize {
    let mut lvl = 0;
    while lvl < MAX_LEVEL - 1 && self.coin_flip() {
        lvl += 1;
    }
    lvl
}
```

💡 **Tai sao**: Không giới hạn -> xui thì tung được ngửa 1000 lần liên tiếp -> node có 1000 tầng -> tốn memory vô nghĩa. MAX_LEVEL = log₂(n_max) là đủ.

### 3. Memory overhead bị coi thường

❌ **Sai**: Nghĩ Skip List tốn memory giống linked list thường.

✅ **Dung**: Mỗi node trung bình có `1/(1-p)` con trỏ.

```
p = 0.5:  mỗi node trung bình 2 con trỏ    (gấp đôi linked list)
p = 0.25: mỗi node trung bình 1.33 con trỏ  (chấp nhận được)
```

💡 **Tai sao**: Nếu dữ liệu nhỏ (ví dụ: node chỉ chứa 1 số i32), thì overhead con trỏ có thể lớn hơn chính dữ liệu. Cân nhắc khi memory quan trọng.

### 4. Quên shrink level sau delete

❌ **Sai**: Sau khi delete node cao nhất, không giảm `self.level`.

```rust
// Quên dòng này:
while self.level > 0 && self.nodes[0].forward[self.level].is_none() {
    self.level -= 1;
}
```

✅ **Dung**: Luôn kiểm tra và shrink sau delete.

💡 **Tai sao**: Nếu không shrink, search phải duyệt qua các tầng trống — tốn thời gian vô ích, O(max_level) thay vì O(current_level).

## Rust Ecosystem

| Crate | Mô tả | Khi nào dùng |
|---|---|---|
| [`crossbeam-skiplist`](https://docs.rs/crossbeam-skiplist) | Lock-free concurrent skip list | Multi-thread, production |
| [`skiplist`](https://docs.rs/skiplist) | Ordered map/set dựa trên skip list | Single-thread, cần sorted container |

```rust
// crossbeam-skiplist — concurrent skip list
use crossbeam_skiplist::SkipMap;

let map = SkipMap::new();
map.insert(1, "one");
map.insert(2, "two");

// An toàn dùng từ nhiều thread cùng lúc!
assert_eq!(*map.get(&1).unwrap().value(), "one");

// Range query
for entry in map.range(1..=2) {
    println!("{}: {}", entry.key(), entry.value());
}
```

Trong implementation của crate này (`src/skip_list.rs`), ta dùng `Vec` làm arena thay vì raw pointer — an toàn hơn, dễ hiểu hơn, phù hợp để học.

## Practice

| Problem | Gợi ý |
|---|---|
| [Design Skiplist - LeetCode #1206](https://leetcode.com/problems/design-skiplist/) | Implement đúng 3 hàm `search`, `add`, `erase`. Dùng mảng `update` như bài giảng. Nhớ cap MAX_LEVEL! |

Bài #1206 là bài **hiếm hoi** trên LeetCode yêu cầu implement data structure từ đầu. Nếu bạn hiểu chương này, bạn giải được.

Gợi ý thêm cho #1206:
1. Dùng `Vec<Option<usize>>` cho forward pointers (giống code Rust của chúng ta).
2. MAX_LEVEL = 16 là đủ (LeetCode test <= 50,000 operations).
3. Nhớ handle duplicate — `erase` chỉ xóa **1** occurrence.

## Tổng kết

Skip List là một ý tưởng rất đẹp: **thay vì cố gắng cân bằng hoàn hảo (AVL, Red-Black), ta dùng xác suất để đạt kết quả "đủ tốt" với code đơn giản hơn nhiều.**

Bạn gặp Skip List trong thực tế ở:
- **Redis** — Sorted Set dùng Skip List (p=0.25, MAX_LEVEL=32)
- **LevelDB / RocksDB** — memtable dùng Skip List
- **Lucene** — posting list trong search engine

Nếu bạn hiểu linked list và biết tung đồng xu, bạn đã hiểu Skip List!

---

---

[← Fenwick Tree](./02-fenwick-tree.md) | [LRU Cache →](./04-lru-cache.md)
