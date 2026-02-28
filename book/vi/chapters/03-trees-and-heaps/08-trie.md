# Trie (Cây tiền tố)

## Đây là gì?

> Trie trông lạ vì khác hoàn toàn BST/AVL mà bạn vừa học -- không có so sánh trái/phải, không có rotation, không có balance factor. Thay vào đó, mỗi node là 1 ký tự, và đường đi từ root đến node = 1 chuỗi. Nếu bạn hiểu tree traversal (đi từ root xuống leaf), bạn đã hiểu core concept của Trie rồi. Phần code cũng đơn giản đáng ngạc nhiên -- insert chỉ là vòng `for` đi qua từng ký tự.

Mở điện thoại lên, gõ "xin" vào thanh tìm kiếm. Ngay lập tức xuất hiện gợi ý: "xin chào", "xin lỗi", "xin phép"... Bạn chưa gõ xong mà máy đã đoán được. Làm sao nó làm được?

Câu trả lời: **Trie** (đọc là "try", lấy từ chữ "re**trie**val"). Trie là cấu trúc cây mà mỗi node đại diện cho **một ký tự**. Các từ có chung phần đầu (prefix) sẽ **chia sẻ đường đi** từ root. Vì vậy, tìm kiếm prefix cực nhanh: O(m) với m = độ dài prefix, không phụ thuộc vào tổng số từ đã lưu.

**Ở đâu trong thực tế?**
- **Autocomplete**: gợi ý khi gõ tìm kiếm (Google, IDE, bàn phím điện thoại)
- **Spell checker**: kiểm tra chính tả
- **IP routing table**: router internet dùng binary trie
- **Từ điển**: tìm từ nhanh hơn HashMap cho prefix query

Và đây là cấu trúc xuất hiện **cực nhiều** trong phỏng vấn (LeetCode có ~30 bài Trie) và trong production (autocomplete, spell check, router, DNS). Học kỹ chương này rất đáng.

---

## Trie vs BST vs HashMap cho string

Trước khi đi sâu, đặt Trie vào bức tranh tổng. Bạn đã biết HashMap (dùng bên trong Trie!) và BST. Mỗi cấu trúc có "siêu năng lực" riêng:

| Thao tác | HashMap\<String\> | BTreeMap\<String\> | Trie |
|----------|-------------------|-------------------|------|
| Exact search "cat" | **O(m)** hash+compare | O(m log n) | O(m) |
| Prefix search "ca*" | **O(N × m)** scan all 💀 | O(m log n + k) range | **O(m + k)** ⚡ |
| Autocomplete top-K | O(N × m) scan + sort | O(m log n + k) | **O(m + k)** ⚡ |
| Insert | O(m) | O(m log n) | O(m) |
| Memory | O(N × m) | O(N × m) | O(N × m) nhưng shared prefix |
| Sorted iteration | ❌ | ✅ | ✅ (lexicographic!) |

*(m = độ dài chuỗi, n = số chuỗi, k = số kết quả, N = tổng số chuỗi)*

**Kết luận**: HashMap thắng exact lookup. BTreeMap thắng range query trên sorted data. Trie thắng mọi thứ liên quan đến **prefix**. Mỗi cấu trúc có "siêu năng lực" riêng -- chọn đúng tool cho đúng bài toán.

---

## Hoạt động như thế nào?

### Chèn từng ký tự (insert)

Khi chèn một từ, ta đi từ root, mỗi bước đi theo 1 ký tự. Nếu chưa có node cho ký tự đó, tạo mới. Cuối từ, đánh dấu "kết thúc từ".

```
Chèn "cat":

  root
   |
   c
   |
   a
   |
   t*        (* = kết thúc từ)
```

### Chia sẻ prefix

Đây là sức mạnh chính của Trie. Các từ cùng phần đầu chia sẻ node:

```
Chèn "cat", "car", "card", "care":

  root
   |
   c
   |
   a
  / \
 t*   r*
     / \
    d*   e*

"cat"  → c → a → t*
"car"  → c → a → r*
"card" → c → a → r → d*
"care" → c → a → r → e*

Đường c → a được chia sẻ bởi TẤT CẢ 4 từ!
```

Giống cây gia phả: "cat" và "car" là anh em (chung cha "ca"). "card" và "care" là con của "car".

Xem ví dụ lớn hơn để thấy rõ sức mạnh memory sharing:

```
Chèn: "the", "there", "their", "them", "then", "thin", "this"

                root
                 |
                 t
                 |
                 h
                / \
               e    i
              /|\    |\
             r  m* n* n* s*
            / \
           e*  i
                |
                r*

7 từ, tổng 30 ký tự, nhưng Trie chỉ có 14 node!
  Tiết kiệm: 30 - 14 = 16 node (53% ít hơn!)

So sánh: HashMap<String> lưu 7 string riêng biệt = 30 bytes nội dung.
  Trie chia sẻ prefix "th" cho TẤT CẢ 7 từ, "the" cho 5 từ.
```

> **Ghi nhớ**: Với dataset thực tế (từ điển tiếng Anh ~170K từ), Trie tiết kiệm rất nhiều memory nhờ shared prefix. Nhưng với dataset random (ít shared prefix), Trie có thể tốn hơn HashMap vì overhead mỗi node. Prefix sharing càng nhiều → Trie càng hiệu quả.

---

### Search vs starts_with

Hai thao tác tìm kiếm khác nhau:

**search("car")**: Đi c → a → r. Kiểm tra `is_end` ở node r. `is_end = true` → "car" là từ hoàn chỉnh → trả về `true`.

**search("ca")**: Đi c → a. Kiểm tra `is_end` ở node a. `is_end = false` → "ca" KHÔNG phải từ hoàn chỉnh → trả về `false`.

**starts_with("ca")**: Đi c → a. Node tồn tại → CÓ từ bắt đầu bằng "ca" → trả về `true`.

```
Tóm tắt:
  search     = "Từ này có TRONG từ điển không?"
  starts_with = "Có từ nào BẮT ĐẦU bằng chuỗi này không?"
```

---

### Collect words -- autocomplete engine

Đây là thao tác quan trọng nhất cho autocomplete mà `search` và `starts_with` chưa đủ. Bạn cần **liệt kê tất cả từ** có prefix nhất định:

```
Collect tất cả từ có prefix "ca" từ trie chứa
{"cat", "car", "card", "care", "cap", "dog"}:

Bước 1: Đi theo prefix "ca" → đến node 'a'
Bước 2: DFS từ node 'a', thu thập tất cả từ hoàn chỉnh

  DFS trace:
    a → p (is_end=true) → thu "cap"
    a → r (is_end=true) → thu "car"
      r → d (is_end=true) → thu "card"
      r → e (is_end=true) → thu "care"
    a → t (is_end=true) → thu "cat"

  Kết quả: ["cap", "car", "card", "care", "cat"]
  Time: O(m + k) -- m để đến prefix node, k = tổng ký tự trong kết quả
```

Đây chính là "engine" đằng sau autocomplete. Gõ prefix → collect words → hiển thị top kết quả.

Code Rust (có trong `src/trie.rs`):

```rust
pub fn collect_words_with_prefix(&self, prefix: &str) -> Vec<String> {
    let mut current = &self.root;
    // Đi đến prefix node
    for ch in prefix.chars() {
        match current.children.get(&ch) {
            Some(node) => current = node,
            None => return vec![],  // prefix không tồn tại
        }
    }
    // DFS thu thập tất cả từ
    let mut results = Vec::new();
    let mut path = prefix.to_string();
    Self::dfs(current, &mut path, &mut results);
    results
}

fn dfs(node: &TrieNode, path: &mut String, results: &mut Vec<String>) {
    if node.is_end {
        results.push(path.clone());
    }
    let mut keys: Vec<&char> = node.children.keys().collect();
    keys.sort();  // sắp xếp để kết quả theo thứ tự alphabet
    for &ch in &keys {
        path.push(*ch);
        Self::dfs(&node.children[&ch], path, results);
        path.pop();  // backtrack
    }
}
```

Chú ý kỹ thuật **backtrack**: `push` ký tự trước khi đi sâu, `pop` khi quay lại. Pattern này giống hệt DFS trên tree mà bạn đã học!

---

### Xóa (delete)

Xóa một từ gồm 2 bước: bỏ đánh dấu `is_end`, rồi dọn node rỗng. Trace chi tiết:

```
Trie chứa: {"app", "apple", "api"}

          root
           |
           a
           |
           p
          / \
         p*   i*
         |
         l
         |
         e*
```

**Xóa "app":**

```
Bước 1: Đi a → p → p. Node 'p' (thứ 2) có is_end=true → đặt false
Bước 2: Node 'p' (thứ 2) CÓ con (l) → KHÔNG xóa node
Kết quả: "app" đã xóa, "apple" vẫn còn, "api" vẫn còn

          root
           |
           a
           |
           p
          / \
         p    i*      ← 'p' (thứ 2) mất dấu *, nhưng vẫn còn con 'l'
         |
         l
         |
         e*
```

**Xóa tiếp "apple":**

```
Bước 1: Đi a → p → p → l → e. Node 'e' có is_end=true → đặt false
Bước 2: Node 'e' KHÔNG có con, is_end=false → xóa node 'e'
Bước 3: Node 'l' KHÔNG có con, is_end=false → xóa node 'l'
Bước 4: Node 'p' (thứ 2) KHÔNG có con, is_end=false → xóa node 'p'
Bước 5: Node 'p' (thứ 1) CÓ con (i) → DỪNG

Kết quả:
          root
           |
           a
           |
           p
           |
           i*

Chỉ còn "api". Nhánh "app"/"apple" đã bị dọn sạch.
```

Backtrack xóa ngược từ leaf lên -- chỉ xóa node khi nó KHÔNG có con VÀ KHÔNG phải kết thúc từ.

---

## HashMap\<char, TrieNode\> vs Array [26]

Doc hiện tại dùng `HashMap`. Nhưng trong phỏng vấn, bạn sẽ thấy cách dùng array. Đây là trade-off quan trọng:

```rust
// Cách 1: HashMap (code trong project này dùng)
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}
// ✅ Hỗ trợ mọi Unicode (tiếng Việt, emoji)
// ✅ Memory efficient khi alphabet lớn hoặc node sparse
// ❌ Hash overhead mỗi lookup
// ❌ Không cache-friendly (HashMap scatter trên heap)

// Cách 2: Array [26] (competitive programming)
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],  // chỉ a-z
    is_end: bool,
}
// ✅ O(1) lookup (index trực tiếp: children[ch - 'a'])
// ✅ Cache-friendly hơn
// ❌ Chỉ hỗ trợ a-z (26 ký tự)
// ❌ Lãng phí memory nếu node có ít children (26 slot × 8 bytes = 208 bytes/node)

// Cách 3: Array [256] (byte-level, IP routing)
struct TrieNode {
    children: [Option<Box<TrieNode>>; 256],
    is_end: bool,
}
// Dùng cho binary data / IP address lookup
```

**Kết luận**: HashMap cho general purpose (Unicode, production code). Array [26] cho competitive programming (nhanh hơn, chỉ lowercase). Production code thường dùng compressed trie (xem phần tiếp theo).

---

## Compressed Trie / Radix Tree

Khi bạn đọc source code production, bạn sẽ gặp khái niệm này. Ý tưởng đơn giản: gộp chuỗi node chỉ có 1 con thành 1 node duy nhất.

```
Trie thường:              Radix tree (compressed):
    root                      root
     |                         |
     c                        ca
     |                       / \
     a                     t*   r*
    / \                        / \
   t*   r*                   d*   e*
       / \
      d*   e*

Trie: 6 node             Radix: 4 node
  Mỗi node = 1 ký tự       Mỗi node = 1 chuỗi (edge label)
  Khi node chỉ có 1 con    Merge vào edge
```

- **Radix tree** = compressed trie = Patricia tree (3 tên, cùng 1 thứ)
- Ít node hơn → ít memory, ít pointer chasing → nhanh hơn
- Dùng trong: `hashbrown` (HashMap của Rust), Linux kernel routing, `radix_trie` crate

> **Ghi nhớ**: Bạn không cần implement radix tree khi đang học. Nhưng biết nó tồn tại giúp rất nhiều khi đọc source code production.

---

## Code Rust

Code đầy đủ nằm trong `src/trie.rs`.

### Cấu trúc dữ liệu

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,  // ánh xạ ký tự → node con
    is_end: bool,                        // đánh dấu kết thúc từ
}

pub struct Trie {
    root: TrieNode,
}
```

Dùng `HashMap<char, TrieNode>` nên hỗ trợ mọi ký tự Unicode (tiếng Việt, emoji, v.v.), không chỉ 26 chữ cái.

### Chèn (insert)

```rust
pub fn insert(&mut self, word: &str) {
    let mut current = &mut self.root;
    for ch in word.chars() {
        // entry().or_default(): nếu chưa có → tạo mới, nếu có → dùng lại
        current = current.children.entry(ch).or_default();
    }
    current.is_end = true;  // đánh dấu kết thúc từ
}
```

Mỗi ký tự = 1 bước đi xuống trong cây. `entry().or_default()` là cách Rust tạo node khi cần.

### Tìm kiếm (search)

```rust
pub fn search(&self, word: &str) -> bool {
    let mut current = &self.root;
    for ch in word.chars() {
        match current.children.get(&ch) {
            Some(node) => current = node,   // có → đi tiếp
            None => return false,            // không có → từ không tồn tại
        }
    }
    current.is_end  // phải là kết thúc từ mới tính
}
```

### Kiểm tra prefix (starts_with)

```rust
pub fn starts_with(&self, prefix: &str) -> bool {
    let mut current = &self.root;
    for ch in prefix.chars() {
        match current.children.get(&ch) {
            Some(node) => current = node,
            None => return false,
        }
    }
    true  // chỉ cần đường đi tồn tại, không cần is_end
}
```

Gần giống `search`, nhưng không kiểm tra `is_end`. Đơn giản vậy thôi!

### Thu thập từ theo prefix (collect_words_with_prefix)

```rust
pub fn collect_words_with_prefix(&self, prefix: &str) -> Vec<String> {
    let mut current = &self.root;
    for ch in prefix.chars() {
        match current.children.get(&ch) {
            Some(node) => current = node,
            None => return vec![],
        }
    }
    let mut results = Vec::new();
    let mut path = prefix.to_string();
    Self::dfs(current, &mut path, &mut results);
    results
}

fn dfs(node: &TrieNode, path: &mut String, results: &mut Vec<String>) {
    if node.is_end {
        results.push(path.clone());
    }
    let mut keys: Vec<&char> = node.children.keys().collect();
    keys.sort();
    for &ch in &keys {
        path.push(*ch);
        Self::dfs(&node.children[&ch], path, results);
        path.pop();  // backtrack
    }
}
```

### Xóa (delete)

```rust
pub fn delete(&mut self, word: &str) -> bool {
    fn remove(node: &mut TrieNode, word: &[char], depth: usize) -> (bool, bool) {
        if depth == word.len() {
            if !node.is_end {
                return (false, false); // word not found
            }
            node.is_end = false;
            return (true, node.children.is_empty());
        }

        let ch = word[depth];
        let Some(child) = node.children.get_mut(&ch) else {
            return (false, false);
        };

        let (found, should_delete_child) = remove(child, word, depth + 1);
        if should_delete_child {
            node.children.remove(&ch);
        }

        (found, !node.is_end && node.children.is_empty())
    }

    let chars: Vec<char> = word.chars().collect();
    let (found, _) = remove(&mut self.root, &chars, 0);
    found
}
```

Đệ quy đi đến cuối từ, bỏ `is_end`, rồi backtrack lên xóa node rỗng. Trả về `(found, should_delete)` để cha biết có nên xóa con không.

---

## Trie trong thực tế

### a) Autocomplete / Search Suggestion

Google search, IDE autocomplete, bàn phím điện thoại -- đều dùng Trie (hoặc biến thể). Mỗi node có thể lưu thêm **frequency** để rank kết quả:

```
Trie với frequency:

          root
           |
           p
           |
           r
           |
           o
          /|\
         g  b  j
         |  |  |
         r  l  e
         |  |  |
         a  e  c
         |  |  |
         m* m* t*
       (50)(30)(20)

Gõ "pro" → collect → sort by freq → ["program", "problem", "project"]
```

Thực tế, Google còn kết hợp Trie với machine learning để personalize gợi ý. Nhưng nền tảng vẫn là prefix lookup.

### b) Spell Checker

- Kiểm tra từ có trong từ điển không: `search("teh")` → false
- Gợi ý sửa: tìm từ gần giống (edit distance ≤ 2) trong Trie
- Nhanh hơn brute force HashMap vì có thể **prune branches sớm** -- nếu prefix không match, bỏ cả nhánh

### c) IP Routing Table (Binary Trie)

Router internet dùng binary trie cho IP lookup:

- IP address = chuỗi 32 bit → mỗi node là 0 hoặc 1
- **Longest prefix match**: tìm route cụ thể nhất cho IP
- Ví dụ: `192.168.1.0/24` match cụ thể hơn `192.168.0.0/16` → router chọn /24

```
Simplified binary trie (4-bit demo):

     root
    /    \
   0      1
  / \      \
 0   1      0
 |   |      |
 0   0      1
 ↓   ↓      ↓
/8  /8     /8

IP 0100... → đi 0 → 1 → 0 → 0 → match route /8
```

### d) T9 Dictionary (bàn phím điện thoại cũ)

Thời chưa có smartphone, mỗi nút số (2-9) map nhiều chữ cái: 2=abc, 3=def, ...

- Gõ "4663" → Trie search → "gone", "good", "home", "hood"...
- Mỗi node chứa **tập ký tự có thể** thay vì 1 ký tự cụ thể
- Trie giúp thu hẹp kết quả cực nhanh theo từng nút bấm

---

## Ví dụ autocomplete trên điện thoại

```
Trie chứa: "xin chào", "xin lỗi", "xin phép", "xinh đẹp"

Người dùng gõ "xin":

  root
   |
   x
   |
   i
   |
   n
  / \
 ' '   h
 |      |
 ...   đ...

starts_with("xin") = true
→ Gợi ý: "xin chào", "xin lỗi", "xin phép"

Người dùng gõ thêm "h":
starts_with("xinh") = true
→ Gợi ý: "xinh đẹp"
```

Mỗi ký tự gõ thêm thu hẹp kết quả. Trie giúp làm việc này trong O(m).

---

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Giải thích |
|----------|-----------|--------|------------|
| `insert` | O(m) | O(m) | m = độ dài từ, tối đa tạo m node mới |
| `search` | O(m) | O(1) | Đi m bước, không tạo gì |
| `starts_with` | O(m) | O(1) | Giống search |
| `collect_words` | O(m + k) | O(k) | m = prefix, k = tổng ký tự kết quả |
| `delete` | O(m) | O(m) stack | Đệ quy m tầng |
| **Tổng bộ nhớ** | -- | **O(N × m)** | N = số từ, m = độ dài trung bình |

### So sánh với HashMap

```
                      Tìm từ      Tìm prefix     Bộ nhớ
HashMap<String>       O(m)        O(N × m) 💀     O(N × m)
Trie                  O(m)        O(m + k) ⚡     O(N × m)*

* Trie thường dùng ÍT hơn nhờ chia sẻ prefix
```

HashMap tìm từ nhanh, nhưng tìm prefix phải quét toàn bộ. Trie tìm prefix trong O(m + k) -- lý tưởng cho autocomplete.

---

## Ví dụ

### Các thao tác cơ bản

```rust
use rust_ds2a::trie::Trie;

let mut trie = Trie::new();
trie.insert("hello");
trie.insert("help");
trie.insert("world");

assert!(trie.search("hello"));        // có trong trie
assert!(trie.search("help"));         // có
assert!(!trie.search("hell"));        // KHÔNG có (chưa hoàn chỉnh)
assert!(trie.starts_with("hel"));     // có từ bắt đầu bằng "hel"
assert!(!trie.starts_with("xyz"));    // không có
```

Chú ý: "hell" không phải từ hoàn chỉnh (không có `is_end`), nhưng "hel" là prefix hợp lệ.

### Autocomplete

```rust
use rust_ds2a::trie::Trie;

let mut trie = Trie::new();
trie.insert("cat");
trie.insert("car");
trie.insert("card");
trie.insert("care");
trie.insert("cap");
trie.insert("dog");

let results = trie.collect_words_with_prefix("ca");
// → ["cap", "car", "card", "care", "cat"]

let results = trie.collect_words_with_prefix("car");
// → ["car", "card", "care"]

let results = trie.collect_words_with_prefix("xyz");
// → [] (prefix không tồn tại)
```

### Xóa từ

```rust
use rust_ds2a::trie::Trie;

let mut trie = Trie::new();
trie.insert("app");
trie.insert("apple");

assert!(trie.delete("app"));         // xóa "app"
assert!(!trie.search("app"));        // "app" đã bị xóa
assert!(trie.search("apple"));       // "apple" vẫn còn!
```

Xóa "app" không ảnh hưởng "apple" vì "apple" có đường đi dài hơn. Chỉ bỏ đánh dấu `is_end` ở node 'p' thứ hai.

---

## Những cái bẫy hay gặp

### a) Nhầm search với starts_with

❌ Nghĩ `search("hel")` trả `true` vì "hello" bắt đầu bằng "hel"

✅ `search("hel")` trả `false` -- "hel" KHÔNG phải từ hoàn chỉnh. `starts_with("hel")` mới trả `true`.

💡 Khác nhau 1 dòng code (check `is_end`) nhưng ý nghĩa khác hoàn toàn. Đọc kỹ đề bài: "từ có tồn tại không?" → `search`. "Có từ nào bắt đầu bằng...?" → `starts_with`.

### b) Quên case sensitivity

❌ Insert "Hello", search "hello" → mong đợi `true`

✅ "Hello" và "hello" là 2 đường khác nhau trong Trie ('H' ≠ 'h'). Muốn case-insensitive → `word.to_lowercase()` trước khi insert/search.

💡 Production code luôn normalize input. Không chỉ lowercase -- còn có Unicode normalization (é vs e + ◌́).

### c) Memory explosion với alphabet lớn

❌ Dùng array [65536] cho Unicode Trie

✅ Dùng HashMap cho Unicode (hàng nghìn ký tự). Array [26] chỉ cho a-z.

💡 Chọn data structure cho children dựa trên alphabet size: nhỏ (a-z) → array, lớn (Unicode) → HashMap.

### d) Dùng Trie khi HashMap đủ tốt

❌ Dùng Trie để check "email đã tồn tại chưa?" (exact match only)

✅ HashMap O(1) amortized nhanh hơn Trie O(m) cho exact match. Trie chỉ thắng khi cần **prefix operations**.

💡 Hỏi: "Tôi có cần tìm theo prefix không?" Nếu không → HashMap. Nếu có → Trie.

### e) Quên dọn node khi delete

❌ Chỉ đặt `is_end = false` mà không xóa node rỗng → memory leak

✅ Phải backtrack xóa node: node không có con + không phải `is_end` → xóa

💡 Trie phình to dần dù có ít từ nếu không dọn. Code delete trong `src/trie.rs` đã handle việc này đúng cách.

---

## Khi nào dùng / không nên dùng

| Tình huống | Trie? | Thay bằng gì? | Tại sao? |
|------------|-------|---------------|----------|
| Autocomplete / search suggestion | ✅ | -- | Prefix search O(m + k) |
| Spell checker | ✅ | -- | Suggest similar words nhanh |
| IP routing (longest prefix match) | ✅ | -- | Binary trie, O(32) cho IPv4 |
| Từ điển với prefix query | ✅ | -- | Nhanh hơn HashMap cho prefix |
| Exact string lookup only | ❌ | HashMap | O(1) > O(m) |
| Sorted string iteration | ⚠️ | BTreeSet\<String\> | Trie cũng sorted nhưng BTreeSet đơn giản hơn |
| Numeric data | ❌ | BST / HashMap | Trie cho string/sequence |
| Dataset nhỏ (< 100 strings) | ❌ | Vec + linear search | Overhead không đáng |
| Random strings (ít shared prefix) | ⚠️ | HashMap | Trie tốn memory hơn khi ít sharing |

---

## Luyện nhận diện Pattern

### a) Implement Trie (LeetCode #208)

Implement `insert`, `search`, `startsWith`. Bạn vừa học xong -- thử tự viết từ đầu không nhìn code!

**Gợi ý**: Cấu trúc y hệt code trong bài. HashMap hoặc array [26] đều được.

### b) Word Search II (LeetCode #212)

Cho board 2D ký tự và list words, tìm tất cả words xuất hiện trên board (đi 4 hướng liền kề).

**Gợi ý**: Brute force check từng word = O(N × M × 4^L). Dùng Trie chứa tất cả words → DFS trên board + Trie cùng lúc → prune sớm khi prefix không match. Trie biến bài từ TLE thành AC.

### c) Longest Common Prefix (LeetCode #14)

Cho mảng strings, tìm longest common prefix. Ví dụ: `["flower","flow","flight"]` → `"fl"`.

**Gợi ý**: Insert tất cả vào Trie, rồi đi từ root xuống cho đến khi node có > 1 child hoặc `is_end = true`. Có thể giải không cần Trie, nhưng Trie giúp hiểu rõ khái niệm prefix sharing.

---

## Trie trong Rust ecosystem

- **`trie-rs`** crate -- Trie implementation cho Rust, hỗ trợ generic key types
- **`radix_trie`** crate -- Radix tree (compressed trie), memory efficient hơn
- **`fst`** crate (finite state transducer) -- Trie variant cực kỳ memory efficient, dùng bởi `tantivy` (search engine viết bằng Rust)
- **`aho-corasick`** crate -- multi-pattern string matching dùng Trie + failure links, dùng bởi `ripgrep`

### Kafka connection

Topic name lookup trong Kafka-like systems có thể dùng Trie nếu hỗ trợ wildcard subscription (Kafka hỗ trợ pattern subscription). Consumer group management khi có hàng nghìn topics cũng benefit từ prefix-based lookup.

---

## Preview chương tiếp theo -- Hash Map

Trie dùng `HashMap` bên trong mỗi node cho children lookup. Nhưng HashMap bản thân nó hoạt động như thế nào? Hashing là gì, collision xảy ra khi nào và giải quyết ra sao?

Chương tiếp theo sẽ khám phá **Hash Map** -- cấu trúc có O(1) lookup được dùng nhiều nhất trong mọi ngôn ngữ lập trình. Bạn sẽ hiểu tại sao `HashMap` trong Rust nhanh đến vậy, và khi nào nó KHÔNG phải lựa chọn tốt nhất.

---

---

[← B-Tree](./07-b-tree.md) | [Hash Map →](../04-hashing/01-hash-map.md)
