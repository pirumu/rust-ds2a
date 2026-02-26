# Radix Sort

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

### Tối ưu: dùng base 256 thay vì base 10

Trong code thực tế, thay vì xử lý từng chữ số (0-9), ta xử lý **từng byte** (0-255). Một số 32-bit có 4 byte, nên chỉ cần **đúng 4 lượt** bất kể số lớn cỡ nào.

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

        // Đếm số lượng phần tử có mỗi giá trị "chữ số"
        for &val in arr.iter() {
            let digit = ((val / exp) % 256) as usize;
            count[digit] += 1;
        }

        // Tính vị trí bắt đầu của mỗi ô (prefix sum)
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Đặt phần tử vào đúng vị trí (duyệt ngược để giữ stable)
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
- Thuật toán này chỉ hoạt động với `u32`. Để hỗ trợ số âm hay kiểu khác, cần xử lý thêm (ví dụ: xor bit đầu).

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

**Khi nào nên dùng?**

- Khi sắp xếp số nguyên hoặc chuỗi có độ dài cố định.
- Khi n lớn và kích thước khóa nhỏ (ví dụ: sắp xếp 10 triệu số nguyên 32-bit).
- Không phù hợp khi khóa có độ dài thay đổi lớn (ví dụ: chuỗi có độ dài từ 1 đến 10.000 ký tự).

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
