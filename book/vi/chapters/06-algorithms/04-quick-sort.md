# Quick Sort

> 💡 **Đừng lo lắng:** Chương này dài hơn các chương sort trước, nhưng đừng lo. Nếu bạn đã hiểu Merge Sort ở [Chương 03](./03-merge-sort.md), bạn đã nắm được 70% rồi. Quick Sort cũng là Divide and Conquer — chỉ khác ở chỗ "chia" thông minh hơn. Đọc từ từ, chạy trace bằng tay, rồi sẽ thấy nó không khó đâu.

---

## Đây là gì?

Tưởng tượng bạn đang phân loại rác. Bạn chọn một mốc (ví dụ: "rác tái chế"). Tất cả rác nhỏ hơn mốc (rác hữu cơ) bỏ sang trái. Tất cả lớn hơn mốc (rác công nghiệp) bỏ sang phải. Rồi lặp lại việc phân loại cho từng bên.

Quick Sort (sắp xếp nhanh) hoạt động chính xác như vậy:

1. Chọn một phần tử làm **pivot** (mốc)
2. **Phân hoạch** (partition): phần tử nhỏ hơn pivot bỏ sang trái, lớn hơn bỏ sang phải
3. **Đệ quy**: lặp lại cho từng bên

Đây là thuật toán sắp xếp nhanh nhất trong thực tế (trung bình). Nhưng trường hợp xấu nhất có thể chậm O(n^2).

### Bridge: Quick Sort vs Merge Sort — cùng cha khác mẹ

Cả hai đều dùng **Divide and Conquer**, nhưng triết lý ngược nhau:

```
Merge Sort:  "Chia DỄ, gộp PHỨC TẠP"
             Chia đôi → đệ quy → merge 2 nửa đã sort (tốn công ở bước merge)

Quick Sort:  "Chia THÔNG MINH, gộp DỄ"
             Partition (phân hoạch) → đệ quy → xong! (không cần merge)
```

Merge Sort dồn hết công sức vào bước **gộp** (merge). Quick Sort dồn hết công sức vào bước **chia** (partition). Sau khi partition xong, pivot đã đúng vị trí, hai bên tự sort xong là xong — không cần gộp gì cả.

---

## Hoạt động như thế nào?

### Lomuto Partition — từng bước

Chọn phần tử cuối làm pivot. Dùng chỉ số `i` đánh dấu ranh giới giữa "phần tử <= pivot" và "phần tử > pivot".

```
Mảng:  [3, 7, 8, 5, 2, 1, 9, 4]     pivot = 4 (phần tử cuối)
                                       i = 0

j=0: arr[0]=3 <= 4?  Có!  swap(0,0), i=1
     [3, 7, 8, 5, 2, 1, 9, 4]
      ^
      <= pivot

j=1: arr[1]=7 <= 4?  Không!  Bỏ qua
     [3, 7, 8, 5, 2, 1, 9, 4]
      ^  ^
      <=  >

j=2: arr[2]=8 <= 4?  Không!  Bỏ qua

j=3: arr[3]=5 <= 4?  Không!  Bỏ qua

j=4: arr[4]=2 <= 4?  Có!  swap(1,4), i=2
     [3, 2, 8, 5, 7, 1, 9, 4]
      ^--^
      <=    >

j=5: arr[5]=1 <= 4?  Có!  swap(2,5), i=3
     [3, 2, 1, 5, 7, 8, 9, 4]
      ^-----^
        <=       >

j=6: arr[6]=9 <= 4?  Không!  Bỏ qua

Đặt pivot vào vị trí i=3: swap(3, 7)
     [3, 2, 1, 4, 7, 8, 9, 5]
      <=pivot  ^  >pivot
               pivot đã đúng vị trí!
```

Sau partition:

```
     [3, 2, 1]    4    [7, 8, 9, 5]
      ^               ^
      đệ quy trái      đệ quy phải
```

Pivot `4` đã nằm đúng chỗ. Chỉ cần sắp xếp 2 bên còn lại.

### Hoare Partition — production dùng cái này

Lomuto đơn giản để hiểu, nhưng trong thực tế, **Hoare partition** được dùng nhiều hơn vì ít swap hơn (trung bình ít hơn 3 lần so với Lomuto).

Ý tưởng: đặt 2 con trỏ ở 2 đầu mảng, đi vào giữa. Khi con trỏ trái tìm thấy phần tử >= pivot và con trỏ phải tìm thấy phần tử <= pivot, swap chúng.

```
Mảng:  [3, 7, 8, 5, 2, 1, 9, 4]    pivot = arr[0] = 3
        L→                    ←R

Bước 1: L tìm phần tử >= 3: arr[0]=3 >= 3? Có! L dừng ở 0
        R tìm phần tử <= 3: arr[7]=4? Không. arr[6]=9? Không.
                             arr[5]=1 <= 3? Có! R dừng ở 5
        swap(0, 5):
        [1, 7, 8, 5, 2, 3, 9, 4]
         ^              ^
         L              R

Bước 2: L tiến: arr[1]=7 >= 3? Có! L dừng ở 1
        R lùi:  arr[4]=2 <= 3? Có! R dừng ở 4
        swap(1, 4):
        [1, 2, 8, 5, 7, 3, 9, 4]
            ^        ^
            L        R

Bước 3: L tiến: arr[2]=8 >= 3? Có! L dừng ở 2
        R lùi:  arr[3]=5 <= 3? Không. arr[2]=8 <= 3? Không.
        R < L? Chưa. Nhưng R=1 < L=2 → DỪNG!

Partition xong:
        [1, 2 | 8, 5, 7, 3, 9, 4]
         <=3        >=3
```

Hoare trả về vị trí ranh giới (R=1). Đệ quy cho `[lo..R]` và `[R+1..hi]`.

**Tại sao Hoare nhanh hơn?** Vì nó swap 2 phần tử "sai bên" cùng lúc, trong khi Lomuto phải dời từng phần tử một qua ranh giới `i`.

### 3-Way Partition — Dutch National Flag

Khi mảng có **nhiều phần tử trùng nhau**, cả Lomuto và Hoare đều lãng phí thời gian sort lại các phần tử bằng pivot. 3-way partition giải quyết vấn đề này.

Tên gọi Dutch National Flag (Cờ Hà Lan) vì cờ Hà Lan có 3 màu: đỏ, trắng, xanh — tương ứng 3 vùng: < pivot, = pivot, > pivot.

```
Ý tưởng: chia mảng thành 3 vùng
    [  < pivot  |  = pivot  |  chưa xét  |  > pivot  ]
     ^           ^            ^             ^
     lo          lt           i             gt        hi

lt = ranh giới vùng "< pivot" (less than)
gt = ranh giới vùng "> pivot" (greater than)
i  = phần tử đang xét
```

Trace với mảng có nhiều duplicates:

```
Mảng:  [4, 2, 4, 1, 4, 3, 4]    pivot = arr[0] = 4
        lt                 gt
        i

i=0: arr[0]=4 == pivot → i++
     [4, 2, 4, 1, 4, 3, 4]
      lt                 gt
         i

i=1: arr[1]=2 < pivot → swap(lt, i), lt++, i++
     [2, 4, 4, 1, 4, 3, 4]
         lt              gt
            i

i=2: arr[2]=4 == pivot → i++
     [2, 4, 4, 1, 4, 3, 4]
         lt              gt
               i

i=3: arr[3]=1 < pivot → swap(lt, i), lt++, i++
     [2, 1, 4, 4, 4, 3, 4]
            lt           gt
                  i

i=4: arr[4]=4 == pivot → i++
     [2, 1, 4, 4, 4, 3, 4]
            lt           gt
                     i

i=5: arr[5]=3 < pivot → swap(lt, i), lt++, i++
     [2, 1, 3, 4, 4, 4, 4]
               lt        gt
                        i

i=6: arr[6]=4 == pivot → i++
     i > gt → DỪNG!

Kết quả:
     [2, 1, 3 | 4, 4, 4, 4]
       < 4       = 4 (KHÔNG CẦN SORT LẠI!)
```

Tất cả 4 phần tử `4` đều đã đúng vị trí. Chỉ cần đệ quy cho vùng `[2, 1, 3]`. Nếu dùng Lomuto, ta phải xử lý thêm 4 phần tử `4` ở các bước đệ quy sau.

> **Khi nào 3-way partition tỏa sáng?** Khi mảng có nhiều duplicates (ví dụ: sort điểm thi của 10.000 học sinh chỉ có giá trị 0-10). Trong trường hợp cực đoan (tất cả bằng nhau), 3-way partition chạy O(n) thay vì O(n log n)!

### Minh họa quá trình đệ quy

```
         [3, 7, 8, 5, 2, 1, 9, 4]
                  pivot=4
         /           |           \
    [3, 2, 1]       [4]       [7, 8, 9, 5]
     pivot=1                    pivot=5
     /    \                    /    |    \
   []    [3,2]            [5]     [7,8,9]
          pivot=2                  pivot=9
          / \                     /    \
        []  [3]               [7,8]   []
                              pivot=8
                              / \
                            [7] []

Kết quả: [1, 2, 3, 4, 5, 7, 8, 9]
```

### Chiến lược chọn pivot

| Chiến lược | Ưu điểm | Nhược điểm |
|-----------|---------|-----------|
| Phần tử cuối | Đơn giản nhất | O(n^2) trên mảng đã sắp xếp |
| Phần tử ngẫu nhiên | Trung bình tốt, expected O(n log n) | Cần random |
| Trung vị 3 phần tử | Tránh trường hợp xấu với mảng đã sort | Nhiều phép so sánh hơn |

> **Tại sao trường hợp xấu là O(n^2)?** Khi pivot luôn là phần tử nhỏ nhất hoặc lớn nhất (ví dụ: mảng đã sắp xếp, chọn phần tử cuối), mỗi lần partition chỉ tách được 1 phần tử. Cần n lần partition, mỗi lần duyệt n phần tử -> O(n^2).

### Randomized Pivot — "kẻ thù không đoán được"

Nếu bạn **luôn** chọn phần tử cuối, kẻ thù (dữ liệu xấu) có thể tạo mảng khiến bạn chạy O(n^2). Nhưng nếu bạn chọn **ngẫu nhiên**, không ai đoán được bạn sẽ chọn gì. Kết quả: **expected** (kỳ vọng) O(n log n) cho MỌI input.

```rust
use rand::Rng;

fn randomized_partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    // Chọn ngẫu nhiên 1 phần tử, swap nó về cuối, rồi dùng Lomuto bình thường
    let pivot_idx = rand::rng().random_range(lo..=hi);
    arr.swap(pivot_idx, hi);
    lomuto_partition(arr, lo, hi)
}
```

Chỉ cần 2 dòng thêm, mà đổi từ worst-case O(n^2) sang **expected** O(n log n). Đây là lý do randomized Quick Sort được dùng rất nhiều trong thực tế.

> **"Expected" nghĩa là gì?** Không phải "có thể", mà là "trung bình toán học qua mọi lần chọn ngẫu nhiên". Giống như tung đồng xu 1000 lần — có thể ra 1000 mặt sấp, nhưng xác suất gần 0. Tương tự, randomized Quick Sort *về lý thuyết* có thể chạy O(n^2), nhưng xác suất thấp đến mức bạn sẽ không bao giờ gặp trong thực tế.

---

## Code Rust

### Lomuto Partition (cơ bản)

```rust
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    quick_sort_helper(arr, 0, n - 1);
}

fn quick_sort_helper<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot = lomuto_partition(arr, lo, hi);

    // Sắp xếp bên trái pivot
    if pivot > 0 {
        quick_sort_helper(arr, lo, pivot - 1);
    }
    // Sắp xếp bên phải pivot
    quick_sort_helper(arr, pivot + 1, hi);
}

fn lomuto_partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    // Chọn phần tử cuối làm pivot
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= arr[hi] {
            arr.swap(i, j);
            i += 1;
        }
    }
    // Đặt pivot vào đúng vị trí
    arr.swap(i, hi);
    i
}
```

### Hoare Partition

```rust
fn hoare_partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    // Dùng phần tử đầu làm pivot
    let mut left = lo;
    let mut right = hi;

    // Lưu giá trị pivot bằng cách dùng index, không clone
    // left sẽ dừng ở phần tử >= pivot, right ở phần tử <= pivot
    loop {
        while arr[left] < arr[lo] {
            left += 1;
        }
        while arr[right] > arr[lo] {
            right -= 1;
        }
        if left >= right {
            return right;
        }
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}

// Lưu ý: khi dùng Hoare, đệ quy khác Lomuto!
fn quick_sort_hoare<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let p = hoare_partition(arr, lo, hi);
    quick_sort_hoare(arr, lo, p);       // <= p, không phải p - 1
    quick_sort_hoare(arr, p + 1, hi);
}
```

### 3-Way Partition (Dutch National Flag)

```rust
fn quick_sort_3way<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let mut lt = lo;       // arr[lo..lt]   < pivot
    let mut i = lo + 1;    // arr[lt..i]    = pivot
    let mut gt = hi;       // arr[gt+1..hi] > pivot

    // Dùng arr[lo] làm pivot
    while i <= gt {
        if arr[i] < arr[lt] {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i] > arr[lt] {
            arr.swap(i, gt);
            // Không tăng i! Phần tử mới swap về chưa xét
            if gt == 0 { break; }  // tránh underflow
            gt -= 1;
        } else {
            // arr[i] == pivot
            i += 1;
        }
    }

    // Đệ quy cho vùng < pivot và > pivot
    // Vùng = pivot đã đúng chỗ, SKIP!
    if lt > lo {
        quick_sort_3way(arr, lo, lt - 1);
    }
    if gt < hi {
        quick_sort_3way(arr, gt + 1, hi);
    }
}
```

**Ghi chú về Rust:**

- Dùng `usize` cho chỉ số, nên phải kiểm tra `pivot > 0` trước khi trừ 1 để tránh lỗi tràn số (underflow). `0usize - 1` sẽ panic!
- Không cần `Clone` — mọi thứ đều dùng `swap` (đổi chỗ tại chỗ), tiết kiệm bộ nhớ.
- Quick Sort là **in-place** — không cần mảng phụ, chỉ dùng O(log n) bộ nhớ cho đệ quy.
- Ở 3-way partition, dòng `if gt == 0 { break; }` là trick Rust đặc thù — vì `gt` là `usize`, nếu `gt = 0` mà trừ 1 thì panic. C/C++ dùng `int` nên không gặp vấn đề này.

---

## Introsort — "Quick Sort thông minh" trong production

Trong thực tế, không ai dùng Quick Sort "thuần" cả. Vấn đề O(n^2) worst-case quá nguy hiểm. Giải pháp: **Introsort** (introspective sort).

```
Introsort = Quick Sort + Heap Sort fallback + Insertion Sort cho mảng nhỏ

Thuật toán:
1. Bắt đầu bằng Quick Sort
2. Theo dõi độ sâu đệ quy. Nếu vượt 2*log2(n) → CHUYỂN sang Heap Sort
3. Khi partition nhỏ hơn ~16 phần tử → CHUYỂN sang Insertion Sort
```

Tại sao kết hợp 3 thuật toán?

- **Quick Sort**: nhanh nhất trung bình nhờ cache locality
- **Heap Sort**: đảm bảo O(n log n) worst-case (fallback khi Quick Sort chọn pivot tệ)
- **Insertion Sort**: nhanh nhất cho mảng nhỏ (ít overhead)

> **Fact:** C++ `std::sort` dùng Introsort. Rust `sort_unstable` cũng dùng pattern-defeating quicksort (pdqsort) — một biến thể hiện đại hơn của Introsort. Chi tiết ở phần Rust Ecosystem bên dưới.

---

## Pitfalls

**1. O(n^2) trên mảng đã sort (Lomuto)**

- **Sai:** Dùng Lomuto partition (chọn phần tử cuối) trên mảng đã sắp xếp
- **Đúng:** Dùng randomized pivot hoặc median-of-3
- **Tại sao:** Mảng `[1,2,3,4,5]`, pivot=5, partition tách `[1,2,3,4]` và `[]`. Tiếp: pivot=4, tách `[1,2,3]` và `[]`. Mỗi lần chỉ bớt 1 phần tử -> n lần partition -> O(n^2).

```
[1, 2, 3, 4, 5]  pivot=5  →  [1,2,3,4] | 5 | []     ← chỉ bớt 1!
[1, 2, 3, 4]     pivot=4  →  [1,2,3]   | 4 | []     ← chỉ bớt 1!
[1, 2, 3]        pivot=3  →  [1,2]     | 3 | []     ← chỉ bớt 1!
...
Tổng: n + (n-1) + (n-2) + ... + 1 = O(n^2)
```

**2. `usize` underflow khi `pivot = 0`**

- **Sai:** `quick_sort_helper(arr, lo, pivot - 1)` khi pivot có thể = 0
- **Đúng:** Kiểm tra `if pivot > 0` trước khi trừ
- **Tại sao:** Rust dùng `usize` (số không âm). `0usize - 1` = panic trong debug, wrap thành `18446744073709551615` trong release. Cả hai đều thảm họa.

```rust
// ❌ SAI - panic khi pivot = 0
quick_sort_helper(arr, lo, pivot - 1);

// ✅ ĐÚNG
if pivot > 0 {
    quick_sort_helper(arr, lo, pivot - 1);
}
```

**3. Stack overflow với mảng lớn + worst-case pivot**

- **Sai:** Đệ quy bình thường với mảng 1 triệu phần tử đã sort
- **Đúng:** Dùng tail-call optimization (đệ quy bên nhỏ trước, loop bên lớn)
- **Tại sao:** O(n) đệ quy sâu = 1 triệu stack frame = boom.

```rust
// ✅ Tail-call optimization
fn quick_sort_optimized<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize) {
    while lo < hi {
        let pivot = lomuto_partition(arr, lo, hi);
        // Đệ quy bên NHỎ hơn, loop bên LỚN hơn
        if pivot.saturating_sub(lo) < hi.saturating_sub(pivot) {
            if pivot > 0 {
                quick_sort_optimized(arr, lo, pivot - 1);
            }
            lo = pivot + 1;  // loop thay vì đệ quy
        } else {
            quick_sort_optimized(arr, pivot + 1, hi);
            hi = pivot.saturating_sub(1);  // loop thay vì đệ quy
        }
    }
}
```

---

## Quick Sort vs Merge Sort — Final Showdown

| Tiêu chí | Quick Sort | Merge Sort | Ai thắng? |
|----------|-----------|-----------|-----------|
| Trung bình | O(n log n) | O(n log n) | Quick Sort (hằng số nhỏ hơn) |
| Xấu nhất | O(n^2) | O(n log n) | Merge Sort |
| Bộ nhớ | O(log n) | O(n) | Quick Sort |
| Cache locality | Tuyệt vời (dữ liệu liền kề) | Kém (copy sang mảng phụ) | Quick Sort |
| Ổn định (stable) | Không | Có | Merge Sort |
| In-place | Có | Không | Quick Sort |
| External sort | Không phù hợp | Rất phù hợp | Merge Sort |

**Tại sao Quick Sort nhanh hơn trong thực tế?**

```
RAM hiện đại đọc dữ liệu theo "cache line" (64 bytes liền kề).

Quick Sort: partition duyệt mảng tuần tự → ít cache miss
            [█ █ █ █ █ █ █ █]  ← CPU cache đọc sẵn dữ liệu gần đó
             → → → → → → → →

Merge Sort: merge cần nhảy giữa mảng gốc và mảng phụ → nhiều cache miss
            [█ █ █ █]  →  [_ _ _ _ _ _ _ _]  ← copy qua lại
                 ↕              ↕
            (mảng phụ)    (mảng gốc)
```

Cache locality nghe có vẻ nhỏ, nhưng trong thực tế nó tạo ra khác biệt 2-3 lần tốc độ. Đây là lý do Quick Sort thường thắng dù cả hai đều O(n log n).

---

## Khi nào dùng gì?

| Tình huống | Nên dùng | Tại sao |
|-----------|---------|---------|
| Sort mảng bình thường | Quick Sort (randomized) | Nhanh nhất trung bình, in-place |
| Cần đảm bảo O(n log n) | Merge Sort hoặc Heap Sort | Quick Sort có thể O(n^2) |
| Cần stable sort | Merge Sort | Quick Sort không stable |
| Bộ nhớ giới hạn | Heap Sort | O(1) bộ nhớ phụ |
| Mảng có nhiều duplicates | 3-way Quick Sort | O(n) khi tất cả bằng nhau |
| Dữ liệu trên đĩa | Merge Sort | Đọc tuần tự, phù hợp external sort |
| Production code (Rust) | `slice.sort_unstable()` | pdqsort — best of all worlds |
| Production code + stable | `slice.sort()` | Merge sort variant |

---

## Độ phức tạp

| Trường hợp | Thời gian | Bộ nhớ |
|-----------|----------|--------|
| Tốt nhất | O(n log n) | O(log n) |
| Trung bình | O(n log n) | O(log n) |
| Xấu nhất | O(n^2) | O(n) |

**Giải thích thực tế:**

- **Trung bình O(n log n)**: trong thực tế, Quick Sort thường nhanh hơn Merge Sort vì ít cache miss hơn (dữ liệu nằm gần nhau trong bộ nhớ).
- **O(log n) bộ nhớ**: chỉ là độ sâu đệ quy (stack). Không cần mảng phụ như Merge Sort.
- **Không ổn định (unstable)**: các phần tử bằng nhau có thể bị đảo thứ tự sau khi swap.
- **Trường hợp xấu O(n^2)**: xảy ra khi pivot luôn là min hoặc max. Cách khắc phục: chọn pivot ngẫu nhiên hoặc dùng median of three.

---

## Rust Ecosystem

### `sort()` vs `sort_unstable()` — dùng cái nào?

```rust
let mut v = vec![5, 3, 1, 4, 2];

v.sort();            // Stable, dùng merge sort variant, O(n) extra memory
v.sort_unstable();   // Unstable, dùng pdqsort, O(1) extra memory, NHANH HƠN
```

| | `sort()` | `sort_unstable()` |
|--|---------|-------------------|
| Thuật toán | TimSort (merge sort variant) | pdqsort (pattern-defeating quicksort) |
| Stable? | Có | Không |
| Bộ nhớ | O(n) | O(log n) |
| Tốc độ | Nhanh | Nhanh hơn ~10-30% |
| Khi nào dùng? | Cần giữ thứ tự phần tử bằng nhau | Mặc định, khi không cần stable |

### pdqsort — Rust `sort_unstable()` dùng cái gì?

pdqsort (pattern-defeating quicksort) là Introsort phiên bản 2.0:

- Quick Sort với Hoare partition là nền tảng
- Phát hiện mảng đã sort hoặc gần sort -> chuyển sang insertion sort
- Phát hiện nhiều duplicates -> chuyển sang 3-way partition
- Đệ quy quá sâu -> chuyển sang Heap Sort
- Mảng nhỏ (<= 20 phần tử) -> Insertion Sort

Tất cả những gì bạn học trong chương này — Lomuto, Hoare, 3-way, Introsort — đều được kết hợp trong `sort_unstable()`. Hiểu từng phần giúp bạn hiểu tại sao Rust standard library nhanh đến vậy.

### KaCrab tips

```rust
// ✅ Dùng sort_unstable khi không cần stable (hầu hết trường hợp)
let mut scores = vec![85, 92, 78, 92, 88];
scores.sort_unstable();

// ✅ Sort by key — sort struct theo field
let mut students = vec![("An", 85), ("Binh", 92), ("Chi", 78)];
students.sort_unstable_by_key(|s| s.1);  // sort theo điểm

// ✅ Sort descending
students.sort_unstable_by(|a, b| b.1.cmp(&a.1));  // điểm giảm dần

// ✅ Partial sort: chỉ cần k phần tử nhỏ nhất
let mut v = vec![5, 3, 1, 4, 2];
v.select_nth_unstable(2);
// v[0], v[1] <= v[2] <= v[3], v[4]
// Giống Quick Select! O(n) average
```

---

## Practice

### Sort Colors (LeetCode #75) — 3-way partition

Cho mảng chỉ chứa 0, 1, 2. Sort in-place, one pass.

Đây chính là Dutch National Flag! Pivot = 1.

```rust
fn sort_colors(nums: &mut Vec<i32>) {
    if nums.is_empty() { return; }
    let mut lo = 0usize;
    let mut mid = 0usize;
    let mut hi = nums.len() - 1;

    while mid <= hi {
        match nums[mid] {
            0 => {
                nums.swap(lo, mid);
                lo += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                nums.swap(mid, hi);
                if hi == 0 { break; }
                hi -= 1;
                // Không tăng mid — phần tử mới swap về chưa xét
            }
            _ => unreachable!(),
        }
    }
}
```

### Kth Largest Element (LeetCode #215) — Quick Select

Tìm phần tử lớn thứ k. Không cần sort toàn bộ — chỉ cần partition đúng chỗ.

Quick Select = Quick Sort nhưng **chỉ đệ quy 1 bên** -> O(n) average thay vì O(n log n).

```rust
fn find_kth_largest(nums: &mut Vec<i32>, k: usize) -> i32 {
    let target = nums.len() - k;  // vị trí trong mảng sorted
    quick_select(nums, 0, nums.len() - 1, target)
}

fn quick_select(nums: &mut Vec<i32>, lo: usize, hi: usize, target: usize) -> i32 {
    if lo == hi {
        return nums[lo];
    }

    // Lomuto partition
    let mut i = lo;
    for j in lo..hi {
        if nums[j] <= nums[hi] {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, hi);

    // Chỉ đệ quy 1 BÊN — bên chứa target
    if i == target {
        nums[i]
    } else if target < i {
        quick_select(nums, lo, i - 1, target)
    } else {
        quick_select(nums, i + 1, hi, target)
    }
}
```

> **Gợi ý:** Trong Rust, `slice.select_nth_unstable(k)` chính là Quick Select. Không cần tự viết trong production!

---

## Ví dụ

```rust
use rust_ds2a::sorting::quick_sort;

let mut v = vec![3, 7, 8, 5, 2, 1, 9, 4];
quick_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 4, 5, 7, 8, 9]);

// Mảng 1 phần tử
let mut single = vec![42];
quick_sort(&mut single);
assert_eq!(single, vec![42]);

// Mảng đã sắp xếp ngược — trường hợp xấu với Lomuto (pivot cuối)
let mut v = vec![5, 4, 3, 2, 1];
quick_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 4, 5]);
```

---

## Tổng kết

Quick Sort là thuật toán sort quan trọng nhất cần hiểu. Không phải vì nó "tốt nhất" (không thuật toán nào tốt nhất mọi trường hợp), mà vì:

1. **Partition** là kỹ thuật nền tảng — Quick Select, Dutch National Flag, rất nhiều bài LeetCode
2. **Cache locality** — hiểu tại sao thuật toán "trên giấy" giống nhau nhưng thực tế khác nhau
3. **Randomization** — kỹ thuật biến worst-case thành expected-case
4. **Introsort/pdqsort** — thực tế kết hợp nhiều thuật toán, không dùng 1 thuật toán thuần

---

[← Merge Sort](./03-merge-sort.md) | [Heap Sort →](./05-heap-sort.md)
