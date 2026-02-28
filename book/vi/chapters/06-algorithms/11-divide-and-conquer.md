# Divide and Conquer

> 💡 **Đừng lo lắng:** Nếu bạn đã hiểu Merge Sort (Chương 3) và Quick Sort (Chương 4), bạn đã biết Divide and Conquer rồi — chỉ là chưa gọi tên thôi. Chương này không dạy thuật toán mới khó hơn. Mình chỉ **zoom ra** để nhìn cái pattern chung đằng sau những thứ bạn đã biết. Thở đi, chill thôi.

---

## Bridge — Bạn đã biết D&C rồi đó

Nhớ Merge Sort không? Chia mảng đôi → sort từng nửa → merge lại. Nhớ Quick Sort không? Chọn pivot, partition → sort 2 bên. Nhớ Binary Search không? So sánh giữa → bỏ nửa không cần.

**Cả 3 thằng đều là Divide and Conquer.** Giờ mình generalize cái pattern chung — để lần sau gặp bài mới, bạn nhận ra ngay: "À, bài này chia nhỏ được, D&C thôi!"

---

## Đây là gì?

Bầu cử có hàng triệu phiếu. Không ai ngồi đếm tất cả một chỗ. Thay vào đó, mỗi phường đếm phiếu của mình, báo kết quả lên quận, quận tổng hợp báo lên thành phố, thành phố báo lên trung ương. Mỗi cấp chỉ làm một việc nhỏ — **đếm phần của mình** hoặc **cộng kết quả từ cấp dưới**. Đó chính là Divide and Conquer.

Hay nghĩ thế này: bạn cần dọn nhà mà nhà bừa quá. Một mình dọn cả nhà? Ngợp. Nhưng chia ra: phòng khách, phòng ngủ, bếp, nhà tắm. Mỗi phòng lại chia: giường, tủ, bàn. Mỗi phần nhỏ thì dễ dọn. Dọn xong từng phần → cả nhà sạch. Đó là D&C.

**Divide and Conquer** (chia để trị) là mô hình thiết kế thuật toán gồm 3 bước:

1. **Divide** (chia) — tách bài toán lớn thành các bài toán con nhỏ hơn cùng loại
2. **Conquer** (trị) — giải từng bài con (nếu còn lớn thì tiếp tục chia, nếu đủ nhỏ thì giải trực tiếp)
3. **Combine** (gộp) — gộp lời giải các bài con thành lời giải cho bài gốc

Bạn đã gặp pattern này rồi: Merge Sort chia mảng làm đôi rồi merge lại (Chương 3), Quick Sort partition rồi sort 2 bên (Chương 4), Binary Search bỏ nửa mảng mỗi bước (Chương 7). Chương này mình nhìn lại chúng dưới góc nhìn chung — để bạn nhận ra pattern khi gặp bài mới.

---

## D&C Template — Công thức 3 bước

Mọi thuật toán D&C đều theo template này:

```
fn solve(problem):
    // 1. BASE CASE — bài toán đủ nhỏ, giải trực tiếp
    if problem nhỏ đủ:
        return giải_trực_tiếp(problem)

    // 2. DIVIDE — chia thành bài con
    sub1, sub2, ... = chia(problem)

    // 3. CONQUER — giải đệ quy từng bài con
    result1 = solve(sub1)
    result2 = solve(sub2)
    ...

    // 4. COMBINE — gộp kết quả
    return combine(result1, result2, ...)
```

Mỗi thuật toán D&C khác nhau ở **3 chỗ**: chia thế nào, base case là gì, gộp ra sao. Nhưng skeleton luôn giống nhau.

```
┌──────────────────────────────────────────────┐
│              D&C Template                    │
│                                              │
│  ┌─────────┐    ┌──────────┐    ┌─────────┐ │
│  │ DIVIDE  │ →  │ CONQUER  │ →  │ COMBINE │ │
│  │ (chia)  │    │ (đệ quy) │    │ (gộp)   │ │
│  └─────────┘    └──────────┘    └─────────┘ │
│       │               │              │       │
│  Tách bài toán   Giải bài con   Gộp kết quả │
│  thành phần nhỏ  (hoặc base)   thành đáp án │
└──────────────────────────────────────────────┘
```

---

## Hoạt động như thế nào?

### Sơ đồ phân rã bài toán

```
                    Bài toán (kích thước n)
                   /                       \
          Bài con (n/2)              Bài con (n/2)
          /           \              /           \
      Bài con (n/4)   Bài con (n/4) Bài con (n/4)   Bài con (n/4)
        ...             ...           ...             ...
      Base case (kích thước 1)
```

Khi chia đôi mỗi lần, độ sâu đệ quy là O(log n).

### Merge Sort — chia để trị kinh điển

| Giai đoạn | Hành động |
|----------|-----------|
| Divide | Chia mảng thành 2 nửa |
| Conquer | Sắp xếp đệ quy từng nửa |
| Combine | Gộp 2 nửa đã sắp xếp thành 1 mảng |

```
Divide:     [38, 27, 43, 3]
            /              \
       [38, 27]         [43, 3]
       /     \          /     \
     [38]   [27]      [43]   [3]      ← Base case (len=1, đã sorted)

Combine:
     [27, 38]          [3, 43]         ← Merge từng cặp
         \              /
       [3, 27, 38, 43]                 ← Merge final
```

Merge Sort làm **ít việc khi chia** (chỉ tính mid) nhưng **nhiều việc khi gộp** (merge 2 mảng).

### Quick Sort — chia để trị ngược

| Giai đoạn | Hành động |
|----------|-----------|
| Divide | Phân hoạch quanh pivot: nhỏ hơn \| pivot \| lớn hơn |
| Conquer | Sắp xếp đệ quy 2 phần |
| Combine | Không cần — mảng đã sắp tại chỗ |

Quick Sort làm **nhiều việc khi chia** (partition) nhưng **không cần gộp**.

### Binary Search — chia để trị đơn giản nhất

| Giai đoạn | Hành động |
|----------|-----------|
| Divide | So sánh target với phần tử giữa |
| Conquer | Tìm trong nửa phù hợp |
| Combine | Trả kết quả trực tiếp |

Binary Search chỉ đệ quy trên **1** bài con (không phải 2), nên chỉ O(log n).

### So sánh "công việc" của 3 thuật toán

```
             Merge Sort          Quick Sort          Binary Search
             ──────────          ──────────          ─────────────
Divide:      Rẻ (tính mid)      Đắt (partition)     Rẻ (so sánh)
Conquer:     2 bài con           2 bài con           1 bài con
Combine:     Đắt (merge)        Rẻ (không cần)      Rẻ (trả luôn)
Tổng:        O(n log n)         O(n log n) avg      O(log n)
```

---

## Master Theorem — công thức tính Big-O cho D&C

Đừng sợ cái tên. Nó chỉ là một bảng tra thôi.

Khi thuật toán D&C có dạng `T(n) = a * T(n/b) + O(n^d)`:

- **a** = số bài con (chia thành mấy phần?)
- **b** = tỷ lệ chia (mỗi phần nhỏ hơn bao nhiêu lần?)
- **d** = chi phí bước chia/gộp (tốn bao lâu ngoài đệ quy?)

### Bảng tra nhanh — 3 trường hợp

| Trường hợp | Điều kiện | Kết quả | Giải thích dân dã |
|:---:|----------|------------|------------|
| 1 | d > log_b(a) | **O(n^d)** | Bước chia/gộp nặng hơn → nó chi phối |
| 2 | d = log_b(a) | **O(n^d · log n)** | Hai bên "ngang tài" → nhân thêm log n |
| 3 | d < log_b(a) | **O(n^(log_b(a)))** | Đệ quy nặng hơn → nó chi phối |

### Áp dụng cho thuật toán quen thuộc

```
Merge Sort:    T(n) = 2T(n/2) + O(n)
               a=2, b=2, d=1
               log_2(2) = 1 = d  →  Trường hợp 2  →  O(n log n) ✓

Binary Search: T(n) = T(n/2) + O(1)
               a=1, b=2, d=0
               log_2(1) = 0 = d  →  Trường hợp 2  →  O(log n) ✓

Karatsuba:     T(n) = 3T(n/2) + O(n)
               a=3, b=2, d=1
               d=1 < log_2(3) ≈ 1.58  →  Trường hợp 3  →  O(n^1.58) ✓

Strassen:      T(n) = 7T(n/2) + O(n²)
               a=7, b=2, d=2
               d=2 < log_2(7) ≈ 2.81  →  Trường hợp 3  →  O(n^2.81) ✓
```

**Giải thích đơn giản:** Master Theorem cho biết "phần đệ quy" hay "phần chia/gộp" chi phối thuật toán. Nếu chia ra quá nhiều bài con (a lớn), phần đệ quy chi phối. Nếu bước gộp tốn kém (d lớn), phần gộp chi phối.

---

## D&C Beyond Sorting — Không chỉ sort mới dùng D&C

### 1. Karatsuba Multiplication — Nhân số lớn

Bình thường nhân 2 số n chữ số tốn O(n²). Karatsuba phát hiện: thay vì 4 phép nhân con, chỉ cần 3. Nghe ít, nhưng đệ quy xuống nhiều tầng → tiết kiệm rất nhiều.

```
Nhân 1234 × 5678:

Chia:  1234 = 12 × 100 + 34
       5678 = 56 × 100 + 78

Bình thường cần 4 phép nhân: 12×56, 12×78, 34×56, 34×78

Karatsuba chỉ cần 3:
  p1 = 12 × 56 = 672
  p2 = 34 × 78 = 2652
  p3 = (12+34) × (56+78) = 46 × 134 = 6164

  cross = p3 - p1 - p2 = 6164 - 672 - 2652 = 2840

  Kết quả = p1 × 10000 + cross × 100 + p2
          = 6720000 + 284000 + 2652 = 7006652 ✓
```

### 2. Closest Pair of Points — Tìm 2 điểm gần nhất

Cho n điểm trên mặt phẳng, tìm 2 điểm gần nhau nhất. Brute force: O(n²) — so sánh mọi cặp. D&C: O(n log n).

```
Bước 1 (Divide):  Chia điểm thành 2 nửa trái/phải theo trục x
                   ┌───────────┬───────────┐
                   │  •  •     │     •  •  │
                   │     •     │  •        │
                   │  •        │     •     │
                   └───────────┴───────────┘
                      Trái          Phải

Bước 2 (Conquer): Tìm closest pair trong mỗi nửa
                   d_left = 2.5,  d_right = 3.1
                   d = min(2.5, 3.1) = 2.5

Bước 3 (Combine): Kiểm tra các điểm gần đường chia (strip)
                   — chỉ cần xét điểm cách đường chia ≤ d
                   — mỗi điểm chỉ so tối đa 7 điểm khác!
                   ┌───────────┬───────────┐
                   │        •  │  •        │
                   │       ┌───┼───┐       │
                   │       │ • │ • │       │  ← Strip (width = 2d)
                   │       └───┼───┘       │
                   └───────────┴───────────┘
```

Bước combine trông phức tạp nhưng key insight là: trong strip, mỗi điểm chỉ cần so sánh với tối đa 7 điểm khác → combine vẫn O(n).

### 3. Strassen Matrix Multiplication — Nhân ma trận

Nhân 2 ma trận n×n bình thường: O(n³). Strassen chia mỗi ma trận thành 4 block, dùng 7 phép nhân thay vì 8 → O(n^2.81). Cải thiện nhỏ mỗi tầng, nhưng tích lũy qua log n tầng → tiết kiệm đáng kể cho ma trận lớn.

---

## D&C vs Dynamic Programming — Khi nào dùng cái nào?

Đây là câu hỏi kinh điển trong phỏng vấn. Khác biệt cốt lõi:

```
D&C:    Bài toán lớn → chia nhỏ → các bài con KHÔNG LIÊN QUAN nhau
        ┌─────┐
        │  A  │
        ├──┬──┤
        │B │C │    B và C hoàn toàn độc lập
        └──┴──┘    Giải B xong quên luôn, không ảnh hưởng C

DP:     Bài toán lớn → chia nhỏ → các bài con CHỒNG CHÉO nhau
        ┌─────┐
        │  A  │
        ├──┬──┤
        │B │C │    B và C cùng cần kết quả của D!
        ├──┴──┤    → Nếu dùng D&C, tính D hai lần = phí
        │  D  │    → DP lưu D lại, dùng lại = nhanh
        └─────┘
```

| | **D&C** | **DP** |
|---|---------|--------|
| Bài con | Độc lập | Chồng chéo (overlapping) |
| Lưu kết quả? | Không cần | Bắt buộc (memo/table) |
| Hướng giải | Top-down (chia từ trên xuống) | Top-down memo hoặc bottom-up table |
| Ví dụ | Merge Sort, Binary Search | Fibonacci, Knapsack |
| Khi nào? | Bài con không lặp lại | Bài con lặp lại nhiều lần |

**Mẹo nhận biết:** Vẽ cây đệ quy. Nếu thấy cùng một bài con xuất hiện nhiều lần → DP. Nếu mỗi bài con là duy nhất → D&C.

> Xem thêm chi tiết ở Chương 13 — Dynamic Programming.

---

## Code Rust

Divide and Conquer là mô hình thiết kế, không phải một thuật toán cụ thể. Các implementation nằm ở các module tương ứng:

- **Merge Sort** — `sorting::merge_sort` (Chương 3)
- **Quick Sort** — `sorting::quick_sort` (Chương 4)
- **Binary Search** — `searching::binary_search` (Chương 7)

```rust
// Template D&C tổng quát trong Rust
// Mỗi thuật toán cụ thể sẽ implement 3 hàm: divide, conquer, combine

// Merge Sort: chia đôi, đệ quy, gộp
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }       // Base case
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();  // Divide
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);               // Conquer trái
    merge_sort(&mut right);              // Conquer phải
    merge(&left, &right, arr);           // Combine
}

// Binary Search: chia vùng tìm kiếm làm đôi mỗi bước
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;     // Divide
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),  // Tìm thấy
            std::cmp::Ordering::Less => lo = mid + 1,       // Conquer phải
            std::cmp::Ordering::Greater => hi = mid,        // Conquer trái
        }
    }
    None
}
```

### Maximum Subarray — D&C approach

Bài Maximum Subarray (LeetCode #53) có thể giải bằng cả Kadane (Greedy, O(n)) lẫn D&C (O(n log n)). So sánh cả hai giúp bạn hiểu khi nào D&C không phải lựa chọn tốt nhất:

```rust
// D&C approach: chia mảng làm đôi, max subarray nằm ở:
//   1. Hoàn toàn bên trái
//   2. Hoàn toàn bên phải
//   3. Đi qua giữa (cross)
fn max_subarray_dc(arr: &[i32]) -> i32 {
    if arr.len() == 1 { return arr[0]; }  // Base case

    let mid = arr.len() / 2;

    // Conquer: giải 2 nửa
    let left_max = max_subarray_dc(&arr[..mid]);
    let right_max = max_subarray_dc(&arr[mid..]);

    // Combine: tìm max cross subarray
    let mut left_sum = i32::MIN;
    let mut sum = 0;
    for i in (0..mid).rev() {
        sum += arr[i];
        left_sum = left_sum.max(sum);
    }

    let mut right_sum = i32::MIN;
    sum = 0;
    for i in mid..arr.len() {
        sum += arr[i];
        right_sum = right_sum.max(sum);
    }

    let cross_max = left_sum + right_sum;

    // Kết quả = max(trái, phải, cross)
    left_max.max(right_max).max(cross_max)
}

// So sánh với Kadane — đơn giản hơn nhiều, O(n)
fn max_subarray_kadane(arr: &[i32]) -> i32 {
    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];
    for &x in &arr[1..] {
        max_ending_here = x.max(max_ending_here + x);
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}
```

```
Trace Maximum Subarray D&C cho [-2, 1, -3, 4, -1, 2, 1, -5, 4]:

                    [-2, 1, -3, 4, -1, 2, 1, -5, 4]
                   /                                \
         [-2, 1, -3, 4]                    [-1, 2, 1, -5, 4]
         /            \                    /               \
     [-2, 1]      [-3, 4]            [-1, 2]          [1, -5, 4]
      /   \        /   \              /   \            /       \
   [-2]  [1]    [-3]  [4]          [-1]  [2]        [1]    [-5, 4]
                                                            /    \
                                                         [-5]   [4]

Combine lên:
   [-2] [1] → left=-2, right=1, cross=-2+1=-1 → max=1
   [-3] [4] → left=-3, right=4, cross=-3+4=1  → max=4
   [-2,1] [-3,4] → left=1, right=4, cross=1+(-3+4)=2 → max=4

   ... tương tự bên phải ...

   Kết quả cuối: 6 (subarray [4,-1,2,1])
```

---

## Độ phức tạp

| Thuật toán | Bước Divide | Bước Combine | Tổng |
|-----------|------------|-------------|------|
| Merge Sort | O(1) | O(n) | O(n log n) |
| Quick Sort | O(n) | O(1) | O(n log n) trung bình |
| Binary Search | O(1) | O(1) | O(log n) |
| Closest Pair | O(n) sort | O(n) | O(n log n) |
| Karatsuba | O(n) | O(n) | O(n^1.58) |
| Strassen | O(n²) | O(n²) | O(n^2.81) |
| Max Subarray D&C | O(1) | O(n) | O(n log n) |

**Giải thích thực tế:**

- Divide and Conquer thường cho O(n log n) hoặc O(log n), tốt hơn nhiều so với brute force O(n²).
- Nhược điểm: đệ quy tốn stack. Nếu chia không cân bằng (Quick Sort trường hợp xấu), độ sâu đệ quy có thể lên O(n).
- Ưu điểm: dễ song song hóa — 2 bài con có thể giải đồng thời trên 2 CPU. Rust's `rayon` crate tận dụng điều này cực tốt (xem phần Rust Ecosystem bên dưới).

---

## Pitfalls — Bẫy hay gặp

### 1. Base case sai

❌ **Sai:**
```rust
fn dc(arr: &[i32]) -> i32 {
    let mid = arr.len() / 2;
    // Quên base case! Khi arr rỗng → mid = 0 → vòng lặp vô hạn
    dc(&arr[..mid]) + dc(&arr[mid..])
}
```

✅ **Đúng:**
```rust
fn dc(arr: &[i32]) -> i32 {
    if arr.len() <= 1 { return arr.get(0).copied().unwrap_or(0); }
    let mid = arr.len() / 2;
    dc(&arr[..mid]) + dc(&arr[mid..])
}
```

💡 **Tại sao:** Không có base case = đệ quy vô tận → stack overflow. Mỗi lần viết D&C, **luôn viết base case trước**.

### 2. Combine step phức tạp hơn tưởng

❌ **Sai:** Nghĩ rằng Closest Pair chỉ cần lấy min(trái, phải) là xong.

✅ **Đúng:** Phải xét thêm các cặp điểm đi qua đường chia (strip). Đây mới là phần khó nhất.

💡 **Tại sao:** Bài con giải đúng chưa đủ — nếu combine sai, kết quả cuối sai. Combine là nơi dễ sai nhất trong D&C.

### 3. Dùng D&C khi không cần thiết (overkill)

❌ **Sai:** Dùng D&C cho Maximum Subarray → O(n log n).

✅ **Đúng:** Dùng Kadane → O(n), code ngắn hơn, dễ hiểu hơn.

💡 **Tại sao:** D&C không phải luôn tối ưu. Nếu có solution đơn giản hơn (Greedy, DP), ưu tiên cái đó. D&C chỉ nên dùng khi nó thực sự cải thiện complexity hoặc khi bài toán có cấu trúc chia tự nhiên.

### 4. Chia không đều

❌ **Sai:** Quick Sort luôn chọn phần tử đầu làm pivot → mảng đã sorted thành O(n²).

✅ **Đúng:** Dùng median-of-three hoặc random pivot → O(n log n) trung bình.

💡 **Tại sao:** D&C hiệu quả khi chia **cân bằng**. Chia lệch = cây đệ quy sâu = chậm. Xem lại phân tích Quick Sort ở Chương 4.

---

## Khi nào dùng D&C?

| Dấu hiệu | Dùng D&C? | Ví dụ |
|-----------|:---------:|-------|
| Bài toán chia đôi tự nhiên | ✅ Có | Sort, search trên mảng |
| Bài con **độc lập** nhau | ✅ Có | Merge Sort — 2 nửa không liên quan |
| Bài con **chồng chéo** | ❌ Không — dùng DP | Fibonacci, Knapsack |
| Có solution O(n) đơn giản | ❌ Không — overkill | Max Subarray → Kadane |
| Cần tận dụng parallelism | ✅ Có | Sort lớn trên multi-core |
| Input là cây/đệ quy tự nhiên | ✅ Có | Xử lý BST, biểu thức |
| Nhân số lớn / ma trận lớn | ✅ Có | Karatsuba, Strassen |
| Mỗi bước chọn greedy được | ❌ Không — dùng Greedy | Coin change (mệnh giá chuẩn) |

**Mẹo nhanh:** Nếu bạn vẽ cây đệ quy mà thấy mỗi node chia thành 2+ bài con KHÁC NHAU, và kết quả gộp lại được → D&C.

---

## Ví dụ

```rust
use rust_ds2a::sorting::merge_sort;
use rust_ds2a::searching::binary_search;

// Merge Sort: chia để trị kinh điển
let mut v = vec![5, 2, 8, 1, 9];
merge_sort(&mut v);
assert_eq!(v, vec![1, 2, 5, 8, 9]);

// Binary Search: chia vùng tìm kiếm
let idx = binary_search(&v, &8);
assert_eq!(idx, Some(3));
```

---

## Rust Ecosystem — D&C trong thực tế

### Rayon — Parallel D&C miễn phí

Crate `rayon` là ví dụ hoàn hảo cho D&C + parallelism. Sort song song chỉ cần thay 1 dòng:

```rust
use rayon::prelude::*;

let mut data = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];

// Sequential sort
data.sort();

// Parallel sort — rayon tự chia mảng cho nhiều thread
data.par_sort();  // Dùng D&C bên trong: chia mảng → sort song song → merge
```

Rayon dùng **work-stealing** scheduler: mỗi thread lấy 1 bài con, thread nào xong sớm thì "ăn cắp" việc từ thread khác. Đây chính là D&C ở level hệ thống.

### `slice::sort` trong Rust stdlib

Rust stdlib dùng **pdqsort** (pattern-defeating quicksort) — một biến thể Quick Sort (D&C) với nhiều optimization:
- Detect mảng gần sorted → dùng insertion sort
- Detect mảng có nhiều phần tử trùng → xử lý riêng
- Median-of-three pivot → tránh worst case O(n²)

Khi bạn gọi `arr.sort()` trong Rust, bạn đang dùng D&C mà không biết.

### KaCrab Integration

Trong KaCrab, bạn có thể implement D&C problems với Rust slices:

```rust
// Rust slices rất phù hợp cho D&C vì:
// - &arr[..mid] và &arr[mid..] không copy data
// - Zero-cost abstraction: chia slice = chỉ thay đổi pointer + length
// - Borrow checker đảm bảo 2 nửa không overlap → safe parallelism

fn count_inversions(arr: &mut [i32]) -> usize {
    if arr.len() <= 1 { return 0; }
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    let left_inv = count_inversions(&mut left);
    let right_inv = count_inversions(&mut right);
    let split_inv = merge_count(&left, &right, arr);
    left_inv + right_inv + split_inv
}
```

---

## Practice — Luyện tập

### 1. Maximum Subarray (LeetCode #53)

**Đề:** Cho mảng số nguyên, tìm contiguous subarray có tổng lớn nhất.

**2 cách giải:**
- **Kadane (Greedy):** O(n) — tối ưu hơn, xem Chương 12
- **D&C:** O(n log n) — để luyện pattern D&C

**Gợi ý D&C:** Chia đôi mảng. Max subarray hoặc nằm trọn bên trái, trọn bên phải, hoặc đi qua giữa. Code mẫu ở phần Code Rust bên trên.

### 2. Merge k Sorted Lists (LeetCode #23)

**Đề:** Cho k linked list đã sorted, merge thành 1 list sorted.

**Gợi ý D&C:**
```
k lists:  [L1, L2, L3, L4, L5, L6, L7, L8]

Round 1:  merge(L1,L2), merge(L3,L4), merge(L5,L6), merge(L7,L8)
          → [M1, M2, M3, M4]

Round 2:  merge(M1,M2), merge(M3,M4)
          → [N1, N2]

Round 3:  merge(N1,N2)
          → [Result]

Tổng: O(N log k) thay vì O(Nk) nếu merge từng cái một
```

Giống Merge Sort, nhưng thay vì sort mảng, bạn merge k lists. Pattern y hệt: chia đôi danh sách lists → merge đệ quy → combine.

### 3. Closest Pair of Points

**Đề:** Cho n điểm trên mặt phẳng, tìm 2 điểm gần nhau nhất.

**Gợi ý:** Xem phần D&C Beyond Sorting ở trên. Key: bước combine cần kiểm tra strip, nhưng mỗi điểm chỉ so sánh tối đa 7 điểm khác → combine vẫn O(n).

### 4. Count Inversions

**Đề:** Đếm số cặp (i,j) với i < j mà arr[i] > arr[j]. (Ứng dụng: đo mức "lộn xộn" của mảng)

**Gợi ý:** Modify Merge Sort. Trong bước merge, mỗi khi lấy phần tử từ mảng phải (vì nó nhỏ hơn phần tử mảng trái), tất cả phần tử còn lại bên trái đều tạo inversion.

---

## Tổng kết

```
┌─────────────────────────────────────────────────┐
│              DIVIDE AND CONQUER                 │
│                                                 │
│  1. DIVIDE  — chia bài toán thành phần nhỏ      │
│  2. CONQUER — giải đệ quy (hoặc base case)     │
│  3. COMBINE — gộp kết quả                      │
│                                                 │
│  Khi nào dùng:                                  │
│  ✅ Bài con độc lập                             │
│  ✅ Chia tự nhiên (mảng → 2 nửa, cây → 2 con)  │
│  ✅ Cần parallelism                             │
│  ❌ Bài con chồng chéo → dùng DP               │
│  ❌ Có O(n) solution đơn giản hơn               │
│                                                 │
│  Master Theorem: T(n) = aT(n/b) + O(n^d)       │
│  → So sánh d với log_b(a) → tra bảng 3 cases   │
└─────────────────────────────────────────────────┘
```

---

## Tiếp theo — Greedy

D&C chia bài toán rồi giải tất cả bài con. Nhưng nếu mỗi bước bạn chỉ cần chọn **một** lựa chọn tốt nhất, không cần quay lại? Đó là **Greedy** — thuật toán tham lam. Ví dụ: thối tiền bằng tờ lớn nhất có thể. Greedy nhanh hơn D&C (thường O(n log n) hoặc O(n)), nhưng chỉ đúng khi bài toán có "greedy-choice property". Chương sau mình sẽ tìm hiểu khi nào tham lam được và khi nào tham lam sai.

---

[← Sliding Window](./10-sliding-window.md) | [Greedy →](./12-greedy.md)
