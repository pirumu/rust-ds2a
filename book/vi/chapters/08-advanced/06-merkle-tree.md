# Merkle Tree

## Đây là gì?

Bạn đã học Binary Tree ở Phần 3 và Hashing ở Phần 4. Merkle Tree kết hợp cả hai để kiểm tra dữ liệu có bị thay đổi không.

Tưởng tượng bạn là giáo viên chấm bài cho 8 học sinh. Bạn tính **điểm trung bình từng cặp**, rồi **trung bình của từng cặp trung bình**, cứ thế cho đến khi còn **1 con số duy nhất** đại diện cho cả lớp. Nếu 1 học sinh sửa bài, điểm trung bình cặp đó thay đổi, rồi lan lên trên, cuối cùng con số tổng cũng khác. Bạn chỉ cần so 1 con số tổng là biết "có ai sửa bài". Muốn tìm ai sửa? Đi từ trên xuống: nhánh nào số khác thì đi vào nhánh đó, chỉ cần **log n bước** là tìm ra thủ phạm.

**Merkle Tree hoạt động y hệt** — nhưng thay vì "điểm trung bình", nó dùng **hash**. Mỗi lá là hash của 1 khối dữ liệu. Mỗi nút cha là hash của 2 con gộp lại. Thay đổi 1 byte dữ liệu ở lá → hash thay đổi **dây chuyền lên đến root**. So sánh root hash là biết ngay dữ liệu có bị sửa không, và đi xuống cây là tìm ra chỗ sửa.

> Git dùng Merkle Tree! Mỗi khi bạn `git commit`, Git tính hash cho toàn bộ file và thư mục thành một cây hash. Thay đổi 1 dòng code → hash thay đổi dây chuyền lên đến root. Đó là lý do Git phát hiện mọi thay đổi ngay lập tức.

## Ứng dụng thực tế

| Ở đâu | Dùng để làm gì |
|--------|----------------|
| **Git** | Mỗi commit là root hash của Merkle Tree chứa tất cả file. Thay đổi 1 byte → hash khác → Git biết ngay. |
| **Blockchain** | Bitcoin gom hàng ngàn giao dịch vào 1 block. Merkle root nằm trong block header. Muốn chứng minh 1 giao dịch nằm trong block? Chỉ cần proof O(log n), không cần tải cả block. |
| **P2P download** | BitTorrent chia file thành nhiều mảnh, tải từ nhiều người. Merkle Tree giúp kiểm tra từng mảnh có đúng không — nếu ai gửi mảnh sai, phát hiện ngay. |
| **Database sync** | Amazon DynamoDB, Cassandra dùng Merkle Tree để so sánh dữ liệu giữa 2 server. Nếu root hash giống nhau → dữ liệu giống nhau. Khác nhau → đi sâu vào cây để tìm chỗ khác. |

## Cấu trúc

Merkle Tree là **binary tree** mà:
- **Mỗi lá** (leaf) = hash của 1 khối dữ liệu
- **Mỗi nút trong** (internal node) = hash của 2 con nó gộp lại
- **Root** = hash đại diện cho TOÀN BỘ dữ liệu

```
                ┌─────────────┐
                │  Root Hash  │  ← hash(H12 + H34)
                │   H1234     │    "tem niêm phong" cho cả 4 khối
                └──────┬──────┘
                       │
            ┌──────────┴──────────┐
            │                     │
       ┌────┴────┐          ┌────┴────┐
       │  H12    │          │  H34    │
       │hash(H1  │          │hash(H3  │
       │   +H2)  │          │   +H4)  │
       └────┬────┘          └────┬────┘
            │                     │
       ┌────┴────┐          ┌────┴────┐
       │         │          │         │
    ┌──┴──┐  ┌──┴──┐   ┌──┴──┐  ┌──┴──┐
    │ H1  │  │ H2  │   │ H3  │  │ H4  │
    │hash │  │hash │   │hash │  │hash │
    │("A")│  │("B")│   │("C")│  │("D")│
    └─────┘  └─────┘   └─────┘  └─────┘
       ↑        ↑          ↑        ↑
     "A"      "B"        "C"      "D"   ← dữ liệu gốc
```

Nếu ai đó sửa `"C"` thành `"X"`:
- H3 thay đổi (vì hash("X") ≠ hash("C"))
- H34 thay đổi (vì hash(H3 + H4) khác)
- Root hash thay đổi → **phát hiện ngay!**

## Xây dựng (Build)

### Bước 1: Hash từng khối dữ liệu

```
Dữ liệu:   "A"     "B"     "C"     "D"
              │       │       │       │
              ▼       ▼       ▼       ▼
Leaves:     h("A")  h("B")  h("C")  h("D")
```

### Bước 2: Nếu số lá không phải lũy thừa 2, duplicate phần tử cuối

Ví dụ 3 phần tử → duplicate "C" thành 4:

```
"A"   "B"   "C"   "C"   ← duplicate
```

Tại sao? Vì Merkle Tree là binary tree đầy đủ — mỗi nút phải có đúng 2 con.

### Bước 3: Ghép cặp, hash lên

```
Level 2 (lá):    H1      H2      H3      H4
                   \    /          \    /
Level 1:          H12 = hash(H1+H2)  H34 = hash(H3+H4)
                        \          /
Level 0 (root):       H1234 = hash(H12+H34)
```

### Trong Rust

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_data(data: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

fn hash_pair(left: u64, right: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    left.hash(&mut hasher);
    right.hash(&mut hasher);
    hasher.finish()
}
```

Tree được lưu dạng flat array (giống binary heap):
- `nodes[0]` = root
- Con trái của `nodes[i]` = `nodes[2*i + 1]`
- Con phải của `nodes[i]` = `nodes[2*i + 2]`

```rust
pub fn build(data: &[&str]) -> Self {
    let mut leaves: Vec<u64> = data.iter()
        .map(|d| hash_data(d))
        .collect();

    // Pad to power of 2
    while leaves.len().count_ones() != 1 {
        leaves.push(*leaves.last().unwrap());
    }

    let n = leaves.len();
    let total = 2 * n - 1;
    let mut nodes = vec![0u64; total];

    // Fill leaves (cuối mảng)
    for (i, &h) in leaves.iter().enumerate() {
        nodes[total - n + i] = h;
    }

    // Build bottom-up
    for i in (0..total - n).rev() {
        nodes[i] = hash_pair(nodes[2*i + 1], nodes[2*i + 2]);
    }

    Self { nodes, leaf_count: data.len() }
}
```

## Proof Generation — Chứng minh 1 lá thuộc cây

Đây là tính năng hay nhất của Merkle Tree. Bạn muốn chứng minh `"C"` nằm trong cây mà **không cần gửi toàn bộ cây**? Chỉ cần gửi **proof path** — danh sách các sibling hash từ lá lên root.

### Ví dụ: Proof cho `"C"` (index 2)

```
                ┌─────────────┐
                │  Root Hash  │  ← người verify đã biết
                │   H1234     │
                └──────┬──────┘
                       │
            ┌──────────┴──────────┐
            │                     │
       ┌────┴────┐          ┌────┴────┐
       │ ★ H12   │          │  H34    │
       │(sibling)│          │         │
       └─────────┘          └────┬────┘
                                  │
                            ┌────┴────┐
                            │         │
                         ┌──┴──┐  ┌──┴──┐
                         │ H3  │  │★ H4 │
                         │(cần │  │(sib)│
                         │prove│  │     │
                         └─────┘  └─────┘
                            ↑
                          "C"
```

Proof = `[(H4, is_right=true), (H12, is_right=false)]`

Nghĩa là:
1. Từ H3, anh em bên phải là H4
2. Lên 1 level, anh em bên trái là H12

Chỉ cần **2 hash** thay vì toàn bộ cây! Với 1 triệu lá, proof chỉ cần ~20 hash (log₂ 1,000,000 ≈ 20).

```rust
pub fn generate_proof(&self, index: usize) -> Vec<(u64, bool)> {
    let n = (self.nodes.len() + 1) / 2;
    let mut pos = self.nodes.len() - n + index;
    let mut proof = Vec::new();

    while pos > 0 {
        let parent = (pos - 1) / 2;
        let left = 2 * parent + 1;
        let right = 2 * parent + 2;

        if pos == left {
            proof.push((self.nodes[right], true));  // sibling bên phải
        } else {
            proof.push((self.nodes[left], false));   // sibling bên trái
        }
        pos = parent;
    }
    proof
}
```

## Verification — Kiểm chứng

Người verify có 3 thứ:
1. **Root hash** (đã biết trước, tin tưởng)
2. **Dữ liệu cần kiểm tra** (`"C"`)
3. **Proof** từ bên gửi

Cách verify:

```
Bước 1: current = hash("C")              → H3
Bước 2: current = hash(current, H4)       → H34    (H4 ở bên phải)
Bước 3: current = hash(H12, current)       → H1234  (H12 ở bên trái)
Bước 4: current == root_hash?             → ✅ Đúng!
```

Nếu dữ liệu bị sửa (ví dụ `"X"` thay vì `"C"`):

```
Bước 1: current = hash("X")               → H_sai
Bước 2: current = hash(current, H4)        → H34_sai
Bước 3: current = hash(H12, current)        → H_root_sai
Bước 4: H_root_sai == root_hash?           → ❌ Sai! Phát hiện giả mạo!
```

```rust
pub fn verify(root: u64, data: &str, proof: &[(u64, bool)]) -> bool {
    let mut current = hash_data(data);

    for &(sibling, is_right) in proof {
        if is_right {
            current = hash_pair(current, sibling);
        } else {
            current = hash_pair(sibling, current);
        }
    }

    current == root
}
```

## Bảng độ phức tạp

| Thao tác | Time | Space | Ghi chú |
|----------|------|-------|---------|
| **Build** | O(n) | O(n) | n = số khối dữ liệu. Tạo 2n - 1 node. |
| **Root hash** | O(1) | O(1) | Chỉ đọc `nodes[0]`. |
| **Generate proof** | O(log n) | O(log n) | Đi từ lá lên root, mỗi level lấy 1 sibling. |
| **Verify** | O(log n) | O(1) | Hash lại từ lá lên root, so sánh. |

Điểm mạnh lớn nhất: verify chỉ cần **O(log n)** — với 1 tỷ phần tử, chỉ cần ~30 bước hash!

## So sánh với cách kiểm tra "ngây thơ"

| Cách | Để verify 1 phần tử | Dữ liệu cần gửi |
|------|---------------------|------------------|
| Gửi toàn bộ dữ liệu, hash lại | O(n) | O(n) |
| **Merkle proof** | **O(log n)** | **O(log n)** |

Với Bitcoin block chứa ~2000 giao dịch: thay vì gửi cả 2000, chỉ cần gửi ~11 hash (log₂ 2000 ≈ 11). Tiết kiệm gấp ~200 lần!

## Tóm tắt

- Merkle Tree = binary tree of hashes. Root hash là "dấu vân tay" của toàn bộ dữ liệu.
- Thay đổi bất kỳ dữ liệu nào → root hash thay đổi → phát hiện ngay.
- Proof chỉ cần O(log n) hash — cực kỳ hiệu quả cho hệ thống lớn.
- Dùng khắp nơi: Git, blockchain, P2P, distributed database.
