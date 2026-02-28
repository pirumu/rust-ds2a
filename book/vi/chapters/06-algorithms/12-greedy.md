# Greedy Algorithms

> 💡 **Đừng lo lắng:** Greedy là thuật toán trực giác nhất bạn sẽ gặp -- chỉ là "mỗi bước, chọn cái tốt nhất ngay lúc đó". Bạn đã làm greedy mỗi ngày mà không biết: chọn đường đi ngắn nhất tới trường, chọn món rẻ nhất trong menu. Câu hỏi khó duy nhất là "khi nào greedy cho đáp án đúng?" -- và chương này sẽ cho bạn checklist cụ thể.

Nếu bạn đã đọc xong [Divide and Conquer](11-divide-and-conquer.md), bạn biết D&C **chia bài toán thành các bài con** rồi gộp lại. Greedy khác hoàn toàn — nó **không chia**, mà **chọn**. Mỗi bước, chọn cái tốt nhất ngay lúc đó, rồi đi tiếp. Không quay lại. Không hối hận.

> **D&C**: "Chia nhỏ ra, giải từng phần, gộp lại."
>
> **Greedy**: "Mỗi bước, chọn cái ngon nhất. Tin rằng cuối cùng sẽ ngon nhất."

---

## Đây là gì?

Tưởng tượng bạn đang trả tiền và cần thối 36.000đ. Bạn có các tờ: 20k, 10k, 5k, 2k, 1k. Bạn sẽ làm gì? **Lấy tờ lớn nhất có thể trước** — 20k, rồi 10k, rồi 5k, rồi 1k. Xong! 4 tờ. Bạn không cần thử mọi cách kết hợp, chỉ cần **mỗi bước chọn tờ lớn nhất mà không vượt quá số còn lại**.

Đây chính là **Greedy Algorithm** (thuật toán tham lam) — tại mỗi bước, luôn chọn **phương án tốt nhất tại thời điểm đó** (locally optimal), hy vọng dẫn đến kết quả tốt nhất toàn cục (globally optimal).

Khác với Dynamic Programming (xét mọi khả năng), Greedy **không bao giờ quay lại** quyết định cũ. Greedy hoạt động khi bài toán có:

1. **Greedy choice property** — chọn tốt nhất tại chỗ dẫn đến tối ưu toàn cục.
2. **Optimal substructure** — lời giải tối ưu chứa lời giải tối ưu của bài con.

> **Cảnh báo:** Không phải bài nào Greedy cũng cho đáp án tối ưu! Ví dụ: 0/1 Knapsack cần DP, Greedy sẽ cho sai. Nhưng khi đúng, Greedy thường đơn giản và nhanh hơn DP.

---

## "Greedy đúng không?" — Checklist

Trước khi code Greedy, hãy tự hỏi 3 câu:

```
┌─────────────────────────────────────────────────────────┐
│           GREEDY ĐÚNG KHÔNG? — CHECKLIST                │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. Optimal substructure?                               │
│     → Sau khi chọn, bài toán còn lại vẫn               │
│       có cùng cấu trúc?                                │
│                                                         │
│  2. Greedy choice → global optimal?                     │
│     → Chọn tốt nhất tại chỗ có BẢO ĐẢM               │
│       dẫn tới tối ưu toàn cục?                         │
│                                                         │
│  3. Tìm được counter-example không?                     │
│     → Có trường hợp nào Greedy cho sai không?           │
│     → Nếu tìm được → KHÔNG dùng Greedy                 │
│     → Nếu không tìm được → có thể đúng,                │
│       nhưng cần chứng minh                              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Ví dụ áp dụng checklist:**

| Bài toán | Substructure? | Local = Global? | Counter-example? | Greedy? |
|----------|:---:|:---:|:---:|:---:|
| Thối tiền (VNĐ) | Co | Co | Khong | Dung |
| Activity Selection | Co | Co | Khong | Dung |
| Fractional Knapsack | Co | Co | Khong | Dung |
| 0/1 Knapsack | Co | **Khong** | **Co** (xem pitfall) | **Sai!** |

---

## Chứng minh Greedy đúng — hai kỹ thuật

Biết Greedy "có vẻ đúng" là chưa đủ. Trong interview hay thi, bạn cần **chứng minh**. Có hai cách phổ biến:

### 1. Greedy Stays Ahead (Greedy luôn dẫn trước)

Ý tưởng: chứng minh rằng **sau mỗi bước**, lời giải Greedy luôn tốt bằng hoặc tốt hơn bất kỳ lời giải nào khác.

```
Ví dụ: Activity Selection

Gọi G = {g1, g2, ..., gk} là lời giải Greedy (chọn kết thúc sớm nhất)
Gọi O = {o1, o2, ..., om} là lời giải tối ưu bất kỳ

Chứng minh: end(gi) <= end(oi) với mọi i
  - Bước 1: end(g1) <= end(o1)
    (vì g1 kết thúc sớm nhất trong tất cả)
  - Bước i: nếu end(gi-1) <= end(oi-1),
    thì gi chọn hoạt động kết thúc sớm nhất sau gi-1
    → end(gi) <= end(oi)

Vậy Greedy luôn "dẫn trước" → k >= m → Greedy tối ưu.
```

### 2. Exchange Argument (Lập luận trao đổi)

Ý tưởng: lấy lời giải tối ưu, **đổi** từng phần tử thành lựa chọn Greedy, chứng minh kết quả **không xấu hơn**.

```
Ví dụ: Fractional Knapsack

Giả sử lời giải tối ưu O không chọn đồ có ratio cao nhất.
Thay đồ có ratio thấp bằng đồ có ratio cao hơn:
  - Giá trị tăng (vì ratio cao hơn)
  - Trọng lượng giữ nguyên hoặc giảm
→ O' tốt hơn O → mâu thuẫn O tối ưu
→ Vậy lời giải tối ưu PHẢI chọn đồ ratio cao nhất trước.
```

> **Mẹo:** Trong phỏng vấn, bạn không cần chứng minh chi tiết. Chỉ cần nói: "Greedy đúng vì nếu ta đổi bất kỳ lựa chọn nào bằng lựa chọn khác, kết quả không thể tốt hơn" (exchange argument) hoặc "Greedy luôn dẫn trước tại mỗi bước" (stays ahead).

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

Tưởng tượng bạn cần nén văn bản. Ký tự xuất hiện nhiều -> mã ngắn. Ký tự hiếm -> mã dài. Giống như trong tiếng Việt, chữ "a" hay gặp thì nên viết nhanh, chữ "q" hiếm thì viết dài cũng được.

Greedy ở đâu? Mỗi bước, ta **chọn 2 node có tần suất thấp nhất** để gộp. Đây là lựa chọn tham lam — luôn gộp cặp nhỏ nhất trước — và nó dẫn đến bộ mã tối ưu nhờ priority queue (xem lại [Binary Heap](../03-trees-and-heaps/05-binary-heap.md)).

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

## Bài toán Greedy kinh điển khác

### Jump Game (LeetCode #55)

Cho mảng `nums`, mỗi phần tử cho biết bạn có thể nhảy tối đa bao xa từ vị trí đó. Hỏi: có nhảy được tới cuối mảng không?

**Ẩn dụ:** Tưởng tượng bạn nhảy qua các phiến đá trên sông. Mỗi phiến đá ghi số bước tối đa bạn có thể nhảy. Bạn chỉ cần biết: "tầm xa nhất mình với tới được là bao nhiêu?"

**Greedy:** Duyệt qua mảng, luôn cập nhật **vị trí xa nhất có thể tới** (max_reach).

```
nums = [2, 3, 1, 1, 4]

i=0: nums[0]=2, max_reach = max(0, 0+2) = 2
i=1: nums[1]=3, max_reach = max(2, 1+3) = 4   (4 >= 4, tới cuối!)
→ true

nums = [3, 2, 1, 0, 4]

i=0: nums[0]=3, max_reach = max(0, 0+3) = 3
i=1: nums[1]=2, max_reach = max(3, 1+2) = 3
i=2: nums[2]=1, max_reach = max(3, 2+1) = 3
i=3: nums[3]=0, max_reach = max(3, 3+0) = 3   (3 < 4, không tới cuối!)
i=4: i=4 > max_reach=3, không thể tới i=4
→ false
```

```rust
/// Jump Game — can you reach the last index?
pub fn can_jump(nums: &[i32]) -> bool {
    let mut max_reach: usize = 0;
    for (i, &val) in nums.iter().enumerate() {
        if i > max_reach {
            return false; // Không thể tới vị trí i
        }
        max_reach = max_reach.max(i + val as usize);
    }
    true
}
```

### Jump Game II (LeetCode #45)

Cùng bài, nhưng bây giờ hỏi: **ít nhất bao nhiêu bước** để tới cuối?

**Greedy:** Ở mỗi "vòng nhảy", tìm vị trí xa nhất có thể tới. Khi hết tầm của vòng hiện tại, bắt đầu vòng mới.

```
nums = [2, 3, 1, 1, 4]

Vòng 1: Từ i=0, tầm = [0..2]
  i=0: max_reach = 0+2 = 2
  i=1: max_reach = 1+3 = 4     ← hết tầm vòng 1 (i == end=2? chưa)
  i=2: max_reach = max(4,2+1) = 4  ← hết tầm vòng 1 (i==end=2)
  → jumps = 1, end = 4

Vòng 2: Tầm mới = [3..4]
  i=3: max_reach = max(4,3+1) = 4
  → nhưng max_reach=4 >= last index → tới rồi!
  → jumps = 2

Đáp án: 2 bước
```

```rust
/// Jump Game II — minimum jumps to reach the end.
pub fn jump(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n <= 1 { return 0; }

    let mut jumps = 0;
    let mut current_end = 0;   // Tầm xa nhất của vòng nhảy hiện tại
    let mut farthest = 0;      // Tầm xa nhất tìm được

    for i in 0..n - 1 {
        farthest = farthest.max(i + nums[i] as usize);
        if i == current_end {
            jumps += 1;
            current_end = farthest;
            if current_end >= n - 1 { break; }
        }
    }
    jumps
}
```

### Task Scheduler (LeetCode #621)

Cho danh sách task (ký tự A-Z) và `cooldown` n. Cùng một task phải cách nhau ít nhất n intervals. Tìm **số intervals ít nhất** để hoàn thành hết.

**Ẩn dụ:** Bạn nướng bánh. Mỗi lần nướng loại bánh A xong, phải đợi lò nguội n phút trước khi nướng lại bánh A. Trong lúc đợi, bạn có thể nướng bánh khác.

**Greedy:** Task xuất hiện nhiều nhất quyết định tổng thời gian. Xếp task có tần suất cao nhất trước.

```
tasks = [A,A,A,B,B,B], n = 2

Task nhiều nhất: A=3, B=3  (có 2 task cùng tần suất max)
max_freq = 3, count_max = 2 (cả A và B đều xuất hiện 3 lần)

Xếp lịch:
  A B _ | A B _ | A B
  ^---^ idle    ^---^

Công thức: max((max_freq - 1) * (n + 1) + count_max, total_tasks)
         = max((3 - 1) * (2 + 1) + 2, 6)
         = max(8, 6) = 8
```

```rust
/// Task Scheduler — minimum intervals to finish all tasks.
pub fn least_interval(tasks: &[char], n: i32) -> i32 {
    let mut freq = [0i32; 26];
    for &t in tasks {
        freq[(t as u8 - b'A') as usize] += 1;
    }

    let max_freq = *freq.iter().max().unwrap();
    let count_max = freq.iter().filter(|&&f| f == max_freq).count() as i32;

    let result = (max_freq - 1) * (n + 1) + count_max;
    result.max(tasks.len() as i32)
}
```

### Minimum Number of Platforms

Cho danh sách thời gian đến và đi của các tàu. Tìm **số sân ga ít nhất** cần thiết để không tàu nào phải chờ.

**Ẩn dụ:** Giống bãi đỗ xe — xe vào, xe ra, bạn cần biết lúc cao điểm có bao nhiêu xe trong bãi cùng lúc.

**Greedy:** Sắp xếp thời gian đến và đi riêng biệt. Duyệt song song kiểu merge.

```
Arrivals:   [9:00, 9:40, 9:50, 11:00, 15:00, 18:00]
Departures: [9:10, 12:00, 11:20, 11:30, 19:00, 20:00]

Sau khi sắp xếp:
  arr: [9:00, 9:40, 9:50, 11:00, 15:00, 18:00]
  dep: [9:10, 11:20, 11:30, 12:00, 19:00, 20:00]

  9:00  → tàu đến    platforms = 1,  max = 1
  9:10  → tàu đi     platforms = 0
  9:40  → tàu đến    platforms = 1
  9:50  → tàu đến    platforms = 2,  max = 2
  11:00 → tàu đến    platforms = 3,  max = 3  ← cao điểm!
  11:20 → tàu đi     platforms = 2
  ...

Đáp án: 3 sân ga
```

```rust
/// Minimum platforms needed for a train station.
pub fn min_platforms(arrivals: &mut [i32], departures: &mut [i32]) -> i32 {
    arrivals.sort();
    departures.sort();

    let mut platforms = 0;
    let mut max_platforms = 0;
    let (mut i, mut j) = (0, 0);

    while i < arrivals.len() {
        if arrivals[i] <= departures[j] {
            platforms += 1;          // Tàu đến
            max_platforms = max_platforms.max(platforms);
            i += 1;
        } else {
            platforms -= 1;          // Tàu đi
            j += 1;
        }
    }
    max_platforms
}
```

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

## Greedy vs DP — khi nào dùng gì?

Đây là câu hỏi quan trọng nhất. Greedy và DP đều cần **optimal substructure**, nhưng khác nhau ở cách tiếp cận:

```
Greedy:  Chọn 1 con đường → đi luôn → không quay lại
DP:      Thử TẤT CẢ con đường → chọn đường tốt nhất

Greedy = "Tôi biết đường nào tốt nhất ngay bây giờ"
DP     = "Tôi không biết, phải thử hết mới biết"
```

| | Greedy | DP |
|--|--------|-----|
| Tốc độ | Nhanh hơn | Chậm hơn |
| Luôn tối ưu? | Không (chỉ khi thỏa 2 tính chất) | Co |
| Code | don gian | Phuc tap hon |
| Cần proof? | **Co** (exchange / stays ahead) | Khong (chỉ cần recurrence) |
| Ví dụ đúng | Activity Selection, Fractional Knapsack | 0/1 Knapsack, Edit Distance |
| Overlapping subproblems? | Khong can | **Can** (đây là lý do cần memo) |

### Bảng "Khi nào dùng gì?"

| Dấu hiệu | Dùng gì |
|-----------|---------|
| Chọn tốt nhất tại chỗ luôn cho kết quả tối ưu | **Greedy** |
| Cần thử nhiều cách rồi chọn tối ưu | **DP** |
| Bài có overlapping subproblems | **DP** |
| Bài yêu cầu "tối thiểu số bước" và mỗi bước có lựa chọn rõ ràng | **Greedy** (nhưng kiểm tra counter-example!) |
| Lấy một phần được (fractional) | **Greedy** |
| Phải lấy nguyên (0/1) | Thường **DP** |
| Sắp xếp + duyệt 1 lần là xong | **Greedy** |
| Cần bảng 2D/1D để lưu kết quả trung gian | **DP** |

---

## Pitfalls — Bẫy thường gặp

### Bẫy 1: Dùng Greedy cho 0/1 Knapsack

- **Sai:** Sắp theo ratio, lấy từ cao nhất xuống (như Fractional Knapsack)
- **Dung:** Dùng DP vì không thể lấy "một phần" đồ vật
- **Tai sao:**

```
Items: (w=10, v=60), (w=20, v=100), (w=30, v=120)
Capacity = 50

Greedy (theo ratio): lấy item 1 (ratio=6) + item 2 (ratio=5)
  → w=30, v=160
  → còn 20 capacity nhưng item 3 nặng 30 → bỏ

Tối ưu thật: lấy item 2 + item 3
  → w=50, v=220 ← tốt hơn nhiều!

Greedy cho 160, nhưng đáp án đúng là 220!
```

### Bẫy 2: Greedy mà không chứng minh

- **Sai:** "Trông có vẻ Greedy được" → code luôn
- **Dung:** Dành 2 phút tìm counter-example trước khi code
- **Tai sao:** Rất nhiều bài "trông giống Greedy" nhưng thật ra cần DP hoặc cách khác. Thói quen tốt: nghĩ 1 phút, thử 2-3 test case nhỏ bằng tay.

### Bẫy 3: Sai tiêu chí Greedy

- **Sai:** Activity Selection — chọn hoạt động **ngắn nhất** (thời lượng ít nhất)
- **Dung:** Chọn hoạt động **kết thúc sớm nhất**
- **Tai sao:**

```
Chọn theo thời lượng ngắn nhất:

A: [0, 2)    thời lượng = 2
B: [1, 3)    thời lượng = 2
C: [2, 4)    thời lượng = 2

Chọn A → bỏ B (chồng chéo) → chọn C → 2 hoạt động
Nhưng {A, C} đã tối ưu rồi...

Thử lại với:
A: [0, 10)    thời lượng = 10
B: [1, 2)     thời lượng = 1   ← ngắn nhất
C: [3, 4)     thời lượng = 1
D: [5, 6)     thời lượng = 1

Chọn theo thời lượng: B, C, D → 3  ← đúng
Chọn A: chỉ được 1

OK, nhưng thử:
A: [0, 4)     thời lượng = 4
B: [1, 2)     thời lượng = 1   ← ngắn nhất
C: [3, 5)     thời lượng = 2
D: [2, 8)     thời lượng = 6

Chọn B (ngắn nhất) → bỏ A, D → chọn C → 2
Chọn kết thúc sớm nhất: B [1,2) → C [3,5) → 2
Tối ưu: {B, C} = 2. Cả hai đúng...

Counter-example thật cho "chọn ngắn nhất":
A: [0, 3)   thời lượng = 3
B: [2, 5)   thời lượng = 3
C: [1, 2)   thời lượng = 1   ← ngắn nhất
D: [4, 6)   thời lượng = 2

Chọn ngắn nhất: C [1,2) → D [4,6) → 2
Chọn kết thúc sớm: C [1,2) → A [0,3)? Không, A chồng C.
  Sắp theo kết thúc: C[1,2), A[0,3), B[2,5), D[4,6)
  Chọn C → A (0<2, bỏ) → B (2>=2, chọn) → D (4<5, bỏ) → 2

Hmm, cả hai cho 2. Nhưng "kết thúc sớm nhất" LUÔN đúng
(đã chứng minh), còn "ngắn nhất" có thể sai trong trường hợp khác.
```

Bài học: **tiêu chí Greedy quan trọng hơn việc "có dùng Greedy không"**. Chọn sai tiêu chí = sai kết quả.

---

## Độ phức tạp

| Thuật toán | Thời gian | Bộ nhớ |
|-----------|----------|--------|
| Activity Selection | O(n log n) | O(n) |
| Fractional Knapsack | O(n log n) | O(n) |
| Huffman Encoding | O(n log n) | O(n) |
| Jump Game | O(n) | O(1) |
| Jump Game II | O(n) | O(1) |
| Task Scheduler | O(n) | O(1) (26 ký tự) |
| Minimum Platforms | O(n log n) | O(1) |

**Giải thích thực tế:**

- Chi phí chủ yếu là **sắp xếp** (hoặc thao tác priority queue). Phần Greedy chỉ duyệt 1 lần O(n).
- So với DP: Greedy nhanh hơn nhiều. Activity Selection bằng DP sẽ tốn O(n^2), bằng Greedy chỉ O(n log n).
- Jump Game không cần sort → O(n) thuần. Đây là Greedy ở dạng đẹp nhất.

---

## Rust Ecosystem — Greedy trong thực tế

Greedy không chỉ là lý thuyết. Bạn gặp nó **khắp nơi** trong Rust ecosystem:

**1. Allocators**: Memory allocator dùng "first fit" hoặc "best fit" — cả hai đều là chiến lược Greedy. `jemalloc` (default allocator cũ của Rust) dùng best-fit để chọn vùng nhớ.

**2. Regex engine**: Rust's `regex` crate dùng **greedy matching** mặc định — `.*` match nhiều nhất có thể. Muốn non-greedy (lazy) thì dùng `.*?`.

```rust
// Greedy: .* lấy nhiều nhất có thể
// "abc123def" với regex r"a(.*)f" → captures "bc123de"

// Non-greedy: .*? lấy ít nhất có thể
// "abc123def" với regex r"a(.*?)f" → captures "bc123de" (cùng vì chỉ 1 f)
```

**3. Cargo dependency resolution**: Khi giải quyết dependencies, Cargo dùng chiến lược Greedy-like — chọn version mới nhất compatible trước, rồi backtrack nếu conflict.

**4. `Iterator` combinators**: Nhiều method trên iterator có tính Greedy:
- `take_while()` — lấy liên tục cho đến khi điều kiện sai, không quay lại
- `max()`, `min()` — duyệt 1 lần, luôn giữ giá trị tốt nhất
- `fold()` — tích lũy quyết định qua từng phần tử

**5. `BinaryHeap` cho Huffman**: Rust cung cấp `std::collections::BinaryHeap` (max-heap). Để dùng như min-heap cho Huffman, ta đảo `Ord`:

```rust
// Trong src/greedy.rs — đảo ordering để BinaryHeap thành min-heap
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq().cmp(&self.freq())  // Đảo ngược!
    }
}
```

Đây là pattern phổ biến trong Rust vì standard library chỉ có max-heap. Bạn sẽ gặp lại pattern này ở [Dijkstra](../05-graphs/04-dijkstra.md) — cũng cần min-heap cho priority queue.

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

---

## Luyện tập

| Bài | Gợi ý | Độ khó |
|-----|-------|--------|
| [Jump Game #55](https://leetcode.com/problems/jump-game/) | Track max_reach, duyệt từ trái → phải | Medium |
| [Jump Game II #45](https://leetcode.com/problems/jump-game-ii/) | BFS-level approach: đếm "vòng nhảy" | Medium |
| [Task Scheduler #621](https://leetcode.com/problems/task-scheduler/) | Đếm max frequency, tính idle slots | Medium |
| [Minimum Number of Platforms](https://www.geeksforgeeks.org/minimum-number-platforms-required-railwaybus-station/) | Sort arrivals + departures riêng, merge-scan | Medium |
| [Assign Cookies #455](https://leetcode.com/problems/assign-cookies/) | Sort cả 2 mảng, two pointers Greedy | Easy |
| [Non-overlapping Intervals #435](https://leetcode.com/problems/non-overlapping-intervals/) | Giống Activity Selection đảo ngược | Medium |
| [Gas Station #134](https://leetcode.com/problems/gas-station/) | Track surplus, nếu total >= 0 thì có đáp án | Medium |

**Mẹo chung:** Khi gặp bài Greedy trên LeetCode, hãy:
1. Tìm tiêu chí Greedy (sort theo gì? chọn gì trước?)
2. Tìm counter-example (2 phút)
3. Nếu không tìm được counter-example → code Greedy
4. Nếu Greedy sai → chuyển sang DP

---

## Chương tiếp theo

Bạn vừa học Greedy — chọn tốt nhất tại mỗi bước, không quay lại. Nhưng nhiều bài **không thể** giải bằng Greedy vì lựa chọn tại chỗ không đảm bảo tối ưu toàn cục.

Vậy khi Greedy thất bại, ta làm gì? Ta **thử tất cả** — nhưng thông minh. Ta lưu kết quả các bài con đã giải, không tính lại. Đó chính là [Dynamic Programming](13-dynamic-programming.md) — chương tiếp theo, và cũng là một trong những kỹ thuật mạnh nhất trong DSA.

> **Preview:** DP sẽ giải được 0/1 Knapsack (bài mà Greedy thất bại), Edit Distance, Longest Common Subsequence, và nhiều bài kinh điển khác. Nếu Greedy là "chọn nhanh, đi luôn", thì DP là "thử hết, nhớ hết, chọn tốt nhất".

---

[← Divide & Conquer](./11-divide-and-conquer.md) | [Dynamic Programming →](./13-dynamic-programming.md)
