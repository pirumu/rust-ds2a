# String

> 💡 **Đừng lo lắng:** `String` vs `&str` nghe rối nhưng thực ra chỉ là: một cái bạn sở hữu, một cái bạn mượn. Giống `Vec<T>` vs `&[T]` ở chương trước vậy thôi. Rust không cố làm khó bạn -- nó chỉ muốn bạn biết rõ ai đang giữ dữ liệu. Đọc chương này xong bạn sẽ thấy đơn giản hơn Rust làm nó trông.

## Đây là gì?

> String trong Rust nổi tiếng là **phần gây bối rối nhất** cho người mới. `String` vs `&str`, UTF-8, bytes vs chars -- nghe rối lắm. Nhưng thực ra chỉ cần nhớ **2 thứ**: `String` là chuỗi bạn sở hữu (có thể thay đổi), `&str` là chuỗi bạn mượn (chỉ đọc). Còn UTF-8? Đọc xong chương này bạn sẽ thấy nó không đáng sợ như tên gọi.

Bạn nhắn tin cho bạn bè mỗi ngày. Mỗi tin nhắn là một dãy ký tự nối tiếp nhau -- chữ cái, dấu cách, emoji. Đó chính là **string** -- một dãy các ký tự liên tiếp trong bộ nhớ.

Nhưng trong Rust, string không đơn giản như vậy. Hầu hết ngôn ngữ giấu đi cách lưu trữ bên trong. Rust thì không -- nó để lộ hẳn **UTF-8 encoding** (cách mã hóa ký tự). Tại sao? Vì Rust muốn bạn hiểu rõ mình đang làm gì, tránh bug ngầm.

### Tại sao phải quan tâm UTF-8?

Hãy lấy chữ **"Việt Nam"** làm ví dụ:

```
"Việt Nam" -- con người thấy 8 ký tự

Nhưng máy tính thấy:
V  = 1 byte
i  = 1 byte
ệ  = 3 bytes  ← chữ có dấu!
t  = 1 byte
   = 1 byte (dấu cách)
N  = 1 byte
a  = 1 byte
m  = 1 byte
─────────────
Tổng: 10 bytes cho 8 ký tự!
```

Chữ `ệ` cần 3 bytes vì nó có cả dấu mũ và dấu nặng. UTF-8 dùng 1-4 bytes cho mỗi ký tự. Chữ ASCII (a-z, 0-9) chỉ 1 byte. Chữ có dấu tiếng Việt thường 2-3 bytes. Emoji có thể 4 bytes.

### Hai kiểu string trong Rust

Rust có hai kiểu chính: `String` và `&str`. Nghĩ đơn giản:

- **`String`** = bạn **sở hữu** cuốn sách. Muốn viết thêm, xóa bớt đều được.
- **`&str`** = bạn **mượn** đọc cuốn sách. Chỉ đọc, không sửa.

```
String (sở hữu, nằm trên heap)        &str (mượn, chỉ đọc)
stack: ptr ──┐  len: 5  cap: 8        stack: ptr ──► trỏ vào dữ liệu UTF-8 bất kỳ
             │                                len: 5
heap:        ▼
+---+---+---+---+---+---+---+---+
| h | e | l | l | o |   |   |   |   (bytes, mã hóa UTF-8)
+---+---+---+---+---+---+---+---+
```

| Kiểu | Sở hữu? | Thay đổi được? | Khi nào dùng? |
|------|----------|----------------|---------------|
| `String` | Có | Có | Tạo mới, chỉnh sửa string |
| `&str` | Không (mượn) | Không | Đọc string, tham số hàm |

**Mẹo nhỏ:** Tham số hàm thì nhận `&str`. Trả về string mới thì trả `String`.

### Không thể truy cập bằng index!

Đây là điểm nhiều người bị sốc khi học Rust:

```rust
let s = "Việt Nam";
// s[0]            // LỖI! Rust không cho index string
// Vì byte thứ 0 hay ký tự thứ 0? Mập mờ!

s.chars().nth(0)   // Some('V') -- lấy ký tự thứ 0, nhưng O(n)
&s[0..1]           // "V" -- cắt theo byte, cẩn thận!
&s[2..5]           // "ệ" -- đúng vì ệ chiếm byte 2-4
// &s[2..3]        // PANIC! Cắt giữa chừng ký tự multi-byte
```

### .chars() vs .bytes() vs .as_bytes()

Ba cách phổ biến để duyệt qua string. Khác biệt lớn:

| Method | Trả về | Khi nào dùng |
|--------|--------|-------------|
| `.chars()` | Từng ký tự Unicode | Text thông thường, tiếng Việt, emoji |
| `.bytes()` | Từng byte (u8) | Biết chắc input là ASCII, cần tốc độ |
| `.as_bytes()` | `&[u8]` (slice) | Muốn index theo byte, biết chắc ASCII |

Thử với `"Việt"`:

```rust
"Việt".chars().collect::<Vec<_>>()
// ['V', 'i', 'ệ', 't']  — 4 ký tự, đúng!

"Việt".bytes().collect::<Vec<_>>()
// [86, 105, 225, 187, 135, 116]  — 6 bytes, vì ệ = 3 bytes

"Việt".as_bytes()
// &[86, 105, 225, 187, 135, 116]  — giống .bytes() nhưng trả slice
// Có thể index: "Việt".as_bytes()[0] == 86 (byte của 'V')
```

**Quy tắc đơn giản:** Không chắc? Dùng `.chars()`. Biết chắc ASCII? Dùng `.bytes()` để nhanh hơn.

## Những cái bẫy hay gặp với String

### ❌ Bẫy 1: Panic khi slice giữa chừng multi-byte char

```rust
let s = "Việt Nam";
// ❌ Sai:
let bad = &s[2..3];  // PANIC! byte 2-3 nằm giữa chữ ệ (3 bytes)

// ✅ Đúng:
let good: String = s.chars().skip(2).take(1).collect();
// good = "ệ"
```

💡 Rust panic ở **runtime**, không phải compile time. Đây là một trong ít chỗ Rust không bắt được lúc compile. Luôn cẩn thận khi dùng byte slice `&s[a..b]` với string chứa tiếng Việt.

### ❌ Bẫy 2: String concatenation trong vòng lặp — O(n²) ẩn

```rust
// ❌ Sai: mỗi lần + tạo String mới, copy toàn bộ dữ liệu cũ
let mut result = String::new();
for word in words {
    result = result + &word + " ";  // O(n) mỗi lần → tổng O(n²)
}

// ✅ Đúng: dùng push_str, tránh tạo String mới
let mut result = String::with_capacity(total_len);  // cấp phát 1 lần
for word in words {
    result.push_str(&word);
    result.push(' ');
}

// ✅ Hoặc tốt hơn: dùng iterator
let result: String = words.join(" ");
```

💡 Tương tự như Vec reallocation ở chương trước. Operator `+` tạo String mới mỗi lần, copy hết nội dung cũ sang. 10 từ thì copy 10 lần. 1000 từ thì copy 1000 lần.

### ❌ Bẫy 3: Nhầm len() trả về bytes, không phải ký tự

```rust
let s = "Việt";

// ❌ Sai: tưởng len() đếm số chữ
println!("{}", s.len());           // 6 (bytes!)

// ✅ Đúng: dùng chars().count() để đếm ký tự
println!("{}", s.chars().count()); // 4 (ký tự)
```

💡 `"Việt".len()` = 6 vì `ệ` chiếm 3 bytes. `.len()` luôn trả về số bytes. Muốn đếm ký tự phải dùng `.chars().count()`.

### ❌ Bẫy 4: Clone String không cần thiết

```rust
// ❌ Sai: hàm nhận String, caller phải clone
fn count_vowels(s: String) -> usize {
    s.chars().filter(|c| "aeiou".contains(*c)).count()
}
let name = String::from("Việt Nam");
let n = count_vowels(name.clone());  // clone cả string chỉ để đọc!

// ✅ Đúng: hàm nhận &str, không cần clone
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiou".contains(*c)).count()
}
let name = String::from("Việt Nam");
let n = count_vowels(&name);  // chỉ mượn, không copy
```

💡 Quy tắc đã đề cập ở trên: tham số hàm thì nhận `&str`. Hàm chỉ cần **đọc** string, không cần **sở hữu** nó.

## Bản đồ Pattern

Trước khi đi vào code, hãy nhìn bức tranh toàn cảnh. Hầu hết bài toán string đều rơi vào 3 pattern chính:

| Pattern | Khi nào dùng? | Câu hỏi nhận diện | Ví dụ |
|---------|--------------|-------------------|-------|
| **Frequency Count** (HashMap đếm ký tự) | Cần biết ký tự xuất hiện bao nhiêu lần | "Bài này có cần **đếm** hay **so sánh tần suất** ký tự không?" | anagram, first non-repeating |
| **Two Pointers** | So sánh từ 2 đầu vào giữa | "Bài này có cần **so sánh đối xứng** hoặc **xử lý từ 2 phía** không?" | palindrome, reverse |
| **Sliding Window / Single Pass** | Xử lý từng đoạn liên tiếp | "Bài này có cần xem **chuỗi con liên tiếp** hoặc **duyệt 1 lượt** không?" | compress, longest substring |

```
Bài toán string
    ├── Cần đếm ký tự?
    │   └── ✅ Frequency Count (HashMap)
    ├── So sánh đối xứng / 2 đầu?
    │   └── ✅ Two Pointers
    └── Xử lý đoạn liên tiếp?
        └── ✅ Sliding Window / Single Pass
```

Mỗi bài toán bên dưới đều thuộc một (hoặc kết hợp) các pattern này. Hãy chú ý xem pattern nào được dùng ở đâu.

## Hoạt động như thế nào?

Chương này cài đặt 5 bài toán string kinh điển, hay gặp trong phỏng vấn:

1. **Đảo ngược string** -- "hello" → "olleh" *(Two Pointers / Iterator)*
2. **Kiểm tra palindrome** -- "racecar" đọc xuôi ngược giống nhau *(Two Pointers)*
3. **Kiểm tra anagram** -- "listen" và "silent" cùng ký tự *(Frequency Count)*
4. **Ký tự đầu tiên không lặp** -- "aabcc" → 'b' *(Frequency Count)*
5. **Nén string** -- "aaabbc" → "a3b2c1" *(Single Pass)*

## Code Rust

Tất cả code nằm trong `src/strings.rs`.

### reverse_string -- Đảo ngược chuỗi *(Two Pointers / Iterator)*

Tưởng tượng bạn xếp từng chữ cái vào chồng đĩa, rồi lấy ra lại. Chữ cuối vào sẽ ra đầu tiên.

```rust
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
```

Ba bước:
- `.chars()` -- tách thành từng ký tự Unicode (không phải byte!)
- `.rev()` -- đảo ngược thứ tự
- `.collect()` -- ghép lại thành `String` mới

Thử với tiếng Việt:

```
reverse_string("Việt") = "tệiV"
```

Nếu đảo theo byte thay vì theo char, kết quả sẽ là rác. `.chars()` đảm bảo đảo đúng.

### is_palindrome -- Kiểm tra đọc xuôi ngược giống nhau *(Two Pointers)*

Palindrome (chuỗi đối xứng) là chuỗi đọc xuôi hay ngược đều giống nhau. Ví dụ: "racecar", "madam".

Bài toán thêm phần thú vị: bỏ qua dấu cách và hoa/thường. "A man a plan a canal Panama" vẫn là palindrome.

**Cách 1: Vec + index** — đơn giản, dễ hiểu

```rust
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())  // bỏ dấu cách, dấu câu
        .map(|c| c.to_ascii_lowercase())  // đồng nhất hoa thường
        .collect();

    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}
```

**Cách 2: Two Pointers** — nhấn mạnh pattern, phỏng vấn nên dùng cách này

```rust
pub fn is_palindrome_two_pointers(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if chars.is_empty() { return true; }

    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
```

```
"A man a plan a canal Panama"
Bước 1: Lọc + lowercase → "amanaplanacanalpanama"
Bước 2: Two pointers:
        a m a n a p l a n a c a n a l p a n a m a
        L──►                                 ◄──R
        a == a ✓  →  m == m ✓  →  a == a ✓  → ...
        Tất cả đều khớp → true!
```

**Cách 3: Idiomatic Rust** — ngắn nhất, production code nên dùng

```rust
pub fn is_palindrome_idiomatic(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cleaned.iter().eq(cleaned.iter().rev())
}
```

So sánh iterator xuôi với iterator ngược. Nếu bằng nhau → palindrome. Cực ngắn gọn.

**Khi nào chọn cách nào?**

| Cách | Ưu điểm | Dùng khi |
|------|---------|---------|
| Vec + index | Dễ hiểu, dễ debug | Mới học, cần hiểu rõ logic |
| Two Pointers | Thể hiện hiểu pattern | Phỏng vấn, mở rộng thành bài phức tạp hơn |
| Idiomatic | Ngắn gọn, ít bug | Production code, code review |

> **Ghi chú Big-O:** Cách Two Pointers chỉ duyệt n/2 lần, nhưng ta vẫn viết O(n) vì Big-O bỏ hằng số. Nhớ lại chương 1!

### are_anagrams -- Kiểm tra đảo chữ *(Frequency Count)*

Hai từ là anagram (đảo chữ) nếu dùng cùng một bộ ký tự, chỉ khác thứ tự. Giống như xếp lại các viên gạch Scrabble.

Ý tưởng: đếm tần suất ký tự. Chuỗi 1 thì cộng, chuỗi 2 thì trừ. Nếu tất cả về 0 → anagram.

```rust
use std::collections::HashMap;

pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s1.chars().filter(|c| c.is_alphanumeric()) {
        *counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }

    for c in s2.chars().filter(|c| c.is_alphanumeric()) {
        *counts.entry(c.to_ascii_lowercase()).or_insert(0) -= 1;
    }

    counts.values().all(|&v| v == 0)
}
```

```
"Astronomer" vs "Moon starer"

Bước 1 -- đếm "Astronomer":
  a:1  s:1  t:1  r:2  o:2  n:1  m:1  e:1

Bước 2 -- trừ "Moonstarer":
  a:0  s:0  t:0  r:0  o:0  n:0  m:0  e:0

Tất cả = 0 → là anagram! ✓
```

### first_non_repeating_char -- Ký tự đầu tiên không lặp *(Frequency Count)*

Giống như điểm danh trong lớp -- ai chỉ xuất hiện đúng 1 lần?

Cần 2 lượt:
1. Đếm tần suất mỗi ký tự
2. Duyệt lại theo thứ tự gốc, tìm ký tự đầu tiên xuất hiện 1 lần

```rust
pub fn first_non_repeating_char(s: &str) -> Option<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();

    for &c in &chars {
        *counts.entry(c).or_insert(0) += 1;
    }

    chars.into_iter().find(|c| counts[c] == 1)
}
```

```
Input: "aabcc"

Bước 1 -- đếm:  a:2  b:1  c:2
Bước 2 -- duyệt: a(2 lần, bỏ) → a(2 lần, bỏ) → b(1 lần, CHỌN!)

Kết quả: Some('b')
```

Tại sao phải thu vào `Vec<char>` trước? Vì `HashMap` không giữ thứ tự. Ta cần duyệt lại theo thứ tự ban đầu.

### compress -- Nén chuỗi (Run-Length Encoding) *(Single Pass)*

Tưởng tượng bạn đọc to: "aaabbc" → "a xuất hiện 3 lần, b xuất hiện 2 lần, c xuất hiện 1 lần" → "a3b2c1".

Đây là dạng nén đơn giản gọi là Run-Length Encoding.

```rust
pub fn compress(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut chars = s.chars();
    let mut current = chars.next().unwrap();
    let mut count: usize = 1;

    for c in chars {
        if c == current {
            count += 1;        // cùng ký tự, tăng đếm
        } else {
            result.push(current);
            result.push_str(&count.to_string());
            current = c;       // chuyển sang ký tự mới
            count = 1;
        }
    }
    // đừng quên ký tự cuối!
    result.push(current);
    result.push_str(&count.to_string());

    result
}
```

```
Input:  "aaabbc"

Duyệt qua từng ký tự:
  a → a → a → b → b → c
  ╰─ count=3 ─╯  ╰─ 2 ─╯  ╰1╯

Kết quả: "a3b2c1"
```

#### Edge case: Khi nén lại **dài hơn** gốc

Không phải lúc nào nén cũng tốt hơn:

```
"ab"   → "a1b1"    (2 chars → 4 chars — phản tác dụng! ❌)
"abcd" → "a1b1c1d1" (4 chars → 8 chars — tệ gấp đôi! ❌)
"aaa"  → "a3"      (3 chars → 2 chars — có lợi! ✅)
```

Trường hợp xấu nhất: string không có ký tự nào lặp liên tiếp. Mỗi ký tự gốc trở thành 2 ký tự (chữ + số 1), kết quả dài gấp đôi.

```
"abcd" → "a1b1c1d1"

a  →  a  →  a  →  b  →  b  →  c  →  c  →  d  →  d
(gốc)  1    (gốc)  1    (gốc)  1    (gốc)  1
4 chars →  8 chars. Tệ!
```

#### compress_if_shorter -- chỉ nén khi có lợi

Trong thực tế, ta thường muốn giữ nguyên string gốc nếu nén không giúp gì:

```rust
pub fn compress_if_shorter(s: &str) -> String {
    let compressed = compress(s);
    if compressed.len() < s.len() {
        compressed
    } else {
        s.to_string()
    }
}
```

```
compress_if_shorter("aaaaabbbcc") → "a5b3c2"  (10 → 6, nén được!)
compress_if_shorter("ab")         → "ab"       (2 → 4? Không! Giữ gốc)
compress_if_shorter("aaabbc")     → "aaabbc"   (6 → 6, bằng nhau, giữ gốc)
```

💡 **Trade-off:** Run-Length Encoding chỉ hiệu quả khi có nhiều ký tự lặp liên tiếp (dữ liệu pixel, DNA sequences). Với văn bản thông thường (tin nhắn, code) thì gần như không bao giờ nén được.

## String Builder Pattern

Khi cần xây dựng string từ nhiều mảnh nhỏ, cách bạn ghép ảnh hưởng lớn đến tốc độ:

```rust
// 🐌 Chậm: String::new() + loop với +
// Mỗi lần + tạo String mới, copy dữ liệu cũ
let mut s = String::new();
for i in 0..1000 {
    s = s + &i.to_string();  // O(n) mỗi lần → tổng O(n²)
}

// 🚀 Nhanh: String::with_capacity() + push_str
// Cấp phát đủ bộ nhớ 1 lần, không cần realloc
let mut s = String::with_capacity(4000);
for i in 0..1000 {
    s.push_str(&i.to_string());  // O(1) amortized → tổng O(n)
}

// ✨ Idiomatic: Iterator + collect
// Rust tự tối ưu, code ngắn gọn nhất
let s: String = (0..1000).map(|i| i.to_string()).collect();
```

| Cách | Tốc độ | Khi nào dùng |
|------|--------|-------------|
| `+` trong loop | O(n²) | **Đừng dùng!** |
| `with_capacity` + `push_str` | O(n) | Biết trước độ dài, cần kiểm soát |
| Iterator + `collect` | O(n) | Transform chuỗi, idiomatic nhất |

**Quy tắc:** Biết trước độ dài? Dùng `with_capacity`. Đang transform? Dùng iterator + `collect`.

## Độ phức tạp

| Hàm | Thời gian | Bộ nhớ | Pattern | Ghi chú |
|-----|-----------|--------|---------|---------|
| `reverse_string` | O(n) | O(n) | Iterator | Duyệt 1 lần, tạo string mới |
| `is_palindrome` | O(n) | O(n) | Two Pointers | Vec + index, duyệt n/2 lần |
| `is_palindrome_two_pointers` | O(n) | O(n) | Two Pointers | left/right pointers, duyệt n/2 lần |
| `is_palindrome_idiomatic` | O(n) | O(n) | Iterator | So sánh xuôi-ngược |
| `are_anagrams` | O(n + m) | O(n + m) | Frequency Count | HashMap đếm 2 chuỗi |
| `first_non_repeating_char` | O(n) | O(n) | Frequency Count | 2 lượt duyệt + HashMap |
| `compress` | O(n) | O(n) | Single Pass | Duyệt 1 lần, nén RLE |
| `compress_if_shorter` | O(n) | O(n) | Single Pass | Nén + so sánh độ dài |

> **Nhắc lại:** `is_palindrome` duyệt n/2 lần nhưng vẫn viết O(n) vì Big-O bỏ hằng số (chương 1).

## Luyện nhận diện Pattern

Đọc mô tả bài toán, xác định nên dùng pattern nào, rồi mở đáp án để kiểm tra.

### Bài 1: Tìm tất cả anagram của pattern trong string dài

Cho string `s = "cbaebabacd"` và `p = "abc"`. Tìm tất cả vị trí bắt đầu trong `s` mà substring có độ dài bằng `p` là anagram của `p`.

*Gợi ý: Bạn có cần so sánh tần suất ký tự của từng "cửa sổ" không?*

<details>
<summary>Đáp án</summary>

**Pattern:** Sliding Window + Frequency Count

**Tại sao:** Ta cần kiểm tra tần suất ký tự trong mỗi cửa sổ có kích thước `p.len()`. Thay vì đếm lại từ đầu mỗi lần (O(n·m)), ta trượt cửa sổ: thêm ký tự mới bên phải, bỏ ký tự cũ bên trái, rồi so sánh 2 HashMap.

```rust
fn find_anagrams(s: &str, p: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    if s.len() < p.len() { return result; }

    let mut p_count = HashMap::new();
    let mut w_count = HashMap::new();

    for &c in &p { *p_count.entry(c).or_insert(0) += 1; }
    for &c in &s[..p.len()] { *w_count.entry(c).or_insert(0) += 1; }

    if w_count == p_count { result.push(0); }

    for i in p.len()..s.len() {
        *w_count.entry(s[i]).or_insert(0) += 1;       // thêm phải
        let left = s[i - p.len()];
        *w_count.entry(left).or_insert(0) -= 1;       // bỏ trái
        if w_count[&left] == 0 { w_count.remove(&left); }
        if w_count == p_count { result.push(i - p.len() + 1); }
    }
    result
}
```
</details>

### Bài 2: Kiểm tra 2 string có isomorphic không

"egg" và "add" là isomorphic (e↔a, g↔d). "foo" và "bar" thì không (o map vào cả a và r).

*Gợi ý: Mỗi ký tự ở string 1 phải map 1-1 với ký tự ở string 2. Cần kiểm tra cả 2 chiều.*

<details>
<summary>Đáp án</summary>

**Pattern:** Frequency Count (HashMap 2 chiều)

**Tại sao:** Ta cần mapping 1-1 giữa 2 bộ ký tự. HashMap từ s→t kiểm tra "e luôn map thành a". HashMap từ t→s kiểm tra "a chỉ được map bởi e". Thiếu 1 chiều sẽ sai.

```rust
fn is_isomorphic(s: &str, t: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    if s.len() != t.len() { return false; }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();

    for (&sc, &tc) in s.iter().zip(t.iter()) {
        match (s_to_t.get(&sc), t_to_s.get(&tc)) {
            (Some(&mapped), _) if mapped != tc => return false,
            (_, Some(&mapped)) if mapped != sc => return false,
            _ => {
                s_to_t.insert(sc, tc);
                t_to_s.insert(tc, sc);
            }
        }
    }
    true
}
```
</details>

### Bài 3: Tìm substring dài nhất không có ký tự lặp

Cho `s = "abcabcbb"`. Tìm độ dài substring dài nhất mà không có ký tự nào xuất hiện 2 lần. Đáp án: `"abc"` có độ dài 3.

*Gợi ý: Khi gặp ký tự đã thấy, bạn cần thu hẹp "cửa sổ" từ bên trái.*

<details>
<summary>Đáp án</summary>

**Pattern:** Sliding Window + HashSet

**Tại sao:** Ta duy trì một cửa sổ `[left, right]` không chứa ký tự lặp. Khi `right` gặp ký tự đã có trong window, ta dịch `left` sang phải cho đến khi bỏ được ký tự trùng. HashSet theo dõi ký tự hiện có trong window.

```rust
fn longest_unique_substring(s: &str) -> usize {
    use std::collections::HashSet;
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    let mut left = 0;
    let mut max_len = 0;

    for right in 0..chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        max_len = max_len.max(right - left + 1);
    }
    max_len
}
```
</details>

### Bài 4: Đếm số từ trong chuỗi (bỏ qua dấu cách thừa)

Cho `s = "  Hello   World  "`. Đếm số từ. Đáp án: 2.

*Gợi ý: Dấu cách liên tiếp và dấu cách ở đầu/cuối là bẫy. Rust có method sẵn cho việc này.*

<details>
<summary>Đáp án</summary>

**Pattern:** Single Pass (split + filter)

**Tại sao:** Bẫy phổ biến là dùng `split(' ')` — nó tạo ra các string rỗng giữa các dấu cách liên tiếp. Dùng `split_whitespace()` thì tự bỏ qua mọi loại whitespace liên tiếp.

```rust
fn count_words(s: &str) -> usize {
    // ❌ Sai: split(' ') tạo "" giữa các dấu cách liên tiếp
    // "  Hello   World  ".split(' ').count() → 7 (sai!)

    // ✅ Đúng: split_whitespace tự xử lý
    s.split_whitespace().count()  // → 2 (đúng!)
}
```
</details>

## Ví dụ

```rust
use rust_ds2a::strings::*;

fn main() {
    // === Đảo ngược ===
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("Việt"), "tệiV");   // hoạt động đúng với tiếng Việt!

    // === Palindrome (3 cách, cùng kết quả) ===
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome_two_pointers("A man a plan a canal Panama"));
    assert!(is_palindrome_idiomatic("madam"));
    assert!(!is_palindrome("hello"));

    // === Anagram ===
    assert!(are_anagrams("listen", "silent"));
    assert!(are_anagrams("Astronomer", "Moon starer"));
    assert!(!are_anagrams("hello", "world"));

    // === Ký tự không lặp ===
    assert_eq!(first_non_repeating_char("aabcc"), Some('b'));
    assert_eq!(first_non_repeating_char("aabb"), None);  // tất cả đều lặp

    // === Nén chuỗi ===
    assert_eq!(compress("aaabbc"), "a3b2c1");
    assert_eq!(compress("abcd"), "a1b1c1d1");  // không nén được

    // === Nén thông minh ===
    assert_eq!(compress_if_shorter("aaaaabbbcc"), "a5b3c2");  // nén được!
    assert_eq!(compress_if_shorter("ab"), "ab");               // giữ gốc
}
```

---

---

[← Arrays & Slices](./02-arrays.md) | [Singly Linked List →](../02-linear-structures/01-singly-linked-list.md)
