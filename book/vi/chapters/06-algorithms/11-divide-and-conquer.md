# Divide and Conquer

## Đây là gì?

Bầu cử có hàng triệu phiếu. Không ai ngồi đếm tất cả một chỗ. Thay vào đó, mỗi phường đếm phiếu của mình, báo kết quả lên quận, quận tổng hợp báo lên thành phố, thành phố báo lên trung ương. Mỗi cấp chỉ làm một việc nhỏ — **đếm phần của mình** hoặc **cộng kết quả từ cấp dưới**. Đó chính là Divide and Conquer.

**Divide and Conquer** (chia để trị) là mô hình thiết kế thuật toán gồm 3 bước:

1. **Divide** (chia) — tách bài toán lớn thành các bài toán con nhỏ hơn cùng loại
2. **Conquer** (trị) — giải từng bài con (nếu còn lớn thì tiếp tục chia, nếu đủ nhỏ thì giải trực tiếp)
3. **Combine** (gộp) — gộp lời giải các bài con thành lời giải cho bài gốc

Bạn đã gặp pattern này rồi: Merge Sort chia mảng làm đôi rồi merge lại, Binary Search bỏ nửa mảng mỗi bước. Chương này mình nhìn lại chúng dưới góc nhìn chung — để bạn nhận ra pattern khi gặp bài mới.

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
     [38]   [27]      [43]   [3]

Combine:
     [27, 38]          [3, 43]
         \              /
       [3, 27, 38, 43]
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

### Master Theorem — công thức tính Big-O cho chia để trị

Với công thức đệ quy dạng `T(n) = a * T(n/b) + O(n^d)`:

- **a** = số bài con
- **b** = tỷ lệ chia (chia cho bao nhiêu)
- **d** = chi phí bước chia/gộp

| Điều kiện | Độ phức tạp |
|----------|------------|
| d > log_b(a) | O(n^d) |
| d = log_b(a) | O(n^d * log n) |
| d < log_b(a) | O(n^(log_b(a))) |

**Áp dụng:**

```
Merge Sort:    T(n) = 2T(n/2) + O(n)
               a=2, b=2, d=1
               d = log_2(2) = 1  -->  O(n log n)

Binary Search: T(n) = T(n/2) + O(1)
               a=1, b=2, d=0
               d = log_2(1) = 0  -->  O(log n)

Karatsuba (nhân số lớn): T(n) = 3T(n/2) + O(n)
               a=3, b=2, d=1
               d=1 < log_2(3)≈1.58  -->  O(n^1.58)
```

**Giải thích đơn giản:** Master Theorem cho biết "phần đệ quy" hay "phần chia/gộp" chi phối thuật toán. Nếu chia ra quá nhiều bài con (a lớn), phần đệ quy chi phối. Nếu bước gộp tốn kém (d lớn), phần gộp chi phối.

---

## Code Rust

Divide and Conquer là mô hình thiết kế, không phải một thuật toán cụ thể. Các implementation nằm ở các module tương ứng:

- **Merge Sort** — `sorting::merge_sort` (Chương 2)
- **Quick Sort** — `sorting::quick_sort` (Chương 3)
- **Binary Search** — `searching::binary_search` (Chương 6)

```rust
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

---

## Độ phức tạp

| Thuật toán | Bước Divide | Bước Combine | Tổng |
|-----------|------------|-------------|------|
| Merge Sort | O(1) | O(n) | O(n log n) |
| Quick Sort | O(n) | O(1) | O(n log n) trung bình |
| Binary Search | O(1) | O(1) | O(log n) |

**Giải thích thực tế:**

- Divide and Conquer thường cho O(n log n) hoặc O(log n), tốt hơn nhiều so với brute force O(n^2).
- Nhược điểm: đệ quy tốn stack. Nếu chia không cân bằng (Quick Sort trường hợp xấu), độ sâu đệ quy có thể lên O(n).
- Ưu điểm: dễ song song hóa — 2 bài con có thể giải đồng thời trên 2 CPU.

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
