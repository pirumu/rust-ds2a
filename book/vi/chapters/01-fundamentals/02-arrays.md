# Arrays & Slices

## Đây là gì?

Tưởng tượng dãy tủ locker ở trường. Mỗi tủ có số: 0, 1, 2, 3... Bạn muốn lấy đồ ở tủ số 5? Đi thẳng tới tủ 5, mở ra, lấy đồ. Không cần mở từng tủ từ đầu.

**Array** (mảng) hoạt động y như vậy. Nó là một dãy ô nhớ **liền nhau** trong bộ nhớ, mỗi ô chứa 1 phần tử cùng kiểu. Vì các ô nằm cạnh nhau, máy tính có thể nhảy thẳng tới bất kỳ ô nào bằng công thức: `địa_chỉ_gốc + chỉ_số * kích_thước_phần_tử`. Đó là lý do truy cập theo index là O(1) -- nhanh như chớp.

Rust có **3 kiểu** array:

**1. Array** (`[T; N]`) -- mảng có kích thước cố định, biết lúc compile. Giống dãy locker xây sẵn, không thể thêm tủ mới.

**2. Slice** (`&[T]`) -- một "cửa sổ" nhìn vào dữ liệu. Nó chỉ là 1 con trỏ + độ dài. Không sở hữu dữ liệu, chỉ mượn (borrow). Giống như bạn nhìn qua kính vào dãy locker -- thấy được nhưng không phải của bạn.

**3. Vec** (`Vec<T>`) -- mảng động trên heap, có thể co giãn. Giống dãy locker thuê -- cần thêm thì thuê thêm. Bên trong nó là 1 con trỏ, 1 độ dài (len), và 1 sức chứa (capacity).

## Hoạt động như thế nào?

### Bộ nhớ trông như thế nào

```
Stack array [i32; 5]          Heap Vec<i32>
+-----------------------+     stack:  ptr ──┐  len: 5  cap: 8
| 10 | 20 | 30 | 40 | 50 |           │
+-----------------------+     heap:   ▼
 contiguous, fixed size       +-------------------------------+
                              | 10 | 20 | 30 | 40 | 50 | _ | _ | _ |
                              +-------------------------------+
                               ◄── len = 5 ──►◄── unused ──►

Slice &[i32]
  ptr ──► points into any contiguous [i32] data
  len: number of elements visible through this slice
```

Vec có cả `len` (số phần tử hiện tại) và `capacity` (số ô đã cấp phát). Khi `len == capacity` mà bạn thêm phần tử mới, Rust sẽ **cấp phát lại** vùng nhớ lớn hơn (thường gấp đôi) rồi copy dữ liệu sang. Đó là lý do `push` thỉnh thoảng hơi chậm.

Slice thì mượn dữ liệu, nên borrow checker của Rust sẽ đảm bảo bạn **không dùng slice khi dữ liệu gốc đã bị xóa**.

### Khi nào dùng kiểu nào?

| Kiểu | Trên Heap? | Co giãn? | Khi nào dùng |
|------|-----------|----------|--------------|
| `[T; N]` | Không | Không | Buffer nhỏ, kích thước biết trước |
| `&[T]` / `&mut [T]` | Không (mượn) | Không | Tham số hàm, nhìn vào 1 phần dữ liệu |
| `Vec<T>` | Có | Có | Khi cần thêm/bớt phần tử |

**Quy tắc đơn giản:** Hàm nhận vào thì dùng `&[T]` (linh hoạt nhất). Cần sở hữu và thay đổi thì dùng `Vec<T>`.

## Code Rust

Tất cả code nằm trong `src/arrays.rs`.

### Linear search -- Tìm kiếm tuyến tính

Giống như tìm bạn trong hàng người: nhìn từng người một từ đầu đến cuối.

```rust
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}
```

Hàm này dùng **generic** -- nghĩa là hoạt động với bất kỳ kiểu `T` nào so sánh được (implement `PartialEq`). Số nguyên, chuỗi, hay kiểu tự tạo đều được.

Tìm thấy thì trả về `Some(vị_trí)`. Không thấy thì `None`. Đây là cách Rust xử lý "có thể không có kết quả" thay vì trả về -1 như ngôn ngữ khác.

### Binary search -- Tìm kiếm nhị phân

**Yêu cầu:** mảng phải **đã sắp xếp**.

Giống trò đoán số: mỗi lần bạn loại bỏ một nửa.

```rust
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}
```

Minh họa từng bước, tìm số 23 trong `[2, 5, 8, 12, 16, 23, 38, 56, 72, 91]`:

```
Bước 1: lo=0, hi=10, mid=5 → arr[5]=23 → ĐÚNG! Trả về Some(5)
```

Nếu tìm số 8:

```
Bước 1: lo=0, hi=10, mid=5 → arr[5]=23 > 8  → hi=5
Bước 2: lo=0, hi=5,  mid=2 → arr[2]=8  → ĐÚNG! Trả về Some(2)
```

Chi tiết nhỏ nhưng quan trọng: `lo + (hi - lo) / 2` thay vì `(lo + hi) / 2` để **tránh tràn số** khi index rất lớn.

### Reverse -- Đảo ngược mảng

Hai con trỏ đi từ hai đầu vào giữa, hoán đổi phần tử:

```rust
pub fn reverse<T>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let mut left = 0;
    let mut right = len - 1;
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}
```

```
Trước: [1, 2, 3, 4, 5]
        ^           ^      swap(0,4) → [5, 2, 3, 4, 1]
           ^     ^         swap(1,3) → [5, 4, 3, 2, 1]
              ^            phần tử giữa đứng yên
Sau:   [5, 4, 3, 2, 1]
```

Giống xếp lại hàng người quay ngược: người đầu đổi chỗ người cuối, rồi tiến vào.

### Rotate left -- Xoay trái mảng

Dịch mảng sang trái k vị trí. Dùng thủ thuật "ba lần đảo ngược":

```rust
pub fn rotate_left<T>(arr: &mut [T], k: usize) {
    let len = arr.len();
    if len == 0 { return; }
    let k = k % len;
    if k == 0 { return; }
    arr[..k].reverse();
    arr[k..].reverse();
    arr.reverse();
}
```

Tại sao 3 lần đảo ngược lại xoay được? Xem từng bước với `[1,2,3,4,5]`, k=2:

```
Ban đầu:              [1, 2, 3, 4, 5]
Đảo ngược 2 phần đầu: [2, 1, 3, 4, 5]
Đảo ngược 3 phần sau: [2, 1, 5, 4, 3]
Đảo ngược toàn bộ:    [3, 4, 5, 1, 2]   ← đã xoay trái 2 vị trí!
```

Hay phải không? O(n) time, O(1) space -- không cần mảng tạm.

### Find duplicates -- Tìm phần tử trùng

Dùng HashSet -- tưởng tượng như một cuốn sổ ghi lại "đã gặp ai rồi":

```rust
use std::collections::HashSet;

pub fn find_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for &val in arr {
        if !seen.insert(val) {
            duplicates.insert(val);
        }
    }

    let mut result: Vec<i32> = duplicates.into_iter().collect();
    result.sort();
    result
}
```

`seen.insert(val)` trả về `false` nếu giá trị **đã có** trong set. Khi đó ta biết nó là trùng lặp và cho vào `duplicates`.

Giống điểm danh lớp: gọi tên, nếu ai đã có mặt rồi mà xuất hiện lần nữa thì biết là trùng.

Cuối cùng `sort()` để kết quả luôn ra **cùng thứ tự** (vì HashSet không đảm bảo thứ tự).

### Maximum subarray sum -- Tổng mảng con lớn nhất (Kadane's algorithm)

Bài toán: cho mảng số (có âm), tìm dãy con liên tiếp có tổng lớn nhất.

Ví dụ thực tế: bạn theo dõi lãi/lỗ mỗi ngày. Muốn tìm khoảng thời gian liên tiếp mà **lợi nhuận cao nhất**.

```rust
pub fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() { return 0; }

    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];

    for &val in &arr[1..] {
        max_ending_here = val.max(max_ending_here + val);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}
```

Ý tưởng chính: ở mỗi vị trí, ta quyết định **tiếp tục** dãy hiện tại hay **bắt đầu lại** từ đây. Nếu tổng cũ + phần tử hiện tại còn nhỏ hơn chính phần tử hiện tại, thì bỏ dãy cũ, bắt đầu lại.

```
Mảng:              [-2,  1, -3,  4, -1,  2,  1, -5,  4]
max_ending_here:   [-2,  1, -2,  4,  3,  5,  6,  1,  5]
max_so_far:        [-2,  1,  1,  4,  4,  5,  6,  6,  6]
                                                 ^
                                     đáp án = 6 (dãy con [4, -1, 2, 1])
```

Đi qua từng phần tử:
- Ở -2: chỉ có -2, max = -2
- Ở 1: chọn bắt đầu lại (1 > -2+1=-1), max = 1
- Ở -3: tiếp tục (1+(-3)=-2), max vẫn = 1
- Ở 4: bắt đầu lại (4 > -2+4=2), max = 4
- Ở -1: tiếp tục (4-1=3), max = 4
- Ở 2: tiếp tục (3+2=5), max = 5
- Ở 1: tiếp tục (5+1=6), max = 6
- Ở -5: tiếp tục (6-5=1), max vẫn = 6
- Ở 4: tiếp tục (1+4=5), max vẫn = 6

## Độ phức tạp

| Hàm | Time | Space | Ghi chú |
|-----|------|-------|---------|
| `linear_search` | O(n) | O(1) | Xui nhất phải duyệt hết |
| `binary_search` | O(log n) | O(1) | Mảng phải sắp xếp trước |
| `reverse` | O(n) | O(1) | Hoán đổi tại chỗ |
| `rotate_left` | O(n) | O(1) | 3 lần reverse, mỗi lần O(n) |
| `find_duplicates` | O(n) | O(n) | HashSet dùng thêm bộ nhớ |
| `max_subarray_sum` | O(n) | O(1) | Chỉ duyệt 1 lần (Kadane) |

Để ý: `binary_search` nhanh hơn `linear_search` rất nhiều (O(log n) vs O(n)), nhưng **đổi lại** mảng phải được sắp xếp. Không có gì miễn phí!

## Ví dụ

### Sử dụng thư viện

```rust
use rust_ds2a::arrays::*;

fn main() {
    // Linear search -- tìm "bob" trong danh sách
    let names = vec!["alice", "bob", "carol"];
    assert_eq!(linear_search(&names, &"bob"), Some(1));

    // Binary search -- mảng ĐÃ sắp xếp
    let sorted = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];
    assert_eq!(binary_search(&sorted, &23), Some(5));

    // Reverse -- đảo ngược
    let mut v = vec![1, 2, 3, 4];
    reverse(&mut v);
    assert_eq!(v, vec![4, 3, 2, 1]);

    // Rotate left -- xoay trái 2 vị trí
    let mut v = vec![1, 2, 3, 4, 5];
    rotate_left(&mut v, 2);
    assert_eq!(v, vec![3, 4, 5, 1, 2]);

    // Find duplicates -- tìm phần tử trùng
    let dups = find_duplicates(&[4, 3, 2, 7, 8, 2, 3, 1]);
    assert_eq!(dups, vec![2, 3]);

    // Maximum subarray sum -- tổng dãy con lớn nhất
    assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
```
