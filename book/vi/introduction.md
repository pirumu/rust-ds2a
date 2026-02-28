# Cấu trúc dữ liệu & Giải thuật với Rust

Cuốn sách học **Cấu trúc dữ liệu & Giải thuật** bằng **Rust**, viết hoàn toàn bằng tiếng Việt. Mỗi chương bắt đầu bằng ví dụ thực tế, rồi mới vào lý thuyết và code. Sách dùng **Rust Edition 2024**.

## Tại sao cuốn sách này tồn tại?

DSA từ trước đến nay vẫn luôn khó tiếp cận — thuật ngữ trừu tượng, tài liệu chủ yếu bằng tiếng Anh, và cảm giác "mình không đủ giỏi để hiểu mấy thứ này". Cuốn sách này muốn thay đổi điều đó: giải thích mọi thứ bằng tiếng Việt, bắt đầu từ ví dụ đời thường, rồi mới đi vào lý thuyết và code.

Nhưng ngoài việc làm DSA dễ tiếp cận hơn, có một lý do sâu hơn để cuốn sách này ra đời — và nó liên quan đến chính thời đại chúng ta đang sống.

**Ngày nay AI càng viết code giỏi, bạn càng cần hiểu DSA — không phải ít hơn, mà nhiều hơn.**

Nghe có vẻ ngược đời. AI generate được cả một module trong vài giây, vậy học DSA làm gì? Nhưng hãy nghĩ thế này: khi bạn để AI viết code, vai trò của bạn thay đổi. Bạn không còn chỉ là người *viết* — bạn trở thành người *đánh giá*. Và đánh giá thực ra khó hơn viết.

Viết code sai, compiler sẽ la bạn. Nhưng khi AI đưa ra một đoạn code chạy được, pass test, trông sạch đẹp — mà bên trong dùng sai cấu trúc dữ liệu, chọn thuật toán O(n²) trong khi O(n log n) là đủ, hoặc allocate memory một cách lãng phí — **không ai la bạn cả**. Code vẫn chạy. Bug không nổ hôm nay. Nó nổ khi traffic tăng gấp 10, khi dataset lớn lên, khi production không tha thứ cho sự thiếu hiểu biết.

AI là một developer cực kỳ nhanh, cực kỳ tự tin, và hiếm khi nói "em không biết". Nó đưa ra giải pháp cho mọi thứ — nhưng nó không thể biết tường tận context hệ thống của bạn, không hiểu trade-off thực sự giữa memory và speed trong infra cụ thể của bạn, và đặc biệt nó không chịu trách nhiệm khi code của nó làm sập service lúc 2 giờ sáng. **Bạn mới là người chịu trách nhiệm đó.**

Và đây chính là chỗ DSA trở thành siêu năng lực. Không phải để bạn tự tay implement Red-Black Tree mỗi ngày — mà để khi đọc code AI generate, bạn tự nhiên đặt được những câu hỏi đúng:
- "Tại sao dùng HashMap ở đây thay vì BTreeMap?"
- "Đoạn này có bị worst-case O(n²) không?"
- "Memory layout này có cache-friendly không?"
- "Có cần thiết phải clone toàn bộ vector không, hay chỉ cần một slice?"

Những câu hỏi này không đến từ việc biết prompt AI. Chúng đến từ việc thực sự hiểu cách máy tính xử lý dữ liệu. Và tin tốt là: bạn không cần phải giỏi sẵn để bắt đầu. Cuốn sách này được viết chính cho những người đang ở điểm xuất phát — để từng bước xây nền tảng vững, để dù công cụ có thay đổi thế nào, bạn vẫn là người hiểu chuyện gì đang xảy ra bên dưới.

## Tại sao Rust?

Vì đây là ngôn ngữ mình thích và dùng hàng ngày. Rust có type system chặt chẽ, compiler khắt khe -- implement DSA trong Rust buộc bạn phải nghĩ rõ ràng về ownership, memory layout, và error handling thay vì đoán mò.
Nếu bạn đang dùng ngôn ngữ khác, các concept DSA trong sách này vẫn áp dụng được -- chỉ cần translate syntax.

> Cảnh báo thực tế: Rust có learning curve dốc hơn Python hay Go. Bạn sẽ đấm nhau với borrow checker. Nhưng khi vượt qua được, bạn hiểu máy tính hoạt động thế nào ở mức mà hầu hết developer không có.

## Cách đọc cuốn sách này

Mỗi chương có 5 phần:

1. **Đây là gì?** — Ví dụ thực tế trước, rồi mới giải thích khái niệm
2. **Hoạt động như thế nào?** — Từng bước với hình vẽ ASCII
3. **Code Rust** — Code đầy đủ, giải thích từng phần
4. **Độ phức tạp** — Bảng Big-O, giải thích nó có nghĩa gì trong thực tế
5. **Ví dụ** — Chạy thử code

## Cần gì trước khi đọc?

- Biết cơ bản về Rust (ownership, borrowing, lifetimes)
- Cài Rust toolchain (`rustup`)
- Cài `mdbook` (`cargo install mdbook`)

## Chạy code

```bash
git clone https://github.com/pirumu/rust-ds2a.git
cd rust-ds2a

cargo test        # chạy toàn bộ test
mdbook build      # build sách
mdbook serve      # đọc trên trình duyệt (hot reload)
```

---

[Phân tích độ phức tạp (Big-O) →](chapters/01-fundamentals/01-complexity.md)
