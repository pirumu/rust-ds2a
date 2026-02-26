# Top-K Problems

## Đây là gì?

Bạn đã học Heap ở Phần 3 và Quick Sort ở Phần 6. Giờ mình dùng chúng để giải bài "tìm K phần tử lớn/nhỏ nhất" — một dạng bài cực kỳ phổ biến trong phỏng vấn.

Tưởng tượng bạn muốn tìm **3 quán phở ngon nhất Sài Gòn**. Bạn không cần xếp hạng TẤT CẢ hàng ngàn quán phở — chỉ cần giữ lại top 3. Mỗi khi thử quán mới ngon hơn quán kém nhất trong top 3, bạn thay thế nó. Đây chính là ý tưởng của **min-heap size K**.

Hoặc kiểu khác: bạn có danh sách 1000 học sinh, muốn tìm điểm cao thứ 500 (median). Sort toàn bộ thì lãng phí — **Quick Select** cho bạn đáp án trong O(n) trung bình mà không cần sort.

> **Tại sao quan trọng?** Top-K xuất hiện khắp nơi: autocomplete (top K gợi ý), recommendation system (K sản phẩm phù hợp nhất), monitoring (K server chậm nhất), database (LIMIT K)...

---

## Quick Select — tìm phần tử nhỏ thứ K

### Ý tưởng

Nhớ Quick Sort không? Nó chọn pivot, chia mảng thành 2 phần (nhỏ hơn pivot | lớn hơn pivot), rồi sort cả 2 phần.

Quick Select cũng làm vậy, nhưng **chỉ đi vào 1 phần** — phần chứa index K. Nên trung bình chỉ mất O(n) thay vì O(n log n).

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

## Top K Frequent Elements — K phần tử xuất hiện nhiều nhất

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

## K Closest Points to Origin — K điểm gần gốc tọa độ nhất

### Bài toán

Cho danh sách điểm 2D, tìm K điểm gần gốc tọa độ (0,0) nhất.

Khoảng cách = √(x² + y²), nhưng ta so sánh x² + y² (bỏ căn) để tránh float.

### Ý tưởng

Giống Top K Frequent, nhưng thay tần suất bằng khoảng cách. Dùng **max-heap size K** — phần tử xa nhất trong top K nằm ở đỉnh, dễ dàng so sánh với điểm mới.

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

---

## Merge K Sorted Arrays — gộp K mảng đã sắp xếp

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

> **Tại sao không merge từng cặp?** Merge 2 mảng mỗi lần: O(N * K). Dùng heap: O(N log K) — nhanh hơn nhiều khi K lớn.

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

## Code Rust

```rust,noplayground
{{#include ../../../../src/top_k.rs}}
```
