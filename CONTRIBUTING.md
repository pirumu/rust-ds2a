# Đóng góp cho rust-ds2a

Cảm ơn bạn muốn đóng góp! Dự án này viết cho những bạn struggle với DSA, nên mọi đóng góp đều cần giữ đúng tinh thần: **đơn giản, dễ hiểu, thực tế**.

## Cách đóng góp

### 1. Báo lỗi (Bug Report)

Mở Issue với:
- Mô tả lỗi
- Cách tái hiện (reproduce)
- Output mong đợi vs output thực tế

### 2. Đề xuất nội dung mới

Mở Issue với tag `enhancement` và mô tả:
- Chủ đề muốn thêm
- Tại sao chủ đề này hữu ích cho người mới học DSA

### 3. Gửi Pull Request

```bash
# Fork repo, clone về máy
git clone https://github.com/<your-username>/rust-ds2a.git
cd rust-ds2a

# Tạo branch mới
git checkout -b feat/ten-tinh-nang

# Code, test, commit
cargo test
mdbook build
git commit -m "feat: mô tả ngắn gọn"

# Push và tạo PR
git push origin feat/ten-tinh-nang
```

## Quy tắc viết nội dung

### Ngôn ngữ
- Viết bằng **tiếng Việt**
- Thuật ngữ kỹ thuật giữ nguyên tiếng Anh (array, stack, Big-O, linked list, ...)
- Lần đầu nhắc đến thuật ngữ: tiếng Anh + giải thích tiếng Việt
  - Ví dụ: "Stack — tưởng tượng như một chồng đĩa..."

### Giọng văn
- Như bạn bè giải thích cho nhau, **không** như giáo sư giảng bài
- Câu ngắn, đơn giản
- Ví dụ thực tế TRƯỚC, rồi mới vào lý thuyết
- Tránh jargon — nếu buộc phải dùng, giải thích ngay

### Cấu trúc mỗi chương
1. **Ví dụ thực tế** — analogy dễ hiểu
2. **Đây là gì?** — giải thích concept
3. **Cách hoạt động** — sơ đồ ASCII, từng bước
4. **Code Rust** — implementation có comment
5. **Bảng độ phức tạp** — Time & Space

### Code Rust
- Mỗi function có docstring: mô tả + `**Time:** O(n). **Space:** O(1).`
- Tests trong `#[cfg(test)] mod tests`
- Nhóm test theo function: `// ---- function_name ----`
- Cover: happy path, edge cases, empty input

## Chạy test

```bash
# Toàn bộ test suite
cargo test

# Test một module cụ thể
cargo test prefix_sum

# Kiểm tra style
cargo clippy

# Build sách
mdbook build

# Xem sách trên trình duyệt
mdbook serve
```

## Commit messages

Dùng [Conventional Commits](https://www.conventionalcommits.org/):

- `feat: thêm chương Segment Tree`
- `fix: sửa lỗi test trong prefix_sum`
- `docs: cập nhật README`
- `refactor: đơn giản hoá LRU Cache`

## License

Khi đóng góp, bạn đồng ý rằng code của bạn được phát hành theo [MIT License](LICENSE).
