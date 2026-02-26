# Quick Sort

## Đây là gì?

Tưởng tượng bạn đang phân loại rác. Bạn chọn một mốc (ví dụ: "rác tái chế"). Tất cả rác nhỏ hơn mốc (rác hữu cơ) bỏ sang trái. Tất cả lớn hơn mốc (rác công nghiệp) bỏ sang phải. Rồi lặp lại việc phân loại cho từng bên.

Quick Sort (sắp xếp nhanh) hoạt động chính xác như vậy:

1. Chọn một phần tử làm **pivot** (mốc)
2. **Phân hoạch** (partition): phần tử nhỏ hơn pivot bỏ sang trái, lớn hơn bỏ sang phải
3. **Đệ quy**: lặp lại cho từng bên

Đây là thuật toán sắp xếp nhanh nhất trong thực tế (trung bình). Nhưng trường hợp xấu nhất có thể chậm O(n^2).

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
| Phần tử ngẫu nhiên | Trung bình tốt | Code dài hơn |
| Trung vị 3 phần tử | Tránh trường hợp xấu với mảng đã sort | Nhiều phép so sánh hơn |

> **Tại sao trường hợp xấu là O(n^2)?** Khi pivot luôn là phần tử nhỏ nhất hoặc lớn nhất (ví dụ: mảng đã sắp xếp, chọn phần tử cuối), mỗi lần partition chỉ tách được 1 phần tử. Cần n lần partition, mỗi lần duyệt n phần tử -> O(n^2).

---

## Code Rust

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

**Ghi chú về Rust:**

- Dùng `usize` cho chỉ số, nên phải kiểm tra `pivot > 0` trước khi trừ 1 để tránh lỗi tràn số (underflow). `0usize - 1` sẽ panic!
- Không cần `Clone` — mọi thứ đều dùng `swap` (đổi chỗ tại chỗ), tiết kiệm bộ nhớ.
- Quick Sort là **in-place** — không cần mảng phụ, chỉ dùng O(log n) bộ nhớ cho đệ quy.

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
