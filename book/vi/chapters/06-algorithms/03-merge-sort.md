# Merge Sort

> 💡 **Đừng lo lắng:** Nếu bạn đang nghĩ "Đệ quy đã khó rồi, giờ lại thêm chia mảng rồi gộp lại?" — bình tĩnh. Merge Sort nghe phức tạp nhưng ý tưởng cốt lõi cực kỳ đơn giản. Bạn đã biết đệ quy ở [chương Recursion](../06-algorithms/01-recursion.md), và bạn đã quen so sánh cặp phần tử ở [Bubble/Insertion Sort](../06-algorithms/02-basic-sorting.md). Merge Sort chỉ kết hợp 2 thứ đó lại. Thật sự, bước khó nhất là **bước gộp** (merge). Và bước đó... chỉ là so sánh 2 phần tử đầu tiên rồi chọn cái nhỏ hơn. Vậy thôi. Chương này sẽ đi từng bước rất chậm.

---

## Đây là gì?

### Bridge: Divide and Conquer lần đầu

Đây là lần đầu tiên chúng ta gặp pattern **Divide and Conquer** (chia để trị). Pattern này sẽ xuất hiện lại ở [Quick Sort](../06-algorithms/04-quick-sort.md) và cả [chương Divide & Conquer](../06-algorithms/11-divide-and-conquer.md) sau này. Nắm vững ở đây = dễ thở sau này.

Công thức Divide and Conquer luôn có 3 bước:

```
Chia → Giải → Gộp
(Divide → Conquer → Combine)
```

Mỗi thuật toán Divide and Conquer chỉ khác nhau ở **cách chia** và **cách gộp**. Merge Sort chia rất đơn giản (cắt đôi), nhưng gộp thì cần suy nghĩ.

### Ẩn dụ: Giáo viên chia bài kiểm tra

Tưởng tượng bạn là giáo viên và cần sắp xếp 100 bài kiểm tra theo tên. Một mình làm thì lâu. Bạn **chia đống bài thành 2 nửa**, đưa mỗi nửa cho một học sinh sắp xếp. Khi cả 2 nửa đã sắp xếp xong, bạn **gộp lại** thành một đống duy nhất bằng cách so sánh bài trên đầu của mỗi đống.

Mỗi học sinh cũng có thể chia tiếp nửa của mình cho 2 học sinh khác. Cứ chia cho đến khi mỗi người chỉ cầm 1 bài — 1 bài thì tự động đã "sắp xếp" rồi. Rồi gộp ngược lại.

Đây chính là Merge Sort (sắp xếp trộn):

1. **Chia** mảng thành 2 nửa
2. **Sắp xếp** từng nửa (gọi đệ quy)
3. **Gộp** 2 nửa đã sắp xếp lại thành một mảng hoàn chỉnh

Điểm mạnh: **luôn** chạy trong O(n log n), không phụ thuộc vào dữ liệu đầu vào. Đổi lại, cần thêm O(n) bộ nhớ phụ.

---

## Hoạt động như thế nào?

### Giai đoạn chia (Divide)

Chia mảng thành nửa, rồi chia tiếp, cho đến khi mỗi phần chỉ còn 1 phần tử (1 phần tử tự động đã sắp xếp).

```
                  [38, 27, 43, 3, 9, 82, 10]
                 /                           \
          [38, 27, 43, 3]              [9, 82, 10]
          /            \                /         \
      [38, 27]      [43, 3]        [9, 82]      [10]
      /     \       /     \        /     \
    [38]   [27]   [43]   [3]    [9]    [82]
     |       |      |      |     |       |
     v       v      v      v     v       v
   (1 phần tử = đã sắp xếp, không cần làm gì)
```

Bước này đơn giản. Chỉ cần tìm `mid = len / 2` rồi cắt đôi. Không cần suy nghĩ gì.

### Giai đoạn gộp (Merge) — bức tranh tổng

Gộp từng cặp mảng nhỏ đã sắp xếp thành mảng lớn hơn. Mỗi lần gộp: so sánh phần tử đầu của 2 mảng, lấy phần tử nhỏ hơn.

```
Gộp [38] + [27]  --> [27, 38]
Gộp [43] + [3]   --> [3, 43]
Gộp [9] + [82]   --> [9, 82]

Gộp [27, 38] + [3, 43]  --> [3, 27, 38, 43]
Gộp [9, 82] + [10]      --> [9, 10, 82]

Gộp [3, 27, 38, 43] + [9, 10, 82]  --> [3, 9, 10, 27, 38, 43, 82]
```

### Merge step trace chi tiết: pointer by pointer

Đây là phần quan trọng nhất. Hãy xem kỹ cách gộp 2 sorted array thành 1 sorted array.

Ví dụ nhỏ trước — gộp `[3, 27]` và `[9, 10]`:

```
Trái:  [3, 27]     Phải: [9, 10]     Kết quả: []
        ^L                 ^R

Bước 1: L=3 vs R=9 → 3 < 9 → lấy 3, di chuyển L
Trái:  [3, 27]     Phải: [9, 10]     Kết quả: [3]
           ^L              ^R

Bước 2: L=27 vs R=9 → 27 > 9 → lấy 9, di chuyển R
Trái:  [3, 27]     Phải: [9, 10]     Kết quả: [3, 9]
           ^L                 ^R

Bước 3: L=27 vs R=10 → 27 > 10 → lấy 10, di chuyển R
Trái:  [3, 27]     Phải: [9, 10]     Kết quả: [3, 9, 10]
           ^L                    ^R (hết!)

Bước 4: R hết rồi → lấy tất cả còn lại bên L: 27
                                      Kết quả: [3, 9, 10, 27] ✓
```

Bây giờ bước gộp cuối cùng, lớn hơn:

```
Trái:  [3, 27, 38, 43]     Phải: [9, 10, 82]     Kết quả: []
        ^L                         ^R

Bước 1: L=3  vs R=9  → lấy 3      Kết quả: [3]
            ^L                      ^R

Bước 2: L=27 vs R=9  → lấy 9      Kết quả: [3, 9]
            ^L                          ^R

Bước 3: L=27 vs R=10 → lấy 10     Kết quả: [3, 9, 10]
            ^L                              ^R

Bước 4: L=27 vs R=82 → lấy 27     Kết quả: [3, 9, 10, 27]
                ^L                          ^R

Bước 5: L=38 vs R=82 → lấy 38     Kết quả: [3, 9, 10, 27, 38]
                    ^L                      ^R

Bước 6: L=43 vs R=82 → lấy 43     Kết quả: [3, 9, 10, 27, 38, 43]
                        ^L (hết!)           ^R

Bước 7: L hết → lấy tất cả R: 82  Kết quả: [3, 9, 10, 27, 38, 43, 82] ✓
```

**Tại sao cách này hiệu quả?** Vì cả 2 nửa **đã được sắp xếp**, nên mỗi lần chỉ cần nhìn phần tử đầu tiên của mỗi bên. Không cần tìm kiếm gì thêm!

**Nhận xét quan trọng:** Merge step giống y hệt cách bạn xếp 2 cọc bài đã sắp xếp thành 1 cọc — lật lá trên cùng của mỗi cọc, lấy lá nhỏ hơn bỏ vào đống kết quả. Nếu cọc nào hết trước, đổ nguyên cọc còn lại vào.

---

## Code Rust

```rust
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return; // Mảng 0 hoặc 1 phần tử: đã sắp xếp
    }
    let mid = n / 2;

    // Chia thành 2 nửa (tạo bản sao)
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    // Sắp xếp đệ quy từng nửa
    merge_sort(&mut left);
    merge_sort(&mut right);

    // Gộp 2 nửa đã sắp xếp lại
    merge(&left, &right, arr);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], out: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    // So sánh phần tử đầu của 2 mảng, lấy phần tử nhỏ hơn
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i].clone();
            i += 1;
        } else {
            out[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy phần còn lại của bên trái (nếu có)
    while i < left.len() {
        out[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    // Copy phần còn lại của bên phải (nếu có)
    while j < right.len() {
        out[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
```

**Ghi chú về Rust:**

- `T: Clone` cần thiết vì ta tạo bản sao của các phần tử vào vector tạm. Đây là chi phí của Merge Sort — cần bộ nhớ phụ.
- Dùng `<=` (không phải `<`) khi so sánh để giữ thứ tự tương đối của các phần tử bằng nhau. Nhờ vậy, Merge Sort là **stable** (ổn định).
- `to_vec()` tạo một vector mới từ slice — đây là nơi bộ nhớ O(n) được cấp phát.

---

## Bottom-Up Merge Sort (không đệ quy)

"Thầy ơi, em sợ đệ quy, có cách nào khác không?"

Có! Bottom-Up Merge Sort làm ngược lại: thay vì chia từ trên xuống rồi gộp lên, ta **bắt đầu từ dưới** — coi mỗi phần tử là một mảng đã sắp xếp, rồi gộp dần lên.

### Ẩn dụ: Giải đấu tennis

Tưởng tượng 8 người chơi tennis. Vòng 1: ghép cặp 1v1 (tạo nhóm 2 người xếp hạng). Vòng 2: ghép 2 nhóm lại (tạo nhóm 4). Vòng 3: ghép 2 nhóm 4 (tạo nhóm 8). Xong!

```
Bắt đầu: [38] [27] [43] [3] [9] [82] [10] [5]
          \  /      \  /     \ /      \  /
width=1:  [27,38]  [3,43]  [9,82]   [5,10]
            \    /            \     /
width=2:  [3,27,38,43]     [5,9,10,82]
               \           /
width=4:  [3,5,9,10,27,38,43,82]
```

### Code

```rust
pub fn merge_sort_bottom_up<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut width = 1; // Kích thước mỗi "nhóm" cần gộp
    while width < n {
        let mut start = 0;
        while start < n {
            let mid = (start + width).min(n);
            let end = (start + 2 * width).min(n);

            // Gộp arr[start..mid] và arr[mid..end]
            if mid < end {
                let left = arr[start..mid].to_vec();
                let right = arr[mid..end].to_vec();
                merge(&left, &right, &mut arr[start..end]);
            }

            start += 2 * width;
        }
        width *= 2; // Nhân đôi kích thước nhóm
    }
}
```

**Khi nào dùng Bottom-Up?**

- Khi bạn lo stack overflow với mảng cực lớn (top-down đệ quy sâu O(log n) lớp).
- Khi bạn muốn tránh overhead của function call đệ quy.
- Trong thực tế: `std::stable_sort` của C++ dùng bottom-up merge sort bên trong.

---

## Stable Sort: Tại sao quan trọng?

Merge Sort là **stable sort** — nghĩa là 2 phần tử có giá trị bằng nhau sẽ giữ nguyên thứ tự ban đầu.

**Ví dụ thực tế:** Bạn có danh sách sinh viên đã sắp xếp theo tên. Giờ muốn sắp xếp lại theo điểm. Với stable sort, 2 sinh viên cùng điểm sẽ vẫn theo thứ tự tên (từ lần sắp xếp trước).

```
Ban đầu (đã xếp theo tên):
  An    - 8 điểm
  Bình  - 9 điểm
  Châu  - 8 điểm
  Dũng  - 9 điểm

Sau stable sort theo điểm:
  An    - 8 điểm    ← An vẫn trước Châu (giữ thứ tự tên)
  Châu  - 8 điểm
  Bình  - 9 điểm    ← Bình vẫn trước Dũng
  Dũng  - 9 điểm

Nếu unstable sort (như Quick Sort), có thể ra:
  Châu  - 8 điểm    ← Châu nhảy trước An!
  An    - 8 điểm
  Dũng  - 9 điểm
  Bình  - 9 điểm
```

**Trade-off Merge Sort vs Quick Sort:**

| | Merge Sort | Quick Sort |
|---|---|---|
| Worst case | O(n log n) luôn | O(n^2) nếu chọn pivot tệ |
| Bộ nhớ thêm | O(n) — cần mảng tạm | O(log n) — chỉ cần stack |
| Stable? | Có | Không (thường) |
| Cache performance | Kém hơn (nhảy giữa mảng tạm) | Tốt hơn (in-place, tuần tự) |
| Thực tế | Dùng cho linked list, external sort | Dùng cho array trong RAM |

---

## Inversion Count: Ứng dụng phỏng vấn kinh điển

### Bài toán

**Inversion** = cặp (i, j) mà i < j nhưng arr[i] > arr[j]. Nói cách khác: cặp phần tử bị "ngược thứ tự".

```
[2, 4, 1, 3, 5]

Các inversions:
  (2,1)  — 2 đứng trước 1 nhưng 2 > 1
  (4,1)  — 4 đứng trước 1 nhưng 4 > 1
  (4,3)  — 4 đứng trước 3 nhưng 4 > 3

Tổng: 3 inversions
```

Brute force: duyệt mọi cặp → O(n^2). Nhưng dùng **modified merge sort** → O(n log n)!

### Ý tưởng

Trong bước merge, khi ta lấy phần tử từ mảng **phải** (vì nó nhỏ hơn phần tử đang xét ở mảng trái), tất cả phần tử còn lại ở mảng trái đều tạo inversion với phần tử đó.

```
Trái: [2, 4]     Phải: [1, 3]
       ^L                ^R

L=2 vs R=1 → R nhỏ hơn → lấy R=1
  Inversion count += (số phần tử còn lại bên trái) = 2
  (vì cả 2 và 4 đều > 1, tạo ra 2 inversions)

Trái: [2, 4]     Phải: [1, 3]
       ^L                   ^R

L=2 vs R=3 → L nhỏ hơn → lấy L=2 (không có inversion)

Trái: [2, 4]     Phải: [1, 3]
          ^L                ^R

L=4 vs R=3 → R nhỏ hơn → lấy R=3
  Inversion count += 1 (chỉ còn 4 bên trái)

Tổng inversions ở bước này: 2 + 1 = 3 ✓
```

### Code

```rust
pub fn count_inversions<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    // Đếm inversions ở nửa trái + nửa phải + cross inversions
    let mut count = 0;
    count += count_inversions(&mut left);
    count += count_inversions(&mut right);
    count += merge_count(&left, &right, arr);
    count
}

fn merge_count<T: Ord + Clone>(left: &[T], right: &[T], out: &mut [T]) -> usize {
    let (mut i, mut j, mut k) = (0, 0, 0);
    let mut inversions = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i].clone();
            i += 1;
        } else {
            out[k] = right[j].clone();
            j += 1;
            // Tất cả phần tử còn lại bên trái đều > right[j]
            inversions += left.len() - i;
        }
        k += 1;
    }

    while i < left.len() {
        out[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        out[k] = right[j].clone();
        j += 1;
        k += 1;
    }

    inversions
}
```

Kỹ thuật này xuất hiện trong nhiều bài phỏng vấn, đặc biệt LeetCode #315 (Count of Smaller Numbers After Self).

---

## External Sorting: Khi data không vừa RAM

Tưởng tượng bạn cần sắp xếp 100GB data nhưng chỉ có 4GB RAM. Không thể load hết vào bộ nhớ. Làm sao?

Merge Sort sinh ra cho việc này:

```
100GB file trên đĩa
        |
   ┌────┴────┐
   │ Bước 1: │  Đọc từng chunk 4GB vào RAM
   │  Chia   │  Sắp xếp trong RAM (dùng bất kỳ sort nào)
   │         │  Ghi chunk đã sắp xếp ra đĩa
   └────┬────┘
        │
   25 chunks đã sắp xếp (mỗi chunk 4GB)
        │
   ┌────┴────┐
   │ Bước 2: │  Mở 25 files cùng lúc
   │  Gộp    │  Dùng min-heap để tìm phần tử nhỏ nhất
   │         │  (xem lại Priority Queue ở Phần 3)
   │         │  Ghi kết quả tuần tự ra file output
   └────┬────┘
        │
   1 file 100GB đã sắp xếp ✓
```

**Tại sao Merge Sort phù hợp cho external sort?**

- Đọc/ghi **tuần tự** (sequential) — đĩa cứng thích đọc tuần tự, ghét random access.
- Không cần random access như Quick Sort (Quick Sort nhảy lung tung trong mảng).
- Bước gộp chỉ cần giữ 1 phần tử từ mỗi chunk trong RAM.

Đây là lý do database systems (PostgreSQL, MySQL) dùng external merge sort khi cần `ORDER BY` trên bảng lớn.

---

## Độ phức tạp

| Trường hợp | Thời gian | Bộ nhớ |
|-----------|----------|--------|
| Tốt nhất | O(n log n) | O(n) |
| Trung bình | O(n log n) | O(n) |
| Xấu nhất | O(n log n) | O(n) |

**Giải thích thực tế:**

- **O(n log n)**: chia mảng làm đôi log(n) lần, mỗi lần gộp tốn O(n). Ví dụ: 1.000.000 phần tử chỉ cần khoảng 20 lần chia x 1.000.000 phép gộp = 20.000.000 phép tính. Nhanh hơn rất nhiều so với O(n^2) = 1.000.000.000.000!
- **O(n) bộ nhớ**: cần thêm một mảng tạm cùng kích thước. Nếu mảng 1GB, bạn cần thêm 1GB RAM. Đây là nhược điểm chính.
- **Ổn định (stable)**: Có — phần tử bằng nhau giữ nguyên thứ tự ban đầu.
- **Độ sâu đệ quy**: O(log n) — với 1.000.000 phần tử, chỉ sâu khoảng 20 lớp đệ quy. Nếu vẫn lo, dùng Bottom-Up.

---

## Pitfalls: Lỗi thường gặp

### 1. Quên allocate temp array

❌ **Sai:** Cố merge in-place mà không dùng mảng tạm

```rust
// Cố ghi đè trực tiếp lên mảng gốc trong khi đang đọc nó
fn bad_merge(arr: &mut [i32], mid: usize) {
    let mut i = 0;
    let mut j = mid;
    // BUG: khi ghi arr[0] = arr[j], giá trị cũ arr[0] bị mất!
}
```

✅ **Đúng:** Tạo bản sao trước khi merge

```rust
let left = arr[..mid].to_vec();   // Copy ra ngoài
let right = arr[mid..].to_vec();  // Copy ra ngoài
merge(&left, &right, arr);        // Giờ ghi vào arr an toàn
```

💡 **Tại sao:** Khi merge, bạn đang đọc và ghi cùng một mảng. Nếu không copy ra ngoài, giá trị sẽ bị ghi đè trước khi bạn kịp đọc.

### 2. Off-by-one trong merge

❌ **Sai:** Dùng `<` thay vì `<=` khi so sánh

```rust
if left[i] < right[j] {  // Dùng < thay vì <=
```

✅ **Đúng:** Dùng `<=` để đảm bảo stability

```rust
if left[i] <= right[j] {  // <= giữ thứ tự ban đầu khi bằng nhau
```

💡 **Tại sao:** Nếu dùng `<`, khi 2 phần tử bằng nhau, phần tử bên phải sẽ được lấy trước → mất tính stable. Với `<=`, phần tử bên trái (xuất hiện trước trong mảng gốc) được ưu tiên.

### 3. Quên copy phần còn lại

❌ **Sai:** Chỉ có vòng while chính, không copy tail

```rust
while i < left.len() && j < right.len() {
    // merge logic...
}
// Quên: còn phần tử chưa copy!
```

✅ **Đúng:** Luôn copy tail của cả 2 bên

```rust
while i < left.len() { out[k] = left[i].clone(); i += 1; k += 1; }
while j < right.len() { out[k] = right[j].clone(); j += 1; k += 1; }
```

💡 **Tại sao:** Khi 1 bên hết, bên còn lại vẫn có phần tử chưa được copy. Quên bước này = mất dữ liệu.

### 4. O(n) space overhead

❌ **Sai:** "Merge Sort tốt hơn Quick Sort mọi mặt vì luôn O(n log n)"

✅ **Đúng:** Merge Sort cần O(n) extra space, Quick Sort chỉ cần O(log n) stack space

💡 **Tại sao:** Trong thực tế, nếu RAM hạn chế và data vừa trong bộ nhớ, Quick Sort thường nhanh hơn vì cache-friendly hơn và không cần allocate. Trade-off luôn tồn tại.

---

## Khi nào dùng Merge Sort?

| Tình huống | Dùng Merge Sort? | Lý do |
|-----------|:-:|-------|
| Cần đảm bảo O(n log n) worst case | ✅ | Quick Sort worst case = O(n^2) |
| Cần stable sort | ✅ | Quick Sort thường unstable |
| Sắp xếp linked list | ✅ | Không cần random access, merge dễ dàng |
| External sort (data > RAM) | ✅ | Đọc tuần tự, gộp chunk |
| Merge K sorted arrays/lists | ✅ | Bài toán = merge step mở rộng |
| Array nhỏ trong RAM | ❌ | Quick Sort/Insertion Sort nhanh hơn |
| Bộ nhớ rất hạn chế | ❌ | O(n) extra space là vấn đề |
| Cần sort in-place | ❌ | In-place merge sort tồn tại nhưng phức tạp |

---

## Rust Ecosystem

Rust cực kỳ coi trọng Merge Sort:

- **`[T]::sort()`** trong standard library dùng **timsort** — một biến thể tối ưu của merge sort. Nó stable, O(n log n) worst case, và tận dụng dữ liệu đã sắp xếp sẵn (natural runs).
- **`[T]::sort_unstable()`** dùng pattern-defeating quicksort (pdqsort) — nhanh hơn nhưng unstable.

```rust
// Rust standard library — đã dùng merge sort variant bên trong!
let mut v = vec![5, 3, 1, 4, 2];
v.sort();           // Stable sort (timsort, dựa trên merge sort)
v.sort_unstable();  // Unstable sort (pdqsort, dựa trên quick sort)
```

**KaCrab tip:** Khi nào dùng `.sort()` vs `.sort_unstable()`?
- Cần giữ thứ tự tương đối → `.sort()`
- Chỉ cần sắp xếp, không quan tâm thứ tự tương đối → `.sort_unstable()` (nhanh hơn ~10-20%)

Crate **`rayon`** cung cấp parallel merge sort:

```rust
use rayon::slice::ParallelSliceMut;

let mut v = vec![5, 3, 1, 4, 2];
v.par_sort();           // Parallel stable sort — tự chia việc cho nhiều core
v.par_sort_unstable();  // Parallel unstable sort
```

Merge Sort song song (parallel) rất tự nhiên — bước chia thành 2 nửa = 2 task độc lập, chạy trên 2 core. Đây là lý do merge sort được ưa chuộng trong distributed computing (MapReduce, Spark).

---

## Ví dụ

```rust
use rust_ds2a::sorting::merge_sort;

let mut v = vec![38, 27, 43, 3, 9, 82, 10];
merge_sort(&mut v);
assert_eq!(v, vec![3, 9, 10, 27, 38, 43, 82]);

// Hoạt động với bất kỳ kiểu dữ liệu nào có Ord + Clone
let mut words = vec!["delta", "alpha", "charlie", "bravo"];
merge_sort(&mut words);
assert_eq!(words, vec!["alpha", "bravo", "charlie", "delta"]);
```

---

## Practice

| Bài | Tên | Gợi ý |
|-----|-----|-------|
| [#912](https://leetcode.com/problems/sort-an-array/) | Sort an Array | Implement merge sort từ đầu. TLE nếu dùng O(n^2) sort. |
| [#315](https://leetcode.com/problems/count-of-smaller-numbers-after-self/) | Count of Smaller Numbers After Self | Modified merge sort để đếm inversions. Hard nhưng classic. |
| [#23](https://leetcode.com/problems/merge-k-sorted-lists/) | Merge k Sorted Lists | Mở rộng merge step: dùng min-heap hoặc divide and conquer. Xem lại [Priority Queue](../03-trees-and-heaps/06-priority-queue.md). |
| [#148](https://leetcode.com/problems/sort-list/) | Sort List | Merge sort trên linked list — không cần extra O(n) space cho merge! |
| [#88](https://leetcode.com/problems/merge-sorted-array/) | Merge Sorted Array | Bước merge cô lập. Merge ngược từ cuối để tránh allocate. |

---

## Nhìn về phía trước: Quick Sort

Merge Sort chia mảng rất đơn giản (cắt đôi ở giữa) nhưng gộp thì tốn công (cần mảng tạm). [Quick Sort](./04-quick-sort.md) làm ngược lại — **chia thì tốn công** (partition, phân hoạch phần tử quanh pivot) nhưng **không cần gộp** gì cả, vì sau khi partition xong thì mọi thứ đã đúng vị trí.

```
Merge Sort:  chia đơn giản  →  gộp phức tạp  →  stable, O(n) space
Quick Sort:  chia phức tạp  →  gộp = 0        →  unstable, O(1) space

Cả 2 đều là Divide and Conquer. Chỉ khác "công sức" nằm ở bước nào.
```

Nếu bạn hiểu Merge Sort, Quick Sort sẽ rất tự nhiên. Hẹn gặp ở chương tiếp!

---

[← Basic Sorting](./02-basic-sorting.md) | [Quick Sort →](./04-quick-sort.md)
