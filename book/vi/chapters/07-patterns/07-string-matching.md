# String Matching

> 💡 **Đừng lo lắng:** Chương này dài và có nhiều công thức. Nhưng đừng lo — mình sẽ đi **từng bước một**. Bạn không cần hiểu hết ngay lần đầu. Đọc phần Naive trước, hiểu rồi mới qua KMP. KMP khó nhất nằm ở LPS table — mình sẽ trace từng bước. Rabin-Karp thì dễ hơn nếu bạn đã quen hash từ Phần 4. Cứ từ từ, không ai sinh ra đã biết KMP cả.

## Đây là gì?

Bạn đã học Strings ở Phần 1 và Hashing ở Phần 4. Giờ mình kết hợp để tìm chuỗi con trong chuỗi lớn — như tìm một từ trong cuốn sách.

Tưởng tượng bạn có cuốn sách 500 trang, và cần tìm từ "algorithm". Bạn sẽ làm thế nào?

- **Cách 1 (Naive):** Đọc từng trang, từng dòng, từng chữ. Mỗi lần thấy chữ "a", bạn kiểm tra "l", "g", "o"... Nếu sai, quay lại và bắt đầu từ chữ tiếp theo.
- **Cách 2 (KMP):** Khi so sánh sai, bạn **nhớ** phần đã khớp và nhảy thông minh, không cần quay lại từ đầu.
- **Cách 3 (Rabin-Karp):** Thay vì so từng chữ, bạn tạo **fingerprint** (dấu vân tay) cho từ cần tìm, rồi so fingerprint. Chỉ khi fingerprint khớp mới kiểm tra chi tiết.

Cả 3 thuật toán đều trả về **danh sách vị trí** mà pattern xuất hiện trong text.

---

## Naive Search — Brute Force

### Ý tưởng

Đơn giản nhất: thử mọi vị trí trong text, tại mỗi vị trí so sánh từng ký tự với pattern.

### Ví dụ

Tìm `"abc"` trong `"xabcabc"`:

```
text:    x  a  b  c  a  b  c
         0  1  2  3  4  5  6

Vị trí 0: x vs a  -> Sai, bỏ qua
Vị trí 1: a vs a  -> Đúng
           b vs b  -> Đúng
           c vs c  -> Đúng  --> Tìm thấy tại vị trí 1!
Vị trí 2: b vs a  -> Sai, bỏ qua
Vị trí 3: c vs a  -> Sai, bỏ qua
Vị trí 4: a vs a  -> Đúng
           b vs b  -> Đúng
           c vs c  -> Đúng  --> Tìm thấy tại vị trí 4!

Kết quả: [1, 4]
```

### Code Rust

```rust
pub fn naive_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();

    if p.is_empty() || p.len() > t.len() {
        return result;
    }

    for i in 0..=t.len() - p.len() {
        let mut matched = true;
        for j in 0..p.len() {
            if t[i + j] != p[j] {
                matched = false;
                break;
            }
        }
        if matched {
            result.push(i);
        }
    }
    result
}
```

**Nhược điểm:** Khi pattern dài và text có nhiều ký tự lặp (ví dụ tìm `"aaab"` trong `"aaaaaaaaab"`), thuật toán phải so sánh rất nhiều lần rồi mới phát hiện sai ở ký tự cuối.

---

## KMP (Knuth-Morris-Pratt)

### Vấn đề của Naive

Khi so sánh sai, Naive quay lại vị trí tiếp theo trong text và bắt đầu lại từ đầu pattern. Nhưng nếu phần đã khớp có **prefix trùng suffix**, ta có thể tận dụng thông tin đó!

### LPS Table — chìa khóa của KMP

LPS = **Longest Proper Prefix which is also Suffix** (tiền tố dài nhất cũng là hậu tố).

"Proper" nghĩa là prefix không được bằng cả chuỗi.

Ví dụ với pattern `"ababaca"`:

```
Ký tự:     a  b  a  b  a  c  a
Index:      0  1  2  3  4  5  6
LPS:        0  0  1  2  3  0  1

Giải thích từng giá trị:
- lps[0] = 0  "a"       -> Không có proper prefix
- lps[1] = 0  "ab"      -> Prefix "a" != Suffix "b"
- lps[2] = 1  "aba"     -> Prefix "a" == Suffix "a"         (dài 1)
- lps[3] = 2  "abab"    -> Prefix "ab" == Suffix "ab"       (dài 2)
- lps[4] = 3  "ababa"   -> Prefix "aba" == Suffix "aba"     (dài 3)
- lps[5] = 0  "ababac"  -> Không có prefix nào trùng suffix
- lps[6] = 1  "ababaca" -> Prefix "a" == Suffix "a"         (dài 1)
```

### Xây LPS table — trace chi tiết từng bước

Đây là phần khó nhất của KMP. Mình sẽ đi **từng bước** với pattern `"aabaaab"`.

Ý tưởng: ta dùng 2 con trỏ — `i` duyệt qua pattern, `len` theo dõi độ dài prefix hiện tại đang khớp.

```
Pattern:  a  a  b  a  a  a  b
Index:    0  1  2  3  4  5  6

Khởi tạo: lps = [0, 0, 0, 0, 0, 0, 0], len = 0, i = 1

--- Bước 1: i=1, len=0 ---
  p[1]='a' == p[0]='a'?  CO!
  len = 1, lps[1] = 1, i = 2
  lps = [0, 1, 0, 0, 0, 0, 0]

--- Bước 2: i=2, len=1 ---
  p[2]='b' == p[1]='a'?  KHONG!
  len != 0, nên len = lps[0] = 0     (quay lại, KHONG tang i)

--- Bước 3: i=2, len=0 ---
  p[2]='b' == p[0]='a'?  KHONG!
  len == 0, nên lps[2] = 0, i = 3
  lps = [0, 1, 0, 0, 0, 0, 0]

--- Bước 4: i=3, len=0 ---
  p[3]='a' == p[0]='a'?  CO!
  len = 1, lps[3] = 1, i = 4
  lps = [0, 1, 0, 1, 0, 0, 0]

--- Bước 5: i=4, len=1 ---
  p[4]='a' == p[1]='a'?  CO!
  len = 2, lps[4] = 2, i = 5
  lps = [0, 1, 0, 1, 2, 0, 0]

--- Bước 6: i=5, len=2 ---
  p[5]='a' == p[2]='b'?  KHONG!
  len != 0, nên len = lps[1] = 1     (quay lại, KHONG tang i)
                                       ^^ day la buoc quan trong!

--- Bước 7: i=5, len=1 ---
  p[5]='a' == p[1]='a'?  CO!
  len = 2, lps[5] = 2, i = 6
  lps = [0, 1, 0, 1, 2, 2, 0]

--- Bước 8: i=6, len=2 ---
  p[6]='b' == p[2]='b'?  CO!
  len = 3, lps[6] = 3, i = 7
  lps = [0, 1, 0, 1, 2, 2, 3]

KET QUA: lps = [0, 1, 0, 1, 2, 2, 3]
```

**Bước 6 là điểm mấu chốt:** Khi `p[5] != p[2]`, ta KHÔNG reset `len` về 0 ngay. Thay vào đó, ta dùng `lps[len-1]` để "quay lại" một prefix ngắn hơn có thể vẫn khớp. Đây chính là lý do LPS table xây được trong O(m) — nó tự dùng chính nó!

### Cách xây LPS table

```rust
pub fn build_lps(pattern: &str) -> Vec<usize> {
    let p = pattern.as_bytes();
    let m = p.len();
    let mut lps = vec![0usize; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if p[i] == p[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];  // Quay lại, KHÔNG tăng i
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}
```

### KMP Matching — minh họa chi tiết

Tìm pattern `"abab"` trong text `"ababcababd"`:

```
LPS cho "abab": [0, 0, 1, 2]

Bước 1: So sánh text[0..4] với pattern
  text:    a  b  a  b  c  a  b  a  b  d
  pattern: a  b  a  b
           ^  ^  ^  ^
           Khớp hết! --> Tìm thấy tại vị trí 0

  j = lps[3] = 2  (nhảy thông minh, giữ 2 ký tự đã khớp)

Bước 2: Tiếp tục từ i=4, j=2
  text:    a  b  a  b  c  a  b  a  b  d
                       ^
  pattern:       a  b  a  b
                       ^
  text[4]='c' != pattern[2]='a'
  j = lps[1] = 0

Bước 3: Tiếp tục từ i=4, j=0
  text:    a  b  a  b  c  a  b  a  b  d
                       ^
  pattern:             a  b  a  b
                       ^
  text[4]='c' != pattern[0]='a'
  j = 0, nên i += 1

Bước 4: Từ i=5, j=0
  text:    a  b  a  b  c  a  b  a  b  d
                          ^  ^  ^  ^
  pattern:                a  b  a  b
                          ^  ^  ^  ^
  Khớp hết! --> Tìm thấy tại vị trí 5

Kết quả: [0, 5]
```

Điểm quan trọng: Ở bước 1, khi tìm thấy match, KMP **không quay lại** i=1 để thử lại. Nhờ LPS table, nó biết prefix `"ab"` của pattern trùng với suffix của phần vừa khớp, nên nhảy thẳng đến vị trí tiếp theo có khả năng khớp.

### Code KMP

```rust
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n {
        return result;
    }

    let lps = build_lps(pattern);
    let mut i = 0;
    let mut j = 0;

    while i < n {
        if t[i] == p[j] {
            i += 1;
            j += 1;
        }
        if j == m {
            result.push(i - m);
            j = lps[j - 1];
        } else if i < n && t[i] != p[j] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    result
}
```

---

## Rabin-Karp — Dùng Hash

### Ý tưởng

Thay vì so sánh từng ký tự, ta tính **hash** (fingerprint) cho pattern. Rồi trượt một cửa sổ trên text, mỗi lần tính hash của cửa sổ đó. Nếu hash khớp, mới so sánh chi tiết.

Cái hay là: ta dùng **rolling hash** — khi trượt cửa sổ sang phải 1 ký tự, ta chỉ cần bỏ ký tự đầu và thêm ký tự cuối, thay vì tính lại hash từ đầu.

### Rolling Hash — tại sao O(1) mỗi lần trượt?

Giả sử pattern dài `m = 3`, base = 256. Hash là polynomial:

```
Hash("abc") = a * 256² + b * 256¹ + c * 256⁰
```

Khi trượt cửa sổ từ `"abc"` sang `"bcd"`:

```
Hash("abc") = a * 256² + b * 256¹ + c * 256⁰

Bước 1: Bỏ ký tự đầu (a)
  Hash("abc") - a * 256²  =  b * 256¹ + c * 256⁰

Bước 2: Nhân tất cả với 256 (đẩy mọi ký tự lên 1 bậc)
  (b * 256¹ + c * 256⁰) * 256  =  b * 256² + c * 256¹

Bước 3: Cộng ký tự mới (d)
  b * 256² + c * 256¹ + d * 256⁰  =  Hash("bcd")
```

**3 phép tính: trừ, nhân, cộng. Luôn O(1), bất kể pattern dài bao nhiêu!**

Đây là lý do Rabin-Karp mạnh — mỗi cửa sổ chỉ tốn O(1) thay vì O(m) để tính hash.

### Hash Collision — kẻ phá bĩnh

Vì hash dùng modulo (chia dư), hai chuỗi khác nhau có thể ra cùng hash. Ví dụ:

```
Hash("abc") % 1000000007 = 6382179
Hash("xyz") % 1000000007 = 6382179    <-- trùng! (ví dụ giả định)
```

Khi hash khớp, ta **phải** kiểm tra lại từng ký tự để chắc chắn. Nếu không, kết quả sẽ sai.

**Tại sao dùng modulus lớn (10^9 + 7)?** Modulus càng lớn, xác suất collision càng nhỏ. Con số `1_000_000_007` là số nguyên tố, giúp hash phân bố đều hơn.

### Minh họa

Tìm `"ab"` trong `"xababc"`:

```
Pattern hash: hash("ab") = 24930

i=0: hash("xa") = 30817  != 24930  -> Bỏ qua
i=1: hash("ab") = 24930  == 24930  -> Kiểm tra: "ab" == "ab" -> Tìm thấy!
i=2: hash("ba") = 25185  != 24930  -> Bỏ qua
i=3: hash("ab") = 24930  == 24930  -> Kiểm tra: "ab" == "ab" -> Tìm thấy!
i=4: hash("bc") = 25187  != 24930  -> Bỏ qua

Kết quả: [1, 3]
```

### Code Rabin-Karp

```rust
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n { return result; }

    let base: u64 = 256;
    let modulus: u64 = 1_000_000_007;

    // Tính base^(m-1) % modulus
    let mut h: u64 = 1;
    for _ in 0..m - 1 {
        h = (h * base) % modulus;
    }

    // Tính hash cho pattern và cửa sổ đầu tiên
    let mut p_hash: u64 = 0;
    let mut t_hash: u64 = 0;
    for i in 0..m {
        p_hash = (p_hash * base + p[i] as u64) % modulus;
        t_hash = (t_hash * base + t[i] as u64) % modulus;
    }

    for i in 0..=n - m {
        if p_hash == t_hash {
            if t[i..i + m] == p[..] {
                result.push(i);
            }
        }
        if i < n - m {
            // Rolling hash: bỏ ký tự đầu, thêm ký tự cuối
            t_hash = (t_hash + modulus - (t[i] as u64 * h) % modulus) % modulus;
            t_hash = (t_hash * base + t[i + m] as u64) % modulus;
        }
    }
    result
}
```

**Tại sao cần kiểm tra lại khi hash khớp?** Vì hai chuỗi khác nhau có thể có cùng hash (hash collision). Kiểm tra từng ký tự đảm bảo kết quả chính xác.

---

## Pitfalls — Bẫy thường gặp

### 1. Hash collision trong Rabin-Karp

❌ **Sai:** Hash khớp thì chuỗi khớp, không cần kiểm tra lại.

```rust
// SAI - tin hash mù quáng
if p_hash == t_hash {
    result.push(i);  // Có thể sai!
}
```

✅ **Đúng:** Luôn kiểm tra từng ký tự khi hash khớp.

```rust
// DUNG - kiem tra lai
if p_hash == t_hash {
    if t[i..i + m] == p[..] {  // Xác nhận thật sự khớp
        result.push(i);
    }
}
```

💡 **Tại sao:** Hash collision xảy ra khi 2 chuỗi khác nhau có cùng hash value. Modulus càng nhỏ, collision càng nhiều. Nếu bỏ bước kiểm tra, bạn sẽ trả về false positive.

### 2. KMP failure function off-by-one

❌ **Sai:** Khi tìm thấy match (j == m), dùng `j = lps[j]`.

```rust
// SAI - index out of bounds!
if j == m {
    result.push(i - m);
    j = lps[j];  // lps chi co index 0..m-1, j=m la out of bounds!
}
```

✅ **Đúng:** Dùng `j = lps[j - 1]`.

```rust
// DUNG
if j == m {
    result.push(i - m);
    j = lps[j - 1];  // Quay lai prefix dai nhat co the khop tiep
}
```

💡 **Tại sao:** LPS table có `m` phần tử, index từ `0` đến `m-1`. Khi `j == m`, `lps[j]` vượt ngoài mảng. Ta cần `lps[j-1]` vì đó là LPS value của ký tự cuối cùng trong pattern.

### 3. Quên xử lý len != 0 trong build_lps

❌ **Sai:** Khi mismatch, luôn set `lps[i] = 0` và tăng `i`.

```rust
// SAI - bo qua thong tin tu prefix truoc do
if p[i] != p[len] {
    lps[i] = 0;
    i += 1;
}
```

✅ **Đúng:** Khi `len != 0`, quay lại bằng `len = lps[len-1]` mà KHÔNG tăng `i`.

```rust
// DUNG
if p[i] != p[len] {
    if len != 0 {
        len = lps[len - 1];  // Thu prefix ngan hon, KHONG tang i
    } else {
        lps[i] = 0;
        i += 1;
    }
}
```

💡 **Tại sao:** Khi mismatch tại `len > 0`, có thể một prefix ngắn hơn vẫn khớp. Nếu bỏ qua bước này, LPS table sẽ sai, dẫn đến KMP bỏ sót kết quả.

---

## So sánh 3 thuật toán

| Thuật toán | Ưu điểm | Nhược điểm | Khi nào dùng? |
|-----------|---------|------------|---------------|
| **Naive** | Đơn giản, dễ hiểu | Chậm với text/pattern lớn | Text ngắn, debug |
| **KMP** | Luôn O(n+m), ổn định | Cần xây LPS table | Pattern lặp nhiều, cần worst-case tốt |
| **Rabin-Karp** | Dễ mở rộng cho multi-pattern | Worst case vẫn O(n*m) | Tìm nhiều pattern cùng lúc |

---

## Bảng độ phức tạp

| Thuật toán | Thời gian (trung bình) | Thời gian (xấu nhất) | Bộ nhớ |
|-----------|----------------------|---------------------|--------|
| **Naive Search** | O(n * m) | O(n * m) | O(1) |
| **KMP** | O(n + m) | O(n + m) | O(m) |
| **Rabin-Karp** | O(n + m) | O(n * m) | O(1) |

Trong đó: **n** = độ dài text, **m** = độ dài pattern.

---

## Khi nào dùng thuật toán nào?

| Tình huống | Dùng gì? | Lý do |
|-----------|---------|-------|
| Text ngắn (< 1000 ký tự) | **Naive** | Overhead của KMP/Rabin-Karp không đáng |
| Pattern có nhiều ký tự lặp (`"aaaa"`, `"abab"`) | **KMP** | Naive sẽ chậm vì so sánh nhiều, KMP nhảy thông minh |
| Tìm **nhiều pattern** cùng lúc trong 1 text | **Rabin-Karp** | Tính hash 1 lần cho text, so với nhiều pattern hash |
| Cần **worst-case guarantee** | **KMP** | Luôn O(n+m), không phụ thuộc vào hash collision |
| Phỏng vấn, không nhớ KMP | **Rabin-Karp** | Dễ code hơn KMP nếu bạn quen hash |
| Production code | **Dùng thư viện** | `str::find()`, `regex` crate đã tối ưu sẵn |

---

## Thế giới thật dùng gì?

Bạn có thể thắc mắc: "OK mình học 3 thuật toán, nhưng grep hay IDE dùng gì?"

### `grep` dùng Boyer-Moore

`grep` (và hầu hết text editor) dùng **Boyer-Moore** hoặc biến thể của nó. Ý tưởng ngược lại với Naive — Boyer-Moore so sánh **từ cuối pattern** về đầu. Khi gặp mismatch, nó có thể nhảy **cả đoạn dài**, nhanh hơn KMP trong thực tế.

```
Text:    T H E _ C A T _ S A T _ O N _ T H E _ M A T
Pattern: T H E _ M A T

So sánh từ cuối: T vs T -> khớp, A vs A -> khớp, M vs C -> SAI!
'C' không có trong pattern -> nhảy 7 ký tự!

Rất nhanh vì phần lớn ký tự được bỏ qua hoàn toàn.
```

Boyer-Moore trung bình nhanh hơn KMP, nhưng worst case vẫn O(n*m). Vì vậy grep dùng biến thể Boyer-Moore-Horspool để cân bằng.

### Rust `str::find()` dùng Two-Way Algorithm

Rust standard library dùng **Two-Way algorithm** — một thuật toán ít nổi tiếng hơn nhưng rất hay:
- **O(n + m)** worst-case (như KMP)
- **O(1) bộ nhớ** (không cần LPS table!)
- Chia pattern thành 2 phần, so sánh thông minh

Đây là lý do bạn nên dùng `str::find()` hoặc `str::contains()` trong production code — nó đã được tối ưu hơn bất kỳ implementation nào bạn tự viết.

### Regex engine

Khi bạn dùng regex (crate `regex` trong Rust), engine bên trong dùng **Aho-Corasick** (multi-pattern matching trên automaton) kết hợp với **NFA/DFA** cho các pattern phức tạp.

---

## Rust Ecosystem

```rust
// str::find() - tim vi tri dau tien
let text = "hello world hello";
assert_eq!(text.find("world"), Some(6));

// str::contains() - kiem tra co chua khong
assert!(text.contains("world"));

// str::matches() - dem so lan xuat hien
let count = text.matches("hello").count();
assert_eq!(count, 2);

// str::match_indices() - lay tat ca vi tri
let positions: Vec<(usize, &str)> = text.match_indices("hello").collect();
assert_eq!(positions, vec![(0, "hello"), (12, "hello")]);

// regex crate - pattern phuc tap
// Cargo.toml: regex = "1"
// use regex::Regex;
// let re = Regex::new(r"\b\w{5}\b").unwrap();  // tim tu 5 ky tu
// for mat in re.find_iter(text) {
//     println!("{} at {}", mat.as_str(), mat.start());
// }
```

**Lời khuyên:** Trong production, luôn dùng `str::find()` hoặc `regex` crate. Tự implement chỉ để học và phỏng vấn.

---

## Practice — Luyện tập

### Implement strStr() — LeetCode #28

> Tìm vị trí đầu tiên của `needle` trong `haystack`. Trả về `-1` nếu không tìm thấy.

Đây là bài kinh điển để luyện string matching. Bạn có thể dùng Naive, KMP, hoặc Rabin-Karp.

```rust
// Gợi ý: đây chính là str::find() nhưng tự viết
fn str_str(haystack: &str, needle: &str) -> i32 {
    if needle.is_empty() { return 0; }
    // Dùng KMP hoặc Rabin-Karp ở đây
    // ...
    -1
}
```

### Repeated Substring Pattern — LeetCode #459

> Cho chuỗi `s`, kiểm tra xem `s` có thể tạo từ việc lặp lại một substring hay không.
>
> Ví dụ: `"abab"` -> `true` (lặp `"ab"`), `"abc"` -> `false`

**Mẹo hay:** Nối `s + s`, bỏ ký tự đầu và cuối. Nếu tìm thấy `s` trong chuỗi mới -> `true`. Dùng KMP để tìm.

```rust
fn repeated_substring(s: &str) -> bool {
    let doubled = format!("{}{}",s, s);
    let inner = &doubled[1..doubled.len() - 1];
    // Tim s trong inner bang KMP
    kmp_search(inner, s).len() > 0
}
```

### Longest Happy Prefix — LeetCode #1392

> Tìm chuỗi dài nhất vừa là prefix vừa là suffix (nhưng không phải cả chuỗi).
>
> Ví dụ: `"level"` -> `"l"`, `"ababab"` -> `"abab"`

**Mẹo:** Đây chính xác là giá trị `lps[m-1]` trong KMP! Bài này kiểm tra xem bạn có thật sự hiểu LPS table không.

```rust
fn longest_happy_prefix(s: &str) -> String {
    let lps = build_lps(s);
    let len = lps[s.len() - 1];
    s[..len].to_string()
}
```

---

## Tổng kết

- **Naive Search**: cách đơn giản nhất, thử mọi vị trí. Tốt cho text ngắn.
- **KMP**: dùng LPS table để nhảy thông minh, không bao giờ quay lại text. Luôn O(n+m).
- **Rabin-Karp**: dùng rolling hash để so sánh nhanh, chỉ kiểm tra chi tiết khi hash khớp.
- Ba thuật toán cho cùng kết quả, chỉ khác nhau về tốc độ và cách tiếp cận.
- Trong thực tế: `grep` dùng Boyer-Moore, Rust `str::find()` dùng Two-Way algorithm, regex engine dùng Aho-Corasick + NFA/DFA.
- Tự implement để hiểu, dùng thư viện trong production.

---

## Tiếp theo

Chương sau: **[Graph Patterns](./08-graph-patterns.md)** — tổng hợp các pattern graph bạn đã học ở Phần 5: BFS, DFS, Union-Find, Topological Sort. Chương đó không dạy gì mới — chỉ chỉ cho bạn cách nhận diện "bài này dùng cái gì" qua một flowchart đơn giản.

---

[← Top-K Problems](./06-top-k.md) | [Graph Patterns →](./08-graph-patterns.md)
