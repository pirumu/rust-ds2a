# Merge Sort

## Đây là gì?

Tưởng tượng bạn là giáo viên và cần sắp xếp 100 bài kiểm tra theo tên. Một mình làm thì lâu. Bạn **chia đống bài thành 2 nửa**, đưa mỗi nửa cho một học sinh sắp xếp. Khi cả 2 nửa đã sắp xếp xong, bạn **gộp lại** thành một đống duy nhất bằng cách so sánh bài trên đầu của mỗi đống.

Đây chính là Merge Sort (sắp xếp trộn) — một thuật toán theo kiểu **Divide and Conquer** (chia để trị):

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

### Giai đoạn gộp (Merge)

Gộp từng cặp mảng nhỏ đã sắp xếp thành mảng lớn hơn. Mỗi lần gộp: so sánh phần tử đầu của 2 mảng, lấy phần tử nhỏ hơn.

```
Gộp [38] + [27]  --> [27, 38]
Gộp [43] + [3]   --> [3, 43]
Gộp [9] + [82]   --> [9, 82]

Gộp [27, 38] + [3, 43]  --> [3, 27, 38, 43]
Gộp [9, 82] + [10]      --> [9, 10, 82]

Gộp [3, 27, 38, 43] + [9, 10, 82]  --> [3, 9, 10, 27, 38, 43, 82]
```

### Chi tiết bước gộp

Hãy xem kỹ bước gộp cuối cùng:

```
Trái:  [3, 27, 38, 43]     Phải: [9, 10, 82]
        ^                          ^

So sánh 3 và 9  --> lấy 3
           ^                       ^
So sánh 27 và 9  --> lấy 9
           ^                           ^
So sánh 27 và 10  --> lấy 10
           ^                                ^
So sánh 27 và 82  --> lấy 27
               ^                            ^
So sánh 38 và 82  --> lấy 38
                   ^                        ^
So sánh 43 và 82  --> lấy 43
                                            ^
Hết bên trái  --> lấy tất cả còn lại: 82

Kết quả: [3, 9, 10, 27, 38, 43, 82]
```

**Tại sao cách này hiệu quả?** Vì cả 2 nửa **đã được sắp xếp**, nên mỗi lần chỉ cần nhìn phần tử đầu tiên của mỗi bên. Không cần tìm kiếm gì thêm!

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

## Độ phức tạp

| Trường hợp | Thời gian | Bộ nhớ |
|-----------|----------|--------|
| Tốt nhất | O(n log n) | O(n) |
| Trung bình | O(n log n) | O(n) |
| Xấu nhất | O(n log n) | O(n) |

**Giải thích thực tế:**

- **O(n log n)**: chia mảng làm đôi log(n) lần, mỗi lần gộp tốn O(n). Ví dụ: 1.000.000 phần tử chỉ cần khoảng 20 lần chia x 1.000.000 phép gộp = 20.000.000 phép tính. Nhanh hơn rất nhiều so với O(n^2)!
- **O(n) bộ nhớ**: cần thêm một mảng tạm cùng kích thước. Nếu mảng 1GB, bạn cần thêm 1GB RAM. Đây là nhược điểm chính.
- **Ổn định (stable)**: Có — phần tử bằng nhau giữ nguyên thứ tự ban đầu.
- **Độ sâu đệ quy**: O(log n) — với 1.000.000 phần tử, chỉ sâu khoảng 20 lớp đệ quy.

**Khi nào nên dùng?**

- Khi cần đảm bảo O(n log n) trong MỌI trường hợp (không như Quick Sort có thể xuống O(n^2)).
- Khi cần sắp xếp ổn định.
- Khi dữ liệu quá lớn để nằm trong RAM và cần sắp xếp trên đĩa (external sort) — Merge Sort rất phù hợp vì đọc dữ liệu tuần tự.

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
