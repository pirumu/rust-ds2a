# String Matching

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

### Rolling Hash

```
Hash("abc") = a * 256² + b * 256¹ + c * 256⁰

Trượt sang phải:
Hash("bcd") = (Hash("abc") - a * 256²) * 256 + d

Chỉ cần O(1) cho mỗi lần trượt!
```

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

## Tóm tắt

- **Naive Search**: cách đơn giản nhất, thử mọi vị trí. Tốt cho text ngắn.
- **KMP**: dùng LPS table để nhảy thông minh, không bao giờ quay lại text. Luôn O(n+m).
- **Rabin-Karp**: dùng rolling hash để so sánh nhanh, chỉ kiểm tra chi tiết khi hash khớp.
- Ba thuật toán cho cùng kết quả, chỉ khác nhau về tốc độ và cách tiếp cận.

Hãy nhớ: trong thực tế, hàm `str::find()` của Rust và `Ctrl+F` trong trình duyệt đều dùng các thuật toán tương tự!
