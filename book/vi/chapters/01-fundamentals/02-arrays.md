# Arrays & Slices

## Đây là gì?

> Array là cấu trúc dữ liệu **đầu tiên** bạn học trong series này, và tin vui: nó cũng là cấu trúc **dễ hiểu nhất**. Nếu bạn đã từng dùng mảng trong bất kỳ ngôn ngữ nào (Python list, JavaScript array), bạn đã hiểu 70% rồi. Phần còn lại chỉ là Rust thêm vài "quy tắc an toàn" -- nhìn lạ nhưng quen rất nhanh.

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

#### Vec và `with_capacity` -- tránh cấp phát lại

Khi bạn biết trước (hoặc ước lượng được) sẽ cần bao nhiêu phần tử, dùng `Vec::with_capacity` để Rust cấp phát đủ chỗ ngay từ đầu:

```rust
// ❌ Không dùng capacity -- Vec phải cấp phát lại nhiều lần
let mut v = Vec::new();
for i in 0..10_000 {
    v.push(i);  // mỗi lần len == cap, Rust phải: cấp vùng mới → copy → giải phóng vùng cũ
}

// ✅ Dùng with_capacity -- cấp phát 1 lần duy nhất
let mut v = Vec::with_capacity(10_000);
for i in 0..10_000 {
    v.push(i);  // không bao giờ phải cấp phát lại
}
```

Quy trình cấp phát lại trông như thế này:

```
push(6) vào Vec có cap=5, len=5:

Trước:  [1, 2, 3, 4, 5]        cap = 5, len = 5 (hết chỗ!)
         ↓ cấp phát vùng mới gấp đôi
Tạm:    [_, _, _, _, _, _, _, _, _, _]   cap = 10
         ↓ copy dữ liệu cũ sang
Tạm:    [1, 2, 3, 4, 5, _, _, _, _, _]  cap = 10
         ↓ thêm phần tử mới
Sau:    [1, 2, 3, 4, 5, 6, _, _, _, _]  cap = 10, len = 6
         ↓ giải phóng vùng cũ
```

**Khi nào dùng `with_capacity`?**
- Biết chính xác số phần tử: `Vec::with_capacity(n)`
- Ước lượng được: `Vec::with_capacity(estimated_size)`
- Không biết: cứ dùng `Vec::new()` -- Rust tự xử lý, chỉ chậm hơn chút

**Anti-pattern cần tránh:** Đừng over-allocate quá nhiều. `Vec::with_capacity(1_000_000)` cho mảng 100 phần tử = lãng phí ~4MB bộ nhớ.

Slice thì mượn dữ liệu, nên borrow checker của Rust sẽ đảm bảo bạn **không dùng slice khi dữ liệu gốc đã bị xóa**.

### Khi nào dùng kiểu nào?

| Kiểu | Trên Heap? | Co giãn? | Khi nào dùng |
|------|-----------|----------|--------------|
| `[T; N]` | Không | Không | Buffer nhỏ, kích thước biết trước |
| `&[T]` / `&mut [T]` | Không (mượn) | Không | Tham số hàm, nhìn vào 1 phần dữ liệu |
| `Vec<T>` | Có | Có | Khi cần thêm/bớt phần tử |

**Quy tắc đơn giản:** Hàm nhận vào thì dùng `&[T]` (linh hoạt nhất). Cần sở hữu và thay đổi thì dùng `Vec<T>`.

## Những cái bẫy hay gặp

Mấy lỗi này ai cũng mắc ít nhất 1 lần. Biết trước thì đỡ đau:

### a) Off-by-one trong binary search

Binary search nhìn đơn giản nhưng cực dễ sai ở biên. Bug này nguy hiểm vì có thể chạy đúng 99% test cases nhưng sai ở edge case.

❌ **Code sai:**

```rust
fn binary_search_bug(arr: &[i32], target: &i32) -> Option<usize> {
    let mut lo = 0;
    let mut hi = arr.len() - 1;  // 💥 nếu arr rỗng → underflow!

    while lo <= hi {              // 💥 khi hi = 0, lo = 1 → underflow khi so sánh!
        let mid = (lo + hi) / 2; // 💥 có thể tràn số nếu lo + hi > usize::MAX
        if arr[mid] == *target {
            return Some(mid);
        } else if arr[mid] < *target {
            lo = mid + 1;
        } else {
            hi = mid - 1;        // 💥 khi mid = 0 → underflow!
        }
    }
    None
}
```

✅ **Code đúng:**

```rust
fn binary_search_safe(arr: &[i32], target: &i32) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();  // half-open range [lo, hi)

    while lo < hi {                  // không bao giờ underflow
        let mid = lo + (hi - lo) / 2; // không bao giờ tràn
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid, // mid, không phải mid - 1
        }
    }
    None
}
```

💡 **Tại sao?** Dùng **half-open range** `[lo, hi)` -- `hi` là "qua khỏi phần tử cuối". Cách này tránh underflow khi `hi = 0`, và `lo < hi` thay vì `lo <= hi` giúp tránh vòng lặp vô hạn.

### b) Index out of bounds panic

Rust không cho phép truy cập ngoài mảng như C/C++ (undefined behavior). Thay vào đó, Rust **panic** -- chương trình dừng ngay lập tức. Đây thực ra là **điều tốt** vì bug được phát hiện ngay, thay vì âm thầm đọc rác.

❌ **Code sẽ panic:**

```rust
let arr = [10, 20, 30];
let val = arr[5];  // 💥 panic: index out of bounds
```

✅ **Code an toàn:**

```rust
let arr = [10, 20, 30];

// .get() trả về Option -- không panic
match arr.get(5) {
    Some(val) => println!("Tìm thấy: {val}"),
    None => println!("Index không hợp lệ"),
}

// Hoặc ngắn hơn:
let val = arr.get(5).unwrap_or(&0);  // mặc định = 0 nếu ngoài mảng
```

💡 **Quy tắc:** Dùng `arr[i]` khi bạn **chắc chắn** index hợp lệ (ví dụ: trong vòng lặp `for i in 0..arr.len()`). Dùng `arr.get(i)` khi index đến từ bên ngoài (user input, tính toán phức tạp).

### c) Vec reallocation ẩn -- push chậm không rõ lý do

Đã nói ở phần trên, nhưng tóm lại:

❌ **Code chậm:**

```rust
let mut v = Vec::new();  // cap = 0
for i in 0..1000 {
    v.push(i);  // realloc ở i = 0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512
    //           tổng cộng ~11 lần cấp phát lại + copy
}
```

✅ **Code nhanh:**

```rust
let mut v = Vec::with_capacity(1000);  // cấp phát 1 lần
for i in 0..1000 {
    v.push(i);  // không bao giờ realloc
}
```

💡 Mỗi lần realloc, Rust phải: (1) xin vùng nhớ mới gấp đôi, (2) copy toàn bộ dữ liệu cũ, (3) giải phóng vùng cũ. Với mảng 1 triệu phần tử, đó là hàng triệu lần copy không cần thiết.

### d) Borrow conflict -- vừa iterate vừa modify

Đây là lỗi compile phổ biến nhất với người mới Rust:

❌ **Code không compile:**

```rust
let mut v = vec![1, 2, 3, 4, 5];
for &x in &v {           // immutable borrow bắt đầu ở đây
    if x % 2 == 0 {
        v.push(x * 10);  // 💥 mutable borrow! Rust không cho phép
    }
}
```

✅ **Cách fix -- tách thành 2 bước:**

```rust
let mut v = vec![1, 2, 3, 4, 5];

// Bước 1: đọc và ghi nhận (immutable borrow)
let to_add: Vec<i32> = v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 10)
    .collect();

// Bước 2: thêm vào (mutable borrow -- borrow cũ đã kết thúc)
v.extend(to_add);
```

💡 **Tại sao Rust cấm?** Nếu `v.push()` trigger realloc, toàn bộ dữ liệu chuyển sang vùng nhớ mới. Con trỏ mà vòng `for` đang giữ trở thành **dangling pointer** -- trỏ vào vùng đã giải phóng. C/C++ cho phép điều này và kết quả là bug cực kỳ khó debug. Rust bắt lỗi ngay lúc compile.

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

#### Tại sao 3 lần đảo ngược lại xoay được?

Nghĩ mảng gồm 2 phần: **A** (k phần tử đầu muốn chuyển ra sau) và **B** (phần còn lại muốn chuyển lên trước):

```
Ban đầu:    [ A | B ]       muốn thành → [ B | A ]
```

Gọi Aᴿ là A đảo ngược, Bᴿ là B đảo ngược. Xem phép biến đổi:

```
Bước 1: Đảo A    → [ Aᴿ | B  ]
Bước 2: Đảo B    → [ Aᴿ | Bᴿ ]
Bước 3: Đảo cả   → (Aᴿ Bᴿ)ᴿ = B A   ← đúng rồi!
```

Bước 3 hoạt động vì: đảo ngược một chuỗi đã đảo ngược = trở lại ban đầu, nhưng **thứ tự 2 khối bị đổi chỗ**.

Ví dụ cụ thể với `[1, 2, 3, 4, 5]`, k=2:

```
Ban đầu:               [1, 2 | 3, 4, 5]     A=[1,2]  B=[3,4,5]
Bước 1 - đảo A:       [2, 1 | 3, 4, 5]
Bước 2 - đảo B:       [2, 1 | 5, 4, 3]
Bước 3 - đảo toàn bộ: [3, 4, 5, 1, 2]       ← xoay trái 2 vị trí!
```

**So sánh với cách dùng mảng tạm:**

```rust
// Cách "thô" -- cần O(n) bộ nhớ thêm
fn rotate_left_naive<T: Clone>(arr: &mut [T], k: usize) {
    let k = k % arr.len();
    let temp: Vec<T> = arr[..k].to_vec();   // clone k phần tử đầu
    arr.copy_within(k.., 0);                 // dịch phần sau lên
    arr[arr.len()-k..].clone_from_slice(&temp); // đặt k phần tử vào cuối
}
```

Trick 3 reverse hay hơn vì: O(1) space, không cần clone, không cần allocate. Chỉ swap tại chỗ.

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

#### Tại sao cần 2 HashSet?

Nếu chỉ dùng 1 set, bạn biết phần tử đã gặp rồi, nhưng **không tránh được đưa nó vào kết quả nhiều lần**:

```rust
// ❌ Chỉ dùng 1 set → số 2 xuất hiện 3 lần → kết quả có 2 lần số 2!
let mut seen = HashSet::new();
let mut result = Vec::new();
for &val in &[1, 2, 3, 2, 2] {
    if !seen.insert(val) {
        result.push(val);  // push 2 hai lần!
    }
}
// result = [2, 2] ← sai, chỉ muốn [2]
```

Set `duplicates` đóng vai trò **khử trùng** kết quả. Vì HashSet tự động bỏ qua giá trị đã có, dù gặp số 2 bao nhiêu lần thì `duplicates` chỉ chứa 1 lần.

#### Các biến thể

**Chỉ cần biết "có trùng không" (true/false)?** Đơn giản hơn nhiều:

```rust
pub fn has_duplicates(arr: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &val in arr {
        if !seen.insert(val) {
            return true;  // tìm thấy trùng, dừng ngay
        }
    }
    false
}
```

**Mảng đã sort?** Có thể dùng O(1) space -- chỉ cần so sánh phần tử kề nhau:

```rust
pub fn find_duplicates_sorted(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 1..arr.len() {
        if arr[i] == arr[i - 1] {
            if result.last() != Some(&arr[i]) {
                result.push(arr[i]);
            }
        }
    }
    result
}
```

| Phương pháp | Time | Space | Yêu cầu |
|---|---|---|---|
| 2 HashSet | O(n) | O(n) | Không |
| So sánh kề nhau | O(n) | O(1) | Mảng đã sort |

Trade-off: nếu mảng chưa sort, sort trước tốn O(n log n). Nếu đã sort sẵn thì cách kề nhau tiết kiệm bộ nhớ hơn nhiều.

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

### Two Pointers -- Kỹ thuật hai con trỏ

Nhiều bài toán array có thể giải bằng cách dùng **2 con trỏ** di chuyển trên mảng. Thay vì dùng 2 vòng lặp lồng nhau O(n²), 2 con trỏ thường giúp giải trong O(n).

Có 2 kiểu chính:

**Kiểu 1: Hai đầu vào giữa** -- 1 con trỏ ở đầu, 1 ở cuối, tiến vào giữa. Dùng khi cần xét cặp phần tử đối xứng.

**Kiểu 2: Cùng chiều khác tốc** (slow/fast) -- cả 2 đi từ đầu nhưng 1 nhanh 1 chậm. Dùng khi cần "lọc" hoặc "nén" mảng tại chỗ.

#### Ví dụ 1: Kiểm tra palindrome (hai đầu vào giữa)

Palindrome là chuỗi đọc xuôi ngược giống nhau: "racecar", "madam", "121".

```rust
pub fn is_palindrome(s: &[u8]) -> bool {
    if s.is_empty() { return true; }
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

```
"racecar"
 ^     ^   r == r ✓
  ^   ^    a == a ✓
   ^ ^     c == c ✓
    ^      gặp nhau → palindrome!
```

Giống `reverse` ở trên phải không? Cùng pattern, khác hành động: `reverse` swap, `is_palindrome` so sánh.

#### Ví dụ 2: Xóa phần tử trùng trong mảng đã sort (slow/fast)

Bài LeetCode kinh điển: cho mảng sorted, xóa trùng **tại chỗ**, trả về độ dài mới.

```rust
pub fn remove_duplicates_sorted(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() { return 0; }

    let mut slow = 0;  // vị trí ghi tiếp theo
    for fast in 1..arr.len() {
        if arr[fast] != arr[slow] {
            slow += 1;
            arr[slow] = arr[fast];
        }
    }

    let new_len = slow + 1;
    arr.truncate(new_len);
    new_len
}
```

```
[1, 1, 2, 2, 3]
 s  f              1 == 1 → bỏ qua
 s     f           2 != 1 → slow++, ghi arr[1]=2
    s     f        2 == 2 → bỏ qua
    s        f     3 != 2 → slow++, ghi arr[2]=3
       s           kết quả: [1, 2, 3], new_len = 3
```

`slow` đánh dấu "phần mảng đã sạch", `fast` quét phía trước tìm giá trị mới. Mỗi khi `fast` gặp giá trị khác, copy vào vị trí `slow+1`.

#### Khi nào dùng kiểu nào?

| Kiểu | Khi nào | Ví dụ |
|------|---------|-------|
| Hai đầu vào giữa | Xét cặp đối xứng, tìm 2 số cộng bằng target | palindrome, two-sum sorted, container |
| Cùng chiều khác tốc | Lọc/nén tại chỗ, phân vùng | remove duplicates, move zeroes, partition |

### Sliding Window -- Cửa sổ trượt

Bạn có quán trà sữa, theo dõi doanh thu mỗi ngày. Muốn tìm **3 ngày liên tiếp** mà tổng doanh thu cao nhất. Cách "thô": cộng mỗi bộ 3 ngày, so sánh. Nhưng thông minh hơn: khi "cửa sổ" trượt sang phải 1 ngày, chỉ cần **trừ ngày cũ nhất, cộng ngày mới nhất**. Đó là sliding window.

Có 2 loại:

**Fixed-size window:** cửa sổ luôn cùng kích thước k.

**Variable-size window:** cửa sổ co giãn tùy điều kiện.

#### Fixed-size: Tìm subarray độ dài k có tổng lớn nhất

```rust
pub fn max_sum_subarray_k(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() { return None; }

    // Tính tổng cửa sổ đầu tiên
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;

    // Trượt cửa sổ: bỏ phần tử đầu, thêm phần tử mới
    for i in k..arr.len() {
        window_sum += arr[i] - arr[i - k];
        max_sum = max_sum.max(window_sum);
    }

    Some(max_sum)
}
```

```
arr = [2, 1, 5, 1, 3, 2], k = 3

Cửa sổ 1: [2, 1, 5] = 8
                ↓ trượt: bỏ 2, thêm 1
Cửa sổ 2: [1, 5, 1] = 7         (8 - 2 + 1 = 7)
                ↓ trượt: bỏ 1, thêm 3
Cửa sổ 3: [5, 1, 3] = 9         (7 - 1 + 3 = 9)  ← max!
                ↓ trượt: bỏ 5, thêm 2
Cửa sổ 4: [1, 3, 2] = 6         (9 - 5 + 2 = 6)

Đáp án: 9
```

Mỗi phần tử chỉ bị cộng 1 lần và trừ 1 lần → O(n), không phải O(n×k).

#### Variable-size: Tìm subarray ngắn nhất có tổng ≥ target

```rust
pub fn min_subarray_len(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut sum = 0;
    let mut min_len = usize::MAX;

    for right in 0..arr.len() {
        sum += arr[right];                  // mở rộng cửa sổ

        while sum >= target {               // thu hẹp cửa sổ khi thỏa mãn
            min_len = min_len.min(right - left + 1);
            sum -= arr[left];
            left += 1;
        }
    }

    if min_len == usize::MAX { None } else { Some(min_len) }
}
```

```
arr = [2, 3, 1, 2, 4, 3], target = 7

right=0: sum=2                             [2]
right=1: sum=5                             [2,3]
right=2: sum=6                             [2,3,1]
right=3: sum=8 ≥ 7 → len=4, thu hẹp       [2,3,1,2]
         sum=6 (bỏ 2), thoát while         [3,1,2]
right=4: sum=10 ≥ 7 → len=4, thu hẹp      [3,1,2,4]
         sum=7 ≥ 7 → len=3, thu hẹp       [1,2,4]
         sum=6, thoát while                [2,4]
right=5: sum=9 ≥ 7 → len=3, thu hẹp       [2,4,3]
         sum=7 ≥ 7 → len=2, thu hẹp       [4,3]  ← min!
         sum=3, thoát while                [3]

Đáp án: 2 (subarray [4, 3])
```

**Tại sao O(n) chứ không phải O(n²)?** Nhìn vòng `while` bên trong có vẻ là nested loop, nhưng: `left` chỉ tăng, không bao giờ giảm. Tổng số lần `left` tăng qua toàn bộ thuật toán ≤ n. Nên mỗi phần tử chỉ **enter** window 1 lần và **exit** window 1 lần = O(2n) = O(n).

## Độ phức tạp

| Hàm | Time | Space | Pattern | Ghi chú |
|-----|------|-------|---------|---------|
| `linear_search` | O(n) | O(1) | Duyệt | Xui nhất phải duyệt hết |
| `binary_search` | O(log n) | O(1) | Chia đôi | Mảng phải sắp xếp trước |
| `reverse` | O(n) | O(1) | Two pointers | Hoán đổi tại chỗ |
| `rotate_left` | O(n) | O(1) | Three reverses | 3 lần reverse, mỗi lần O(n) |
| `find_duplicates` | O(n) | O(n) | HashSet | 2 set: seen + duplicates |
| `find_duplicates_sorted` | O(n) | O(1) | So sánh kề | Cần mảng đã sort |
| `max_subarray_sum` | O(n) | O(1) | Kadane's | Quyết định tiếp/bắt đầu lại |
| `is_palindrome` | O(n) | O(1) | Two pointers | Hai đầu vào giữa |
| `remove_duplicates_sorted` | O(n) | O(1) | Slow/fast | Cùng chiều khác tốc |
| `max_sum_subarray_k` | O(n) | O(1) | Fixed window | Trượt: trừ cũ, cộng mới |
| `min_subarray_len` | O(n) | O(1) | Variable window | Co giãn theo điều kiện |

Để ý: `binary_search` nhanh hơn `linear_search` rất nhiều (O(log n) vs O(n)), nhưng **đổi lại** mảng phải được sắp xếp. Không có gì miễn phí!

## Nhận diện Pattern -- Luyện tập

Đọc mô tả bài toán, tự hỏi: dùng cấu trúc gì? Pattern nào?

**Bài 1:** Cho chuỗi, tìm **substring dài nhất** mà không có ký tự nào lặp lại. Ví dụ: `"abcabcbb"` → `"abc"` (độ dài 3).

*Gợi ý:* Khi gặp ký tự trùng, phần đầu cửa sổ cần dịch sang phải...

<details>
<summary>Đáp án và giải thích</summary>

**Cấu trúc dữ liệu:** `&[u8]` hoặc `&str` + `HashSet` để theo dõi ký tự trong cửa sổ
**Pattern:** Variable-size sliding window -- mở rộng `right` để thêm ký tự, thu hẹp `left` khi gặp trùng
**Độ phức tạp:** Time O(n), Space O(k) với k là kích thước bảng chữ cái

</details>

**Bài 2:** Cho 2 mảng `original` và `rotated`. Kiểm tra `rotated` có phải là kết quả xoay (rotate) của `original` không. Ví dụ: `[3,4,5,1,2]` là rotation của `[1,2,3,4,5]`.

*Gợi ý:* Nếu nối `original` với chính nó thì sao?

<details>
<summary>Đáp án và giải thích</summary>

**Cấu trúc dữ liệu:** `Vec<T>` hoặc `&[T]`
**Pattern:** Nối `original + original` thành mảng mới, rồi kiểm tra `rotated` có phải là subarray. Hoặc: tìm điểm bắt đầu trong `original` rồi so sánh vòng tròn
**Độ phức tạp:** Time O(n), Space O(n) (cách nối mảng) hoặc O(1) space (cách so sánh vòng tròn)

</details>

**Bài 3:** Cho mảng số nguyên và một `target`. Tìm **2 số** trong mảng cộng lại bằng `target`. Trả về index của chúng. Ví dụ: `[2, 7, 11, 15]`, target = 9 → `[0, 1]` (vì 2+7=9).

*Gợi ý:* Với mỗi số, bạn biết cần tìm số nào nữa. Dùng gì để tìm nhanh O(1)?

<details>
<summary>Đáp án và giải thích</summary>

**Cấu trúc dữ liệu:** `HashMap<i32, usize>` -- map giá trị → index
**Pattern:** Duyệt mảng, với mỗi `arr[i]`, tìm `target - arr[i]` trong HashMap. Nếu có → trả kết quả. Nếu không → thêm `arr[i]` vào map
**Độ phức tạp:** Time O(n), Space O(n)

*Nếu mảng đã sort:* dùng Two Pointers (hai đầu vào giữa) → Time O(n), Space O(1)

</details>

**Bài 4:** Cho mảng `height` biểu diễn các cột dọc. Tìm 2 cột sao cho lượng nước chứa được giữa chúng là **lớn nhất**. Ví dụ: `[1,8,6,2,5,4,8,3,7]` → diện tích max = 49 (giữa cột 8 ở index 1 và cột 7 ở index 8).

*Gợi ý:* Diện tích = min(2 cột) × khoảng cách. Bắt đầu từ 2 đầu, di chuyển cột thấp hơn vào...

<details>
<summary>Đáp án và giải thích</summary>

**Cấu trúc dữ liệu:** `&[i32]`
**Pattern:** Two pointers (hai đầu vào giữa) -- tính diện tích, di chuyển con trỏ ở cột thấp hơn (vì di chuyển cột cao không thể tăng diện tích)
**Độ phức tạp:** Time O(n), Space O(1)

</details>

**Bài 5:** Cho 2 mảng sorted `a` và `b`, merge thành 1 mảng sorted. Ví dụ: `[1,3,5]` + `[2,4,6]` → `[1,2,3,4,5,6]`.

*Gợi ý:* Dùng 2 con trỏ, mỗi cái theo dõi 1 mảng...

<details>
<summary>Đáp án và giải thích</summary>

**Cấu trúc dữ liệu:** `Vec<T>` cho kết quả, `&[T]` cho 2 mảng đầu vào
**Pattern:** Two pointers (cùng chiều) -- 1 con trỏ mỗi mảng, so sánh phần tử nhỏ hơn → push vào kết quả, tiến con trỏ đó. Khi 1 mảng hết → đổ phần còn lại của mảng kia
**Độ phức tạp:** Time O(n+m), Space O(n+m)

</details>

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

    // Two pointers -- kiểm tra palindrome
    assert!(is_palindrome(b"racecar"));
    assert!(!is_palindrome(b"hello"));

    // Sliding window -- tìm tổng lớn nhất của 3 phần tử liên tiếp
    assert_eq!(max_sum_subarray_k(&[2, 1, 5, 1, 3, 2], 3), Some(9));

    // Sliding window -- subarray ngắn nhất có tổng >= 7
    assert_eq!(min_subarray_len(&[2, 3, 1, 2, 4, 3], 7), Some(2));
}
```

---

---

[← Phân tích độ phức tạp (Big-O)](./01-complexity.md) | [Strings →](./03-strings.md)
