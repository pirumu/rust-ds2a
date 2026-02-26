# DSA & Algorithms in Rust

Cuốn sách học Cấu trúc dữ liệu & Giải thuật bằng Rust — viết cho những bạn thấy tư duy logic khó, không tự tin với tiếng Anh, và muốn hiểu DSA theo cách dễ nhất có thể.

**[Xem trên GitHub](book/SUMMARY.md)**

## Viết cho ai?

Cho những bạn như mình — từng struggle với DSA, đọc sách giáo khoa thấy khô khan, và cần ai đó giải thích như bạn bè ngồi cà phê nói chuyện. Mỗi chương bắt đầu bằng ví dụ thực tế, rồi mới vào lý thuyết.

## Nội dung

### Phần 1: Nền tảng
- [Phân tích độ phức tạp (Big-O)](book/chapters/01-fundamentals/01-complexity.md)
- [Arrays & Slices](book/chapters/01-fundamentals/02-arrays.md)
- [Strings](book/chapters/01-fundamentals/03-strings.md)

### Phần 2: Cấu trúc tuyến tính
- [Singly Linked List](book/chapters/02-linear-structures/01-singly-linked-list.md)
- [Doubly Linked List](book/chapters/02-linear-structures/02-doubly-linked-list.md)
- [Stack](book/chapters/02-linear-structures/03-stack.md)
- [Queue](book/chapters/02-linear-structures/04-queue.md)
- [Deque](book/chapters/02-linear-structures/05-deque.md)

### Phần 3: Cây & Heap
- [Binary Tree](book/chapters/03-trees-and-heaps/01-binary-tree.md)
- [Binary Search Tree](book/chapters/03-trees-and-heaps/02-bst.md)
- [AVL Tree](book/chapters/03-trees-and-heaps/03-avl-tree.md)
- [Red-Black Tree](book/chapters/03-trees-and-heaps/04-red-black-tree.md)
- [B-Tree](book/chapters/03-trees-and-heaps/05-b-tree.md)
- [Binary Heap](book/chapters/03-trees-and-heaps/06-binary-heap.md)
- [Priority Queue](book/chapters/03-trees-and-heaps/07-priority-queue.md)
- [Trie](book/chapters/03-trees-and-heaps/08-trie.md)

### Phần 4: Hashing
- [Hash Map](book/chapters/04-hashing/01-hash-map.md)
- [Hash Set](book/chapters/04-hashing/02-hash-set.md)
- [Bloom Filter](book/chapters/04-hashing/03-bloom-filter.md)
- [Consistent Hashing](book/chapters/04-hashing/04-consistent-hashing.md)

### Phần 5: Đồ thị
- [Biểu diễn đồ thị](book/chapters/05-graphs/01-graph-representations.md)
- [BFS — Duyệt theo chiều rộng](book/chapters/05-graphs/02-bfs.md)
- [DFS — Duyệt theo chiều sâu](book/chapters/05-graphs/03-dfs.md)
- [Dijkstra](book/chapters/05-graphs/04-dijkstra.md)
- [Bellman-Ford](book/chapters/05-graphs/05-bellman-ford.md)
- [Floyd-Warshall](book/chapters/05-graphs/06-floyd-warshall.md)
- [Prim's MST](book/chapters/05-graphs/07-prim.md)
- [Kruskal's MST](book/chapters/05-graphs/08-kruskal.md)
- [Topological Sort](book/chapters/05-graphs/09-topological-sort.md)
- [Union-Find](book/chapters/05-graphs/10-union-find.md)

### Phần 6: Giải thuật
- [Recursion](book/chapters/06-algorithms/01-recursion.md)
- [Bubble, Selection & Insertion Sort](book/chapters/06-algorithms/02-basic-sorting.md)
- [Merge Sort](book/chapters/06-algorithms/03-merge-sort.md)
- [Quick Sort](book/chapters/06-algorithms/04-quick-sort.md)
- [Heap Sort](book/chapters/06-algorithms/05-heap-sort.md)
- [Radix Sort](book/chapters/06-algorithms/06-radix-sort.md)
- [Binary Search](book/chapters/06-algorithms/07-binary-search.md)
- [Two Pointers](book/chapters/06-algorithms/08-two-pointers.md)
- [Prefix Sum](book/chapters/06-algorithms/09-prefix-sum.md)
- [Sliding Window](book/chapters/06-algorithms/10-sliding-window.md)
- [Divide & Conquer](book/chapters/06-algorithms/11-divide-and-conquer.md)
- [Greedy](book/chapters/06-algorithms/12-greedy.md)
- [Dynamic Programming](book/chapters/06-algorithms/13-dynamic-programming.md)
- [Backtracking](book/chapters/06-algorithms/14-backtracking.md)

### Phần 7: Kỹ thuật giải bài — từ cơ bản đến phỏng vấn
- [Bit Manipulation](book/chapters/07-patterns/01-bit-manipulation.md)
- [Monotonic Stack](book/chapters/07-patterns/02-monotonic-stack.md)
- [Intervals](book/chapters/07-patterns/03-intervals.md)
- [Matrix Traversal](book/chapters/07-patterns/04-matrix-traversal.md)
- [Linked List Tricks](book/chapters/07-patterns/05-linked-list-tricks.md)
- [Top-K Problems](book/chapters/07-patterns/06-top-k.md)
- [String Matching (KMP, Rabin-Karp)](book/chapters/07-patterns/07-string-matching.md)
- [Graph Patterns](book/chapters/07-patterns/08-graph-patterns.md)
- [Tree Patterns](book/chapters/07-patterns/09-tree-patterns.md)

### Phần 8: Cấu trúc nâng cao
- [Segment Tree](book/chapters/08-advanced/01-segment-tree.md)
- [Fenwick Tree (BIT)](book/chapters/08-advanced/02-fenwick-tree.md)
- [Skip List](book/chapters/08-advanced/03-skip-list.md)
- [LRU Cache](book/chapters/08-advanced/04-lru-cache.md)
- [LFU Cache](book/chapters/08-advanced/05-lfu-cache.md)
- [Merkle Tree](book/chapters/08-advanced/06-merkle-tree.md)
- [Design Structures](book/chapters/08-advanced/07-design-structures.md)

## Bắt đầu

```bash
# Clone repo
git clone https://github.com/pirumu/rust-ds2a.git
cd rust-ds2a

# Chạy toàn bộ test
cargo test

# Build sách
mdbook build

# Đọc sách trên trình duyệt (hot reload)
mdbook serve
```

## Mỗi chương gồm gì?

1. **Ví dụ thực tế** — hiểu bản chất trước khi vào code
2. **Cách hoạt động** — từng bước với sơ đồ ASCII
3. **Code Rust** — implementation đầy đủ, có giải thích
4. **Độ phức tạp** — Time & Space Big-O
5. **Liên kết kiến thức** — chương này xây trên kiến thức nào, mở đường cho chương nào

## Thống kê

| | Số lượng |
|---|---|
| Chương | 60 |
| Phần | 8 |
| Module Rust | 41 |
| Tests | 643 |

## License

MIT
