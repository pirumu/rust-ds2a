# Top-K Problems

> 💡 **Đừng lo lắng:** Chương này nghe có vẻ "nặng" nhưng thật ra bạn ĐÃ biết hết các công cụ rồi. Heap? Học ở Phần 3. Quick Sort/Partition? Phần 6. Chương này chỉ là **ghép lego** -- lắp các mảnh bạn đã có thành pattern giải bài. Nếu bạn quên Heap, quay lại đọc lại 5 phút rồi quay về đây. Không sao cả.

## Đây là gì?

Bạn đã học Heap ở Phần 3 và Quick Sort ở Phần 6. Giờ mình dùng chúng để giải bài "tìm K phần tử lớn/nhỏ nhất" -- một dạng bài cực kỳ phổ biến trong phỏng vấn.

Tưởng tượng bạn muốn tìm **3 quán phở ngon nhất Sài Gòn**. Bạn không cần xếp hạng TẤT CẢ hàng ngàn quán phở -- chỉ cần giữ lại top 3. Mỗi khi thử quán mới ngon hơn quán kém nhất trong top 3, bạn thay thế nó. Đây chính là ý tưởng của **min-heap size K**.

Hoặc kiểu khác: bạn có danh sách 1000 học sinh, muốn tìm điểm cao thứ 500 (median). Sort toàn bộ thì lãng phí -- **Quick Select** cho bạn đáp án trong O(n) trung bình mà không cần sort.

> **Tại sao quan trọng?** Top-K xuất hiện khắp nơi: autocomplete (top K gợi ý), recommendation system (K sản phẩm phù hợp nhất), monitoring (K server chậm nhất), database (LIMIT K)...

> **Bridge:** Priority Queue mà bạn học ở chương Heap chính là tool chính cho Top-K. Nếu bạn nắm được `BinaryHeap` và `Reverse`, bạn đã có 80% vũ khí cần thiết.

---

## 3 cách tiếp cận -- So sánh trước khi đi sâu

Cho mảng n phần tử, tìm K phần tử lớn nhất. Có 3 cách:

```
Cách 1: Sort rồi lấy K phần tử cuối
─────────────────────────────────────
  [5, 3, 8, 1, 9, 2, 7]
        ↓ sort
  [1, 2, 3, 5, 7, 8, 9]
                  └──┘ lấy K=2 cuối → [8, 9]

  Time: O(n log n)   Space: O(1)*
  * tùy thuật toán sort

Cách 2: Min-Heap size K
────────────────────────
  Duyệt từng phần tử, giữ heap luôn có đúng K phần tử lớn nhất:
  - Heap chưa đầy K → push vào
  - Heap đầy K rồi → so với min(heap):
    - Phần tử mới > min? → pop min, push mới
    - Phần tử mới <= min? → bỏ qua

  Time: O(n log k)   Space: O(k)

Cách 3: Quick Select
─────────────────────
  Partition mảng quanh pivot, chỉ đi vào nửa chứa index K.
  Không cần sort toàn bộ!

  Time: O(n) avg, O(n²) worst   Space: O(1)
```

| Cách | Time | Space | Ưu điểm | Nhược điểm |
|---|---|---|---|---|
| Sort | O(n log n) | O(1)* | Đơn giản, dễ code | Chậm khi n lớn, k nhỏ |
| Min-Heap | O(n log k) | O(k) | Tốt khi k << n, stream data | Cần biết dùng heap |
| Quick Select | O(n) avg | O(1) | Nhanh nhất trung bình | Worst case O(n²), modify mảng |

**Chọn cái nào?** Đa số phỏng vấn, **Min-Heap** là câu trả lời an toàn nhất. Nhanh, ổn định, và dễ giải thích.

---

## Min-Heap cho Top-K -- Pattern cốt lõi

Đây là pattern quan trọng nhất chương này. Học nắm kỹ pattern này, các bài sau chỉ là biến thể.

### Ý tưởng cốt lõi

Giữ một min-heap có đúng K phần tử. Phần tử nhỏ nhất luôn nằm ở đỉnh heap (đó là "cửa gác"). Bất kỳ phần tử nào muốn vào top-K phải "đánh bại" phần tử yếu nhất hiện tại.

```
Tìm top 3 lớn nhất trong [5, 3, 8, 1, 9, 2, 7]:

Duyệt 5 → heap: [5]                   size=1 < k=3, push
Duyệt 3 → heap: [3, 5]                size=2 < k=3, push
Duyệt 8 → heap: [3, 5, 8]             size=3 = k, đầy rồi!
Duyệt 1 → 1 < min(heap)=3? Bỏ qua    ← yếu hơn "cửa gác"
Duyệt 9 → 9 > min(heap)=3? Pop 3, push 9
           heap: [5, 8, 9]
Duyệt 2 → 2 < min(heap)=5? Bỏ qua
Duyệt 7 → 7 > min(heap)=5? Pop 5, push 7
           heap: [7, 8, 9]             ← Top 3!
```

### Pseudocode

```
fn top_k(data, k):
    min_heap = new MinHeap()
    for item in data:
        heap.push(item)
        if heap.size() > k:
            heap.pop()      // loại phần tử nhỏ nhất
    return heap.elements()  // k phần tử còn lại = top k
```

Nhìn lại: mỗi phần tử chỉ cần 1 lần push (O(log k)) và tối đa 1 lần pop (O(log k)). Tổng: O(n log k). Khi k nhỏ hơn n nhiều (ví dụ tìm top 10 trong 1 triệu), log k rất nhỏ so với log n.

---

## Quick Select -- tìm phần tử nhỏ thứ K

### Ý tưởng

Nhớ Quick Sort không? Nó chọn pivot, chia mảng thành 2 phần (nhỏ hơn pivot | lớn hơn pivot), rồi sort cả 2 phần.

Quick Select cũng làm vậy, nhưng **chỉ đi vào 1 phần** -- phần chứa index K. Nên trung bình chỉ mất O(n) thay vì O(n log n).

```
Tìm phần tử nhỏ thứ 2 (k=2) trong [3, 2, 1, 5, 4]:

Bước 1: Chọn pivot = 4 (phần tử cuối)
        Partition: [3, 2, 1] | 4 | [5]
        Pivot ở index 3 → k=2 < 3 → đi vào nửa trái

Bước 2: [3, 2, 1], chọn pivot = 1
        Partition: [] | 1 | [3, 2]
        Pivot ở index 0 → k=2 > 0 → đi vào nửa phải

Bước 3: [3, 2], chọn pivot = 2
        Partition: [] | 2 | [3]
        Pivot ở index 2 → k=2 == 2 → Tìm thấy! Đáp án = 3

Mảng đã sorted: [1, 2, 3, 4, 5]
                         ^ index 2 = phần tử nhỏ thứ 3 (0-indexed)
```

### Code Rust

```rust
pub fn quick_select(arr: &mut [i32], k: usize) -> i32
```

Gọi `quick_select(&mut arr, 0)` để tìm min, `quick_select(&mut arr, n-1)` để tìm max.

> **Lưu ý:** Quick Select thay đổi mảng gốc (partition sẽ swap). Nếu cần giữ nguyên mảng, clone trước.

---

## Top K Frequent Elements -- K phần tử xuất hiện nhiều nhất

### Bài toán

Cho mảng số nguyên, tìm K số xuất hiện nhiều nhất.

Ví dụ: `[1, 1, 1, 2, 2, 3]`, k=2 → đáp án `[1, 2]` (1 xuất hiện 3 lần, 2 xuất hiện 2 lần).

### Ý tưởng

1. **Đếm tần suất** bằng HashMap: O(n)
2. **Giữ min-heap size K**: duyệt qua từng cặp (số, tần suất). Nếu heap chưa đầy K → push. Nếu đầy rồi và tần suất mới > tần suất nhỏ nhất trong heap → pop + push.

```
nums = [1, 1, 1, 2, 2, 3], k = 2

Bước 1 — Đếm:
  {1: 3, 2: 2, 3: 1}

Bước 2 — Min-heap (giữ top 2):
  Thêm (3, 1) → heap: [(3, 1)]              size=1 < k=2
  Thêm (2, 2) → heap: [(2, 2), (3, 1)]      size=2 == k
  Thêm (1, 3) → freq 1 < min freq 2? Không! → pop (1, 3), push...
    Thực ra (1, 3) có freq=1, min trong heap = freq=2
    freq(3) = 1 < 2 → BỎ QUA, không thêm

  Kết quả: [1, 2] ✓
```

> **Tại sao min-heap mà không max-heap?** Vì ta muốn loại bỏ phần tử có tần suất THẤP NHẤT. Min-heap cho phép peek/pop phần tử nhỏ nhất trong O(1)/O(log k).

---

## K Closest Points to Origin -- K điểm gần gốc tọa độ nhất

### Bài toán

Cho danh sách điểm 2D, tìm K điểm gần gốc tọa độ (0,0) nhất.

Khoảng cách = √(x² + y²), nhưng ta so sánh x² + y² (bỏ căn) để tránh float.

### Ý tưởng

Giống Top K Frequent, nhưng thay tần suất bằng khoảng cách. Dùng **max-heap size K** -- phần tử xa nhất trong top K nằm ở đỉnh, dễ dàng so sánh với điểm mới.

```
Points: [3,3], [5,-1], [-2,4]    k = 2

Khoảng cách²:
  [3,3]  → 9 + 9   = 18
  [5,-1] → 25 + 1  = 26
  [-2,4] → 4 + 16  = 20

Max-heap (giữ K=2 nhỏ nhất):
  Push (18, [3,3])  → heap: [(18, [3,3])]
  Push (26, [5,-1]) → heap: [(26, [5,-1]), (18, [3,3])]   size=2=k
  Push (20, [-2,4]) → 20 < max(26)? Có!
    Pop (26, [5,-1]), Push (20, [-2,4])
    → heap: [(20, [-2,4]), (18, [3,3])]

Kết quả: [[3,3], [-2,4]] ✓
```

> **Khoan, sao lúc nãy dùng min-heap, giờ lại max-heap?**
> - Tìm K **lớn nhất** → dùng **min-heap** (loại nhỏ nhất)
> - Tìm K **nhỏ nhất** → dùng **max-heap** (loại lớn nhất)
>
> Nghe ngược đời, nhưng logic là: heap luôn **loại bỏ phần tử ở đỉnh**, nên đỉnh phải là phần tử bạn muốn loại.

---

## Merge K Sorted Arrays -- gộp K mảng đã sắp xếp

### Bài toán

Cho K mảng, mỗi mảng đã sorted. Gộp thành 1 mảng sorted duy nhất.

### Ý tưởng

Tưởng tượng bạn có **K hàng người** xếp theo chiều cao tăng dần. Bạn muốn gộp thành 1 hàng duy nhất. Cách tốt nhất: **luôn chọn người thấp nhất** trong K người đứng đầu mỗi hàng.

Dùng min-heap chứa K phần tử (1 phần tử đầu tiên của mỗi mảng). Pop min → thêm vào kết quả → push phần tử tiếp theo từ mảng đó.

```
Lists: [1,4,5], [1,3,4], [2,6]

Min-heap ban đầu: [(1, list0), (1, list1), (2, list2)]

Pop (1, list0) → result=[1],     push 4 từ list0 → heap: [(1,l1),(2,l2),(4,l0)]
Pop (1, list1) → result=[1,1],   push 3 từ list1 → heap: [(2,l2),(3,l1),(4,l0)]
Pop (2, list2) → result=[1,1,2], push 6 từ list2 → heap: [(3,l1),(4,l0),(6,l2)]
Pop (3, list1) → result=[1,1,2,3], push 4 từ list1 → heap: [(4,l0),(4,l1),(6,l2)]
Pop (4, list0) → result=[1,1,2,3,4], push 5 → heap: [(4,l1),(5,l0),(6,l2)]
Pop (4, list1) → result=[1,1,2,3,4,4], list1 hết → heap: [(5,l0),(6,l2)]
Pop (5, list0) → result=[1,1,2,3,4,4,5], list0 hết → heap: [(6,l2)]
Pop (6, list2) → result=[1,1,2,3,4,4,5,6], list2 hết → DONE ✓
```

> **Tại sao không merge từng cặp?** Merge 2 mảng mỗi lần: O(N * K). Dùng heap: O(N log K) -- nhanh hơn nhiều khi K lớn.

---

## Pitfalls -- Lỗi hay gặp

### Pitfall 1: Min-heap vs Max-heap cho Top-K

❌ Sai: Tìm K phần tử **lớn nhất** → dùng max-heap size K

✅ Đúng: Tìm K phần tử **lớn nhất** → dùng **min-heap** size K

💡 Tại sao: Max-heap giữ phần tử lớn nhất ở đỉnh. Nếu bạn pop, bạn loại phần tử lớn nhất -- chính là phần tử bạn muốn GIỮ! Min-heap loại phần tử nhỏ nhất -- đúng là phần tử yếu nhất trong top K cần bị thay thế.

```
Bạn muốn giữ top 3 lớn nhất: [9, 8, 7]

Max-heap: đỉnh = 9 → pop = mất 9 → SAI!
Min-heap: đỉnh = 7 → pop = mất 7 → thay bằng phần tử mới lớn hơn → ĐÚNG!
```

### Pitfall 2: Quick Select worst case

❌ Sai: Quick Select luôn O(n)

✅ Đúng: Quick Select **trung bình** O(n), **worst case** O(n²)

💡 Tại sao: Nếu pivot luôn là phần tử nhỏ nhất/lớn nhất (ví dụ mảng đã sorted + chọn phần tử cuối làm pivot), mỗi lần partition chỉ giảm 1 phần tử. n + (n-1) + (n-2) + ... = O(n²). Cách khắc phục: dùng **randomized pivot** hoặc **median-of-three**.

### Pitfall 3: Quên clone mảng khi dùng Quick Select

❌ Sai: `let kth = quick_select(&mut arr, k);` rồi dùng `arr` như ban đầu

✅ Đúng: `let mut copy = arr.clone(); let kth = quick_select(&mut copy, k);`

💡 Tại sao: Quick Select partition (swap) phần tử trong mảng. Sau khi gọi xong, thứ tự phần tử trong mảng đã thay đổi.

### Pitfall 4: Nhầm Rust `BinaryHeap` là min-heap

❌ Sai: `BinaryHeap::new()` rồi mong nó là min-heap

✅ Đúng: Dùng `BinaryHeap<Reverse<T>>` để biến thành min-heap

💡 Tại sao: Rust `BinaryHeap` mặc định là **max-heap**. Wrap giá trị trong `std::cmp::Reverse(...)` để đảo thứ tự.

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Max-heap (mặc định)
let mut max_heap = BinaryHeap::new();
max_heap.push(3);
max_heap.push(1);
assert_eq!(max_heap.peek(), Some(&3)); // lớn nhất ở đỉnh

// Min-heap (dùng Reverse)
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
assert_eq!(min_heap.peek(), Some(&Reverse(1))); // nhỏ nhất ở đỉnh
```

---

## Khi nào dùng gì?

| Tình huống | Dùng | Tại sao |
|---|---|---|
| Tìm **chính xác** phần tử thứ K | Quick Select | O(n) avg, không cần extra space |
| Tìm **top K** lớn/nhỏ nhất | Min/Max-heap size K | O(n log k), ổn định |
| Data đến dạng **stream** (không biết trước hết) | Min/Max-heap size K | Xử lý từng phần tử, không cần toàn bộ data |
| Tìm top K theo **tiêu chí phức tạp** (tần suất, khoảng cách...) | Heap + HashMap/công thức | Tính metric trước, heap giữ top K |
| Gộp **K nguồn sorted** | Min-heap K-way merge | O(N log K), luôn lấy min từ K nguồn |
| n nhỏ, code đơn giản là ưu tiên | Sort + lấy K cuối | Dễ viết, dễ debug, đủ nhanh |
| Cần kết quả **sorted** | Sort hoặc Heap + sort | Quick Select không cho thứ tự |

---

## Bảng độ phức tạp

| Thuật toán | Time | Space | Khi nào dùng |
|---|---|---|---|
| Quick Select | O(n) avg, O(n²) worst | O(1) | Tìm phần tử thứ K (chấp nhận modify mảng) |
| Top K Frequent | O(n log k) | O(n) | K phần tử xuất hiện nhiều nhất |
| K Closest Points | O(n log k) | O(k) | K phần tử gần nhất / nhỏ nhất theo metric |
| Merge K Sorted | O(N log k) | O(N + k) | Gộp K mảng sorted |

**Quy tắc chung:**
- Cần **chính xác phần tử thứ K** → Quick Select
- Cần **top K theo tiêu chí nào đó** → Min/Max-heap size K
- Cần **gộp K nguồn sorted** → Min-heap K-way merge

---

## Rust Ecosystem

### `BinaryHeap` -- tool chính

Rust standard library có `std::collections::BinaryHeap` -- đây là **max-heap**. Để dùng như min-heap, wrap giá trị trong `Reverse`:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Top K lớn nhất → min-heap size K
let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
for &x in &data {
    heap.push(Reverse(x));
    if heap.len() > k {
        heap.pop(); // loại nhỏ nhất
    }
}
// heap chứa K phần tử lớn nhất
```

### Custom ordering với tuple

`BinaryHeap` so sánh theo thứ tự của tuple (so phần tử đầu trước, rồi phần tử sau). Tận dụng điều này:

```rust
// Min-heap theo tần suất, giữ cả value
let mut heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();
//                                 ^^^^^ freq được so sánh trước
//                                        ^^^^ value chỉ dùng khi freq bằng nhau
```

### `select_nth_unstable` -- Quick Select có sẵn!

Từ Rust 1.49, slice có method `select_nth_unstable` -- đây chính là Quick Select:

```rust
let mut arr = vec![5, 3, 8, 1, 9, 2, 7];
arr.select_nth_unstable(2);
// arr[2] giờ là phần tử nhỏ thứ 3 (0-indexed)
// arr[..2] đều <= arr[2]
// arr[3..] đều >= arr[2]
```

Trong phỏng vấn thì viết tay Quick Select để show hiểu biết. Nhưng trong production code, dùng `select_nth_unstable` -- nó dùng Introselect (Quick Select + fallback Median of Medians), nên **worst case O(n)** thay vì O(n²).

### Crate `priority-queue`

Nếu bạn cần decrease-key (thay đổi priority của phần tử đã trong heap -- hữu ích cho Dijkstra), standard `BinaryHeap` không hỗ trợ. Crate [`priority-queue`](https://crates.io/crates/priority-queue) cho phép điều này.

---

## Practice -- Luyện tập

Làm theo thứ tự từ dễ đến khó:

| # | Bài | Difficulty | Pattern | Link |
|---|---|---|---|---|
| 1 | Kth Largest Element in an Array | Medium | Quick Select hoặc Min-heap size K | [LeetCode #215](https://leetcode.com/problems/kth-largest-element-in-an-array/) |
| 2 | Top K Frequent Elements | Medium | HashMap + Min-heap | [LeetCode #347](https://leetcode.com/problems/top-k-frequent-elements/) |
| 3 | K Closest Points to Origin | Medium | Max-heap size K | [LeetCode #973](https://leetcode.com/problems/k-closest-points-to-origin/) |
| 4 | Merge k Sorted Lists | Hard | Min-heap K-way merge | [LeetCode #23](https://leetcode.com/problems/merge-k-sorted-lists/) |
| 5 | Find Median from Data Stream | Hard | 2 heaps: max-heap + min-heap | [LeetCode #295](https://leetcode.com/problems/find-median-from-data-stream/) |

**Gợi ý cho #295 (Find Median):** Dùng 2 heaps -- max-heap giữ nửa nhỏ, min-heap giữ nửa lớn. Median = đỉnh max-heap (hoặc trung bình 2 đỉnh). Đây là ứng dụng đẹp nhất của heap pattern.

```
Data stream: 5, 2, 8, 1

Sau khi thêm 5:    max-heap: [5]      min-heap: []        median = 5
Sau khi thêm 2:    max-heap: [2]      min-heap: [5]       median = (2+5)/2 = 3.5
Sau khi thêm 8:    max-heap: [2]      min-heap: [5, 8]    median = 5
                    ← nửa nhỏ           nửa lớn →
Sau khi thêm 1:    max-heap: [1, 2]   min-heap: [5, 8]    median = (2+5)/2 = 3.5
```

---

## Code Rust

```rust,noplayground
{{#include ../../../../src/top_k.rs}}
```

---

## Chương tiếp theo

Tiếp theo mình sẽ học **String Matching** -- các thuật toán tìm kiếm chuỗi con như KMP và Rabin-Karp. Nếu bạn từng dùng Ctrl+F tìm từ trong văn bản, đó chính là string matching. Mình sẽ xem máy tính làm điều đó hiệu quả thế nào.

---

[← Linked List Tricks](./05-linked-list-tricks.md) | [String Matching →](./07-string-matching.md)
