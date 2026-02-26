# Cấu trúc dữ liệu & Giải thuật với Rust

Chào bạn! Đây là cuốn sách dành cho những ai muốn học cấu trúc dữ liệu và giải thuật nhưng thấy sách giáo khoa quá khô khan, quá học thuật, và xa rời thực tế.

Mình viết cuốn này vì mình cũng từng như bạn — thấy DSA khó hiểu, không biết học để làm gì, và chán nản với những lời giải thích toàn ký hiệu toán học.

Ở đây, mỗi khái niệm đều bắt đầu bằng một ví dụ thực tế dễ hình dung. Code được viết bằng Rust và giải thích từng bước.

## Cách đọc cuốn sách này

Mỗi chương có 5 phần:

1. **Đây là gì?** — Ví dụ thực tế trước, rồi mới giải thích khái niệm
2. **Hoạt động như thế nào?** — Từng bước với hình vẽ ASCII
3. **Code Rust** — Code đầy đủ, giải thích từng phần
4. **Độ phức tạp** — Bảng Big-O, giải thích nó có nghĩa gì trong thực tế
5. **Ví dụ** — Chạy thử code

## Chạy code

```bash
# Test tất cả implementations
cargo test

# Build sách
mdbook build

# Đọc sách trên trình duyệt (tự reload khi thay đổi)
mdbook serve
```

## Cần gì trước khi đọc?

- Biết cơ bản về Rust (ownership, borrowing, lifetimes)
- Cài Rust toolchain (`rustup`)
- Cài `mdbook` (`cargo install mdbook`)
