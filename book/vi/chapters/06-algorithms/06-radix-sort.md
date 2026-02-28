# Radix Sort

> 💡 **Đừng lo lắng:** Nếu bạn vừa đọc xong Heap Sort hay Merge Sort mà thấy mệt — bình tĩnh. Chương này **dễ hơn** bạn tưởng. Radix Sort không có đệ quy, không có "chia để trị", không có cây. Nó đơn giản như... sắp thư ở bưu điện. Thật đấy.

---

## Cầu nối từ chương trước

Ở các chương trước, chúng ta đã học Merge Sort (O(n log n)), Quick Sort (O(n log n) trung bình), Heap Sort (O(n log n)). Bạn có thấy một pattern không? Tất cả đều **O(n log n)**.

Có một định lý toán học chứng minh rằng: **bất kỳ thuật toán sort nào dựa trên so sánh đều không thể nhanh hơn O(n log n)**. Đó là giới hạn lý thuyết (information-theoretic lower bound).

Nhưng nếu ta **không so sánh** thì sao? Nếu thay vì hỏi "A > B?", ta phân loại theo từng chữ số? Đó là ý tưởng của **non-comparison sort** — và Radix Sort là ngôi sao của nhóm này.

```
Comparison sort:    "45 > 24?"  →  Swap  →  "45 > 75?"  →  ...
                    Mỗi phép so sánh chỉ cho 1 bit thông tin (yes/no)

Radix Sort:         Nhìn chữ số cuối: 5, 4, 5, 0, 2, 4, 2, 6
                    Bỏ vào ô tương ứng  →  Xong 1 lượt!
                    Mỗi lượt xử lý TOÀN BỘ n phần tử
```

> **Kết quả:** Radix Sort đạt **O(d * n)** — gần O(n) khi d (số chữ số) là hằng số. Vượt qua rào cản O(n log n)!

---

## Đây là gì?

Tưởng tượng bạn làm ở bưu điện và cần sắp xếp hàng ngàn lá thư theo **mã bưu chính** (zip code). Bạn không so sánh từng cặp thư với nhau. Thay vào đó, bạn:

1. Sắp xếp theo **chữ số cuối cùng** — bỏ thư vào 10 ô (0-9)
2. Gom lại, sắp theo **chữ số thứ hai từ cuối**
3. Gom lại, sắp theo **chữ số thứ ba từ cuối**
4. ... cho đến chữ số đầu tiên

Sau khi xử lý hết các chữ số, toàn bộ đống thư đã sắp xếp!

Đây là **Radix Sort** (sắp xếp theo cơ số) — một thuật toán **không dùng phép so sánh** (non-comparison sort). Thay vì hỏi "A lớn hơn B không?", nó phân loại theo từng chữ số.

> **Tại sao đặc biệt?** Mọi thuật toán sort dựa trên so sánh (Merge Sort, Quick Sort, ...) đều có giới hạn tối thiểu là O(n log n). Radix Sort phá vỡ giới hạn này vì nó không so sánh — đạt hiệu suất **gần O(n)** khi kích thước khóa (số chữ số) là hằng số.

---

## Gia đình Non-Comparison Sort

Trước khi đi sâu vào Radix Sort, hãy biết nó thuộc "gia đình" nào:

```
Non-Comparison Sort Family
├── Counting Sort    — Đếm số lần xuất hiện, suy ra vị trí
│                      Giới hạn: cần biết range của dữ liệu
├── Radix Sort       — Dùng Counting Sort làm bước con
│                      Sort theo từng digit, lặp d lần
└── Bucket Sort      — Chia dữ liệu vào các "xô" theo range
                       Rồi sort từng xô riêng
```

Điểm chung: tất cả đều **cần biết gì đó về dữ liệu** (range, số digit, phân bố). Đổi lại, chúng nhanh hơn O(n log n).

---

## Counting Sort — Building Block quan trọng

Trước khi hiểu Radix Sort, bạn **phải** hiểu Counting Sort vì nó là subroutine (bước con) bên trong Radix Sort.

### Ý tưởng

Bạn có 10 bạn học sinh, điểm từ 0-5. Thay vì so sánh từng cặp, bạn **đếm** xem có bao nhiêu bạn được 0 điểm, 1 điểm, ..., 5 điểm. Xong rồi xếp lại theo thứ tự.

### Trace chi tiết

Sắp xếp: `[4, 2, 2, 8, 3, 3, 1]` (range 0-9)

```
Bước 1: ĐẾM (Count)
  Giá trị: 0  1  2  3  4  5  6  7  8  9
  Số lần:  0  1  2  2  1  0  0  0  1  0
           ↑     ↑  ↑  ↑           ↑
          ko có  2x 2x 1x          1x

Bước 2: PREFIX SUM (Tổng tích lũy)
  count[i] = count[i] + count[i-1]

  Giá trị: 0  1  2  3  4  5  6  7  8  9
  Prefix:  0  1  3  5  6  6  6  6  7  7
                 ↑
                 "Có 3 phần tử ≤ 2"
                 → phần tử giá trị 2 sẽ nằm ở vị trí 2 (index = prefix - 1)

Bước 3: ĐẶT VÀO VỊ TRÍ (duyệt ngược để stable)
  Duyệt [4, 2, 2, 8, 3, 3, 1] từ phải sang trái:

  1 → prefix[1] = 1 → đặt ở index 0 → prefix[1] = 0
  3 → prefix[3] = 5 → đặt ở index 4 → prefix[3] = 4
  3 → prefix[3] = 4 → đặt ở index 3 → prefix[3] = 3
  8 → prefix[8] = 7 → đặt ở index 6 → prefix[8] = 6
  2 → prefix[2] = 3 → đặt ở index 2 → prefix[2] = 2
  2 → prefix[2] = 2 → đặt ở index 1 → prefix[2] = 1
  4 → prefix[4] = 6 → đặt ở index 5 → prefix[4] = 5

Kết quả: [1, 2, 2, 3, 3, 4, 8]  ✓
```

> **Tại sao duyệt ngược?** Để giữ tính **stable** — các phần tử bằng nhau giữ nguyên thứ tự ban đầu. Điều này **cực kỳ quan trọng** khi Counting Sort được dùng làm bước con trong Radix Sort.

---

## Hoạt động như thế nào?

### LSD Radix Sort (Least Significant Digit — từ phải sang trái)

Sắp xếp: `[170, 45, 75, 90, 802, 24, 2, 66]`

**Lượt 1 — chữ số hàng đơn vị:**

```
Phân loại vào các ô (bucket) theo chữ số cuối:
  Ô 0: [170, 90]
  Ô 2: [802, 2]
  Ô 4: [24]
  Ô 5: [45, 75]
  Ô 6: [66]

Gom lại: [170, 90, 802, 2, 24, 45, 75, 66]
```

**Lượt 2 — chữ số hàng chục:**

```
Phân loại theo chữ số hàng chục:
  Ô 0: [802, 2]        (802 -> 0, 2 -> 0)
  Ô 2: [24]            (24 -> 2)
  Ô 4: [45]            (45 -> 4)
  Ô 6: [66]            (66 -> 6)
  Ô 7: [170, 75]       (170 -> 7, 75 -> 7)
  Ô 9: [90]            (90 -> 9)

Gom lại: [802, 2, 24, 45, 66, 170, 75, 90]
```

**Lượt 3 — chữ số hàng trăm:**

```
Phân loại theo chữ số hàng trăm:
  Ô 0: [2, 24, 45, 66, 75, 90]   (không có hàng trăm = 0)
  Ô 1: [170]
  Ô 8: [802]

Gom lại: [2, 24, 45, 66, 75, 90, 170, 802]   <-- Đã sắp xếp!
```

### Tại sao phải sort từ phải sang trái?

```
Vì mỗi lượt sort là ỔN ĐỊNH (stable):

Sau lượt 1: thứ tự theo hàng đơn vị đúng
Sau lượt 2: thứ tự theo hàng chục đúng, MÀ vẫn giữ thứ tự hàng đơn vị
            (nhờ tính stable)
Sau lượt 3: thứ tự theo hàng trăm đúng, MÀ vẫn giữ thứ tự 2 chữ số cuối
            (nhờ tính stable của 2 lượt trước)

=> Cuối cùng, toàn bộ số đã đúng thứ tự!
```

> **Tính stable là xương sống của LSD Radix Sort.** Nếu bước con sort không stable, toàn bộ thuật toán sẽ sai. Đó là lý do ta dùng Counting Sort (stable) chứ không dùng Quick Sort (không stable).

---

## LSD vs MSD — Hai hướng tiếp cận

```
LSD (Least Significant Digit)        MSD (Most Significant Digit)
─────────────────────────────         ─────────────────────────────
Sort từ PHẢI sang TRÁI               Sort từ TRÁI sang PHẢI
(chữ số ít quan trọng trước)         (chữ số quan trọng nhất trước)

Xử lý TOÀN BỘ mảng mỗi lượt        Chia thành nhóm, sort ĐỆ QUY
                                      trong từng nhóm

Stable ✓                             Cần thêm xử lý để stable
Dễ implement ✓                       Phức tạp hơn

Phù hợp: số nguyên, key cố định     Phù hợp: string (khác độ dài)
Ví dụ: sort 10 triệu số u32         Ví dụ: sort tên người theo bảng chữ cái
```

### Khi nào dùng MSD?

Khi sort **strings có độ dài khác nhau**. MSD xử lý chữ cái đầu tiên trước — giống cách bạn tra từ điển. Những từ bắt đầu bằng "A" được nhóm lại, rồi sort theo chữ thứ 2 trong nhóm "A".

```
Ví dụ sort strings bằng MSD:
  ["cat", "car", "dog", "do", "cab"]

  Lượt 1 (ký tự 1): c: [cat, car, cab]    d: [dog, do]
  Lượt 2 (ký tự 2): ca: [cat, car, cab]   do: [dog, do]
  Lượt 3 (ký tự 3): cab, car, cat         do, dog

  Kết quả: ["cab", "car", "cat", "do", "dog"]
```

> Trong chương này, ta tập trung vào **LSD** vì nó phổ biến và dễ implement hơn.

---

## Tối ưu: dùng base 256 thay vì base 10

Trong code thực tế, thay vì xử lý từng chữ số (0-9), ta xử lý **từng byte** (0-255). Một số 32-bit có 4 byte, nên chỉ cần **đúng 4 lượt** bất kể số lớn cỡ nào.

```
Base 10:  số 1,000,000 cần 7 lượt  (7 chữ số)
Base 256: số 1,000,000 cần 4 lượt  (4 byte)
          số 4 tỷ      cũng cần 4 lượt!

Ngoài ra:
  (val / exp) % 256   có thể viết thành   (val >> shift) & 0xFF
  → CPU thực hiện bit shift cực nhanh so với phép chia
```

---

## Code Rust

```rust
pub fn radix_sort(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }
    let max_val = match arr.iter().max() {
        Some(&v) => v,
        None => return,
    };

    let mut exp: u32 = 1;
    let mut output = vec![0u32; arr.len()];

    // Lặp từng "chữ số" (base 256 = 1 byte)
    while max_val / exp > 0 {
        let mut count = [0usize; 256];

        // Bước 1: Đếm (Counting Sort - count phase)
        for &val in arr.iter() {
            let digit = ((val / exp) % 256) as usize;
            count[digit] += 1;
        }

        // Bước 2: Prefix sum — tính vị trí kết thúc của mỗi ô
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Bước 3: Đặt phần tử vào đúng vị trí (duyệt ngược để giữ stable)
        for &val in arr.iter().rev() {
            let digit = ((val / exp) % 256) as usize;
            count[digit] -= 1;
            output[count[digit]] = val;
        }

        // Copy kết quả về mảng gốc
        arr.copy_from_slice(&output);

        // Tránh tràn số khi nhân exp
        if exp > u32::MAX / 256 {
            break;
        }
        exp *= 256;
    }
}
```

**Ghi chú về Rust:**

- `copy_from_slice` nhanh hơn copy từng phần tử một vì nó dùng `memcpy` bên dưới.
- Duyệt ngược (`arr.iter().rev()`) khi đặt phần tử để giữ tính **stable**. Nếu duyệt xuôi, thứ tự của các phần tử có cùng chữ số sẽ bị đảo.
- Kiểm tra `exp > u32::MAX / 256` để tránh tràn số (overflow) khi nhân `exp * 256`.
- Thuật toán này chỉ hoạt động với `u32`. Để hỗ trợ số âm hay kiểu khác, cần xử lý thêm (xem phần Pitfalls bên dưới).

---

## Pitfalls — Những cái bẫy hay gặp

### 1. Quên tính stable

❌ **Sai:** Duyệt xuôi khi đặt phần tử vào output

```rust
// SAI — mất tính stable!
for &val in arr.iter() {
    let digit = ((val / exp) % 256) as usize;
    output[count[digit]] = val;
    count[digit] += 1;
}
```

✅ **Đúng:** Duyệt ngược + giảm count

```rust
for &val in arr.iter().rev() {
    let digit = ((val / exp) % 256) as usize;
    count[digit] -= 1;
    output[count[digit]] = val;
}
```

💡 **Tại sao:** Prefix sum cho ta vị trí **cuối** của mỗi nhóm. Duyệt ngược + giảm count = điền từ cuối lên đầu, giữ nguyên thứ tự tương đối. Nếu mất stable, lượt sau sẽ phá hỏng kết quả lượt trước.

### 2. Dùng cho số âm (signed integers)

❌ **Sai:** Áp dụng trực tiếp radix sort cho `i32`

```rust
// SAI — số âm có bit đầu = 1, sẽ bị xếp SAU số dương!
// -1 (0xFFFFFFFF) > 1 (0x00000001) trong unsigned
```

✅ **Đúng:** Lật bit dấu (flip sign bit) trước khi sort, rồi lật lại

```rust
// Trước sort: XOR bit cao nhất
let flipped: Vec<u32> = arr.iter().map(|&x| (x as u32) ^ 0x80000000).collect();
// Sort flipped...
// Sau sort: XOR lại
```

💡 **Tại sao:** Trong biểu diễn two's complement, số âm có bit đầu = 1. Khi coi như unsigned, chúng lớn hơn số dương. Lật bit dấu biến -128 thành 0, -1 thành 127, 0 thành 128, 127 thành 255 — đúng thứ tự!

### 3. Dùng cho float

❌ **Sai:** Áp dụng trực tiếp cho `f32`/`f64`

✅ **Đúng:** Float (IEEE 754) có thể sort bằng radix nếu xử lý bit đặc biệt:
- Số dương: lật bit dấu
- Số âm: lật tất cả bit (vì float âm có thứ tự ngược trong biểu diễn bit)

💡 **Tại sao:** Float không đơn giản như integer. NaN, -0.0, infinity đều cần xử lý riêng. Trừ khi bạn thật sự cần, hãy dùng comparison sort cho float.

---

## Độ phức tạp

| Chỉ số | Giá trị |
|--------|---------|
| Thời gian | O(d * (n + b)) |
| Bộ nhớ | O(n + b) |

Trong đó:
- **d** = số lượt (số "chữ số"). Với base 256 trên u32: d = 4
- **n** = số lượng phần tử
- **b** = cơ số (base). Ở đây b = 256

**Giải thích thực tế:**

- **Ổn định (stable)**: Có — thứ tự của các phần tử bằng nhau không đổi.
- **Không tại chỗ (not in-place)**: cần mảng phụ O(n).
- Với số nguyên 32-bit, base 256: chỉ cần **đúng 4 lượt** bất kể mảng có 1.000 hay 1.000.000 phần tử. Hiệu quả thực tế gần như **O(n)**!

---

## Khi nào dùng Radix Sort?

| Tình huống | Dùng Radix Sort? | Lý do |
|------------|:-:|-------|
| 10 triệu số `u32` | ✅ | n lớn, d = 4 cố định, nhanh hơn comparison sort |
| 100 phần tử | ❌ | n nhỏ, overhead setup không đáng. Dùng insertion sort |
| Strings ngắn cùng độ dài (ID 8 ký tự) | ✅ | d cố định, mỗi ký tự là 1 "digit" |
| Strings độ dài thay đổi lớn (1 - 10,000 ký tự) | ❌ | d quá lớn, lãng phí. Dùng comparison sort |
| Floating point | ⚠️ | Được, nhưng cần xử lý bit phức tạp |
| Số âm (`i32`) | ⚠️ | Được, nhưng cần flip sign bit |
| Cần sort tại chỗ (in-place) | ❌ | Radix Sort cần O(n) bộ nhớ phụ |
| Dữ liệu gần sorted | ❌ | Tim Sort (Rust default) cực nhanh với dữ liệu gần sorted |

### So sánh nhanh: Radix Sort vs `.sort()` (Tim Sort)

```
n = 1,000,000 số u32 ngẫu nhiên (benchmark tham khảo):

  Tim Sort (.sort()):    ~70ms    O(n log n)
  Radix Sort (base 256): ~25ms    O(4n) ≈ O(n)

Nhưng khi n = 100:
  Tim Sort:    ~0.001ms   (nhanh, ít overhead)
  Radix Sort:  ~0.003ms   (chậm hơn vì setup count array 256 phần tử)
```

> **Quy tắc ngón tay cái:** Radix Sort thắng khi **n > 10,000** và data là integer/fixed-length string. Dưới đó, cứ dùng `.sort()`.

---

## Ví dụ

```rust
use rust_ds2a::sorting::radix_sort;

let mut v: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
radix_sort(&mut v);
assert_eq!(v, vec![2, 24, 45, 66, 75, 90, 170, 802]);

// Mảng toàn số 0
let mut v: Vec<u32> = vec![0, 0, 0];
radix_sort(&mut v);
assert_eq!(v, vec![0, 0, 0]);

// Mảng lớn — vẫn chỉ 4 lượt!
let mut v: Vec<u32> = vec![999999, 1, 500000, 123456, 2];
radix_sort(&mut v);
assert_eq!(v, vec![1, 2, 123456, 500000, 999999]);
```

---

## Rust Ecosystem

### `rdst` crate

Crate [`rdst`](https://crates.io/crates/rdst) cung cấp radix sort tối ưu cho Rust, hỗ trợ cả signed integer, float, và string:

```rust
// cargo add rdst
use rdst::RadixSort;

let mut data: Vec<i32> = vec![-5, 3, -1, 7, 0];
data.radix_sort_unstable();
// [-5, -1, 0, 3, 7]
```

### `voracious_radix_sort`

Crate [`voracious_radix_sort`](https://crates.io/crates/voracious_radix_sort) tự động chọn strategy tốt nhất dựa trên data size và type.

### Khi nào dùng crate thay vì tự viết?

- **Production code:** Dùng crate. Chúng đã xử lý edge case (negative, float, SIMD optimization).
- **Phỏng vấn / học tập:** Tự viết. Hiểu thuật toán quan trọng hơn tốc độ.
- **`.sort()` / `.sort_unstable()`:** Mặc định của Rust (Tim Sort / PDQ Sort) rất tốt cho hầu hết trường hợp. Chỉ chuyển sang radix sort khi benchmark cho thấy nó nhanh hơn.

---

## Practice — Luyện tập

| Bài | Gợi ý |
|-----|-------|
| [Maximum Gap #164](https://leetcode.com/problems/maximum-gap/) | Sort bằng radix, rồi tìm max gap giữa 2 phần tử liên tiếp. Hoặc dùng bucket sort (pigeonhole principle). |
| [Sort an Array #912](https://leetcode.com/problems/sort-an-array/) | Thay vì merge/quick sort, thử submit bằng radix sort. Xử lý số âm bằng flip sign bit. So sánh runtime. |
| [Sort Colors #75](https://leetcode.com/problems/sort-colors/) | Counting Sort thuần (range chỉ 0-2). Bài tập nền tảng cho subroutine. |

---

## Tóm tắt

```
                    Radix Sort
                    ──────────
Ý tưởng:           Sort theo từng digit, dùng Counting Sort
Thời gian:          O(d * (n + b))  ≈  O(n) khi d nhỏ
Bộ nhớ:             O(n + b)
Stable:             Có (bắt buộc cho LSD hoạt động đúng)
So sánh:            KHÔNG — đây là non-comparison sort

Khi nào dùng:       n lớn + integer/fixed-length key
Khi nào KHÔNG dùng: n nhỏ, float, key dài thay đổi
```

---

## Tiếp theo

Chúng ta đã xong phần **sorting**! Mảng đã được sắp xếp, giờ câu hỏi tiếp theo tự nhiên là: **tìm kiếm trong mảng sorted có nhanh hơn không?**

Câu trả lời là CÓ — và Binary Search sẽ cho bạn thấy cách tìm 1 phần tử trong 1 triệu phần tử chỉ với **20 bước**. Từ O(n) xuống O(log n). Chương tiếp theo nhé!

---

[← Heap Sort](./05-heap-sort.md) | [Binary Search →](./07-binary-search.md)
