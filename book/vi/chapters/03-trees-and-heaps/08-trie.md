# Trie

## Đây là gì?

Mở điện thoại lên, gõ "xin" vào thanh tìm kiếm. Ngay lập tức xuất hiện gợi ý: "xin chào", "xin lỗi", "xin phép"... Bạn chưa gõ xong mà máy đã đoán được. Làm sao nó làm được?

Câu trả lời: **Trie** (đọc là "try", lấy từ chữ "re**trie**val"). Trie là cấu trúc cây mà mỗi node đại diện cho **một ký tự**. Các từ có chung phần đầu (prefix) sẽ **chia sẻ đường đi** từ root. Vì vậy, tìm kiếm prefix cực nhanh: O(m) với m = độ dài prefix, không phụ thuộc vào tổng số từ đã lưu.

**Ở đâu trong thực tế?**
- **Autocomplete**: gợi ý khi gõ tìm kiếm
- **Spell checker**: kiểm tra chính tả
- **IP routing table**: dùng binary trie
- **Từ điển**: tìm từ nhanh hơn hash map cho prefix query

## Hoạt động như thế nào?

### Chèn từng ký tự

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

Đây là sức mạnh chính của trie. Các từ cùng phần đầu chia sẻ node:

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

### Search vs starts_with

Hai thao tác tìm kiếm khác nhau:

**search("car")**: Đi c → a → r. Kiểm tra `is_end` ở node r. `is_end = true` → "car" là từ hoàn chỉnh → trả về `true`.

**search("ca")**: Đi c → a. Kiểm tra `is_end` ở node a. `is_end = false` → "ca" KHÔNG phải từ hoàn chỉnh → trả về `false`.

**starts_with("ca")**: Đi c → a. Node tồn tại → CÓ từ bắt đầu bằng "ca" → trả về `true`.

```
Tóm tắt:
  search    = "Từ này có TRONG từ điển không?"
  starts_with = "Có từ nào BẮT ĐẦU bằng chuỗi này không?"
```

### Xóa (delete)

Xóa một từ: bỏ đánh dấu `is_end`. Nếu node không còn con và không phải kết thúc từ khác, cắt nhánh.

```
Xóa "cat" khỏi {"cat", "car"}:

Trước:          Sau:
  root           root
   |              |
   c              c
   |              |
   a              a
  / \              \
 t*   r*           r*

Node 't' có is_end=true, không có con → xóa.
Node 'a' vẫn có con 'r' → giữ lại.
```

### Ví dụ autocomplete trên điện thoại

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

### Xóa (delete)

```rust
pub fn delete(&mut self, word: &str) -> bool {
    // Đệ quy đi đến cuối từ
    // Bỏ đánh dấu is_end
    // Nếu node không có con và không phải kết thúc từ khác → xóa
    // Trả về true nếu cha có thể xóa node này
}
```

## Độ phức tạp

| Thao tác | Thời gian | Bộ nhớ | Giải thích |
|----------|-----------|--------|------------|
| `insert` | O(m) | O(m) | m = độ dài từ, tối đa tạo m node mới |
| `search` | O(m) | O(1) | Đi m bước, không tạo gì |
| `starts_with` | O(m) | O(1) | Giống search |
| `delete` | O(m) | O(m) stack | Đệ quy m tầng |
| **Tổng bộ nhớ** | -- | **O(N * m)** | N = số từ, m = độ dài trung bình |

### So sánh với HashMap

```
                      Tìm từ      Tìm prefix     Bộ nhớ
HashMap<String>       O(m)        O(n * m) 💀     O(N * m)
Trie                  O(m)        O(m) ⚡         O(N * m)*

* Trie thường dùng ÍT hơn nhờ chia sẻ prefix
```

HashMap tìm từ nhanh, nhưng tìm prefix phải quét toàn bộ. Trie tìm prefix trong O(m) -- lý tưởng cho autocomplete.

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
