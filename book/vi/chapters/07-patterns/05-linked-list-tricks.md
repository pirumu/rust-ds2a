# Linked List Tricks

> 💡 **Đừng lo lắng:** Nếu bạn đang nghĩ "trời ơi, linked list trong Rust phức tạp lắm" -- bình tĩnh. Đúng là Rust strict hơn C/Java về ownership, nhưng mấy **trick** ở đây thì logic giống hệt mọi ngôn ngữ. Bạn chỉ cần hiểu ý tưởng, code Rust mình sẽ giải thích từng dòng. Chương này không yêu cầu bạn tự viết linked list từ đầu. Bạn đã làm điều đó ở Phần 2. Giờ mình tập trung vào **patterns** -- những mẹo hay hỏi trong phỏng vấn.

---

## Tại sao cần học?

Bạn đã học Linked List ở Phần 2. Giờ mình sẽ học 5 tricks mà phỏng vấn hay hỏi nhất -- tất cả dùng **two pointers** trên linked list.

Linked list khó vì bạn không có index. Không thể nhảy tới giữa danh sách như array. Bạn chỉ có thể đi từng bước, từ node này sang node kế tiếp. Vậy làm sao tìm giữa? Làm sao biết có vòng lặp?

Câu trả lời: dùng **hai con trỏ chạy với tốc độ khác nhau**.

> **Bridge từ Chương 6:** Nhớ two pointers trên array không? Một trái một phải, hoặc cả hai cùng hướng. Fast/slow pointer trên linked list chính là **biến thể cùng hướng** -- cả hai bắt đầu từ head, nhưng đi với tốc độ khác nhau. Cùng kỹ thuật, khác cấu trúc dữ liệu!

---

## Dummy Node Trick — Vũ khí bí mật

Trước khi vào các bài chính, mình giới thiệu một trick nhỏ nhưng cực kỳ hữu ích: **dummy node** (node giả).

### Vấn đề

Khi bạn thêm/xóa node trong linked list, head có thể thay đổi. Code phải xử lý riêng trường hợp head = None hoặc head bị xóa. Rất rối!

```
Không có dummy node:
  if head is None:
      head = new_node       <-- xử lý riêng
  else:
      current.next = new_node

Có dummy node:
  dummy -> head -> ... -> tail
  ^
  Luôn có node trước head!
  --> Không cần xử lý riêng edge case
```

### Ý tưởng

Tạo một node "giả" đứng trước head. Dummy node không chứa dữ liệu thật -- nó chỉ là chỗ đứng để code luôn có `prev` node. Cuối cùng, trả về `dummy.next` là head thật.

```rust
// Dummy node pattern trong Rust:
let mut dummy = Box::new(Node { val: 0, next: head });
let mut current = &mut dummy;

// ... thao tác trên current.next ...

dummy.next  // <-- head thật sự
```

Bạn sẽ thấy dummy node xuất hiện trong merge sorted lists, reverse in groups, và rất nhiều bài khác. Khi nào thấy bài yêu cầu **modify head** hoặc **tạo list mới**, hãy nghĩ đến dummy node trước!

---

## Hình ảnh: Hai người chạy trên đường đua

Tưởng tượng hai người chạy trên một con đường thẳng:

```
Người chậm (Rùa):    1 bước mỗi lần
Người nhanh (Thỏ):   2 bước mỗi lần

Bắt đầu:
  Rùa ──▶
  Thỏ ──▶▶

Sau 1 vòng:
  ○──○──○──○──○──○──○──○──○──○
  ^     ^
  Rùa   Thỏ

Sau 2 vòng:
  ○──○──○──○──○──○──○──○──○──○
        ^           ^
        Rùa         Thỏ

Sau 3 vòng:
  ○──○──○──○──○──○──○──○──○──○
              ^                 ^
              Rùa               Thỏ (đến cuối!)
```

Khi Thỏ đến cuối, Rùa đang ở **giữa đường**! Vì Thỏ chạy nhanh gấp đôi, nên khi Thỏ đi hết đường thì Rùa mới đi được nửa.

Đây là ý tưởng cốt lõi cho mọi trick trong chương này.

---

## 1. Find Middle -- Tìm phần tử ở giữa

### Bài toán

Cho linked list, tìm giá trị ở node giữa.

### Ý tưởng

Dùng slow pointer (1 bước) và fast pointer (2 bước). Khi fast đến cuối, slow ở giữa.

```
List: 1 -> 2 -> 3 -> 4 -> 5

Bước 0:  [1] -> [2] -> [3] -> [4] -> [5]
          S
          F

Bước 1:  [1] -> [2] -> [3] -> [4] -> [5]
                  S
                          F

Bước 2:  [1] -> [2] -> [3] -> [4] -> [5]
                          S
                                          F (hết!)

--> slow ở node 3. Đó là giữa!
```

Với list chẵn phần tử:

```
List: 1 -> 2 -> 3 -> 4

Bước 0:  [1] -> [2] -> [3] -> [4]
          S
          F

Bước 1:  [1] -> [2] -> [3] -> [4]
                  S
                          F

Bước 2:  fast.next.next = None --> dừng!

--> slow ở node 2 (phần tử giữa bên trái)
```

### Code

```rust
pub fn find_middle(head: &Link) -> Option<i32> {
    let vals = to_vec(head);
    if vals.is_empty() {
        return None;
    }
    let mut slow = 0;
    let mut fast = 0;
    while fast + 1 < vals.len() && fast + 2 <= vals.len() {
        slow += 1;
        fast += 2;
        if fast >= vals.len() {
            break;
        }
    }
    Some(vals[slow])
}
```

**Time:** O(n). **Space:** O(1) (nếu dùng con trỏ trực tiếp; ở đây ta dùng Vec phụ vì Rust borrow rules).

---

## 2. Detect Cycle -- Phát hiện vòng lặp (Floyd's Algorithm)

### Bài toán

Linked list có thể bị "hỏng" -- node cuối trỏ ngược lại một node trước đó, tạo vòng lặp vô hạn.

```
1 -> 2 -> 3 -> 4
          ^    |
          |    v
          6 <- 5

Nếu duyệt bình thường: 1, 2, 3, 4, 5, 6, 3, 4, 5, 6, ... (lặp mãi!)
```

### Ý tưởng: Rùa và Thỏ trên đường tròn

Nếu đường đua có vòng tròn, Thỏ (nhanh hơn) sẽ **chạy vòng lại** và gặp Rùa. Giống như khi bạn chạy bộ trên sân vận động -- người nhanh sẽ bắt kịp người chậm!

```
Không có cycle:
  Rùa: 1 -> 2 -> 3 -> 4 -> 5 -> END
  Thỏ: 1 -> 3 -> 5 -> END
  --> Thỏ đến cuối, không gặp Rùa = KHÔNG có cycle

Có cycle:
  1 -> 2 -> 3 -> 4 -> 5
            ^         |
            |         v
            7 <- 6 <--+

  Bước 0: Rùa=1, Thỏ=1
  Bước 1: Rùa=2, Thỏ=3
  Bước 2: Rùa=3, Thỏ=5
  Bước 3: Rùa=4, Thỏ=7
  Bước 4: Rùa=5, Thỏ=4  (Thỏ đang vòng!)
  Bước 5: Rùa=6, Thỏ=6  --> GẶP NHAU! Có cycle!
```

### Tại sao Floyd's chắc chắn hoạt động?

Nhiều bạn thắc mắc: "Sao biết Thỏ chắc chắn gặp Rùa? Lỡ nó nhảy qua luôn thì sao?"

Đây là cách hiểu đơn giản nhất:

```
Khi cả hai đã vào trong vòng:
  - Mỗi bước, khoảng cách giữa Thỏ và Rùa GIẢM đi 1
  - Vì: Thỏ đi 2, Rùa đi 1 --> Thỏ tiến gần hơn 1 bước

  Ví dụ: vòng có 6 node, khoảng cách ban đầu = 4

  Bước 0: khoảng cách = 4
  Bước 1: khoảng cách = 3    (Thỏ gần hơn 1)
  Bước 2: khoảng cách = 2
  Bước 3: khoảng cách = 1
  Bước 4: khoảng cách = 0    --> GẶP!
```

Khoảng cách giảm 1 mỗi bước, nên **không thể nhảy qua**. Luôn gặp nhau sau tối đa `n` bước (n = kích thước vòng).

### Code

Vì Rust dùng `Box` (ownership) nên không thể tạo cycle thật. Mình dùng mảng `next[]` để mô phỏng:

```rust
/// next[i] = index of next node, usize::MAX = end
pub fn has_cycle(next: &[usize]) -> bool {
    if next.is_empty() {
        return false;
    }
    let mut slow: usize = 0;
    let mut fast: usize = 0;
    loop {
        // slow đi 1 bước
        if next[slow] == usize::MAX { return false; }
        slow = next[slow];

        // fast đi 2 bước
        if next[fast] == usize::MAX { return false; }
        fast = next[fast];
        if next[fast] == usize::MAX { return false; }
        fast = next[fast];

        if slow == fast { return true; }
    }
}
```

**Time:** O(n). **Space:** O(1).

---

## 3. Reverse -- Đảo ngược linked list

### Bài toán

Đảo ngược hướng của tất cả các mũi tên.

```
Trước:   1 -> 2 -> 3 -> 4 -> 5 -> None
Sau:     5 -> 4 -> 3 -> 2 -> 1 -> None
```

### Ý tưởng

Tưởng tượng một hàng người đang quay mặt sang phải. Bạn muốn tất cả quay sang trái. Bạn đi từ đầu hàng, bảo từng người quay lại:

```
Bước 0:  prev=None   current=1   next=2
         None  <-x-  [1] -> [2] -> [3] -> None

Bước 1:  prev=1      current=2   next=3
         None <- [1] <-x- [2] -> [3] -> None

Bước 2:  prev=2      current=3   next=None
         None <- [1] <- [2] <-x- [3] -> None

Bước 3:  current=None --> dừng!
         None <- [1] <- [2] <- [3]
                                 ^
                                head mới!
```

Ba biến: `prev`, `current`, `next`. Mỗi bước:
1. Lưu `next = current.next`
2. Đổi hướng: `current.next = prev`
3. Tiến lên: `prev = current`, `current = next`

### Code

```rust
pub fn reverse(head: Link) -> Link {
    let mut prev: Link = None;
    let mut current = head;
    while let Some(mut node) = current {
        current = node.next.take();  // lưu next
        node.next = prev;            // đổi hướng
        prev = Some(node);           // tiến lên
    }
    prev
}
```

**Time:** O(n). **Space:** O(1).

---

## 4. Palindrome Check -- Kiểm tra palindrome

### Bài toán

Linked list `1 -> 2 -> 3 -> 2 -> 1` là palindrome (đọc xuôi ngược giống nhau).
Linked list `1 -> 2 -> 3` thì không.

### Ý tưởng

Thu thập giá trị vào array, rồi dùng two pointers từ hai đầu:

```
List: 1 -> 2 -> 3 -> 2 -> 1
Array: [1, 2, 3, 2, 1]

  L                 R
  [1,  2,  3,  2,  1]
   ^               ^    1 == 1 ✓
       ^       ^         2 == 2 ✓
           ^             L >= R --> palindrome!
```

### Code

```rust
pub fn is_palindrome(head: &Link) -> bool {
    let vals = to_vec(head);
    let mut left = 0;
    let mut right = vals.len().wrapping_sub(1);
    while left < right {
        if vals[left] != vals[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

**Time:** O(n). **Space:** O(n).

---

## 5. Merge Two Sorted Lists -- Gộp hai danh sách đã sắp xếp

### Bài toán

Cho hai linked list đã sắp xếp, gộp thành một list sắp xếp.

### Ý tưởng

Giống như xếp hai chồng bài đã sắp xếp thành một chồng. Mỗi lần so sánh lá bài trên cùng của hai chồng, lấy lá nhỏ hơn:

```
L1: 1 -> 3 -> 5
L2: 2 -> 4 -> 6

Bước 1: So sánh 1 vs 2 --> lấy 1.   Kết quả: 1
Bước 2: So sánh 3 vs 2 --> lấy 2.   Kết quả: 1 -> 2
Bước 3: So sánh 3 vs 4 --> lấy 3.   Kết quả: 1 -> 2 -> 3
Bước 4: So sánh 5 vs 4 --> lấy 4.   Kết quả: 1 -> 2 -> 3 -> 4
Bước 5: So sánh 5 vs 6 --> lấy 5.   Kết quả: 1 -> 2 -> 3 -> 4 -> 5
Bước 6: L1 hết, lấy 6.              Kết quả: 1 -> 2 -> 3 -> 4 -> 5 -> 6
```

### Code

```rust
pub fn merge_two_sorted(l1: Link, l2: Link) -> Link {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val <= n2.val {
                n1.next = merge_two_sorted(n1.next.take(), Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_sorted(Some(n1), n2.next.take());
                Some(n2)
            }
        }
    }
}
```

**Time:** O(n + m). **Space:** O(1) extra (tái sử dụng node có sẵn, nhưng call stack dùng O(n+m)).

---

## 6. Reverse in K-Groups -- Đảo ngược từng nhóm K

### Bài toán

Cho linked list và số nguyên `k`, đảo ngược từng nhóm `k` node. Nếu nhóm cuối ít hơn `k` node thì giữ nguyên.

```
Input:  1 -> 2 -> 3 -> 4 -> 5,  k = 3
Output: 3 -> 2 -> 1 -> 4 -> 5
        [đảo nhóm 1] [giữ nguyên, chỉ có 2 < k]

Input:  1 -> 2 -> 3 -> 4 -> 5 -> 6,  k = 2
Output: 2 -> 1 -> 4 -> 3 -> 6 -> 5
        [nhóm1] [nhóm2] [nhóm3]
```

### Ý tưởng

Đây là bài **Reverse Linked List** lặp lại nhiều lần. Mỗi lần:

1. Đếm xem có đủ `k` node không
2. Nếu đủ, reverse `k` node đó (dùng đúng trick 3 biến prev/current/next)
3. Nối nhóm vừa reverse với phần còn lại
4. Lặp lại cho nhóm tiếp theo

```
k = 3:
  Bước 1: Tách nhóm [1,2,3] và phần còn lại [4,5]

    [1] -> [2] -> [3]    |    [4] -> [5]
    ^^^^^^^^^^^^^^^^^^         ^^^^^^^^^^^
    reverse nhóm này           giữ nguyên (< k)

  Bước 2: Reverse nhóm [1,2,3]:
    [3] -> [2] -> [1]

  Bước 3: Nối lại:
    [3] -> [2] -> [1] -> [4] -> [5]
```

### Code

```rust
pub fn reverse_k_group(vals: &[i32], k: usize) -> Vec<i32> {
    if k <= 1 {
        return vals.to_vec();
    }
    let mut result = Vec::with_capacity(vals.len());
    let mut i = 0;
    while i + k <= vals.len() {
        // Reverse nhóm k phần tử
        for j in (i..i + k).rev() {
            result.push(vals[j]);
        }
        i += k;
    }
    // Phần còn lại (< k phần tử): giữ nguyên
    for j in i..vals.len() {
        result.push(vals[j]);
    }
    result
}
```

**Time:** O(n). **Space:** O(1) extra (nếu dùng linked list thật, ta swap in-place).

> **Ghi chú:** Trên linked list thật, bài này khó hơn nhiều vì phải cẩn thận nối đuôi nhóm trước với đầu nhóm sau. Đây là nơi **dummy node** cực kỳ hữu ích -- dummy đứng trước head, giúp bạn không phải xử lý riêng nhóm đầu tiên.

---

## Pitfalls — Bẫy hay gặp

### Bẫy 1: Null pointer khi traverse

❌ **Sai:** Truy cập `node.next.next` mà không kiểm tra `node.next` trước

```rust
// CRASH nếu node.next là None!
let grandchild = node.next.unwrap().next;
```

✅ **Đúng:** Luôn kiểm tra trước khi truy cập

```rust
if let Some(ref next_node) = node.next {
    let grandchild = &next_node.next;
}
```

💡 **Tại sao:** Linked list kết thúc bằng None. Fast pointer đi 2 bước, nên phải kiểm tra CẢ HAI bước. Đây là lỗi #1 trong phỏng vấn!

---

### Bẫy 2: Quên update next pointers khi reverse

❌ **Sai:** Chỉ đổi hướng mà quên lưu next trước

```
Trước khi đổi hướng:  A -> B -> C
Đổi B.next = A mà quên lưu C:
  A <- B    C (mất liên kết tới C!)
```

✅ **Đúng:** Luôn lưu `next` TRƯỚC khi đổi hướng

```rust
let next = current.next.take();  // lưu trước!
current.next = prev;              // rồi mới đổi
```

💡 **Tại sao:** Khi bạn thay đổi `current.next`, bạn mất reference tới node tiếp theo. Nếu không lưu trước, phần còn lại của list bị "mồ côi" -- không ai trỏ tới nữa.

---

### Bẫy 3: Memory leak -- Rust cứu bạn!

❌ **Sai (C/C++):** Reverse xong quên free các node cũ --> memory leak

✅ **Đúng (Rust):** Ownership system tự xử lý. Khi bạn dùng `.take()`, ownership chuyển sang biến mới. Khi biến cũ hết scope, Rust tự drop.

💡 **Tại sao:** Đây là lý do Rust tuyệt vời cho linked list tricks. Bạn tập trung vào logic, không lo memory. Compiler bắt lỗi cho bạn trước khi chương trình chạy!

---

### Bẫy 4: Off-by-one trong fast pointer

❌ **Sai:** Dừng fast pointer khi `fast.next == None`

✅ **Đúng:** Phải check cả `fast != None` VÀ `fast.next != None`

```rust
// Find middle -- điều kiện dừng đúng:
while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
    slow = slow.unwrap().next;       // 1 bước
    fast = fast.unwrap().next.next;  // 2 bước
}
```

💡 **Tại sao:** List chẵn vs lẻ phần tử có điều kiện dừng khác nhau. Nếu list chẵn, fast dừng ở cuối; nếu lẻ, fast dừng ở trước cuối. Check cả hai để an toàn!

---

## Bảng độ phức tạp

| Trick | Time | Space | Kỹ thuật |
|-------|------|-------|----------|
| Find Middle | O(n) | O(1) | Fast/Slow pointer |
| Detect Cycle | O(n) | O(1) | Floyd's tortoise & hare |
| Reverse | O(n) | O(1) | 3 biến: prev, current, next |
| Palindrome | O(n) | O(n) | Thu thập + two pointers |
| Merge Sorted | O(n+m) | O(1) | So sánh đầu hai list |
| Reverse K-Group | O(n) | O(1) | Reverse lặp + dummy node |

---

## Khi nào dùng trick nào?

| Bạn cần... | Dùng trick | Gợi ý nhận biết |
|-------------|-----------|-----------------|
| Tìm vị trí giữa / chia đôi list | **Find Middle** | "middle", "split", "half" |
| Kiểm tra list có vòng lặp | **Floyd's Cycle** | "cycle", "loop", "circular" |
| Đảo ngược toàn bộ hoặc một phần | **Reverse** | "reverse", "backward" |
| Kiểm tra đối xứng | **Palindrome** | "palindrome", "same forward and backward" |
| Gộp hai danh sách có thứ tự | **Merge Sorted** | "merge", "combine", "sorted lists" |
| Đảo ngược từng nhóm | **Reverse K-Group** | "k-group", "reverse every k" |
| Tránh edge case khi modify head | **Dummy Node** | Bất kỳ bài nào thay đổi head |

---

## Rust Ecosystem — Linked list trong thực tế

### `std::collections::LinkedList`

Rust standard library có `LinkedList<T>` -- doubly linked list. Nhưng thực tế **hầu như không ai dùng**:

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_back(3);

// Duyệt:
for val in &list {
    println!("{}", val);
}
```

### Tại sao ít dùng?

| | `Vec<T>` | `LinkedList<T>` |
|---|---|---|
| Cache-friendly | Dữ liệu liền nhau trong RAM | Node rải rác khắp nơi |
| Insert/Delete đầu | O(n) -- phải dịch | O(1) |
| Insert/Delete giữa | O(n) | O(1) nếu có con trỏ |
| Random access | O(1) | O(n) |
| **Thực tế nhanh hơn** | **Hầu hết trường hợp** | Rất hiếm khi nhanh hơn |

> **Lời khuyên thực tế:** Trong code production Rust, dùng `Vec<T>` hoặc `VecDeque<T>` thay cho linked list. CPU cache hiện đại làm cho truy cập tuần tự trên `Vec` nhanh hơn linked list dù Big-O nói khác. Linked list chủ yếu hữu ích trong phỏng vấn và một số cấu trúc đặc biệt (LRU Cache, OS scheduler).

### Khi nào linked list thật sự hữu ích?

- **LRU Cache** -- cần O(1) insert/delete ở bất kỳ đâu (kết hợp với HashMap)
- **OS task scheduler** -- cần insert/remove process nhanh
- **Undo history** -- thêm/xóa ở cuối, không cần random access
- **Mô phỏng** -- khi kích thước thay đổi liên tục và không cần index

---

## Practice — Luyện tập

### Đã nắm vững (bài trong chương):

| # | Bài | Trick |
|---|-----|-------|
| 876 | Middle of the Linked List | Fast/Slow |
| 141 | Linked List Cycle | Floyd's |
| 206 | Reverse Linked List | 3-variable reverse |
| 234 | Palindrome Linked List | Collect + two pointers |
| 21 | Merge Two Sorted Lists | Compare heads |

### Thử sức tiếp:

| # | Bài | Gợi ý | Độ khó |
|---|-----|-------|--------|
| 25 | Reverse Nodes in k-Group | Reverse + dummy node + đếm k | Hard |
| 138 | Copy List with Random Pointer | HashMap lưu mapping old -> new node | Medium |
| 146 | LRU Cache | Doubly linked list + HashMap | Medium |
| 142 | Linked List Cycle II | Floyd's + thêm bước tìm điểm vào cycle | Medium |
| 19 | Remove Nth Node From End | Two pointers cách nhau n bước | Medium |
| 143 | Reorder List | Find middle + reverse + merge | Medium |

**Gợi ý cho bài 25 (Reverse Nodes in k-Group):**
- Dùng dummy node trước head
- Với mỗi nhóm: đếm k node, nếu đủ thì reverse nhóm đó
- Cẩn thận nối đuôi nhóm trước với đầu nhóm sau

**Gợi ý cho bài 138 (Copy List with Random Pointer):**
- Bước 1: Tạo copy từng node, lưu vào HashMap `{old_node: new_node}`
- Bước 2: Duyệt lại, set `next` và `random` cho mỗi copy dựa vào HashMap

**Gợi ý cho bài 146 (LRU Cache):**
- Kết hợp HashMap (O(1) lookup) + Doubly Linked List (O(1) move to front)
- Mỗi lần access: move node lên đầu list
- Khi cache đầy: xóa node ở cuối list

---

## Tổng kết

Mấu chốt của linked list tricks:

1. **Không có index** --> dùng hai con trỏ với tốc độ khác nhau
2. **Fast/Slow** giải quyết: tìm giữa, phát hiện cycle
3. **Reverse** chỉ cần 3 biến, không cần thêm bộ nhớ
4. **Merge** giống merge sort -- so sánh đầu, lấy cái nhỏ hơn
5. **Dummy node** giúp tránh edge case khi head thay đổi
6. **Reverse K-Group** = reverse + đếm + nối cẩn thận

Nhớ: trong phỏng vấn, linked list trick luôn hỏi về **two pointers**. Nắm vững 6 bài chính + dummy node là đủ để xử lý hầu hết câu hỏi!

---

## Tiếp theo

Chương sau: **[Top-K Problems](./06-top-k.md)** — tìm K phần tử lớn/nhỏ nhất hiệu quả. Bạn đã biết Heap từ Phần 3 và Quick Sort từ Phần 6 — chương sau chỉ là ghép hai thứ đó lại: min-heap size K và Quick Select.

---

[← Matrix Traversal](./04-matrix-traversal.md) | [Top-K Problems →](./06-top-k.md)
