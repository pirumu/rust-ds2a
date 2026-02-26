# Greedy Algorithms

## Đây là gì?

Tưởng tượng bạn đi ăn buffet. Chiến lược của bạn: **luôn lấy món ngon nhất trước**. Không cần suy nghĩ về tương lai, không cần lên kế hoạch phức tạp — cứ nhìn trước mắt, chọn cái tốt nhất.

Đây chính là **Greedy Algorithm** (thuật toán tham lam) — tại mỗi bước, luôn chọn **phương án tốt nhất tại thời điểm đó** (locally optimal), hy vọng dẫn đến kết quả tốt nhất toàn cục (globally optimal).

Khác với Dynamic Programming (xét mọi khả năng), Greedy **không bao giờ quay lại** quyết định cũ. Greedy hoạt động khi bài toán có:

1. **Greedy choice property** — chọn tốt nhất tại chỗ dẫn đến tối ưu toàn cục.
2. **Optimal substructure** — lời giải tối ưu chứa lời giải tối ưu của bài con.

> **Cảnh báo:** Không phải bài nào Greedy cũng cho đáp án tối ưu! Ví dụ: 0/1 Knapsack cần DP, Greedy sẽ cho sai. Nhưng khi đúng, Greedy thường đơn giản và nhanh hơn DP.

---

## Hoạt động như thế nào?

### Activity Selection — chọn hoạt động

Cho danh sách hoạt động với thời gian bắt đầu và kết thúc. Chọn nhiều hoạt động nhất mà không chồng chéo.

**Chiến lược Greedy:** Luôn chọn hoạt động **kết thúc sớm nhất**.

```
Hoạt động (sắp theo thời gian kết thúc):
  A: [1, 2)   B: [3, 4)   C: [0, 6)   D: [5, 7)   E: [8, 9)   F: [5, 9)

Bước 1: Chọn A [1,2)     kết thúc = 2
Bước 2: Chọn B [3,4)     kết thúc = 4   (3 >= 2, OK)
Bước 3: Bỏ C [0,6)       (0 < 4, chồng chéo!)
Bước 4: Chọn D [5,7)     kết thúc = 7   (5 >= 4, OK)
Bước 5: Chọn E [8,9)     kết thúc = 9   (8 >= 7, OK)
Bước 6: Bỏ F [5,9)       (5 < 9, chồng chéo!)

Đã chọn: {A, B, D, E} = 4 hoạt động
```

**Tại sao chọn kết thúc sớm nhất lại tối ưu?** Vì hoạt động kết thúc sớm nhất sẽ "nhường" nhiều thời gian nhất cho các hoạt động sau. Nếu có lời giải tối ưu nào không chứa hoạt động kết thúc sớm nhất, ta luôn có thể thay thế hoạt động đầu tiên mà không gây xung đột.

### Fractional Knapsack — cái túi phân số

Khác với 0/1 Knapsack, ở đây bạn có thể **lấy một phần** của đồ vật (ví dụ: lấy nửa kg gạo).

**Chiến lược Greedy:** Sắp xếp theo tỷ lệ giá trị/trọng lượng, lấy từ cao nhất.

```
Đồ vật: (w=10,v=60), (w=20,v=100), (w=30,v=120)
Tỷ lệ:  6.0,          5.0,           4.0
Sức chứa = 50

Lấy hết đồ 1:   còn = 40, giá trị = 60
Lấy hết đồ 2:   còn = 20, giá trị = 160
Lấy 2/3 đồ 3:   còn = 0,  giá trị = 160 + 80 = 240

Đáp án: 240
```

### Huffman Encoding — mã hóa Huffman

Xây dựng bộ mã tối ưu bằng cách liên tục gộp 2 ký tự có tần suất thấp nhất.

Tưởng tượng bạn cần nén văn bản. Ký tự xuất hiện nhiều -> mã ngắn. Ký tự hiếm -> mã dài.

```
Tần suất: a=5, b=9, c=12, d=13, e=16, f=45

Xây cây:
  Gộp a(5) + b(9)    = ab(14)
  Gộp c(12) + d(13)  = cd(25)
  Gộp ab(14) + e(16) = abe(30)
  Gộp cd(25) + abe(30) = cdabe(55)
  Gộp f(45) + cdabe(55) = gốc(100)

       (100)
      /     \
   f(45)   (55)
           /   \
        (25)   (30)
        / \    / \
     c(12) d(13) (14) e(16)
                 / \
              a(5) b(9)

Mã: f=0, c=100, d=101, a=1100, b=1101, e=111
```

Ký tự f xuất hiện nhiều nhất (45 lần) -> mã ngắn nhất (1 bit). Ký tự a, b hiếm nhất -> mã dài nhất (4 bit).

---

## Code Rust

```rust
/// Activity Selection — chọn nhiều hoạt động nhất không chồng chéo.
pub fn activity_selection(start: &[usize], end: &[usize]) -> Vec<usize> {
    let n = start.len();
    if n == 0 { return vec![]; }

    // Sắp theo thời gian kết thúc
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by_key(|&i| end[i]);

    // Greedy: luôn chọn hoạt động kết thúc sớm nhất
    let mut selected = vec![indices[0]];
    let mut last_end = end[indices[0]];
    for &i in &indices[1..] {
        if start[i] >= last_end {
            selected.push(i);
            last_end = end[i];
        }
    }
    selected
}

/// Fractional Knapsack — lấy đồ theo tỷ lệ giá trị/trọng lượng.
pub fn fractional_knapsack(weights: &[f64], values: &[f64], capacity: f64) -> f64 {
    let n = weights.len();
    let mut indices: Vec<usize> = (0..n).collect();

    // Sắp theo tỷ lệ giá trị/trọng lượng giảm dần
    indices.sort_by(|&a, &b| {
        let ratio_a = values[a] / weights[a];
        let ratio_b = values[b] / weights[b];
        ratio_b.partial_cmp(&ratio_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut remaining = capacity;
    let mut total = 0.0;
    for &i in &indices {
        if remaining <= 0.0 { break; }
        if weights[i] <= remaining {
            total += values[i];          // Lấy hết
            remaining -= weights[i];
        } else {
            total += values[i] * (remaining / weights[i]);  // Lấy một phần
            remaining = 0.0;
        }
    }
    total
}

// Huffman Encoding xây cây nhị phân từ tần suất ký tự,
// rồi sinh mã prefix-free. Xem src/greedy.rs cho implementation đầy đủ
// bao gồm HuffmanNode enum và thuật toán xây cây.
```

**Ghi chú về Rust:**

- `sort_by_key(|&i| end[i])` sắp xếp chỉ số theo giá trị — cách idiomatic để sắp xếp gián tiếp trong Rust.
- `partial_cmp` dùng cho `f64` vì floating-point có `NaN` (không so sánh được). `Ord` trait không implement cho `f64`, nên phải dùng `partial_cmp` + `unwrap_or`.
- Greedy thường có code đơn giản hơn DP rất nhiều.

---

## Độ phức tạp

| Thuật toán | Thời gian | Bộ nhớ |
|-----------|----------|--------|
| Activity Selection | O(n log n) | O(n) |
| Fractional Knapsack | O(n log n) | O(n) |
| Huffman Encoding | O(n log n) | O(n) |

**Giải thích thực tế:**

- Chi phí chủ yếu là **sắp xếp** (hoặc thao tác priority queue). Phần Greedy chỉ duyệt 1 lần O(n).
- So với DP: Greedy nhanh hơn nhiều. Activity Selection bằng DP sẽ tốn O(n^2), bằng Greedy chỉ O(n log n).

**Greedy vs DP:**

| | Greedy | DP |
|--|--------|-----|
| Tốc độ | Nhanh hơn | Chậm hơn |
| Luôn tối ưu? | Không (chỉ khi thỏa 2 tính chất) | Có |
| Code | Đơn giản | Phức tạp hơn |
| Ví dụ đúng | Activity Selection, Fractional Knapsack | 0/1 Knapsack, Edit Distance |

---

## Ví dụ

```rust
use rust_ds2a::greedy::*;

// Activity Selection
let start = [1, 3, 0, 5, 8, 5];
let end   = [2, 4, 6, 7, 9, 9];
let selected = activity_selection(&start, &end);
assert!(selected.len() >= 3);

// Fractional Knapsack
let weights = [10.0, 20.0, 30.0];
let values  = [60.0, 100.0, 120.0];
let result = fractional_knapsack(&weights, &values, 50.0);
assert!((result - 240.0).abs() < 1e-9);

// Huffman Encoding
let (encoded, table) = huffman_encoding("aabbc");
assert_eq!(table.len(), 3);  // 3 ký tự khác nhau
```
