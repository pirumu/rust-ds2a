# Heap Sort

> 💡 **Đừng lo lắng:** Nếu bạn đã hiểu Binary Heap ở chương 3, chương này gần như "miễn phí". Heap Sort chỉ là: xây heap + lấy max ra lặp lại. Không có ý tưởng mới nào cả. Nếu bạn đã sống sót qua Quick Sort (pivot, partition, worst case O(n²)...) thì Heap Sort đơn giản hơn nhiều -- luôn O(n log n), không có trường hợp xấu bất ngờ. Và đây là chương sort cuối cùng dùng comparison (so sánh). Sau chương này, bạn sẽ có bức tranh đầy đủ về tất cả sorting algorithms phổ biến.

---

## Bridge: Từ Binary Heap đến Heap Sort

Ở [chương Binary Heap](../../chapters/03-trees-and-heaps/05-binary-heap.md), bạn đã học:

- Max-heap: cha luôn >= con. Phần tử lớn nhất luôn ở root.
- `build_heap` (heapify): biến mảng bất kỳ thành heap trong **O(n)**.
- `sift_down`: đẩy node xuống đúng vị trí trong **O(log n)**.

Heap Sort **chỉ kết hợp 2 thao tác này**:

```
build_heap O(n)  +  extract_max n lần × O(log n)  =  O(n log n)
```

Nếu bạn hiểu heap rồi, đọc tiếp sẽ rất nhẹ nhàng.

---

## Đây là gì?

Tưởng tượng bạn có một hộp đồ chơi và muốn sắp xếp chúng theo kích thước. Bạn dùng một chiếc tháp (heap) đặc biệt: **luôn lấy đồ chơi lớn nhất từ đỉnh tháp ra**, đặt nó vào cuối hàng. Rồi sắp lại tháp, lấy tiếp đồ chơi lớn nhất tiếp theo.

Heap Sort (sắp xếp vun đống) hoạt động như vậy:

1. Biến mảng thành **Max-Heap** (cây nhị phân nơi phần tử lớn nhất luôn ở gốc)
2. Lấy phần tử lớn nhất (gốc) ra, đặt vào cuối mảng
3. Sắp lại heap, lặp lại cho đến khi hết

---

## Hoạt động như thế nào?

### Bước 1: Xây dựng Max-Heap (heapify)

Bắt đầu từ node lá cuối cùng không phải lá (last non-leaf), "đẩy xuống" (sift down) mỗi node để thỏa mãn tính chất heap.

```
Mảng ban đầu: [4, 10, 3, 5, 1]

Vẽ thành cây:
              4
            /   \
          10     3
         / \
        5   1

Heapify từ vị trí 1 (node cuối không phải lá):
  Node 10: con là 5 và 1 -> 10 >= cả hai -> OK, không làm gì
  Node 4:  con là 10 và 3 -> 10 > 4 -> Đổi chỗ 4 và 10!

              10
            /    \
           5      3
          / \
         4   1

Kết quả: [10, 5, 3, 4, 1]   (đã là Max-Heap)
```

> **Tại sao bắt đầu từ dưới lên?** Vì node lá đã thỏa mãn tính chất heap (không có con). Chỉ cần xử lý các node có con, từ dưới lên trên.

### Tại sao Build Heap chỉ O(n)? (Chứng minh trực quan)

Nhiều bạn nghĩ: "n/2 node, mỗi node sift-down tối đa O(log n), vậy phải là O(n log n) chứ?" Nhưng không phải. Hãy nhìn kỹ:

```
Cây 15 nodes (4 tầng):

                    ○              Tầng 0: 1 node  → sift-down tối đa 3 bước
                 /     \
               ○         ○         Tầng 1: 2 nodes → sift-down tối đa 2 bước
             /   \     /   \
            ○     ○   ○     ○      Tầng 2: 4 nodes → sift-down tối đa 1 bước
           /\ /\ /\ /\
          ○ ○○ ○○ ○○ ○             Tầng 3: 8 nodes → sift-down 0 bước (lá!)
```

Đếm tổng công việc:

```
Tầng 3 (leaf):  8 nodes × 0 bước = 0
Tầng 2:         4 nodes × 1 bước = 4
Tầng 1:         2 nodes × 2 bước = 4
Tầng 0 (root):  1 node  × 3 bước = 3
                                    ───
                            Tổng  = 11 bước (với 15 nodes!)
```

**Quy luật**: hơn nửa node là lá (0 bước). 1/4 node chỉ sift-down 1 bước. Càng lên cao, node càng ít nhưng bước càng nhiều. Tuy nhiên số node **giảm theo cấp số nhân** trong khi bước chỉ **tăng tuyến tính**.

Công thức toán: `n/4 × 1 + n/8 × 2 + n/16 × 3 + ... ≈ n` (chuỗi hội tụ).

**Trực giác đời thường**: giống như xây tháp người ở rạp xiếc. Đa số người (lá) chỉ đứng yên. Vài người ở giữa điều chỉnh chút. Chỉ 1 người trên cùng phải cân bằng nhiều nhất. Tổng effort vẫn tỷ lệ với số người, không phải số người nhân chiều cao.

### Bước 2: Lấy max ra từng cái

Đổi chỗ gốc (max) với phần tử cuối, thu nhỏ heap đi 1, rồi sift down gốc mới.

```
Heap: [10, 5, 3, 4, 1]

--- Lấy 10 ra ---
Đổi chỗ gốc và cuối: [1, 5, 3, 4, | 10]
Sift down 1:
  1 < 5? Đổi chỗ  -> [5, 1, 3, 4, | 10]
  1 < 4? Đổi chỗ  -> [5, 4, 3, 1, | 10]
Kết quả: [5, 4, 3, 1, | 10]

--- Lấy 5 ra ---
Đổi chỗ gốc và cuối: [1, 4, 3, | 5, 10]
Sift down 1:
  1 < 4? Đổi chỗ  -> [4, 1, 3, | 5, 10]
Kết quả: [4, 1, 3, | 5, 10]

--- Lấy 4 ra ---
Đổi chỗ gốc và cuối: [3, 1, | 4, 5, 10]
Sift down 3:
  3 > 1? OK, không làm gì
Kết quả: [3, 1, | 4, 5, 10]

--- Lấy 3 ra ---
Đổi chỗ gốc và cuối: [1, | 3, 4, 5, 10]

Kết quả cuối: [1, 3, 4, 5, 10]   -- Đã sắp xếp!
```

### Minh họa toàn bộ quá trình

```
Ban đầu:     [4, 10, 3, 5, 1]

Build heap:  [10, 5, 3, 4, 1]
                                    Phần đã sort
Extract 10:  [5, 4, 3, 1, | 10]         ^
Extract 5:   [4, 1, 3, | 5, 10]         ^^
Extract 4:   [3, 1, | 4, 5, 10]         ^^^
Extract 3:   [1, | 3, 4, 5, 10]         ^^^^

Kết quả:     [1, 3, 4, 5, 10]
```

### In-place: Ưu điểm lớn nhất

Để ý: Heap Sort **không cần mảng phụ**. Phần heap thu nhỏ dần, phần sorted lớn dần -- tất cả trong cùng 1 mảng:

```
[  heap phần chưa sort   |  phần đã sort  ]
 ←── shrink              grow ──→

Merge Sort: cần mảng phụ O(n) để merge
Quick Sort: cần stack O(log n) cho recursion
Heap Sort:  O(1) extra memory -- chỉ vài biến tạm!
```

Khi bộ nhớ hạn chế (embedded systems, kernel code), đây là lý do chính để chọn Heap Sort.

---

## Code Rust

```rust
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Bước 1: Xây dựng max-heap
    // Bắt đầu từ node cuối không phải lá (n/2 - 1), đi ngược lên gốc
    for i in (0..n / 2).rev() {
        sift_down(arr, i, n);
    }

    // Bước 2: Lấy max ra từng cái, đặt vào cuối
    for end in (1..n).rev() {
        arr.swap(0, end);      // Đổi chỗ max (gốc) với phần tử cuối
        sift_down(arr, 0, end); // Sắp lại heap (không tính phần đã sort)
    }
}

/// Đẩy node xuống đúng vị trí để thỏa mãn tính chất max-heap.
fn sift_down<T: Ord>(arr: &mut [T], mut idx: usize, len: usize) {
    loop {
        let left = 2 * idx + 1;   // Con trái
        let right = 2 * idx + 2;  // Con phải
        let mut largest = idx;

        // Tìm node lớn nhất trong 3 node: cha, con trái, con phải
        if left < len && arr[left] > arr[largest] {
            largest = left;
        }
        if right < len && arr[right] > arr[largest] {
            largest = right;
        }

        // Nếu cha đã là lớn nhất -> dừng
        if largest == idx {
            break;
        }

        // Đổi chỗ cha với con lớn nhất, rồi tiếp tục đẩy xuống
        arr.swap(idx, largest);
        idx = largest;
    }
}
```

**Ghi chú về Rust:**

- Hàm `sift_down` được dùng lại ở cả 2 bước: xây heap và sắp xếp. Đây là thiết kế tốt -- một hàm, hai mục đích.
- Công thức vị trí trong heap (lưu dạng mảng): con trái = `2*i + 1`, con phải = `2*i + 2`, cha = `(i - 1) / 2`.
- So với code `BinaryHeap` ở [chương 3](../../chapters/03-trees-and-heaps/05-binary-heap.md), `sift_down` ở đây nhận thêm `len` -- vì phần cuối mảng đã sorted, ta không muốn heap "chạm" vào phần đó.

---

## Độ phức tạp

| Trường hợp | Thời gian | Bộ nhớ |
|-----------|----------|--------|
| Tốt nhất | O(n log n) | O(1) |
| Trung bình | O(n log n) | O(1) |
| Xấu nhất | O(n log n) | O(1) |

**Giải thích thực tế:**

- **Luôn O(n log n)**: không có trường hợp xấu như Quick Sort. Dù mảng đã sắp xếp hay sắp ngược, thời gian vẫn như nhau.
- **O(1) bộ nhớ**: sắp xếp tại chỗ, không cần mảng phụ! Đây là ưu điểm lớn so với Merge Sort (cần O(n)).
- **Không ổn định (unstable)**: các phần tử bằng nhau có thể bị đảo thứ tự.
- **Xây dựng heap là O(n)**: như đã chứng minh ở trên, nhờ tính chất sift-down từ dưới lên.

---

## So sánh Sorting Algorithms (Bảng tổng hợp)

Đây là chương sort cuối cùng dùng comparison-based (so sánh phần tử). Hãy nhìn toàn bộ bức tranh:

| Sort | Best | Average | Worst | Space | Stable | In-place |
|------|------|---------|-------|-------|--------|----------|
| [Bubble](02-basic-sorting.md) | O(n) | O(n^2) | O(n^2) | O(1) | Yes | Yes |
| [Selection](02-basic-sorting.md) | O(n^2) | O(n^2) | O(n^2) | O(1) | No | Yes |
| [Insertion](02-basic-sorting.md) | O(n) | O(n^2) | O(n^2) | O(1) | Yes | Yes |
| [Merge Sort](03-merge-sort.md) | O(n log n) | O(n log n) | O(n log n) | O(n) | Yes | No |
| [Quick Sort](04-quick-sort.md) | O(n log n) | O(n log n) | O(n^2) | O(log n) | No | Yes |
| **Heap Sort** | O(n log n) | O(n log n) | O(n log n) | **O(1)** | No | Yes |

**Đọc bảng này thế nào?**

- **Stable** (ổn định): phần tử bằng nhau có giữ nguyên thứ tự ban đầu không? Quan trọng khi sort theo nhiều tiêu chí (sort theo tên rồi sort theo tuổi -- muốn cùng tuổi thì vẫn giữ thứ tự tên).
- **In-place** (tại chỗ): không cần bộ nhớ phụ đáng kể? Merge Sort cần O(n) extra -- gấp đôi memory.

**Nhận xét quan trọng:**

- **Không có algorithm hoàn hảo.** Mỗi cái đánh đổi một thứ gì đó.
- Heap Sort là **duy nhất** vừa O(n log n) worst case, vừa O(1) space. Nhưng đánh đổi: cache performance tệ (sift-down nhảy xa trong mảng).
- Quick Sort nhanh nhất **thực tế** (cache-friendly, constant factor nhỏ), nhưng worst case O(n^2).
- Merge Sort ổn định duy nhất trong nhóm O(n log n), nhưng tốn O(n) memory.

---

## Những cái bẫy hay gặp

### 1. Nghĩ Heap Sort stable

- ❌ Sai: "Heap Sort giữ nguyên thứ tự phần tử bằng nhau"
- ✅ Đúng: Heap Sort **unstable**. Khi swap root với phần tử cuối, thứ tự tương đối bị phá
- 💡 Tại sao: Ví dụ `[(3,'a'), (3,'b'), (1,'c')]`. Sau build heap, root là `(3,'a')`. Swap với cuối `(1,'c')` → `(3,'b')` có thể lên root trước `(3,'a')` → thứ tự 'a','b' bị đảo thành 'b','a'.

### 2. Quên rằng cache performance tệ

- ❌ Sai: "Heap Sort O(n log n) nên nhanh bằng Quick Sort"
- ✅ Đúng: Trên thực tế, Heap Sort **chậm hơn Quick Sort 2-3x** với cùng input
- 💡 Tại sao: `sift_down` nhảy từ index `i` sang `2i+1`, `2i+2` -- nhảy xa trong mảng. Quick Sort scan tuần tự (partition duyệt từ trái sang phải) → cache-friendly hơn nhiều. CPU cache "thích" truy cập liên tiếp, ghét nhảy lung tung.

```
Quick Sort:  [→ → → → → → → →]     Scan tuần tự, cache hit!
Heap Sort:   [↗ ↙ ↗ ↙ ↗ ↙ ↗]     Nhảy theo cây, cache miss!
```

### 3. Nhầm build heap là O(n log n)

- ❌ Sai: "n/2 node × O(log n) mỗi node = O(n log n)"
- ✅ Đúng: Build heap = **O(n)** (xem chứng minh ở trên)
- 💡 Tại sao: Đa số node ở gần leaf, sift-down rất ít bước. Chỉ root mới sift-down log n bước, nhưng root chỉ có 1.

### 4. Sort xong quên phần cuối

- ❌ Sai: Tự implement Heap Sort nhưng sift_down vẫn xét toàn bộ mảng
- ✅ Đúng: `sift_down(arr, 0, end)` -- tham số `end` thu nhỏ dần, không chạm phần đã sorted
- 💡 Tại sao: Nếu sift_down chạm phần đã sorted, max cũ sẽ bị kéo ngược lại vào heap, phá kết quả.

---

## Khi nào dùng Heap Sort?

| Tình huống | Dùng Heap Sort? | Dùng gì thay? | Tại sao? |
|------------|----------------|---------------|----------|
| Cần O(1) extra memory + guaranteed O(n log n) | Yes | -- | Duy nhất đạt cả hai |
| Embedded systems, kernel | Yes | -- | Memory hạn chế |
| Sort general (không ràng buộc đặc biệt) | No | Quick Sort | Quick Sort nhanh hơn thực tế |
| Cần stable sort | No | Merge Sort | Heap Sort unstable |
| Data gần sorted rồi | No | Insertion Sort | O(n) cho nearly sorted |
| Chỉ cần top-K, không cần sort toàn bộ | No | Partial sort (heap pop K lần) | Không cần sort hết |
| Integers trong range nhỏ | No | [Radix Sort](06-radix-sort.md) | O(nk) < O(n log n) |

**Tóm lại**: Heap Sort là "lựa chọn an toàn" -- không bao giờ tệ, nhưng hiếm khi là tốt nhất. Nó giống như mua bảo hiểm: bạn hy vọng không cần dùng (Quick Sort thường nhanh hơn), nhưng khi cần (worst case, memory hạn chế), nó luôn ở đó.

---

## Ví dụ

```rust
use rust_ds2a::sorting::heap_sort;

let mut v = vec![4, 10, 3, 5, 1];
heap_sort(&mut v);
assert_eq!(v, vec![1, 3, 4, 5, 10]);

// Mảng 1 phần tử
let mut v = vec![1];
heap_sort(&mut v);
assert_eq!(v, vec![1]);

// Mảng đã sắp xếp — vẫn O(n log n), không bị chậm như Quick Sort
let mut v = vec![1, 2, 3, 4, 5];
heap_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 4, 5]);
```

---

## Luyện tập

### Bài 1: Kth Largest Element in an Array (LeetCode #215)

Tìm phần tử lớn thứ K trong mảng unsorted. Ví dụ `[3,2,1,5,6,4]`, k=2 thì answer = 5.

**Tại sao liên quan Heap Sort?** Bạn không cần sort toàn bộ. Chỉ cần build max-heap rồi pop K lần. Hoặc dùng min-heap size K (xem chi tiết ở [chương Binary Heap](../../chapters/03-trees-and-heaps/05-binary-heap.md)).

<details>
<summary>Gợi ý</summary>

**Cách 1 -- Heap Sort partial**: Build max-heap O(n), pop K lần O(K log n). Tốt khi K nhỏ.

**Cách 2 -- Min-heap size K**: Duyệt mảng, giữ min-heap size K. Root = phần tử lớn thứ K. O(n log K).

Cách 2 tốt hơn khi K << n.

</details>

<details>
<summary>Code Rust</summary>

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn find_kth_largest(nums: Vec<i32>, k: usize) -> i32 {
    // Min-heap size K
    let mut heap = BinaryHeap::new();
    for num in nums {
        heap.push(Reverse(num));
        if heap.len() > k {
            heap.pop(); // loại phần tử nhỏ nhất
        }
    }
    heap.peek().unwrap().0
}
```

</details>

### Bài 2: Sort an Array (LeetCode #912)

Sort mảng integers. Đây là bài để thực hành implement Heap Sort từ đầu.

<details>
<summary>Gợi ý</summary>

Áp dụng đúng 2 bước:
1. `build_heap`: for i in (0..n/2).rev() { sift_down }
2. Extract: for end in (1..n).rev() { swap(0, end); sift_down(0, end) }

Chú ý: LeetCode #912 có test case lớn, naive O(n^2) sẽ TLE. Heap Sort O(n log n) sẽ pass.

</details>

<details>
<summary>Code Rust</summary>

```rust
fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    // Build max-heap
    for i in (0..n / 2).rev() {
        sift_down(&mut nums, i, n);
    }
    // Extract max repeatedly
    for end in (1..n).rev() {
        nums.swap(0, end);
        sift_down(&mut nums, 0, end);
    }
    nums
}

fn sift_down(arr: &mut [i32], mut idx: usize, len: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest = idx;
        if left < len && arr[left] > arr[largest] {
            largest = left;
        }
        if right < len && arr[right] > arr[largest] {
            largest = right;
        }
        if largest == idx { break; }
        arr.swap(idx, largest);
        idx = largest;
    }
}
```

</details>

---

## Heap Sort trong Rust Ecosystem

### `sort_unstable()` -- dùng Heap Sort như fallback

Rust `slice::sort_unstable()` dùng **pattern-defeating quicksort** (pdqsort). Nhưng khi pdqsort phát hiện input "xấu" (quá nhiều partition tệ), nó **fallback sang Heap Sort** để đảm bảo O(n log n) worst case.

```rust
let mut v = vec![5, 3, 8, 1, 2];
v.sort_unstable(); // pdqsort bên trong, fallback heap sort nếu cần
```

Đây là lý do Heap Sort quan trọng dù ít khi dùng standalone: nó là "bảo hiểm" cho Quick Sort.

### `BinaryHeap::into_sorted_vec()` -- Heap Sort built-in

```rust
use std::collections::BinaryHeap;

let heap = BinaryHeap::from(vec![4, 1, 7, 3, 8]);
let sorted = heap.into_sorted_vec();
assert_eq!(sorted, vec![1, 3, 4, 7, 8]);
// Bên trong chính là heap sort!
```

### KaCrab integration

Trong crate `rust_ds2a`, hàm `heap_sort` nằm trong module `sorting`:

```rust
use rust_ds2a::sorting::heap_sort;

let mut data = vec![42, 17, 93, 5, 28];
heap_sort(&mut data);
// Dùng cùng logic sift_down như BinaryHeap ở chương 3
```

Code `sift_down` trong `heap_sort` và code `sift_down` trong `BinaryHeap` (chương 3) **giống nhau về logic** -- chỉ khác interface (free function vs method). Nếu bạn đã đọc chương Binary Heap, code này sẽ rất quen.

---

## Tiếp theo: Radix Sort

Tất cả sorting algorithms từ Bubble đến Heap Sort đều dùng **comparison** (so sánh 2 phần tử). Có một giới hạn toán học: comparison-based sort **không thể nhanh hơn O(n log n)** trong worst case. Đây là lower bound đã được chứng minh.

Nhưng nếu ta biết thêm thông tin về data (ví dụ: toàn bộ là số nguyên, range giới hạn) thì có thể **phá rào** O(n log n)?

[Chương tiếp theo](06-radix-sort.md) giới thiệu **Radix Sort** -- sort **không dùng comparison**, đạt O(nk) với k là số chữ số. Khi k nhỏ (như sort 1 triệu số điện thoại 10 chữ số), Radix Sort nhanh hơn mọi comparison-based sort.

---

---

[← Quick Sort](./04-quick-sort.md) | [Radix Sort →](./06-radix-sort.md)
