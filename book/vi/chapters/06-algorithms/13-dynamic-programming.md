# Dynamic Programming

> 💡 **Đừng lo lắng:** DP nổi tiếng là "đáng sợ nhất" nhưng thực ra nó chỉ là **recursion + ghi nhớ kết quả**. Nếu bạn đã hiểu recursion ở [chương đệ quy](./01-recursion.md), bạn đã đi được 80% đường rồi. Phần còn lại chỉ là học cách lưu kết quả để không tính lại. Thở sâu, uống ngụm cà phê, rồi mình bắt đầu nhé.

---

## Đây là gì?

Tưởng tượng bạn đang ôn thi. Mỗi lần gặp một bài toán, bạn giải xong rồi **ghi kết quả ra giấy**. Lần sau gặp lại bài y hệt, thay vì giải lại từ đầu, bạn chỉ cần **lật lại trang giấy** đã ghi.

Đây chính là **Dynamic Programming** (quy hoạch động, viết tắt DP) — kỹ thuật tối ưu cho các bài toán có hai tính chất:

1. **Optimal substructure** (cấu trúc con tối ưu) — lời giải tối ưu được xây từ lời giải tối ưu của các bài toán con.
2. **Overlapping sub-problems** (bài toán con chồng chéo) — cùng một bài toán con được giải đi giải lại nhiều lần.

DP loại bỏ việc tính trùng bằng cách lưu kết quả. Có 2 cách tiếp cận:

| Cách tiếp cận | Hướng | Lưu trữ |
|--------------|-------|---------|
| **Memoization** (ghi nhớ) | Top-down — từ bài lớn xuống nhỏ | HashMap hoặc mảng |
| **Tabulation** (lập bảng) | Bottom-up — từ bài nhỏ lên lớn | Bảng DP |

**Tại sao cần DP?** Vì nếu không, nhiều bài toán sẽ có độ phức tạp mũ (exponential). Ví dụ: Fibonacci đệ quy thường mất O(2^n), nhưng với DP chỉ còn O(n).

### DP Framework -- 4 bước giải mọi bài DP

Bất kể bài DP nào, bạn đều có thể đi theo 4 bước này. Giống như công thức nấu ăn -- cứ theo bước là ra món:

| Bước | Hỏi gì? | Ví dụ (Coin Change) |
|------|---------|---------------------|
| **1. Define state** (định nghĩa trạng thái) | `dp[i]` nghĩa là gì? | `dp[i]` = số đồng xu ít nhất để đổi ra `i` xu |
| **2. Transition formula** (công thức chuyển) | `dp[i]` tính từ đâu? | `dp[i] = min(dp[i - c] + 1)` với mọi đồng `c` |
| **3. Base case** (trường hợp gốc) | Bắt đầu từ đâu? | `dp[0] = 0` (đổi 0 xu cần 0 đồng) |
| **4. Iteration order** (thứ tự duyệt) | Duyệt từ đâu đến đâu? | Từ `1` đến `amount`, vì `dp[i]` cần `dp[i-c]` (nhỏ hơn) |

Mỗi bài DP trong chương này, bạn sẽ thấy mình dùng đúng 4 bước này. Cứ luyện nhiều là quen.

### Nhận diện DP -- Khi nào dùng DP?

Gặp bài mới, hãy hỏi 3 câu hỏi này. Nếu cả 3 đều "Có" thì gần như chắc chắn là DP:

- [ ] **Optimal substructure?** -- Lời giải tối ưu có xây được từ lời giải tối ưu của bài nhỏ hơn không?
- [ ] **Overlapping subproblems?** -- Có bài toán con nào bị tính đi tính lại không?
- [ ] **Counting / Optimization?** -- Đề bài hỏi "bao nhiêu cách", "ít nhất", "nhiều nhất", "có thể hay không"?

Nếu đề chỉ hỏi "in ra tất cả đáp án" thì thường là **backtracking** (chương sau), không phải DP.

### Top-down vs Bottom-up -- cùng 1 bài, 2 cách giải

Lấy Fibonacci làm ví dụ để so sánh 2 cách tiếp cận:

```
Top-down (Memoization)              Bottom-up (Tabulation)
──────────────────────              ──────────────────────
Bắt đầu từ bài LỚN,               Bắt đầu từ bài NHỎ,
gọi đệ quy xuống nhỏ,             xây bảng lên lớn,
lưu kết quả vào cache.             điền bảng từ trái sang phải.

fn fib(n, memo) {                  fn fib(n) {
  if memo[n] exists:                 dp[0] = 0; dp[1] = 1;
    return memo[n];                  for i in 2..=n {
  memo[n] = fib(n-1) + fib(n-2);      dp[i] = dp[i-1] + dp[i-2];
  return memo[n];                    }
}                                    return dp[n];
                                   }

Ưu: viết tự nhiên, chỉ tính       Ưu: không dùng stack đệ quy,
    subproblem cần thiết.               dễ tối ưu bộ nhớ.
Nhược: stack overflow nếu n lớn.   Nhược: phải tính TẤT CẢ
                                        subproblem, kể cả cái
                                        không cần.
```

Trong thực tế, bottom-up phổ biến hơn vì dễ tối ưu bộ nhớ và không lo stack overflow. Nhưng top-down tiện khi bạn mới bắt đầu suy nghĩ về bài toán.

---

## Hoạt động như thế nào?

### Fibonacci — ví dụ kinh điển

**Không dùng DP (đệ quy thường):**

```
fib(5)
├── fib(4)
│   ├── fib(3)            <-- tính lần 1
│   │   ├── fib(2)        <-- tính lần 1
│   │   └── fib(1)
│   └── fib(2)            <-- tính lần 2 (TRÙNG!)
└── fib(3)                <-- tính lần 2 (TRÙNG!)
    ├── fib(2)            <-- tính lần 3 (TRÙNG!)
    └── fib(1)

Rất nhiều phép tính bị lặp lại! Tổng: O(2^n) phép tính.
```

**Dùng DP (tabulation — lập bảng từ dưới lên):**

```
F(0)=0  F(1)=1  F(2)=1  F(3)=2  F(4)=3  F(5)=5  F(6)=8  ...

Tính từ trái sang phải. Mỗi ô chỉ cần 2 ô trước đó.
Thực tế chỉ cần lưu 2 biến -> O(1) bộ nhớ!
```

### 0/1 Knapsack — bài toán cái túi

Tưởng tượng bạn đi cắm trại, ba lô chỉ chứa được 7 kg. Bạn có 4 món đồ, mỗi món có trọng lượng và giá trị khác nhau. Chọn món nào để tổng giá trị lớn nhất?

Đồ vật: trọng lượng=[1,3,4,5], giá trị=[1,4,5,7], sức chứa=7.

Bảng DP `dp[i][w]` = giá trị lớn nhất khi dùng i đồ vật đầu tiên với sức chứa w:

```
        w: 0  1  2  3  4  5  6  7
item 0:    0  1  1  1  1  1  1  1
item 1:    0  1  1  4  5  5  5  5
item 2:    0  1  1  4  5  6  6  9
item 3:    0  1  1  4  5  7  8  9

Đáp án: dp[4][7] = 9
```

Mỗi ô: so sánh "không lấy món này" vs "lấy món này", chọn cái lớn hơn.

### Longest Common Subsequence (LCS) — dãy con chung dài nhất

Tìm dãy con chung dài nhất của 2 chuỗi. Ví dụ: "ABCBDAB" và "BDCAB".

```
      ""  B  D  C  A  B
  ""   0  0  0  0  0  0
  A    0  0  0  0  1  1
  B    0  1  1  1  1  2
  C    0  1  1  2  2  2
  B    0  1  1  2  2  3
  D    0  1  2  2  2  3
  A    0  1  2  2  3  3
  B    0  1  2  2  3  4

Độ dài LCS = 4, một LCS = "BDAB"
```

Cách đọc bảng: nếu 2 ký tự giống nhau -> lấy chéo + 1. Khác nhau -> lấy max(trên, trái).

### Coin Change — đổi tiền

Có các mệnh giá xu: [1, 5, 10, 25]. Cần đổi ra đúng 30 xu. Ít nhất cần bao nhiêu đồng?

```
dp[i] = số đồng xu ít nhất để đổi ra i

dp[0]  = 0
dp[1]  = 1  (1 đồng 1xu)
dp[5]  = 1  (1 đồng 5xu)
dp[10] = 1  (1 đồng 10xu)
dp[25] = 1  (1 đồng 25xu)
dp[30] = 2  (25xu + 5xu)
```

### Edit Distance — khoảng cách chỉnh sửa

Cần bao nhiêu phép chỉnh sửa (thêm, xóa, thay) để biến "kitten" thành "sitting"?

```
        ""  s  i  t  t  i  n  g
   ""    0  1  2  3  4  5  6  7
   k     1  1  2  3  4  5  6  7
   i     2  2  1  2  3  4  5  6
   t     3  3  2  1  2  3  4  5
   t     4  4  3  2  1  2  3  4
   e     5  5  4  3  2  2  3  4
   n     6  6  5  4  3  3  2  3

Đáp án: 3 phép chỉnh sửa (thay k->s, thay e->i, thêm g)
```

---

## Code Rust

```rust
/// Fibonacci — O(n) thời gian, O(1) bộ nhớ.
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 { return n; }
    let (mut prev2, mut prev1) = (0u64, 1u64);
    for _ in 2..=n {
        let curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }
    prev1
}

/// 0/1 Knapsack — chọn đồ vật để tối đa giá trị, không vượt sức chứa.
pub fn knapsack_01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=capacity {
            dp[i][w] = dp[i - 1][w];  // Không lấy món i
            if weights[i - 1] <= w {
                // Lấy món i: giá trị = dp trước đó với sức chứa còn lại + giá trị món i
                let take = dp[i - 1][w - weights[i - 1]] + values[i - 1];
                if take > dp[i][w] { dp[i][w] = take; }
            }
        }
    }
    dp[n][capacity]
}

/// LCS — tìm dãy con chung dài nhất.
pub fn longest_common_subsequence(s1: &str, s2: &str) -> String {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i-1] == b[j-1] {
                dp[i-1][j-1] + 1       // Ký tự giống -> lấy chéo + 1
            } else {
                dp[i-1][j].max(dp[i][j-1])  // Khác -> lấy max
            };
        }
    }
    // Truy ngược để tìm chuỗi LCS
    let mut result = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if a[i-1] == b[j-1] { result.push(a[i-1]); i -= 1; j -= 1; }
        else if dp[i-1][j] >= dp[i][j-1] { i -= 1; }
        else { j -= 1; }
    }
    result.reverse();
    result.into_iter().collect()
}

/// Coin Change — số đồng xu ít nhất để đổi ra amount.
pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;  // Đổi 0 xu cần 0 đồng
    for a in 1..=amount {
        for &c in coins {
            if c <= a && dp[a - c] != usize::MAX {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }
    if dp[amount] == usize::MAX { None } else { Some(dp[amount]) }
}

/// Edit Distance — số phép chỉnh sửa tối thiểu.
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }  // Xóa hết -> i phép
    for j in 0..=n { dp[0][j] = j; }  // Thêm hết -> j phép
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i-1] == b[j-1] {
                dp[i-1][j-1]    // Giống nhau -> không cần chỉnh
            } else {
                1 + dp[i-1][j-1]            // Thay thế
                    .min(dp[i-1][j])         // Xóa
                    .min(dp[i][j-1])         // Thêm
            };
        }
    }
    dp[m][n]
}
```

**Ghi chú về Rust:**

- `vec![vec![0; n+1]; m+1]` tạo bảng 2D. Trong Rust, đây là Vec lồng Vec — đơn giản nhưng không tối ưu cache bằng mảng 1D.
- `usize::MAX` dùng làm giá trị "vô cực" trong Coin Change. Phải kiểm tra trước khi cộng để tránh overflow.
- Fibonacci chỉ giữ 2 biến (`prev1`, `prev2`) thay vì cả mảng -> tiết kiệm bộ nhớ từ O(n) xuống O(1).

---

## Matrix DP — DP trên lưới

### Unique Paths — đường đi duy nhất trong lưới

Tưởng tượng bạn ở góc trên-trái của một bảng ô vuông m x n. Bạn chỉ được đi **sang phải** hoặc **xuống dưới**. Hỏi có bao nhiêu cách đi đến góc dưới-phải?

Giống như đi từ nhà đến trường trong thành phố có đường kẻ ô bàn cờ -- bạn chỉ được rẽ phải hoặc đi thẳng xuống.

**Công thức truy hồi:**

```
dp[i][j] = dp[i-1][j] + dp[i][j-1]

Số cách đến ô (i,j) = số cách đến ô trên + số cách đến ô trái.
```

**Ví dụ: lưới 3x4**

```
  col:  0    1    2    3
row 0:  1    1    1    1      <- hàng đầu: chỉ có 1 cách (đi phải)
row 1:  1    2    3    4
row 2:  1    3    6   [10]    <- đáp án: 10 cách

Mỗi ô = ô trên + ô trái.
Ví dụ: dp[2][3] = dp[1][3] + dp[2][2] = 4 + 6 = 10
```

**Rust:** `unique_paths(m, n) -> u64` -- Dùng rolling array chỉ cần 1 hàng, tiết kiệm bộ nhớ.

**Độ phức tạp:** O(m * n) thời gian, O(n) bộ nhớ.

### Min Path Sum — tổng đường đi nhỏ nhất

Vẫn là lưới m x n, nhưng lần này mỗi ô có một **chi phí** (số). Tìm đường đi từ góc trên-trái đến góc dưới-phải sao cho tổng chi phí **nhỏ nhất**.

Giống như đi giao hàng trong thành phố -- mỗi con đường có phí cầu đường khác nhau, bạn muốn tìm đường rẻ nhất.

**Công thức truy hồi:**

```
dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])

Chi phí đến ô (i,j) = chi phí ô này + min(từ trên xuống, từ trái sang).
```

**Ví dụ: grid = [[1,3,1],[1,5,1],[4,2,1]]**

```
Grid:                   Bảng DP:
  1  3  1                 1  4  5
  1  5  1                 2  7  6
  4  2  1                 6  8 [7]

Đường đi tốt nhất: 1 -> 3 -> 1 -> 1 -> 1 = 7? Không!
Đường đi tốt nhất: 1 -> 1 -> 5 -> 1 -> 1 = 9? Cũng không!
Đường đi tốt nhất: 1 -> 3 -> 1 -> 1 -> 1 = 7

Truy ngược: dp[2][2]=7 <- dp[1][2]=6 <- dp[0][2]=5 <- dp[0][1]=4 <- dp[0][0]=1
Đường đi: (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2), tổng = 1+3+1+1+1 = 7
```

**Rust:** `min_path_sum(grid: &[Vec<i32>]) -> i32`

**Độ phức tạp:** O(m * n) thời gian, O(n) bộ nhớ.

---

## String DP — DP trên chuỗi

### Longest Palindromic Subsequence — dãy con đối xứng dài nhất

Palindrome là chuỗi đọc xuôi đọc ngược đều giống nhau (ví dụ: "aba", "racecar"). Bài này tìm **dãy con** dài nhất mà là palindrome. Dãy con (subsequence) không cần liên tiếp -- bạn có thể bỏ qua một số ký tự.

Tưởng tượng bạn có một dãy chữ cái viết trên các tấm thẻ. Bạn muốn chọn ra nhiều thẻ nhất có thể, giữ nguyên thứ tự, sao cho đọc từ trái sang phải và từ phải sang trái đều giống nhau.

**Công thức truy hồi:**

```
Nếu s[i] == s[j]:  dp[i][j] = dp[i+1][j-1] + 2
Nếu s[i] != s[j]:  dp[i][j] = max(dp[i+1][j], dp[i][j-1])

dp[i][j] = độ dài LPS của chuỗi s[i..=j]
```

**Ví dụ: s = "bbbab"**

```
      b  b  b  a  b
  b  [1] 2  3  3  4
  b      1  2  2  3
  b         1  1  3
  a            1  1
  b               1

Đọc: dp[0][4] = 4. Dãy con palindrome dài nhất = "bbbb" (bỏ 'a').

Cách tính dp[0][4]: s[0]='b' == s[4]='b' -> dp[1][3] + 2 = 2 + 2 = 4
Cách tính dp[1][3]: s[1]='b' != s[3]='a' -> max(dp[2][3], dp[1][2]) = max(1, 2) = 2
```

**Rust:** `longest_palindromic_subsequence(s: &str) -> usize`

**Độ phức tạp:** O(n^2) thời gian, O(n^2) bộ nhớ.

### Wildcard Matching — so khớp ký tự đại diện

Bạn có một chuỗi văn bản và một pattern chứa ký tự đặc biệt:
- `?` khớp với **đúng 1** ký tự bất kỳ
- `*` khớp với **bất kỳ dãy ký tự nào** (kể cả rỗng)

Giống như khi bạn tìm file trên máy tính: `*.txt` tìm tất cả file có đuôi .txt, `photo_?.jpg` tìm photo_1.jpg, photo_2.jpg, ...

**Công thức truy hồi:**

```
Nếu p[j] == '*':   dp[i][j] = dp[i][j-1]       (* khớp rỗng)
                             || dp[i-1][j]       (* khớp thêm 1 ký tự)
Nếu p[j] == '?' hoặc p[j] == s[i]:
                    dp[i][j] = dp[i-1][j-1]
Ngược lại:          dp[i][j] = false
```

**Ví dụ: text = "adceb", pattern = "\*a\*b"**

```
         ""   *    a    *    b
  ""      T   T    F    F    F
  a       F   T    T    T    F
  d       F   T    F    T    F
  c       F   T    F    T    F
  e       F   T    F    T    F
  b       F   T    F    T    T  <- khớp!

T = true, F = false

'*' đầu tiên khớp rỗng -> dp[0][1] = T
'*' thứ ba khớp "dce"  -> dp[4][3] = T
'b' cuối khớp 'b'      -> dp[5][4] = T
```

**Rust:** `is_match_wildcard(text: &str, pattern: &str) -> bool`

**Độ phức tạp:** O(m * n) thời gian, O(m * n) bộ nhớ.

### Palindrome Min Cuts — cắt palindrome ít nhất

Cho một chuỗi s, cắt thành các phần sao cho **mỗi phần đều là palindrome**. Tìm **số nhát cắt ít nhất**.

Ví dụ: "aab" -> cắt thành "aa" | "b" -> 1 nhát cắt. Cả "aa" và "b" đều là palindrome.

Giống như cắt một thanh kẹo thành các miếng, mỗi miếng phải có tính đối xứng. Cắt càng ít nhát càng tốt vì mỗi nhát cắt tốn công.

**Công thức truy hồi:**

```
Bước 1: Tính bảng is_pal[i][j] = true nếu s[i..=j] là palindrome
Bước 2: cuts[i] = 0 nếu s[0..=i] đã là palindrome
         cuts[i] = min(cuts[j-1] + 1) với mọi j mà s[j..=i] là palindrome
```

**Ví dụ: s = "aab"**

```
Bảng palindrome:
  is_pal[0][0] = 'a'   -> true
  is_pal[1][1] = 'a'   -> true
  is_pal[2][2] = 'b'   -> true
  is_pal[0][1] = "aa"  -> true
  is_pal[1][2] = "ab"  -> false
  is_pal[0][2] = "aab" -> false

Tính cuts:
  cuts[0] = 0           "a" là palindrome
  cuts[1] = 0           "aa" là palindrome (is_pal[0][1] = true)
  cuts[2] = ?           "aab" không là palindrome
            Thử j=1: is_pal[1][2]="ab"? Không.
            Thử j=2: is_pal[2][2]="b"?  Có! -> cuts[1] + 1 = 0 + 1 = 1
  cuts[2] = 1           Cắt: "aa" | "b"
```

**Rust:** `palindrome_min_cuts(s: &str) -> usize`

**Độ phức tạp:** O(n^2) thời gian, O(n^2) bộ nhớ.

---

## State Machine DP — DP máy trạng thái

Một số bài DP có thể mô hình hóa bằng **máy trạng thái** (state machine). Mỗi ngày bạn ở một trạng thái nào đó, và bạn chuyển giữa các trạng thái theo quy tắc.

### Max Profit with Cooldown — mua bán cổ phiếu có cooldown

Bạn có giá cổ phiếu mỗi ngày. Bạn muốn mua bán để kiếm lời tối đa, nhưng có quy tắc: sau khi **bán**, bạn phải **nghỉ 1 ngày** (cooldown) trước khi mua lại.

Giống như chơi game: sau khi dùng skill mạnh (bán), bạn phải chờ cooldown mới được dùng lại (mua).

**3 trạng thái:**

```
         mua
  REST -------> HOLD
   ^             |
   |    nghỉ     | bán
   |             v
   +------- SOLD
     (bắt buộc nghỉ 1 ngày)

HOLD: đang giữ cổ phiếu (có thể giữ tiếp hoặc bán)
SOLD: vừa bán xong (ngày mai bắt buộc nghỉ)
REST: đang nghỉ (có thể mua hoặc tiếp tục nghỉ)
```

**Công thức truy hồi:**

```
hold[i] = max(hold[i-1], rest[i-1] - price[i])   // giữ tiếp hoặc mua mới
sold[i] = hold[i-1] + price[i]                     // bán hôm nay
rest[i] = max(rest[i-1], sold[i-1])                 // nghỉ hoặc vừa xong cooldown
```

**Ví dụ: prices = [1, 2, 3, 0, 2]**

```
Ngày:     0      1      2      3      4
Giá:      1      2      3      0      2

hold:    -1     -1     -1      1      1
sold:     0      1      2     -1      3
rest:     0      0      1      2      2

Đáp án: max(sold[4], rest[4]) = max(3, 2) = 3
Chiến lược: mua ngày 0, bán ngày 2 (lời 2), nghỉ ngày 3, mua ngày 3, bán ngày 4 (lời 2)
Tổng lời = 2 + ... Hmm, thực ra: mua=1, bán=3 -> lời 2. Nghỉ. Mua=0, bán=2 -> lời 2.
Nhưng cooldown! Bán ngày 2 -> nghỉ ngày 3 -> mua ngày 3 được không? Không! Phải nghỉ ngày 3.
Cách đúng: mua ngày 0 (giá 1), bán ngày 1 (giá 2, lời 1), nghỉ ngày 2, mua ngày 3 (giá 0), bán ngày 4 (giá 2, lời 2). Tổng = 3.
```

**Rust:** `max_profit_with_cooldown(prices: &[i32]) -> i32`

**Độ phức tạp:** O(n) thời gian, O(1) bộ nhớ (chỉ cần 3 biến).

### Max Profit K Transactions — mua bán tối đa k lần

Giống bài trên nhưng không có cooldown. Thay vào đó, bạn chỉ được thực hiện **tối đa k giao dịch** (mua-bán = 1 giao dịch).

Ví dụ k=2: bạn được mua-bán 2 lần.

**Công thức truy hồi:**

```
Với mỗi giao dịch t (từ 0 đến k-1):
  buy[t]  = max(buy[t], sell[t-1] - price)    // mua lần t
  sell[t] = max(sell[t], buy[t] + price)       // bán lần t
```

**Ví dụ: k=2, prices = [3, 2, 6, 5, 0, 3]**

```
          Ngày 0  Ngày 1  Ngày 2  Ngày 3  Ngày 4  Ngày 5
Giá:        3       2       6       5       0       3

buy[0]:    -3      -2      -2      -2       0       0
sell[0]:    0       0       4       4       4       4
buy[1]:    -3      -2       2       2       4       4
sell[1]:    0       0       4       4       4       7

Đáp án: sell[1] = 7
Chiến lược: mua ngày 4 (giá 0), bán ngày 5 (giá 3, lời 3) +
            mua ngày 1 (giá 2), bán ngày 2 (giá 6, lời 4) = 7
```

**Rust:** `max_profit_k_transactions(k: usize, prices: &[i32]) -> i32` -- Khi k >= n/2, tối ưu thành bài unlimited transactions.

**Độ phức tạp:** O(n * k) thời gian, O(k) bộ nhớ.

---

## Bitmask DP — DP dùng bitmask

Bitmask là kỹ thuật dùng **số nhị phân** để biểu diễn **tập hợp con**. Ví dụ: với 4 phần tử {A, B, C, D}, bitmask `1010` nghĩa là chọn B và D.

**Tại sao dùng bitmask?** Vì nó cho phép biểu diễn mọi tập con chỉ bằng 1 số nguyên, rất tiện để làm key trong bảng DP.

### Can Partition Equal Subset — chia mảng thành 2 phần bằng nhau

Cho một mảng số nguyên dương, hỏi: có thể chia thành **2 nhóm** sao cho tổng 2 nhóm **bằng nhau** không?

Giống như chia kẹo cho 2 đứa trẻ -- mỗi đứa phải được số kẹo bằng nhau (tính theo tổng giá trị).

**Ý tưởng:** Nếu tổng mảng là S, ta cần tìm một tập con có tổng = S/2. Đây chính là bài **Subset Sum** -- một biến thể của Knapsack.

**Công thức truy hồi:**

```
dp[s] = true nếu tồn tại tập con có tổng = s

Với mỗi phần tử num (duyệt ngược để không dùng lại):
  dp[s] = dp[s] || dp[s - num]
```

**Ví dụ: nums = [1, 5, 11, 5]**

```
Tổng = 22, target = 11.

Ban đầu:     dp = [T, F, F, F, F, F, F, F, F, F, F, F]
                    0  1  2  3  4  5  6  7  8  9  10 11

Sau num=1:   dp = [T, T, F, F, F, F, F, F, F, F, F, F]
Sau num=5:   dp = [T, T, F, F, F, T, T, F, F, F, F, F]
Sau num=11:  dp = [T, T, F, F, F, T, T, F, F, F, F, T]
                                                       ^-- target = 11, dp[11] = true!

(Không cần duyệt num=5 nữa vì đã tìm được)
Kết quả: Có thể chia! Nhóm 1: {1, 5, 5} = 11, Nhóm 2: {11} = 11.
```

**Rust:** `can_partition_equal_subset(nums: &[i32]) -> bool`

**Độ phức tạp:** O(n * sum) thời gian, O(sum) bộ nhớ.

### Shortest Hamiltonian Path — đường đi Hamilton ngắn nhất

Đây là một trong những bài nổi tiếng nhất của khoa học máy tính. Cho n thành phố và chi phí đi giữa mỗi cặp. Tìm đường đi **qua tất cả thành phố đúng 1 lần** với tổng chi phí nhỏ nhất.

Giống như một nhân viên giao hàng cần giao đến n địa chỉ, mỗi địa chỉ đúng 1 lần, tìm lộ trình ngắn nhất.

**Ý tưởng:** Dùng bitmask để biểu diễn tập các thành phố đã ghé thăm.

```
dp[mask][i] = chi phí nhỏ nhất để đến thành phố i,
              đã ghé thăm đúng các thành phố trong mask

dp[mask | (1<<v)][v] = min(dp[mask][u] + dist[u][v])
  với mọi u đã trong mask, v chưa trong mask
```

**Ví dụ: 3 thành phố, ma trận khoảng cách:**

```
     A   B   C
A  [ 0,  1, 10]
B  [ 1,  0,  1]
C  [10,  1,  0]

Bitmask: A=001, B=010, C=100

Ban đầu:
  dp[001][A] = 0      (xuất phát từ A)
  dp[010][B] = 0      (xuất phát từ B)
  dp[100][C] = 0      (xuất phát từ C)

Mở rộng:
  dp[011][B] = dp[001][A] + dist[A][B] = 0 + 1 = 1     (A -> B)
  dp[011][A] = dp[010][B] + dist[B][A] = 0 + 1 = 1     (B -> A)
  dp[110][C] = dp[010][B] + dist[B][C] = 0 + 1 = 1     (B -> C)
  ...
  dp[111][C] = dp[011][B] + dist[B][C] = 1 + 1 = 2     (A -> B -> C)
  dp[111][A] = dp[110][C] + dist[C][A] = 1 + 10 = 11   (B -> C -> A)
  ...

Đáp án: min(dp[111]) = 2 (đường A -> B -> C hoặc C -> B -> A)
```

**Rust:** `shortest_hamiltonian_path(dist: &[Vec<i32>]) -> i32`

**Độ phức tạp:** O(2^n * n^2) thời gian, O(2^n * n) bộ nhớ. Chỉ chạy được với n nhỏ (khoảng n <= 20).

---

## Interval DP — DP trên khoảng

Interval DP giải các bài mà bạn cần **chọn cách chia một đoạn liên tục** sao cho tối ưu. Bạn thử mọi cách chia đoạn [i, j] tại vị trí k, rồi gộp kết quả.

### Burst Balloons — nổ bóng bay

Có n quả bóng xếp hàng ngang, mỗi quả có một số. Khi nổ quả bóng thứ i, bạn nhận được `nums[left] * nums[i] * nums[right]` xu (left và right là 2 quả bóng còn lại cạnh bên). Tìm cách nổ sao cho thu được **nhiều xu nhất**.

Giống như bóp bóng ở hội chợ -- mỗi quả bóng cho điểm thưởng khác nhau tùy vào vị trí nó bị nổ.

**Mẹo quan trọng:** Thay vì nghĩ "nổ quả nào trước", hãy nghĩ ngược lại -- **quả nào nổ cuối cùng** trong đoạn [left, right]?

```
dp[i][j] = max xu thu được khi nổ hết bóng giữa i và j (exclusive)

dp[i][j] = max(vals[i] * vals[k] * vals[j] + dp[i][k] + dp[k][j])
           với mọi k giữa i và j
```

**Ví dụ: nums = [3, 1, 5, 8]**

```
Thêm biên: vals = [1, 3, 1, 5, 8, 1]
                    ^              ^
                  biên trái    biên phải

Tính dp[i][j] cho mọi khoảng:
  dp[0][2]: nổ bóng 1 (val=3), biên [1, 1] -> 1*3*1 = 3
  dp[1][3]: nổ bóng 2 (val=1), biên [3, 5] -> 3*1*5 = 15
  dp[2][4]: nổ bóng 3 (val=5), biên [1, 8] -> 1*5*8 = 40
  dp[3][5]: nổ bóng 4 (val=8), biên [5, 1] -> 5*8*1 = 40

  dp[0][3]: thử k=1: 1*3*5 + dp[0][1] + dp[1][3] = 15 + 0 + 15 = 30
            thử k=2: 1*1*5 + dp[0][2] + dp[2][3] = 5 + 3 + 0 = 8
            -> dp[0][3] = 30

  ... (tiếp tục cho khoảng dài hơn)

  dp[0][5] = 167  <- đáp án!
```

**Rust:** `burst_balloons(nums: &[i32]) -> i32`

**Độ phức tạp:** O(n^3) thời gian, O(n^2) bộ nhớ.

### Matrix Chain Order — nhân chuỗi ma trận

Nhân nhiều ma trận liền nhau, thứ tự nhân ảnh hưởng đến số phép tính. Ví dụ: nhân 3 ma trận A(10x30), B(30x5), C(5x60):
- (A * B) * C: 10\*30\*5 + 10\*5\*60 = 1500 + 3000 = **4500**
- A * (B * C): 30\*5\*60 + 10\*30\*60 = 9000 + 18000 = **27000**

Cùng kết quả nhưng cách 1 nhanh gấp 6 lần! Bài này tìm cách đặt ngoặc tối ưu.

Giống như khi nấu ăn -- thứ tự bạn trộn nguyên liệu ảnh hưởng đến thời gian. Trộn bột nhỏ trước, rồi mới thêm phần lớn, sẽ nhanh hơn.

**Công thức truy hồi:**

```
dims = [d0, d1, d2, ..., dn]   (ma trận i có kích thước dims[i] x dims[i+1])

dp[i][j] = min cost để nhân ma trận i đến j
dp[i][j] = min(dp[i][k] + dp[k+1][j] + dims[i] * dims[k+1] * dims[j+1])
           với mọi k từ i đến j-1
```

**Ví dụ: dims = [10, 30, 5, 60] (3 ma trận)**

```
Ma trận: A(10x30), B(30x5), C(5x60)

dp[0][0] = 0   (A một mình)
dp[1][1] = 0   (B một mình)
dp[2][2] = 0   (C một mình)

dp[0][1]: k=0 -> 10*30*5 = 1500           (A * B)
dp[1][2]: k=1 -> 30*5*60 = 9000           (B * C)

dp[0][2]: k=0 -> dp[0][0] + dp[1][2] + 10*30*60 = 0 + 9000 + 18000 = 27000
          k=1 -> dp[0][1] + dp[2][2] + 10*5*60  = 1500 + 0 + 3000 = 4500
          -> dp[0][2] = 4500

Đáp án: 4500, cách tối ưu: (A * B) * C
```

**Rust:** `matrix_chain_order(dims: &[usize]) -> usize`

**Độ phức tạp:** O(n^3) thời gian, O(n^2) bộ nhớ.

---

## Độ phức tạp

| Bài toán | Thời gian | Bộ nhớ |
|---------|----------|--------|
| Fibonacci | O(n) | O(1) |
| 0/1 Knapsack | O(n * W) | O(n * W) |
| LCS | O(m * n) | O(m * n) |
| Coin Change | O(amount * \|C\|) | O(amount) |
| Edit Distance | O(m * n) | O(m * n) |
| Unique Paths | O(m * n) | O(n) |
| Min Path Sum | O(m * n) | O(n) |
| Longest Palindromic Subseq | O(n^2) | O(n^2) |
| Wildcard Matching | O(m * n) | O(m * n) |
| Palindrome Min Cuts | O(n^2) | O(n^2) |
| Stock with Cooldown | O(n) | O(1) |
| Stock K Transactions | O(n * k) | O(k) |
| Partition Equal Subset | O(n * sum) | O(sum) |
| Shortest Hamiltonian Path | O(2^n * n^2) | O(2^n * n) |
| Burst Balloons | O(n^3) | O(n^2) |
| Matrix Chain Order | O(n^3) | O(n^2) |

**Giải thích thực tế:**

- DP biến bài toán mũ (exponential) thành đa thức (polynomial). Fibonacci từ O(2^n) -> O(n). Knapsack từ O(2^n) -> O(n*W).
- Nhược điểm: bộ nhớ. Bảng DP có thể rất lớn. Ví dụ: LCS của 2 chuỗi 10.000 ký tự cần bảng 10.000 x 10.000 = 100 triệu ô.
- Mẹo tối ưu: nhiều bài DP chỉ cần hàng trước đó -> có thể giảm bộ nhớ xuống O(n) bằng "rolling array".
- Bitmask DP chỉ dùng được với n nhỏ (n <= 20) vì 2^20 = 1 triệu, 2^25 đã là 33 triệu.
- Interval DP (Burst Balloons, Matrix Chain) tốn O(n^3) -- chấp nhận được với n vài trăm.

### Space Optimization -- Rolling Array

Nhiều bài DP 2D thực ra chỉ cần **hàng trước đó** để tính hàng hiện tại. Thay vì giữ cả bảng `m x n`, ta chỉ cần **2 hàng** (hoặc thậm chí 1 hàng). Kỹ thuật này gọi là **rolling array**.

**Ví dụ: LCS từ O(m*n) bộ nhớ xuống O(n)**

```
Bảng đầy đủ (m+1 hàng):          Rolling array (2 hàng):
┌───┬───┬───┬───┬───┐             ┌───┬───┬───┬───┬───┐
│ 0 │ 0 │ 0 │ 0 │ 0 │  row 0     │ 0 │ 0 │ 0 │ 0 │ 0 │  prev
├───┼───┼───┼───┼───┤             ├───┼───┼───┼───┼───┤
│ 0 │ 0 │ 0 │ 0 │ 1 │  row 1     │ 0 │ 1 │ 1 │ 1 │ 1 │  curr
├───┼───┼───┼───┼───┤             └───┴───┴───┴───┴───┘
│ 0 │ 1 │ 1 │ 1 │ 1 │  row 2     Xong row 2 -> swap prev ↔ curr,
├───┼───┼───┼───┼───┤             tính tiếp row 3...
│ 0 │ 1 │ 1 │ 2 │ 2 │  row 3
└───┴───┴───┴───┴───┘

Mỗi ô dp[i][j] chỉ cần dp[i-1][j], dp[i][j-1], dp[i-1][j-1].
Tất cả đều ở hàng trước (i-1) hoặc hàng hiện tại (i).
-> Chỉ cần 2 hàng!
```

**Khi nào dùng được?** Khi công thức truy hồi chỉ phụ thuộc vào hàng ngay trước đó (hoặc vài hàng trước). Hầu hết Linear DP, Matrix DP, và String DP đều tối ưu được. Interval DP thì không, vì `dp[i][j]` phụ thuộc nhiều khoảng khác nhau.

**Rust tip:** Dùng `std::mem::swap(&mut prev, &mut curr)` để đổi 2 hàng mà không copy dữ liệu.

---

## Phân loại DP

Sau khi học nhiều bài DP, bạn sẽ thấy chúng rơi vào các "họ" chính:

| Họ DP | Đặc điểm | Ví dụ |
|-------|----------|-------|
| **Linear DP** | Duyệt mảng/chuỗi từ trái sang phải | Fibonacci, LIS, Coin Change |
| **Matrix DP** | Bảng 2D, đi phải/xuống | Unique Paths, Min Path Sum |
| **String DP** | 2 chuỗi -> bảng 2D | LCS, Edit Distance, Wildcard |
| **Interval DP** | Chia đoạn [i,j] tại k | Burst Balloons, Matrix Chain |
| **State Machine DP** | Chuyển trạng thái | Stock Cooldown, Stock K Trans |
| **Bitmask DP** | Tập con = số nhị phân | Hamiltonian Path, Subset Sum |

Khi gặp bài mới, hãy tự hỏi: "Bài này thuộc họ nào?" Điều đó giúp bạn chọn đúng cách tiếp cận.

---

## Pitfalls -- Sai lầm thường gặp

**1. Sai iteration order**

❌ Sai: Coin Change duyệt `dp[s]` từ trái sang phải khi dùng 0/1 Knapsack (mỗi item chỉ dùng 1 lần).

✅ Đúng: Duyệt **ngược** (từ phải sang trái) để đảm bảo mỗi item chỉ được chọn 1 lần.

💡 Tại sao: Nếu duyệt xuôi, `dp[s - num]` đã bị cập nhật trong cùng vòng lặp, nghĩa là item đó bị dùng nhiều lần. Duyệt ngược thì `dp[s - num]` vẫn là giá trị của vòng trước.

**2. Quên base case**

❌ Sai: Không set `dp[0] = 0` trong Coin Change, hoặc không set `dp[i][0] = i` trong Edit Distance.

✅ Đúng: Luôn xác định base case trước khi viết vòng lặp.

💡 Tại sao: Base case là "nền móng" của tòa nhà DP. Thiếu nền móng thì mọi tầng trên đều sai.

**3. State definition thiếu**

❌ Sai: Bài Stock with Cooldown mà chỉ dùng `dp[i]` = lợi nhuận tối đa ngày thứ `i`. Thiếu thông tin "đang giữ cổ phiếu hay không".

✅ Đúng: Cần tách thành `hold[i]`, `sold[i]`, `rest[i]` -- mỗi trạng thái là một mảng riêng.

💡 Tại sao: Nếu state không chứa đủ thông tin để ra quyết định, công thức truy hồi sẽ sai. Hãy tự hỏi: "Với `dp[i]`, mình có đủ thông tin để tính `dp[i+1]` không?"

---

## Luyện tập

Bốn bài kinh điển trên LeetCode, xếp theo độ khó tăng dần:

| # | Bài | Họ DP | Gợi ý |
|---|-----|-------|-------|
| 198 | **House Robber** | Linear DP | `dp[i] = max(dp[i-1], dp[i-2] + nums[i])`. Giống Fibonacci nhưng có chọn/không chọn. |
| 322 | **Coin Change** | Linear DP | Đã có trong chương này. Thử tự code lại không nhìn đáp án. |
| 1143 | **Longest Common Subsequence** | String DP | Đã có trong chương này. Thử thêm rolling array optimization. |
| 72 | **Edit Distance** | String DP | Đã có trong chương này. Thử trace tay bảng DP cho "horse" -> "ros". |

**Mẹo luyện DP:** Đừng vội code. Với mỗi bài, hãy **trace tay trên giấy** trước: vẽ bảng DP, điền từng ô, xem pattern. Khi nào hiểu rõ bảng chạy thế nào thì mới mở editor.

---

## Ví dụ

```rust
use rust_ds2a::dynamic_programming::*;

// Fibonacci
assert_eq!(fibonacci(10), 55);

// 0/1 Knapsack
assert_eq!(knapsack_01(&[1,3,4,5], &[1,4,5,7], 7), 9);

// LCS
assert_eq!(longest_common_subsequence("abcde", "ace"), "ace");

// Coin Change
assert_eq!(coin_change(&[1, 5, 10, 25], 30), Some(2));

// Edit Distance
assert_eq!(edit_distance("kitten", "sitting"), 3);

// Unique Paths
assert_eq!(unique_paths(3, 7), 28);

// Min Path Sum
assert_eq!(min_path_sum(&[vec![1,3,1], vec![1,5,1], vec![4,2,1]]), 7);

// Longest Palindromic Subsequence
assert_eq!(longest_palindromic_subsequence("bbbab"), 4);

// Wildcard Matching
assert!(is_match_wildcard("adceb", "*a*b"));

// Palindrome Min Cuts
assert_eq!(palindrome_min_cuts("aab"), 1);

// Stock with Cooldown
assert_eq!(max_profit_with_cooldown(&[1, 2, 3, 0, 2]), 3);

// Stock K Transactions
assert_eq!(max_profit_k_transactions(2, &[3, 2, 6, 5, 0, 3]), 7);

// Partition Equal Subset
assert!(can_partition_equal_subset(&[1, 5, 11, 5]));

// Shortest Hamiltonian Path
assert_eq!(shortest_hamiltonian_path(&[vec![0,1,10], vec![1,0,1], vec![10,1,0]]), 2);

// Burst Balloons
assert_eq!(burst_balloons(&[3, 1, 5, 8]), 167);

// Matrix Chain Order
assert_eq!(matrix_chain_order(&[10, 30, 5, 60]), 4500);
```

---

## Tiếp theo: Backtracking

DP trả lời "đáp án tối ưu là gì?" hoặc "có bao nhiêu cách?". Nhưng nếu đề bài hỏi **"liệt kê tất cả đáp án"** thì sao? Đó là lúc **Backtracking** (quay lui) vào cuộc. Backtracking thử từng khả năng, quay lại khi đi vào ngõ cụt -- giống như đi trong mê cung. Gặp nhau ở [chương tiếp theo](./14-backtracking.md)!

---

[← Greedy](./12-greedy.md) | [Backtracking →](./14-backtracking.md)
