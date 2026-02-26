# Bubble Sort, Selection Sort & Insertion Sort

## Đây là gì?

Bạn đã bao giờ sắp xếp bài khi chơi tiến lên chưa? Ba thuật toán sắp xếp cơ bản này chính là những cách tự nhiên nhất mà con người thường làm khi sắp xếp thứ gì đó bằng tay.

### Bubble Sort — sắp xếp nổi bọt

Tưởng tượng bạn có một hàng bong bóng trong nước. Bong bóng lớn (nhẹ hơn) sẽ từ từ **nổi lên trên mặt nước**. Tương tự, trong Bubble Sort, phần tử lớn nhất sẽ dần dần "nổi" lên cuối mảng sau mỗi lượt duyệt.

Cách hoạt động: đi qua mảng, so sánh từng cặp kề nhau. Nếu sai thứ tự thì đổi chỗ. Sau mỗi lượt, phần tử lớn nhất "nổi" về đúng vị trí.

### Selection Sort — sắp xếp chọn

Giống cách bạn sắp bài khi chơi tiến lên: **nhìn cả đống bài, tìm lá nhỏ nhất, rút ra đặt trước**. Rồi lại tìm lá nhỏ nhất trong số còn lại, đặt tiếp theo. Cứ thế cho đến hết.

### Insertion Sort — sắp xếp chèn

Giống cách bạn cầm bài trên tay: mỗi khi rút thêm một lá bài mới, bạn **chèn nó vào đúng vị trí** trong những lá đã sắp xếp sẵn trên tay.

| Thuật toán | Ý tưởng chính |
|-----------|--------------|
| **Bubble Sort** | Đổi chỗ cặp kề nhau, phần tử lớn "nổi" lên cuối |
| **Selection Sort** | Tìm phần tử nhỏ nhất, đặt vào đầu |
| **Insertion Sort** | Chèn phần tử vào đúng vị trí trong phần đã sắp xếp |

---

## Hoạt động như thế nào?

### Bubble Sort — từng bước

Mảng ban đầu: `[5, 3, 8, 1, 2]`

**Lượt 1:** So sánh từng cặp kề nhau, đổi chỗ nếu sai thứ tự.

```
[5, 3, 8, 1, 2]
 ^--^  5 > 3 ? Có! Đổi chỗ
[3, 5, 8, 1, 2]
    ^--^  5 > 8 ? Không. Giữ nguyên
[3, 5, 8, 1, 2]
       ^--^  8 > 1 ? Có! Đổi chỗ
[3, 5, 1, 8, 2]
          ^--^  8 > 2 ? Có! Đổi chỗ
[3, 5, 1, 2, 8]
                     ^^^ 8 đã "nổi" về cuối!
```

**Lượt 2:** Không cần xét phần tử cuối (đã đúng chỗ).

```
[3, 5, 1, 2, | 8]
 ^--^  Không đổi
    ^--^  5 > 1 ? Đổi chỗ
[3, 1, 5, 2, | 8]
       ^--^  5 > 2 ? Đổi chỗ
[3, 1, 2, 5, | 8]
              ^^^ 5 đã đúng chỗ!
```

**Lượt 3:**

```
[3, 1, 2, | 5, 8]
 ^--^  3 > 1 ? Đổi chỗ
[1, 3, 2, | 5, 8]
    ^--^  3 > 2 ? Đổi chỗ
[1, 2, 3, | 5, 8]
           ^^^ 3 đã đúng chỗ!
```

**Kết quả:** `[1, 2, 3, 5, 8]` -- Đã sắp xếp xong!

> **Mẹo tối ưu:** Nếu một lượt duyệt mà không có bất kỳ đổi chỗ nào, mảng đã được sắp xếp. Dừng luôn, không cần duyệt tiếp.

---

### Selection Sort — từng bước

Mảng ban đầu: `[5, 3, 8, 1, 2]`

```
Lượt 1: Tìm nhỏ nhất trong [5, 3, 8, 1, 2] -> 1 (vị trí 3)
        Đổi chỗ vị trí 0 và 3
        [1, 3, 8, 5, 2]
         ^              phần đã sắp xếp

Lượt 2: Tìm nhỏ nhất trong [3, 8, 5, 2] -> 2 (vị trí 4)
        Đổi chỗ vị trí 1 và 4
        [1, 2, 8, 5, 3]
         ^--^           phần đã sắp xếp

Lượt 3: Tìm nhỏ nhất trong [8, 5, 3] -> 3 (vị trí 4)
        Đổi chỗ vị trí 2 và 4
        [1, 2, 3, 5, 8]
         ^-----^        phần đã sắp xếp

Lượt 4: Tìm nhỏ nhất trong [5, 8] -> 5 (đã đúng chỗ)
        [1, 2, 3, 5, 8]
         ^--------^     phần đã sắp xếp

Kết quả: [1, 2, 3, 5, 8]
```

---

### Insertion Sort — từng bước

Mảng ban đầu: `[5, 3, 8, 1, 2]`

Tưởng tượng vạch `|` ngăn giữa phần đã sắp xếp (trái) và chưa sắp xếp (phải).

```
[5 | 3, 8, 1, 2]   Lấy 3, chèn vào phần trái
  3 < 5 ? Có! Dịch 5 sang phải, đặt 3 trước
[3, 5 | 8, 1, 2]   Lấy 8, chèn vào phần trái
  8 > 5 ? Đúng rồi! Giữ nguyên
[3, 5, 8 | 1, 2]   Lấy 1, chèn vào phần trái
  1 < 8 ? Dịch 8   -> [3, 5, _, 8]
  1 < 5 ? Dịch 5   -> [3, _, 5, 8]
  1 < 3 ? Dịch 3   -> [_, 3, 5, 8]
  Đặt 1 vào đầu    -> [1, 3, 5, 8]
[1, 3, 5, 8 | 2]   Lấy 2, chèn vào phần trái
  2 < 8 ? Dịch 8   -> [1, 3, 5, _, 8]
  2 < 5 ? Dịch 5   -> [1, 3, _, 5, 8]
  2 < 3 ? Dịch 3   -> [1, _, 3, 5, 8]
  2 > 1 ? Đúng!    -> [1, 2, 3, 5, 8]

Kết quả: [1, 2, 3, 5, 8]
```

---

## Code Rust

```rust
/// Bubble Sort — đổi chỗ cặp kề nhau, phần tử lớn "nổi" lên cuối.
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n.saturating_sub(i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // Nếu không có đổi chỗ nào -> mảng đã sắp xếp, dừng sớm
        if !swapped {
            break;
        }
    }
}

/// Selection Sort — tìm phần tử nhỏ nhất, đặt vào đầu phần chưa sắp xếp.
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

/// Insertion Sort — chèn phần tử vào đúng vị trí trong phần đã sắp xếp.
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
```

**Ghi chú về Rust:**

- `T: Ord` cho phép sắp xếp bất kỳ kiểu dữ liệu nào có thể so sánh được (số nguyên, chuỗi, ...).
- `arr.swap(i, j)` đổi chỗ 2 phần tử mà không gặp vấn đề với borrow checker. Nếu bạn viết `arr[i] = arr[j]` sẽ bị lỗi vì Rust không cho mượn mutable 2 lần.
- `saturating_sub` tránh lỗi tràn số khi trừ số không dấu (unsigned). Ví dụ: `0usize - 1` sẽ panic, nhưng `0usize.saturating_sub(1)` trả về `0`.

---

## Độ phức tạp

| Thuật toán | Tốt nhất | Trung bình | Xấu nhất | Bộ nhớ | Ổn định? |
|-----------|---------|-----------|---------|--------|---------|
| Bubble Sort | O(n) | O(n^2) | O(n^2) | O(1) | Có |
| Selection Sort | O(n^2) | O(n^2) | O(n^2) | O(1) | Không |
| Insertion Sort | O(n) | O(n^2) | O(n^2) | O(1) | Có |

**Giải thích thực tế:**

- **O(n^2)** nghĩa là: nếu mảng có 1.000 phần tử, cần khoảng 1.000.000 phép tính. Với 10.000 phần tử -> 100.000.000 phép tính. Quá chậm cho dữ liệu lớn!
- **O(n)** ở trường hợp tốt nhất: khi mảng gần như đã sắp xếp, Bubble Sort và Insertion Sort chỉ cần duyệt qua một lần.
- **Ổn định (stable)**: giữ nguyên thứ tự tương đối của các phần tử bằng nhau. Ví dụ: nếu có 2 sinh viên cùng điểm, thứ tự của họ không bị đảo lộn.

**Khi nào nên dùng?**

- Mảng nhỏ (< 50 phần tử): Insertion Sort rất nhanh và đơn giản.
- Mảng gần như đã sắp xếp: Insertion Sort là lựa chọn tốt nhất.
- Thực tế: nhiều thuật toán sort nâng cao (như Timsort trong Python và Java) dùng Insertion Sort cho các đoạn nhỏ.

---

## Ví dụ

```rust
use rust_ds2a::sorting::{bubble_sort, selection_sort, insertion_sort};

// Bubble Sort với số nguyên
let mut v = vec![5, 3, 8, 1, 2];
bubble_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 5, 8]);

// Selection Sort với chuỗi
let mut v = vec!["cherry", "apple", "banana"];
selection_sort(&mut v);
assert_eq!(v, vec!["apple", "banana", "cherry"]);

// Insertion Sort
let mut v = vec![4, 1, 3, 2];
insertion_sort(&mut v);
assert_eq!(v, vec![1, 2, 3, 4]);
```
