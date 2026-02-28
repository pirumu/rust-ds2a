# Tree Patterns

> 💡 **Đừng lo lắng:** 5 patterns trong chương này cover khoảng **80% bài tree trong phỏng vấn**. Nắm vững 5 bài này, bạn sẽ tự tin giải hầu hết mọi bài tree gặp phải. Nghiêm túc đấy.

## Tại sao cần học?

Bạn đã học Binary Tree và BST ở Phần 3. Giờ mình sẽ học 5 bài toán **kinh điển nhất** về tree trong phỏng vấn. Đây là những bài mà Google, Meta, Amazon hỏi đi hỏi lại.

Tất cả đều dùng **DFS (Depth-First Search)** hoặc **BFS (Breadth-First Search)** -- hai kỹ thuật duyệt cây bạn đã biết. Điểm khác biệt là cách mình **kết hợp thông tin** khi đệ quy quay lại.

> **Quy tắc chung:** Khi bài toán hỏi về quan hệ giữa các node (cha con, đường đi, tổng), nghĩ ngay đến DFS. Khi cần xử lý theo từng tầng, nghĩ đến BFS.

### BFS vs DFS trên tree -- khi nào dùng cái nào?

| Dùng BFS khi... | Dùng DFS khi... |
|---|---|
| Cần xử lý **theo tầng** (level-order) | Cần **đường đi** từ root xuống leaf |
| Tìm node **gần root nhất** | Cần tính toán **chiều sâu/chiều cao** |
| Serialize/Deserialize cây | Cần **tổng hợp thông tin** từ subtree |
| Zigzag level order, right side view | LCA, Max Path Sum, Validate BST |

Nói đơn giản: **level = BFS, path/depth = DFS**. Phần lớn bài tree phỏng vấn dùng DFS.

### DFS return value pattern -- thông tin từ subtree trả về parent

Trong 5 bài dưới đây, bạn sẽ thấy 1 pattern lặp đi lặp lại: **hàm DFS trả về thông tin từ subtree cho node cha dùng**. Đây là xương sống của hầu hết bài tree.

```
Pattern chung:

fn dfs(node) -> ThôngTinTừSubtree {
    // 1. Hỏi nhánh trái
    let left = dfs(node.left);
    // 2. Hỏi nhánh phải
    let right = dfs(node.right);
    // 3. Kết hợp left + right + node hiện tại
    //    --> cập nhật kết quả global (nếu cần)
    // 4. Trả về thông tin cho node cha
}
```

Ví dụ cụ thể:
- **Diameter:** DFS trả về `depth` --> cha dùng `left_depth + right_depth` để tính đường kính
- **Max Path Sum:** DFS trả về `gain 1 nhánh` --> cha dùng `left_gain + right_gain + val` để tính path sum
- **LCA:** DFS trả về `Option<Node>` --> cha kiểm tra cả 2 bên có tìm thấy không

Nắm pattern này, bạn sẽ giải được hầu hết bài tree bằng cách tự hỏi: **"subtree cần trả về thông tin gì cho cha?"**

---

## 1. Lowest Common Ancestor (LCA) -- Tổ tiên chung gần nhất

### Hình ảnh: Gia phả

Tưởng tượng bạn có một cây gia phả. Bạn muốn tìm **ông bà chung gần nhất** của hai người trong gia đình. Ví dụ: anh em ruột thì LCA là cha mẹ. Anh em họ thì LCA là ông bà.

```
Gia phả:
            Ông [3]
           /       \
      Bố [5]      Chú [1]
      /    \
  Con [6]  Con [2]

LCA(6, 2) = 5 (Bố)      -- anh em ruột
LCA(6, 1) = 3 (Ông)      -- anh em họ
LCA(5, 1) = 3 (Ông)      -- anh em ruột
```

### Bài toán

Cho binary tree và hai giá trị `p`, `q`. Tìm node tổ tiên chung gần nhất -- node thấp nhất trong cây mà vẫn là tổ tiên (hoặc chính nó) của cả `p` và `q`.

### Ý tưởng: DFS + bubble up

Dùng DFS đi xuống từng nhánh. Tại mỗi node, ta hỏi:

1. Nhánh trái có tìm thấy `p` hoặc `q` không?
2. Nhánh phải có tìm thấy `p` hoặc `q` không?
3. Bản thân node này có phải `p` hoặc `q` không?

```
Tìm LCA(6, 1) trong cây:

            [3]          <-- cả trái VÀ phải đều tìm thấy --> LCA = 3!
           /    \
        [5]      [1]     <-- node 1 = target q, trả về 1
       /    \
    [6]     [2]
     ^
  target p, trả về 6

Bước 1: DFS đi sâu xuống node 6 --> tìm thấy p=6, trả về 6
Bước 2: DFS node 2 --> không phải p hoặc q, trả về None
Bước 3: Node 5: trái=Some(6), phải=None --> chỉ 1 bên, bubble up Some(6)
Bước 4: DFS node 1 --> tìm thấy q=1, trả về 1
Bước 5: Node 3: trái=Some(6), phải=Some(1) --> CẢ HAI BÊN --> LCA = 3!
```

**Quy tắc:**
- Nếu cả trái lẫn phải đều trả về kết quả --> node hiện tại là LCA
- Nếu node hiện tại là `p` hoặc `q` --> trả về chính nó
- Nếu chỉ một bên trả về --> bubble up kết quả đó

### Code

```rust
pub fn lca(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<i32> {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>,
           p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = node.as_ref()?;
        let val = node.borrow().val;

        if val == p || val == q {
            return Some(node.clone());  // Tìm thấy target!
        }

        let left = dfs(&node.borrow().left, p, q);
        let right = dfs(&node.borrow().right, p, q);

        match (&left, &right) {
            (Some(_), Some(_)) => Some(node.clone()),  // Cả 2 bên --> LCA
            (Some(_), None) => left,                    // Bubble up trái
            (None, Some(_)) => right,                   // Bubble up phải
            (None, None) => None,                       // Không tìm thấy
        }
    }

    dfs(root, p, q).map(|n| n.borrow().val)
}
```

**Time:** O(n) -- duyệt mỗi node nhiều nhất 1 lần. **Space:** O(h) -- call stack, h = chiều cao cây.

---

## 2. Serialize / Deserialize -- Lưu cây vào chuỗi và đọc lại

### Hình ảnh: Lưu game

Bạn đang chơi game, muốn save game để mai chơi tiếp. Cây binary tree cũng vậy -- nó nằm trong bộ nhớ (RAM), tắt máy là mất. Serialize = **lưu cây thành chuỗi text** (để ghi vào file, gửi qua mạng). Deserialize = **đọc chuỗi text, dựng lại cây** giống hệt ban đầu.

### Bài toán

Viết 2 hàm:
- `serialize(tree)` --> chuỗi string
- `deserialize(string)` --> tree giống hệt ban đầu

### Ý tưởng: BFS level-order

Duyệt cây theo từng tầng (BFS). Ghi giá trị từng node, node trống ghi `"null"`. Bỏ `null` thừa ở cuối.

```
Cây:
        [1]              Level 0
       /   \
     [2]   [3]           Level 1
            / \
          [4] [5]        Level 2

BFS duyệt:  1, 2, 3, null, null, 4, 5

Serialize:  "1,2,3,null,null,4,5"
```

Deserialize ngược lại: đọc từng token, dùng queue để gán con trái/phải cho từng node cha.

```
Tokens:  [1] [2] [3] [null] [null] [4] [5]
          ^
          Tạo root = 1, đưa vào queue

Queue: [1]
  Pop 1, đọc token 2 --> gán left(1) = 2, push 2
  Pop 1, đọc token 3 --> gán right(1) = 3, push 3

Queue: [2, 3]
  Pop 2, đọc token null --> left(2) = None
  Pop 2, đọc token null --> right(2) = None
  Pop 3, đọc token 4 --> gán left(3) = 4, push 4
  Pop 3, đọc token 5 --> gán right(3) = 5, push 5

Kết quả: cây giống hệt ban đầu!
```

### Code

```rust
pub fn serialize(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
    if root.is_none() { return String::new(); }

    let mut result: Vec<String> = Vec::new();
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    queue.push_back(root.clone());

    while let Some(front) = queue.pop_front() {
        match front {
            Some(node) => {
                result.push(node.borrow().val.to_string());
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
            None => result.push("null".to_string()),
        }
    }

    // Bỏ null thừa ở cuối
    while result.last().map_or(false, |s| s == "null") {
        result.pop();
    }
    result.join(",")
}

pub fn deserialize(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
    if s.is_empty() { return None; }

    let tokens: Vec<&str> = s.split(',').collect();
    let root = TreeNode::wrap(tokens[0].parse().ok()?);
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());

    let mut i = 1;
    while i < tokens.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if i < tokens.len() && tokens[i] != "null" {
                if let Ok(v) = tokens[i].parse::<i32>() {
                    let left = TreeNode::wrap(v);
                    current.borrow_mut().left = left.clone();
                    queue.push_back(left.unwrap());
                }
            }
            i += 1;
            // Right child
            if i < tokens.len() && tokens[i] != "null" {
                if let Ok(v) = tokens[i].parse::<i32>() {
                    let right = TreeNode::wrap(v);
                    current.borrow_mut().right = right.clone();
                    queue.push_back(right.unwrap());
                }
            }
            i += 1;
        }
    }
    root
}
```

**Time:** O(n). **Space:** O(n) -- queue chứa tối đa 1 tầng.

---

## 3. Max Path Sum -- Tổng đường đi lớn nhất

### Tại sao quan trọng?

Đây là bài kinh điển của phỏng vấn (LeetCode #124, Hard). Nó kiểm tra khả năng bạn xử lý **thông tin đệ quy hai chiều** -- khi kết quả tốt nhất không nhất thiết đi qua root.

### Bài toán

Cho binary tree (có thể chứa giá trị âm). Tìm **tổng lớn nhất** của một đường đi bất kỳ. Đường đi là dãy node liên tiếp -- có thể bắt đầu và kết thúc ở bất kỳ node nào.

### Ý tưởng: DFS + global max

Tại mỗi node, ta tính **đường đi tốt nhất đi qua node đó** = left_gain + val + right_gain. Nếu nhánh nào cho gain âm, ta bỏ (chọn 0).

Nhưng khi trả về cho node cha, ta chỉ được chọn **một bên** (trái hoặc phải) -- vì đường đi không thể rẽ nhánh.

```
Cây:
        [-10]
        /    \
      [9]   [20]
             / \
           [15] [7]

DFS từ dưới lên:

Node 9:  gain = 9 (lá, không có con)
Node 15: gain = 15
Node 7:  gain = 7
Node 20: left_gain=15, right_gain=7
         path qua 20 = 15 + 20 + 7 = 42  --> cập nhật global_max = 42!
         trả về cho cha: 20 + max(15,7) = 35

Node -10: left_gain=max(9,0)=9, right_gain=max(35,0)=35
          path qua -10 = 9 + (-10) + 35 = 34  --> 34 < 42, giữ 42
          trả về: -10 + max(9,35) = 25

Đáp án: global_max = 42  (đường đi: 15 -> 20 -> 7)
```

**Mấu chốt:** `global_max` theo dõi đường đi tốt nhất TẤT CẢ các node. Giá trị trả về cho cha thì chỉ chọn 1 nhánh.

### Code

```rust
pub fn max_path_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>,
           global_max: &mut i32) -> i32 {
        let node = match node.as_ref() {
            Some(n) => n,
            None => return 0,
        };

        let val = node.borrow().val;
        let left_gain = dfs(&node.borrow().left, global_max).max(0);
        let right_gain = dfs(&node.borrow().right, global_max).max(0);

        // Đường đi qua node này (có thể rẽ cả 2 bên)
        let path_sum = val + left_gain + right_gain;
        *global_max = (*global_max).max(path_sum);

        // Trả về cho cha: chỉ được chọn 1 bên
        val + left_gain.max(right_gain)
    }

    let mut global_max = i32::MIN;
    dfs(root, &mut global_max);
    global_max
}
```

**Time:** O(n). **Space:** O(h).

---

## 4. Diameter -- Đường kính cây

### Hình ảnh: Đường đi dài nhất

Tưởng tượng cây binary tree là mạng lưới đường trong thành phố. Diameter = **đường đi dài nhất** giữa hai ngã tư bất kỳ (tính bằng số cạnh). Đường đi này không nhất thiết đi qua gốc!

```
Cây:
         [1]
        /   \
      [2]   [3]
     /   \
   [4]   [5]

Đường kính = 3 (đường đi: 4 -> 2 -> 1 -> 3)
                hoặc:      5 -> 2 -> 1 -> 3

         [1]
        /
      [2]
     /   \
   [3]   [4]
   /       \
 [5]       [6]

Đường kính = 4 (đường đi: 5 -> 3 -> 2 -> 4 -> 6)
Không đi qua root!
```

### Ý tưởng: DFS tính chiều sâu

Giống Max Path Sum nhưng đơn giản hơn. Tại mỗi node:
- Tính chiều sâu nhánh trái (`left_depth`)
- Tính chiều sâu nhánh phải (`right_depth`)
- Đường đi qua node này = `left_depth + right_depth` (số cạnh)
- Cập nhật `best` nếu lớn hơn

```
DFS từ dưới lên:

Node 4: depth=1 (lá)
Node 5: depth=1 (lá)
Node 2: left=1, right=1 --> path qua 2 = 1+1 = 2, best=2
         trả về: 1 + max(1,1) = 2
Node 3: depth=1 (lá)
Node 1: left=2, right=1 --> path qua 1 = 2+1 = 3, best=3!
         trả về: 1 + max(2,1) = 3

Đáp án: best = 3
```

### Code

```rust
pub fn diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>,
           best: &mut usize) -> usize {
        let node = match node.as_ref() {
            Some(n) => n,
            None => return 0,
        };

        let left_depth = dfs(&node.borrow().left, best);
        let right_depth = dfs(&node.borrow().right, best);

        // Đường đi qua node = trái + phải
        *best = (*best).max(left_depth + right_depth);

        // Trả về chiều sâu cho node cha
        1 + left_depth.max(right_depth)
    }

    let mut best = 0;
    dfs(root, &mut best);
    best
}
```

**Time:** O(n). **Space:** O(h).

> **So sánh với Max Path Sum:** Cùng pattern DFS + global variable. Diameter đếm cạnh, Max Path Sum tính tổng giá trị. Nắm 1 bài là hiểu bài kia!

### Path problems -- 3 dạng bạn sẽ gặp

Bài Diameter và Max Path Sum thuộc nhóm **path problems** trên tree. Có 3 dạng phổ biến:

| Dạng | Mô tả | Ví dụ | Cách giải |
|------|--------|-------|-----------|
| **Root-to-leaf** | Đường đi từ gốc xuống lá | Path Sum, Root-to-Leaf Sum | DFS truyền tổng tích lũy xuống |
| **Root-to-any** | Đường đi bắt đầu từ gốc, kết thúc bất kỳ | Path Sum III (phần đơn giản) | DFS + prefix sum |
| **Any-to-any** | Đường đi giữa 2 node bất kỳ | Max Path Sum, Diameter | DFS return value + global max |

Dạng **any-to-any** khó nhất vì đường đi có thể "rẽ nhánh" qua 1 node. Đó là lý do ta cần trick: **cập nhật global max với cả 2 nhánh, nhưng chỉ trả về 1 nhánh cho cha**.

---

## 5. Validate BST -- Kiểm tra BST hợp lệ

### Sai lầm phổ biến

Nhiều người chỉ kiểm tra: "node con trái < node cha" và "node con phải > node cha". Nhưng BST yêu cầu **TẤT CẢ** node trong nhánh trái phải nhỏ hơn root, không chỉ con trực tiếp!

```
Cây SAI mà kiểm tra đơn giản sẽ BỎ LỌT:

         [5]
        /   \
      [1]   [4]        4 < 5? Vẫn ổn nếu chỉ check cha-con
             / \
           [3] [6]      3 < 4? OK. 6 > 4? OK.

Nhưng: 3 nằm bên PHẢI của 5, mà 3 < 5 --> SAI!
       Tất cả node bên phải 5 phải > 5.

Cây ĐÚNG:
         [5]
        /   \
      [1]   [7]
             / \
           [6] [8]     Mọi node phải 5 đều > 5. OK!
```

### Ý tưởng: Truyền min/max bounds

Mỗi node phải nằm trong khoảng `(min, max)`. Ban đầu khoảng là `(-INF, +INF)`. Khi đi sang trái, cập nhật max = giá trị node cha. Khi đi sang phải, cập nhật min = giá trị node cha.

```
Validate cây [5, 1, 4, null, null, 3, 6]:

Node 5: min=-INF, max=+INF --> 5 OK
  Node 1: min=-INF, max=5 --> 1 < 5 OK
  Node 4: min=5, max=+INF --> 4 > 5? KHÔNG! 4 <= 5 --> INVALID!

Validate cây [2, 1, 3]:

Node 2: min=-INF, max=+INF --> OK
  Node 1: min=-INF, max=2 --> 1 < 2 OK
  Node 3: min=2, max=+INF --> 3 > 2 OK
--> VALID!
```

### Code

```rust
pub fn is_valid_bst(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn validate(node: &Option<Rc<RefCell<TreeNode>>>,
                min: i64, max: i64) -> bool {
        let node = match node.as_ref() {
            Some(n) => n,
            None => return true,  // Cây rỗng luôn hợp lệ
        };

        let val = node.borrow().val as i64;

        // val phải nằm trong (min, max) -- strict inequality
        if val <= min || val >= max {
            return false;
        }

        // Trái: tất cả phải < val. Phải: tất cả phải > val
        validate(&node.borrow().left, min, val)
            && validate(&node.borrow().right, val, max)
    }

    validate(root, i64::MIN, i64::MAX)
}
```

> **Tại sao dùng i64?** Vì giá trị node là `i32`. Nếu node có val = `i32::MAX`, bounds cũng cần chứa được giá trị lớn hơn --> dùng `i64` an toàn hơn.

**Time:** O(n). **Space:** O(h).

---

## Bảng độ phức tạp

| Bài toán | Time | Space | Pattern |
|----------|------|-------|---------|
| LCA | O(n) | O(h) | DFS + bubble up |
| Serialize / Deserialize | O(n) | O(n) | BFS level-order |
| Max Path Sum | O(n) | O(h) | DFS + global max |
| Diameter | O(n) | O(h) | DFS + global max |
| Validate BST | O(n) | O(h) | DFS + min/max bounds |

Trong đó h = chiều cao cây. Cây cân bằng: h = O(log n). Cây lệch (skewed): h = O(n).

---

## Tổng kết

5 bài tree patterns đều xoay quanh **DFS đệ quy**. Mấu chốt:

1. **LCA** -- hỏi cả 2 nhánh, nếu cả 2 trả về thì node hiện tại là đáp án
2. **Serialize/Deserialize** -- BFS level-order, `null` cho node trống
3. **Max Path Sum** -- DFS trả về 1 nhánh, nhưng cập nhật global max với cả 2 nhánh
4. **Diameter** -- giống Max Path Sum nhưng đếm cạnh thay vì tổng
5. **Validate BST** -- truyền min/max bounds xuống, đừng chỉ check cha-con!

Pattern chung: **DFS đi xuống, tổng hợp khi quay lên, dùng biến global cho kết quả cross-branch.**

---

## Khi nào dùng pattern nào?

Bạn đọc đề, thấy keyword nào thì dùng pattern tương ứng:

| Keyword trong đề | Pattern | Bài |
|---|---|---|
| "ancestor", "common parent" | LCA (DFS bubble up) | #236 |
| "serialize", "encode/decode", "save/load" | BFS level-order | #297 |
| "maximum path sum", "tổng đường đi lớn nhất" | DFS + global max | #124 |
| "diameter", "longest path", "đường đi dài nhất" | DFS + global max (đếm cạnh) | #543 |
| "valid BST", "kiểm tra BST" | DFS + min/max bounds | #98 |
| Xử lý **theo tầng** | BFS | #102, #199, #103 |
| Xử lý **đường đi/chiều sâu** | DFS | #104, #111, #124 |

---

## Luyện tập

Bạn đã hiểu 5 patterns. Giờ thực hành để nó ngấm vào máu:

| Bài | LeetCode | Độ khó | Pattern |
|-----|----------|--------|---------|
| Lowest Common Ancestor of a Binary Tree | [#236](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/) | Medium | DFS bubble up |
| Binary Tree Maximum Path Sum | [#124](https://leetcode.com/problems/binary-tree-maximum-path-sum/) | Hard | DFS return value + global max |
| Serialize and Deserialize Binary Tree | [#297](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/) | Hard | BFS level-order |
| Diameter of Binary Tree | [#543](https://leetcode.com/problems/diameter-of-binary-tree/) | Easy | DFS + global max |
| Validate Binary Search Tree | [#98](https://leetcode.com/problems/validate-binary-search-tree/) | Medium | DFS + min/max bounds |
| Path Sum III | [#437](https://leetcode.com/problems/path-sum-iii/) | Medium | DFS + prefix sum |

> **Gợi ý thứ tự:** Làm #543 (Diameter) trước vì dễ nhất. Rồi #236 (LCA), #98 (Validate BST), #124 (Max Path Sum), cuối cùng #297 (Serialize). Mỗi bài tự code từ đầu, **đừng copy** -- tay phải nhớ, không chỉ mắt.

---

## Tiếp theo

Phần tiếp theo: **Cấu trúc nâng cao** -- bắt đầu với **Segment Tree**. Đây là cấu trúc dữ liệu mạnh mẽ cho bài toán range query (tìm min/max/sum trong 1 đoạn) với thời gian O(log n). Nếu bạn đã nắm vững DFS trên binary tree ở chương này, Segment Tree sẽ không quá khó -- vì nó cũng là một dạng binary tree đặc biệt!

---

## Code Rust

```rust,noplayground
{{#include ../../../../src/tree_patterns.rs}}
```

---

[← Graph Patterns](./08-graph-patterns.md) | [Segment Tree →](../08-advanced/01-segment-tree.md)
