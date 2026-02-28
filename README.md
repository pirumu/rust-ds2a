# DSA & Algorithms in Rust

Cuốn sách học **Cấu trúc dữ liệu & Giải thuật** bằng **Rust**, viết hoàn toàn bằng tiếng Việt. Mỗi chương bắt đầu bằng ví dụ thực tế, rồi mới vào lý thuyết và code. Sách dùng **Rust Edition 2024**.

**[Xem trên GitHub](book/vi/SUMMARY.md)**

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

## Nội dung

### Phần 1: Nền tảng
- [Phân tích độ phức tạp (Big-O)](book/vi/chapters/01-fundamentals/01-complexity.md)
- [Arrays & Slices](book/vi/chapters/01-fundamentals/02-arrays.md)
- [Strings](book/vi/chapters/01-fundamentals/03-strings.md)

### Phần 2: Cấu trúc tuyến tính
- [Singly Linked List](book/vi/chapters/02-linear-structures/01-singly-linked-list.md)
- [Doubly Linked List](book/vi/chapters/02-linear-structures/02-doubly-linked-list.md)
- [Stack](book/vi/chapters/02-linear-structures/03-stack.md)
- [Queue](book/vi/chapters/02-linear-structures/04-queue.md)
- [Deque](book/vi/chapters/02-linear-structures/05-deque.md)

### Phần 3: Cây & Heap
- [Binary Tree](book/vi/chapters/03-trees-and-heaps/01-binary-tree.md)
- [Binary Search Tree](book/vi/chapters/03-trees-and-heaps/02-bst.md)
- [AVL Tree](book/vi/chapters/03-trees-and-heaps/03-avl-tree.md)
- [Red-Black Tree](book/vi/chapters/03-trees-and-heaps/04-red-black-tree.md)
- [B-Tree](book/vi/chapters/03-trees-and-heaps/05-b-tree.md)
- [Binary Heap](book/vi/chapters/03-trees-and-heaps/06-binary-heap.md)
- [Priority Queue](book/vi/chapters/03-trees-and-heaps/07-priority-queue.md)
- [Trie](book/vi/chapters/03-trees-and-heaps/08-trie.md)

### Phần 4: Hashing
- [Hash Map](book/vi/chapters/04-hashing/01-hash-map.md)
- [Hash Set](book/vi/chapters/04-hashing/02-hash-set.md)
- [Bloom Filter](book/vi/chapters/04-hashing/03-bloom-filter.md)
- [Consistent Hashing](book/vi/chapters/04-hashing/04-consistent-hashing.md)

### Phần 5: Đồ thị
- [Biểu diễn đồ thị](book/vi/chapters/05-graphs/01-graph-representations.md)
- [BFS — Duyệt theo chiều rộng](book/vi/chapters/05-graphs/02-bfs.md)
- [DFS — Duyệt theo chiều sâu](book/vi/chapters/05-graphs/03-dfs.md)
- [Dijkstra](book/vi/chapters/05-graphs/04-dijkstra.md)
- [Bellman-Ford](book/vi/chapters/05-graphs/05-bellman-ford.md)
- [Floyd-Warshall](book/vi/chapters/05-graphs/06-floyd-warshall.md)
- [Prim's MST](book/vi/chapters/05-graphs/07-prim.md)
- [Kruskal's MST](book/vi/chapters/05-graphs/08-kruskal.md)
- [Topological Sort](book/vi/chapters/05-graphs/09-topological-sort.md)
- [Union-Find](book/vi/chapters/05-graphs/10-union-find.md)

### Phần 6: Giải thuật
- [Recursion](book/vi/chapters/06-algorithms/01-recursion.md)
- [Bubble, Selection & Insertion Sort](book/vi/chapters/06-algorithms/02-basic-sorting.md)
- [Merge Sort](book/vi/chapters/06-algorithms/03-merge-sort.md)
- [Quick Sort](book/vi/chapters/06-algorithms/04-quick-sort.md)
- [Heap Sort](book/vi/chapters/06-algorithms/05-heap-sort.md)
- [Radix Sort](book/vi/chapters/06-algorithms/06-radix-sort.md)
- [Binary Search](book/vi/chapters/06-algorithms/07-binary-search.md)
- [Two Pointers](book/vi/chapters/06-algorithms/08-two-pointers.md)
- [Prefix Sum](book/vi/chapters/06-algorithms/09-prefix-sum.md)
- [Sliding Window](book/vi/chapters/06-algorithms/10-sliding-window.md)
- [Divide & Conquer](book/vi/chapters/06-algorithms/11-divide-and-conquer.md)
- [Greedy](book/vi/chapters/06-algorithms/12-greedy.md)
- [Dynamic Programming](book/vi/chapters/06-algorithms/13-dynamic-programming.md)
- [Backtracking](book/vi/chapters/06-algorithms/14-backtracking.md)

### Phần 7: Kỹ thuật giải bài
- [Bit Manipulation](book/vi/chapters/07-patterns/01-bit-manipulation.md)
- [Monotonic Stack](book/vi/chapters/07-patterns/02-monotonic-stack.md)
- [Intervals](book/vi/chapters/07-patterns/03-intervals.md)
- [Matrix Traversal](book/vi/chapters/07-patterns/04-matrix-traversal.md)
- [Linked List Tricks](book/vi/chapters/07-patterns/05-linked-list-tricks.md)
- [Top-K Problems](book/vi/chapters/07-patterns/06-top-k.md)
- [String Matching (KMP, Rabin-Karp)](book/vi/chapters/07-patterns/07-string-matching.md)
- [Graph Patterns](book/vi/chapters/07-patterns/08-graph-patterns.md)
- [Tree Patterns](book/vi/chapters/07-patterns/09-tree-patterns.md)

### Phần 8: Cấu trúc nâng cao
- [Segment Tree](book/vi/chapters/08-advanced/01-segment-tree.md)
- [Fenwick Tree (BIT)](book/vi/chapters/08-advanced/02-fenwick-tree.md)
- [Skip List](book/vi/chapters/08-advanced/03-skip-list.md)
- [LRU Cache](book/vi/chapters/08-advanced/04-lru-cache.md)
- [LFU Cache](book/vi/chapters/08-advanced/05-lfu-cache.md)
- [Merkle Tree](book/vi/chapters/08-advanced/06-merkle-tree.md)
- [Design Structures](book/vi/chapters/08-advanced/07-design-structures.md)

## Bắt đầu
```bash
git clone https://github.com/pirumu/rust-ds2a.git
cd rust-ds2a

cargo test        # chạy toàn bộ test
mdbook build      # build sách
mdbook serve      # đọc trên trình duyệt (hot reload)
```

## Thống kê

| | Số lượng |
|---|---|
| Chương | 58 |
| Phần | 8 |
| Module Rust | 41 |
| Tests | 693 |

## License

MIT
