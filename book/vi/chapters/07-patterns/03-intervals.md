# Intervals — Bài toán khoảng thời gian

> 💡 **Đừng lo lắng:** Nếu bạn thấy bài nào có từ "interval", "meeting", "schedule", "booking" — đừng hoảng. Sort by start, sweep left to right. Hầu hết interval problems follow pattern này. Thật sự chỉ có vậy. Bạn đã biết sort (Phần 6), đã biết greedy (Phần 6). Interval chỉ là **kết hợp 2 thứ đó**. Không có thuật toán mới nào cả.

---

## Đây là gì?

Tưởng tượng bạn là **quản lý phòng họp** ở một công ty. Mỗi cuộc họp có thời gian bắt đầu và kết thúc. Mọi người gửi lịch họp lung tung — có cuộc chồng chéo nhau, có cuộc cách xa nhau. Nhiệm vụ của bạn:

- Gộp những cuộc họp chồng nhau thành 1 block
- Thêm cuộc họp mới vào lịch
- Kiểm tra xem có ai bị trùng lịch không
- Tính xem cần **ít nhất bao nhiêu phòng** để tất cả cuộc họp diễn ra đồng thời

Trong lập trình, **interval** = `[start, end]` — một khoảng từ điểm bắt đầu đến điểm kết thúc. Bài toán interval xuất hiện cực nhiều: lịch biểu, booking hệ thống, merge log files, xử lý time-series data...

---

## Sweep Line — kỹ thuật tổng quát

Trước khi đi vào từng bài, mình nói về **Sweep Line** (đường quét) — framework chung cho hầu hết bài interval.

**Hình dung thế này:** Bạn cầm một cây thước dọc, đặt ở bên trái nhất của trục thời gian, rồi **kéo từ trái sang phải**. Mỗi lần thước chạm vào một điểm bắt đầu hoặc kết thúc, bạn xử lý gì đó.

```
Thước quét từ trái → phải:

   |
   |  [====]     [========]
   |     [====]       [==]
   |
   ──────────────────────────→ thời gian
   ↑
   thước
```

**3 bước của Sweep Line:**

1. **Thu thập events** — mỗi interval tạo 2 events: start và end
2. **Sort events** theo thời gian
3. **Quét từ trái sang phải** — xử lý từng event

Cụ thể hơn, mình sẽ demo kỹ thuật này ở bài Meeting Rooms bên dưới.

---

## Merge Intervals — gộp khoảng chồng nhau

### Ý tưởng

Cho danh sách intervals, gộp tất cả khoảng chồng nhau thành một.

**Ví dụ thực tế:** Bạn có 4 cuộc họp. Cuộc nào chồng giờ thì gộp thành 1 block liên tục.

### Cách hoạt động

**Bước 1:** Sort theo start time.

**Bước 2:** Duyệt từ trái sang phải. Nếu interval hiện tại chồng với interval trước (start <= end trước), thì mở rộng interval trước. Nếu không, thêm interval mới.

```
Đầu vào (chưa sort): [8,10] [1,3] [2,6] [15,18]

Sau khi sort theo start:
  [1,3]  [2,6]  [8,10]  [15,18]

Duyệt:
  Lấy [1,3]

  [2,6]: start=2 <= end=3 → chồng! → mở rộng thành [1,6]

  Timeline:
  1---3
    2------6
  ==========
  1--------6

  [8,10]: start=8 > end=6 → không chồng → thêm mới

  [15,18]: start=15 > end=10 → không chồng → thêm mới

Kết quả: [1,6] [8,10] [15,18]
```

### Code Rust

```rust
pub fn merge_intervals(intervals: &mut Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    if intervals.is_empty() {
        return vec![];
    }
    intervals.sort_by_key(|iv| iv[0]);

    let mut merged: Vec<[i32; 2]> = vec![intervals[0]];

    for iv in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if iv[0] <= last[1] {
            // Chồng nhau → mở rộng end
            last[1] = last[1].max(iv[1]);
        } else {
            // Không chồng → thêm mới
            merged.push(*iv);
        }
    }
    merged
}
```

---

## Insert Interval — chèn interval mới

### Ý tưởng

Cho danh sách intervals **đã sort và không chồng nhau**. Chèn 1 interval mới, merge nếu cần.

**Ví dụ thực tế:** Lịch họp đã sắp xếp gọn gàng, giờ sếp thêm 1 cuộc họp mới. Bạn phải cập nhật lại lịch.

### Cách hoạt động

Chia thành 3 giai đoạn:
1. **Trước:** thêm tất cả interval kết thúc trước khi interval mới bắt đầu.
2. **Merge:** gộp tất cả interval chồng với interval mới.
3. **Sau:** thêm tất cả interval còn lại.

```
Intervals: [1,2] [3,5] [6,7] [8,10] [12,16]
New:       [4,8]

Trước: [1,2] (end=2 < start=4, OK)

Merge:
  [3,5]: start=3 <= end=8 → gộp → new = [3,8]
  [6,7]: start=6 <= end=8 → gộp → new = [3,8]
  [8,10]: start=8 <= end=8 → gộp → new = [3,10]

Sau: [12,16]

Timeline:
  1-2  3---5 6-7 8---10   12----16
           [4--------8]
  ================================
  1-2  [3---------10]     12----16

Kết quả: [1,2] [3,10] [12,16]
```

### Code Rust

```rust
pub fn insert_interval(intervals: &[[i32; 2]], new: [i32; 2]) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    let mut new = new;
    let mut i = 0;
    let n = intervals.len();

    // Giai đoạn 1: thêm tất cả trước new
    while i < n && intervals[i][1] < new[0] {
        result.push(intervals[i]);
        i += 1;
    }

    // Giai đoạn 2: merge các interval chồng nhau
    while i < n && intervals[i][0] <= new[1] {
        new[0] = new[0].min(intervals[i][0]);
        new[1] = new[1].max(intervals[i][1]);
        i += 1;
    }
    result.push(new);

    // Giai đoạn 3: thêm phần còn lại
    while i < n {
        result.push(intervals[i]);
        i += 1;
    }
    result
}
```

---

## Has Overlap — kiểm tra chồng chéo

### Ý tưởng

Kiểm tra xem có bất kỳ 2 interval nào chồng nhau không. Bài này hay gặp tên "Meeting Rooms I" — liệu 1 người có thể tham dự tất cả cuộc họp không?

**Chiến lược:** Sort theo start time, rồi kiểm tra từng cặp liên tiếp.

```
Intervals (đã sort): [1,3] [3,5] [5,8]

  [1,3]: end=3, next start=3 → 3 > 3? Không → OK
  [3,5]: end=5, next start=5 → 5 > 5? Không → OK

  Không chồng! Bạn đi họp hết được.

---

Intervals (đã sort): [1,5] [3,7]

  [1,5]: end=5, next start=3 → 5 > 3? CÓ → Chồng!

  Bạn không thể đi cả 2 cuộc họp.
```

> **Lưu ý:** `end == next_start` (ví dụ [1,3] và [3,5]) **không** coi là chồng. Cuộc họp kết thúc lúc 3h, cuộc tiếp theo bắt đầu lúc 3h — vẫn kịp!

---

## Min Meeting Rooms — số phòng họp tối thiểu

### Ý tưởng

Đây là bài "Meeting Rooms II" kinh điển. Bạn cần tìm **số phòng ít nhất** để tất cả cuộc họp diễn ra mà không xung đột.

**Ví dụ thực tế:** 3 cuộc họp [0,30], [5,10], [15,20]. Cuộc [5,10] chồng với [0,30], nên cần phòng riêng. Cuộc [15,20] cũng chồng với [0,30], nhưng [5,10] đã kết thúc nên tái sử dụng phòng đó.

### Cách 1 — Sort + Min-Heap

**Bước 1:** Sort theo start time.

**Bước 2:** Dùng **min-heap** lưu end time của các phòng đang dùng.

**Bước 3:** Với mỗi cuộc họp:
- Nếu phòng kết thúc sớm nhất đã rảnh (end <= start mới) → tái sử dụng
- Nếu không → mở phòng mới

```
Meetings (sorted): [0,30] [5,10] [15,20]

Bước 1: [0,30] → heap = {30}        → 1 phòng
Bước 2: [5,10] → 30 > 5 (chưa rảnh) → heap = {10, 30} → 2 phòng
Bước 3: [15,20] → 10 <= 15 (rảnh!)  → pop 10, push 20
                                      → heap = {20, 30} → vẫn 2 phòng

Timeline:
  Phòng A: |============================| [0,30]
  Phòng B: |====|          |=====|
           [5,10]          [15,20] (tái sử dụng phòng B)

Đáp án: 2 phòng
```

### Code Rust — Min-Heap

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn min_meeting_rooms(intervals: &[[i32; 2]]) -> usize {
    if intervals.is_empty() {
        return 0;
    }

    let mut sorted: Vec<[i32; 2]> = intervals.to_vec();
    sorted.sort_by_key(|iv| iv[0]);

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for iv in &sorted {
        if let Some(&Reverse(earliest_end)) = heap.peek() {
            if earliest_end <= iv[0] {
                heap.pop(); // Tái sử dụng phòng
            }
        }
        heap.push(Reverse(iv[1]));
    }

    heap.len()
}
```

### Cách 2 — Event-based Sweep Line

Có một cách khác rất trực quan: **biến mỗi interval thành 2 events**.

Hình dung bạn đứng ngoài cửa công ty đếm người ra vào:
- Mỗi khi có cuộc họp **bắt đầu** → +1 người vào (cần thêm phòng)
- Mỗi khi có cuộc họp **kết thúc** → -1 người ra (giải phóng phòng)
- **Số người nhiều nhất cùng lúc** = số phòng cần

```
Meetings: [0,30] [5,10] [15,20]

Events:
  t=0:  +1 (start)    → đang dùng: 1
  t=5:  +1 (start)    → đang dùng: 2  ← MAX
  t=10: -1 (end)      → đang dùng: 1
  t=15: +1 (start)    → đang dùng: 2  ← MAX
  t=20: -1 (end)      → đang dùng: 1
  t=30: -1 (end)      → đang dùng: 0

Max = 2 phòng
```

> **Lưu ý khi sort events:** Nếu 2 events cùng thời điểm, xử lý **end trước start**. Tại sao? Vì cuộc họp kết thúc lúc 10h giải phóng phòng TRƯỚC khi cuộc mới lúc 10h chiếm phòng.

```rust
pub fn min_meeting_rooms_sweep(intervals: &[[i32; 2]]) -> usize {
    let mut events: Vec<(i32, i32)> = Vec::new();

    for iv in intervals {
        events.push((iv[0], 1));   // +1 khi bắt đầu
        events.push((iv[1], -1));  // -1 khi kết thúc
    }

    // Sort theo thời gian. Nếu cùng thời điểm, end (-1) trước start (+1)
    events.sort();

    let mut current_rooms = 0i32;
    let mut max_rooms = 0i32;

    for (_time, delta) in &events {
        current_rooms += delta;
        max_rooms = max_rooms.max(current_rooms);
    }

    max_rooms as usize
}
```

**Khi nào dùng cách nào?**
- **Min-Heap**: khi cần biết cụ thể phòng nào đang dùng, hoặc cần gán cuộc họp vào phòng cụ thể
- **Event-based**: khi chỉ cần đếm số lượng — code đơn giản hơn, dễ hiểu hơn

---

## Interval Intersection — giao của 2 danh sách interval

### Ý tưởng

Cho 2 danh sách intervals (đã sort, không chồng nội bộ). Tìm tất cả phần giao nhau.

**Ví dụ thực tế:** Bạn và đồng nghiệp có lịch rảnh khác nhau. Tìm tất cả khung giờ mà **cả hai cùng rảnh**.

### Cách hoạt động — Two Pointers

Dùng 2 con trỏ, mỗi con trỏ duyệt 1 danh sách. Tại mỗi bước:
- Tính phần giao: `lo = max(a_start, b_start)`, `hi = min(a_end, b_end)`
- Nếu `lo <= hi` → có giao, thêm `[lo, hi]`
- Di chuyển con trỏ có end nhỏ hơn

```
A: [0,2]     [5,10]         [13,23]   [24,25]
B:    [1,5]      [8,12]  [15,24]         [25,26]

i=0,j=0: A=[0,2] B=[1,5] → lo=1, hi=2 → [1,2]. A kết thúc trước → i++
i=1,j=0: A=[5,10] B=[1,5] → lo=5, hi=5 → [5,5]. B kết thúc trước → j++
i=1,j=1: A=[5,10] B=[8,12] → lo=8, hi=10 → [8,10]. A kết thúc trước → i++
i=2,j=1: A=[13,23] B=[8,12] → lo=13, hi=12 → Không giao. B kết thúc trước → j++
i=2,j=2: A=[13,23] B=[15,24] → lo=15, hi=23 → [15,23]. A kết thúc trước → i++
i=3,j=2: A=[24,25] B=[15,24] → lo=24, hi=24 → [24,24]. B kết thúc trước → j++
i=3,j=3: A=[24,25] B=[25,26] → lo=25, hi=25 → [25,25]. A kết thúc trước → i++

Kết quả: [1,2] [5,5] [8,10] [15,23] [24,24] [25,25]
```

### Code Rust

```rust
pub fn interval_intersection(a: &[[i32; 2]], b: &[[i32; 2]]) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        let lo = a[i][0].max(b[j][0]);
        let hi = a[i][1].min(b[j][1]);

        if lo <= hi {
            result.push([lo, hi]);
        }

        if a[i][1] < b[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    result
}
```

---

## Pitfalls — Những lỗi hay mắc

### 1. Quên sort trước khi xử lý

❌ **Sai:** Duyệt intervals mà không sort → merge sai, bỏ sót overlap.

```rust
// SAI — không sort
let mut merged = vec![intervals[0]];
for iv in &intervals[1..] { /* ... */ }
```

✅ **Đúng:** Luôn sort theo start trước.

```rust
intervals.sort_by_key(|iv| iv[0]);
let mut merged = vec![intervals[0]];
for iv in &intervals[1..] { /* ... */ }
```

💡 **Tại sao:** Không sort thì `[8,10]` có thể đứng trước `[1,3]`. Khi duyệt từ trái sang phải, bạn sẽ không phát hiện `[1,3]` chồng với `[2,6]` vì `[8,10]` chen giữa. Sort đảm bảo mọi interval chồng nhau sẽ **nằm cạnh nhau**.

### 2. Nhầm inclusive vs exclusive boundaries

❌ **Sai:** Coi `[1,3]` và `[3,5]` là chồng nhau trong bài Meeting Rooms.

✅ **Đúng:** Đọc kỹ đề. Trong Meeting Rooms, `end == next_start` thường **không chồng** (kịp chạy sang phòng khác). Trong Merge Intervals, `end == next_start` thường **cần merge** (liền kề thì gộp).

💡 **Tại sao:** Tùy vào bài toán. Cuộc họp 9h-10h và 10h-11h: bạn vẫn đi được cả hai (không chồng). Nhưng khoảng `[1,3]` và `[3,5]` trên trục số thì liền nhau nên gộp thành `[1,5]` hợp lý hơn. **Luôn kiểm tra: dùng `<` hay `<=`?**

### 3. Intervals "chạm nhau" (touching)

❌ **Sai:** Xử lý `[1,5]` và `[5,10]` khác nhau tùy hứng.

✅ **Đúng:** Quyết định rõ ràng ngay từ đầu — touching intervals có merge không?

```
[1,5] [5,10]
      ^ chạm nhau tại 5

Merge Intervals → gộp thành [1,10] (dùng <=)
Has Overlap     → KHÔNG chồng (dùng <)
```

💡 **Tại sao:** Đây là edge case kinh điển trong phỏng vấn. Interviewer hay test điểm này. Chú ý: `iv[0] <= last[1]` (merge) vs `iv[0] < last[1]` (overlap) — chỉ khác dấu `=` nhưng kết quả khác hoàn toàn.

### 4. Mở rộng sai chiều khi merge

❌ **Sai:** Khi merge, chỉ lấy `end` của interval mới.

```rust
// SAI
if iv[0] <= last[1] {
    last[1] = iv[1]; // Nếu iv[1] < last[1] thì bị thu hẹp!
}
```

✅ **Đúng:** Lấy `max` của cả hai end.

```rust
// ĐÚNG
if iv[0] <= last[1] {
    last[1] = last[1].max(iv[1]);
}
```

💡 **Tại sao:** Interval `[1,10]` chồng với `[2,5]`. Nếu ghi đè `last[1] = 5`, bạn thu hẹp từ `[1,10]` xuống `[1,5]` — mất data. Phải lấy max.

---

## Khi nào dùng gì?

| Bạn cần... | Dùng technique | Ví dụ bài |
|---|---|---|
| Gộp intervals chồng nhau | Sort + duyệt tuyến tính | Merge Intervals |
| Chèn interval vào list đã sort | 3 giai đoạn (trước/merge/sau) | Insert Interval |
| Kiểm tra có overlap không | Sort + so sánh cặp liên tiếp | Meeting Rooms I |
| Đếm max overlap cùng lúc | Event-based sweep hoặc min-heap | Meeting Rooms II |
| Tìm giao 2 danh sách | Two pointers | Interval Intersection |
| Loại bỏ ít interval nhất để hết overlap | Sort by **end** + greedy | Non-overlapping Intervals |

> **Mẹo nhỏ:** Đa số bài sort by start. Nhưng bài "loại bỏ interval" (Non-overlapping Intervals) thì sort by **end** lại hiệu quả hơn — vì bạn muốn giữ interval kết thúc sớm nhất để dành chỗ cho nhiều interval hơn.

---

## Bảng độ phức tạp

| Bài toán | Time | Space | Kỹ thuật chính |
|---|---|---|---|
| Merge Intervals | O(n log n) | O(n) | Sort + duyệt |
| Insert Interval | O(n) | O(n) | 3 giai đoạn |
| Has Overlap | O(n log n) | O(1) | Sort + so sánh cặp |
| Min Meeting Rooms | O(n log n) | O(n) | Sort + min-heap / sweep |
| Interval Intersection | O(m + n) | O(m + n) | Two pointers |

---

## Rust Ecosystem

Trong thực tế, Rust có một số crate hữu ích khi làm việc với intervals:

- **`std::ops::Range`**: Rust built-in range `0..10` chính là interval. Dùng `.contains()` để check overlap.
- **`itertools`**: `.coalesce()` có thể dùng để merge intervals liên tiếp sau khi sort.
- **`std::collections::BTreeMap`**: Rất tiện cho event-based sweep — insert events theo thời gian, duyệt theo thứ tự tự nhiên.

```rust
use std::collections::BTreeMap;

// Event-based sweep dùng BTreeMap
fn max_overlap_btree(intervals: &[[i32; 2]]) -> i32 {
    let mut events: BTreeMap<i32, i32> = BTreeMap::new();

    for iv in intervals {
        *events.entry(iv[0]).or_insert(0) += 1;   // start: +1
        *events.entry(iv[1]).or_insert(0) -= 1;    // end: -1
    }

    let mut current = 0;
    let mut max_val = 0;
    for (_time, delta) in &events {
        current += delta;
        max_val = max_val.max(current);
    }
    max_val
}
```

> **Ghi chú:** `BTreeMap` tự sort theo key, nên bạn không cần sort thủ công. Nhưng cẩn thận: nếu start và end cùng thời điểm, `BTreeMap` gộp chúng vào 1 entry — nên cần xử lý `+1` và `-1` cùng lúc (cộng dồn delta). Cách này hoạt động đúng vì ta cộng trước rồi mới check max.

---

## Practice — Luyện tập

| # | Bài | Difficulty | Gợi ý |
|---|---|---|---|
| 56 | [Merge Intervals](https://leetcode.com/problems/merge-intervals/) | Medium | Sort + duyệt, template kinh điển |
| 435 | [Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/) | Medium | Sort by **end**, greedy giữ interval kết thúc sớm |
| 253 | [Meeting Rooms II](https://leetcode.com/problems/meeting-rooms-ii/) | Medium | Thử cả 2 cách: min-heap và event-based |
| 57 | [Insert Interval](https://leetcode.com/problems/insert-interval/) | Medium | 3 giai đoạn, cẩn thận boundary |
| 986 | [Interval List Intersections](https://leetcode.com/problems/interval-list-intersections/) | Medium | Two pointers, lo/hi formula |

**Thứ tự gợi ý:** 56 → 57 → 253 → 435 → 986. Bắt đầu từ Merge Intervals vì đó là nền tảng cho tất cả bài còn lại.

---

## Tổng kết

Interval là dạng bài rất phổ biến trong phỏng vấn. Pattern chung:

1. **Sort theo start** (hoặc end tùy bài)
2. **Duyệt tuyến tính** — so sánh interval hiện tại với cái trước
3. Dùng **heap** khi cần theo dõi nhiều "luồng" đồng thời (meeting rooms)
4. Dùng **two pointers** khi có 2 danh sách đã sort
5. Dùng **event-based sweep** khi cần đếm max overlap — đơn giản và trực quan

Hãy nhớ: hầu hết bài interval chỉ là **sorting + greedy** — không có gì phức tạp nếu bạn nắm vững 2 kỹ thuật đó!

---

[← Monotonic Stack](./02-monotonic-stack.md) | [Matrix Traversal →](./04-matrix-traversal.md)
