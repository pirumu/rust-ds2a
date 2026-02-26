# Heap Sort

## Đây là gì?

Tưởng tượng bạn có một hộp đồ chơi và muốn sắp xếp chúng theo kích thước. Bạn dùng một chiếc tháp (heap) đặc biệt: **luôn lấy đồ chơi lớn nhất từ đỉnh tháp ra**, đặt nó vào cuối hàng. Rồi sắp lại tháp, lấy tiếp đồ chơi lớn nhất tiếp theo.

Heap Sort (sắp xếp vun đống) hoạt động như vậy:

1. Biến mảng thành **Max-Heap** (cây nhị phân nơi phần tử lớn nhất luôn ở gốc)
2. Lấy phần tử lớn nhất (gốc) ra, đặt vào cuối mảng
3. Sắp lại heap, lặp lại cho đến khi hết

Liên kết với chương Binary Heap: Heap Sort sử dụng chính cấu trúc Binary Heap đã học. Nếu bạn hiểu heap rồi, thuật toán này chỉ là "build heap + extract max lặp lại".

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

- Hàm `sift_down` được dùng lại ở cả 2 bước: xây heap và sắp xếp. Đây là thiết kế tốt — một hàm, hai mục đích.
- Công thức vị trí trong heap (lưu dạng mảng): con trái = `2*i + 1`, con phải = `2*i + 2`, cha = `(i - 1) / 2`.

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
- **Xây dựng heap là O(n)**: tưởng như là O(n log n), nhưng thực tế chỉ là O(n) nhờ tính chất toán học của sift-down (các node ở tầng dưới có ít việc hơn).

**So sánh với các thuật toán khác:**

| | Quick Sort | Merge Sort | Heap Sort |
|--|-----------|-----------|----------|
| Trung bình | Nhanh nhất | Nhanh | Nhanh |
| Xấu nhất | O(n^2)! | O(n log n) | O(n log n) |
| Bộ nhớ | O(log n) | O(n) | O(1) |
| Ổn định | Không | Có | Không |

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
