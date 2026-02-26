# Linked List Tricks

## Tại sao cần học?

Bạn đã học Linked List ở Phần 2. Giờ mình sẽ học 5 tricks mà phỏng vấn hay hỏi nhất -- tất cả dùng **two pointers** trên linked list.

Linked list khó vì bạn không có index. Không thể nhảy tới giữa danh sách như array. Bạn chỉ có thể đi từng bước, từ node này sang node kế tiếp. Vậy làm sao tìm giữa? Làm sao biết có vòng lặp?

Câu trả lời: dùng **hai con trỏ chạy với tốc độ khác nhau**.

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

## Bảng độ phức tạp

| Trick | Time | Space | Kỹ thuật |
|-------|------|-------|----------|
| Find Middle | O(n) | O(1) | Fast/Slow pointer |
| Detect Cycle | O(n) | O(1) | Floyd's tortoise & hare |
| Reverse | O(n) | O(1) | 3 biến: prev, current, next |
| Palindrome | O(n) | O(n) | Thu thập + two pointers |
| Merge Sorted | O(n+m) | O(1) | So sánh đầu hai list |

---

## Tổng kết

Mấu chốt của linked list tricks:

1. **Không có index** --> dùng hai con trỏ với tốc độ khác nhau
2. **Fast/Slow** giải quyết: tìm giữa, phát hiện cycle
3. **Reverse** chỉ cần 3 biến, không cần thêm bộ nhớ
4. **Merge** giống merge sort -- so sánh đầu, lấy cái nhỏ hơn

Nhớ: trong phỏng vấn, linked list trick luôn hỏi về **two pointers**. Nắm 5 bài này là đủ để xử lý hầu hết câu hỏi!
