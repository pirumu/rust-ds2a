# String

## Đây là gì?

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

## Hoạt động như thế nào?

Chương này cài đặt 5 bài toán string kinh điển, hay gặp trong phỏng vấn:

1. **Đảo ngược string** -- "hello" -> "olleh"
2. **Kiểm tra palindrome** -- "racecar" đọc xuôi ngược giống nhau
3. **Kiểm tra anagram** -- "listen" và "silent" cùng ký tự
4. **Ký tự đầu tiên không lặp** -- "aabcc" -> 'b'
5. **Nén string** -- "aaabbc" -> "a3b2c1"

## Code Rust

Tất cả code nằm trong `src/strings.rs`.

### reverse_string -- Đảo ngược chuỗi

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

### is_palindrome -- Kiểm tra đọc xuôi ngược giống nhau

Palindrome (chuỗi đối xứng) là chuỗi đọc xuôi hay ngược đều giống nhau. Ví dụ: "racecar", "madam".

Bài toán thêm phần thú vị: bỏ qua dấu cách và hoa/thường. "A man a plan a canal Panama" vẫn là palindrome.

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

Cách hoạt động:

```
Input:  "A man a plan a canal Panama"
Bước 1: Lọc + lowercase → "amanaplanacanalpanama"
Bước 2: So sánh đầu-cuối:
        a m a n a p l a n a c a n a l p a n a m a
        ↕                                       ↕
        a                                       a  ✓
          ↕                                   ↕
          m                                   m    ✓
            ...tất cả đều khớp...               ✓
Kết quả: true -- là palindrome!
```

### are_anagrams -- Kiểm tra đảo chữ

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

### first_non_repeating_char -- Ký tự đầu tiên không lặp

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

### compress -- Nén chuỗi (Run-Length Encoding)

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

## Độ phức tạp

| Hàm | Thời gian | Bộ nhớ | Giải thích đơn giản |
|-----|-----------|--------|---------------------|
| `reverse_string` | O(n) | O(n) | Duyệt 1 lần, tạo string mới |
| `is_palindrome` | O(n) | O(n) | Duyệt 1 lần + vector ký tự |
| `are_anagrams` | O(n + m) | O(n + m) | Duyệt cả 2 chuỗi + HashMap |
| `first_non_repeating_char` | O(n) | O(n) | 2 lượt duyệt + HashMap |
| `compress` | O(n) | O(n) | Duyệt 1 lần, tạo string mới |

Trong đó n, m là độ dài 2 chuỗi. Tất cả đều **tuyến tính** -- nhanh!

## Ví dụ

```rust
use rust_ds2a::strings::*;

fn main() {
    // === Đảo ngược ===
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("Việt"), "tệiV");   // hoạt động đúng với tiếng Việt!

    // === Palindrome ===
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("A man a plan a canal Panama"));
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
    assert_eq!(compress("abcd"), "a1b1c1d1");  // không nén được thì giữ nguyên
}
```
