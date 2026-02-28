# Merkle Tree

> 💡 **Đừng lo lắng:** Chương này dùng đúng 2 thứ bạn đã biết:
> - **Binary Tree** (chương 3) — cây mà mỗi node có tối đa 2 con
> - **Hash function** (chương 4) — hàm biến dữ liệu thành "dấu vân tay" cố định
>
> Merkle Tree chỉ là: **mỗi leaf = hash(data), mỗi parent = hash(left + right)**. Hết. Đọc tiếp nhé.

## Đây là gì?

Bạn đã học Binary Tree ở Phần 3 và Hashing ở Phần 4. Merkle Tree kết hợp cả hai để kiểm tra dữ liệu có bị thay đổi không.

Tưởng tượng bạn là giáo viên chấm bài cho 8 học sinh. Bạn tính **điểm trung bình từng cặp**, rồi **trung bình của từng cặp trung bình**, cứ thế cho đến khi còn **1 con số duy nhất** đại diện cho cả lớp. Nếu 1 học sinh sửa bài, điểm trung bình cặp đó thay đổi, rồi lan lên trên, cuối cùng con số tổng cũng khác. Bạn chỉ cần so 1 con số tổng là biết "có ai sửa bài". Muốn tìm ai sửa? Đi từ trên xuống: nhánh nào số khác thì đi vào nhánh đó, chỉ cần **log n bước** là tìm ra thủ phạm.

**Merkle Tree hoạt động y hệt** — nhưng thay vì "điểm trung bình", nó dùng **hash**. Mỗi lá là hash của 1 khối dữ liệu. Mỗi nút cha là hash của 2 con gộp lại. Thay đổi 1 byte dữ liệu ở lá → hash thay đổi **dây chuyền lên đến root**. So sánh root hash là biết ngay dữ liệu có bị sửa không, và đi xuống cây là tìm ra chỗ sửa.

## Tại sao cần Merkle Tree?

Câu hỏi cốt lõi: **làm sao verify dữ liệu lớn mà không cần gửi toàn bộ dữ liệu?**

Thử nghĩ: bạn tải 1 file 4GB từ 100 người lạ trên mạng (torrent). Mỗi người gửi bạn 1 mảnh nhỏ. Làm sao biết mảnh nào bị sai? Hash toàn bộ file mỗi lần nhận 1 mảnh? Quá chậm. Hash từng mảnh riêng lẻ? Phải lưu danh sách hash dài vô tận.

Merkle Tree giải quyết gọn gàng: **1 root hash đại diện cho mọi thứ**, và chỉ cần **O(log n) hash** để verify bất kỳ mảnh nào.

> Git dùng Merkle Tree! Mỗi khi bạn `git commit`, Git tính hash cho toàn bộ file và thư mục thành một cây hash. Thay đổi 1 dòng code → hash thay đổi dây chuyền lên đến root. Đó là lý do Git phát hiện mọi thay đổi ngay lập tức.

## Ứng dụng thực tế

| Ở đâu | Dùng để làm gì |
|--------|----------------|
| **Git** | Mỗi commit là root hash của Merkle Tree chứa tất cả file. Thay đổi 1 byte → hash khác → Git biết ngay. |
| **Bitcoin** | Gom hàng ngàn giao dịch vào 1 block. Merkle root nằm trong block header. Muốn chứng minh 1 giao dịch nằm trong block? Chỉ cần proof O(log n), không cần tải cả block. Light client (SPV wallet) trên điện thoại dựa hoàn toàn vào tính năng này. |
| **Ethereum** | Dùng **Merkle Patricia Trie** — lai giữa Trie (chương 3) và Merkle Tree. Mỗi state (số dư, contract data) đều có proof path. Đó là cách light node verify mà không cần tải cả blockchain. |
| **P2P download** | BitTorrent chia file thành nhiều mảnh, tải từ nhiều người. Merkle Tree giúp kiểm tra từng mảnh có đúng không — nếu ai gửi mảnh sai, phát hiện ngay. |
| **Database sync** | Amazon DynamoDB, Cassandra dùng Merkle Tree để so sánh dữ liệu giữa 2 server. Nếu root hash giống nhau → dữ liệu giống nhau. Khác nhau → đi sâu vào cây để tìm chỗ khác. |
| **Certificate Transparency** | Google dùng Merkle Tree để log mọi SSL certificate. Ai cũng verify được certificate có nằm trong log hay không — giúp phát hiện certificate giả mạo. |

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

### Tại sao proof chỉ cần O(log n)?

Mỗi level của cây, bạn chỉ cần **1 sibling hash** — hash của node "anh em" cùng cha. Cây có log n level → proof có log n phần tử.

```
n = 4 lá        → proof = 2 hash    (log₂ 4 = 2)
n = 1,000 lá    → proof = 10 hash   (log₂ 1000 ≈ 10)
n = 1,000,000   → proof = 20 hash   (log₂ 1M ≈ 20)
n = 1 tỷ        → proof = 30 hash   (log₂ 1B ≈ 30)
```

Đây là lý do Bitcoin light wallet trên điện thoại hoạt động được — verify giao dịch chỉ cần ~11 hash thay vì cả block.

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

## Blockchain, Bitcoin và Git — câu chuyện đằng sau

### Bitcoin: verify mà không cần tải cả block

Mỗi Bitcoin block chứa hàng ngàn giao dịch (transaction). Block header chỉ chứa **1 Merkle root hash**. Khi light client (ví trên điện thoại) muốn kiểm tra "giao dịch này có thật không?", nó không tải cả block. Nó chỉ xin **Merkle proof** — vài chục hash — rồi tự verify.

```
┌──────────────────────────────────────┐
│         Bitcoin Block Header         │
│  ┌──────────┐  ┌──────────────────┐  │
│  │ prev hash│  │  MERKLE ROOT     │  │
│  │          │  │  (1 hash duy nhất │  │
│  │          │  │   đại diện ~2000  │  │
│  │          │  │   giao dịch)      │  │
│  └──────────┘  └──────────────────┘  │
│  timestamp, nonce, ...               │
└──────────────────────────────────────┘
                  │
                  ▼
         Merkle Tree bên dưới
       ┌──────────┴──────────┐
       │                     │
   ┌───┴───┐           ┌───┴───┐
   │       │           │       │
  tx1    tx2          tx3    tx4  ...~2000 giao dịch
```

### Git: mỗi commit là 1 Merkle root

Git không gọi thẳng tên "Merkle Tree" nhưng cấu trúc của nó chính là vậy. Mỗi tree object = hash của tất cả blob (file) và subtree (thư mục) bên trong. Thay đổi 1 dòng code → blob hash khác → tree hash khác → commit hash khác. Đó là lý do Git **không thể bị sửa lén** — mọi thay đổi đều thể hiện trong hash chain.

### Ethereum: Merkle Patricia Trie

Ethereum cần thứ phức tạp hơn Bitcoin vì nó lưu **state** (số dư mỗi account, data mỗi smart contract). Nó dùng **Merkle Patricia Trie** — kết hợp Trie (bạn đã học ở chương 3) với Merkle Tree:
- Trie cho phép tìm kiếm theo key (address)
- Merkle cho phép verify bất kỳ state nào bằng proof

Bạn không cần hiểu chi tiết Merkle Patricia Trie ở đây. Chỉ cần biết: **Merkle Tree là nền tảng, và thực tế người ta biến tấu nó cho phù hợp bài toán cụ thể.**

## Kafka: verify data integrity cho message streaming

Kafka (hệ thống message queue phổ biến) chia dữ liệu thành các partition. Khi replicate partition giữa các broker, làm sao biết dữ liệu có bị sai không? So sánh từng message thì quá chậm.

Giải pháp: xây Merkle Tree cho mỗi partition. Hai broker so sánh root hash — nếu giống thì partition giống nhau. Nếu khác thì đi sâu vào cây tìm đoạn dữ liệu bị khác, chỉ sync đoạn đó. Cassandra, DynamoDB cũng dùng cách này (gọi là **anti-entropy repair**).

Tương tự, khi bạn thiết kế hệ thống file sync (như Dropbox), Merkle Tree giúp tìm ra "file nào thay đổi?" mà không cần so sánh từng file.

## Pitfalls — Bẫy hay gặp

### 1. Chọn hash function không phù hợp

❌ **Sai:** Dùng hash function đơn giản (CRC32, Adler32) cho Merkle Tree bảo mật

✅ **Đúng:** Dùng cryptographic hash (SHA-256, Blake3) khi cần bảo mật thật sự. Dùng hash nhanh (DefaultHasher, xxHash) khi chỉ cần phát hiện lỗi dữ liệu (data corruption)

💡 **Tại sao:** Hash đơn giản dễ bị collision (2 dữ liệu khác nhau cho cùng hash). Attacker có thể tạo dữ liệu giả mà hash giống dữ liệu thật. Cryptographic hash thì tốn CPU hơn nhưng gần như không thể giả mạo.

```
Chỉ detect lỗi:     DefaultHasher, xxHash    → nhanh
Cần bảo mật:         SHA-256, Blake3          → chậm hơn, nhưng an toàn
Bitcoin dùng:        SHA-256 (double hash)
Git dùng:            SHA-1 (đang chuyển sang SHA-256)
```

### 2. Số lá lẻ — quên duplicate

❌ **Sai:** Cây 5 lá, cứ để 1 node không có anh em → cây lệch, proof path sai

✅ **Đúng:** Duplicate phần tử cuối cho đến khi đủ lũy thừa 2

💡 **Tại sao:** Merkle Tree cần là full binary tree. Nếu 1 node không có sibling, generate_proof không biết lấy sibling hash ở đâu. Duplicate lá cuối là convention phổ biến nhất (Bitcoin cũng làm vậy).

```
5 lá gốc:     A  B  C  D  E
Sau pad:       A  B  C  D  E  E  E  E   ← pad lên 8 (lũy thừa 2 gần nhất)
```

### 3. Thứ tự hash left/right bị ngược

❌ **Sai:** Lúc build thì `hash(left + right)`, lúc verify thì `hash(right + left)` → root hash khác → verify luôn fail

✅ **Đúng:** Build và verify phải dùng cùng convention: luôn `hash(left + right)`, dùng `is_right` flag trong proof để xác định vị trí

💡 **Tại sao:** `hash("AB") ≠ hash("BA")`. Đảo thứ tự = hash khác hoàn toàn. Đây là bug kinh điển khi tự implement Merkle Tree — code chạy không lỗi nhưng verify cứ fail.

### 4. Second preimage attack — cây bị nhầm leaf với internal node

❌ **Sai:** Hash leaf và internal node dùng cùng hàm `hash(data)`

✅ **Đúng:** Prefix khác nhau: leaf = `hash(0x00 || data)`, internal = `hash(0x01 || left || right)`

💡 **Tại sao:** Nếu không phân biệt, attacker có thể tạo 1 leaf mà hash giống 1 internal node, đánh lừa verifier. Thêm prefix `0x00` / `0x01` để phân biệt rõ ràng. Bitcoin và Certificate Transparency đều làm vậy. (Trong code ví dụ ở trên ta bỏ qua bước này cho đơn giản.)

## Khi nào dùng?

| Tình huống | Dùng Merkle Tree? | Lý do |
|------------|-------------------|-------|
| Verify 1 phần tử trong tập dữ liệu lớn | **Dùng** | Proof O(log n) thay vì gửi cả tập |
| Sync dữ liệu giữa 2 server | **Dùng** | So root hash → tìm nhanh chỗ khác nhau |
| Detect file bị corrupt | **Dùng** | Root hash thay đổi nếu bất kỳ byte nào sai |
| Blockchain / certificate log | **Dùng** | Cho phép light client verify mà không cần full data |
| Dữ liệu thay đổi liên tục (database CRUD) | **Cân nhắc** | Mỗi lần thay đổi phải rebuild path O(log n). Nếu thay đổi quá thường xuyên, overhead đáng kể |
| Chỉ cần hash 1 blob duy nhất | **Không cần** | Hash thẳng là đủ, không cần cây |
| Dữ liệu nhỏ (< 100 phần tử) | **Không cần** | Overhead của tree structure không đáng — hash toàn bộ nhanh hơn |

## Practice — Thiết kế hệ thống File Sync

> Bạn đang thiết kế hệ thống đồng bộ file giống Dropbox. 2 máy tính có cùng folder với hàng ngàn file. Khi 1 file thay đổi, cần sync sang máy kia. Thiết kế dùng Merkle Tree.

Gợi ý nghĩ:

1. **Mỗi file là 1 leaf** — hash(nội dung file) là leaf hash
2. **Mỗi folder là 1 internal node** — hash(tất cả con)
3. **Root hash đại diện cho toàn bộ folder**

Khi cần sync:
```
Máy A gửi root hash cho Máy B
    │
    ├── Root hash giống? → Không cần sync!
    │
    └── Root hash khác?
         │
         ├── So sánh level 1 (các subfolder)
         │     Folder "src/" hash khác → đi sâu vào
         │     Folder "docs/" hash giống → bỏ qua
         │
         └── So sánh level 2 (các file trong src/)
               "main.rs" hash khác → sync file này!
               "lib.rs" hash giống → bỏ qua
```

**Kết quả:** Thay vì so sánh 10,000 file (O(n)), chỉ cần so sánh theo nhánh cây — tìm ra file thay đổi trong O(log n) bước. Đây chính là cách **rsync** và **ZFS** hoạt động.

## Rust Ecosystem

| Crate | Dùng khi nào |
|-------|-------------|
| [`rs-merkle`](https://crates.io/crates/rs-merkle) | General-purpose Merkle Tree. Hỗ trợ SHA-256 và custom hash. API đơn giản, dễ bắt đầu. |
| [`merkle-cbt`](https://crates.io/crates/merkle-cbt) | Complete Binary Tree Merkle. Tối ưu cho cây đầy đủ (giống implementation của ta). |
| [`tiny-keccak`](https://crates.io/crates/tiny-keccak) | Keccak/SHA-3 hash — dùng nếu muốn tương thích Ethereum. |
| [`blake3`](https://crates.io/crates/blake3) | Hash function cực nhanh (SIMD optimized). Lý tưởng khi cần hash nhiều data, không cần tương thích Bitcoin/Ethereum. |
| [`sha2`](https://crates.io/crates/sha2) | SHA-256 — tương thích Bitcoin, Certificate Transparency. |

Trong production, bạn sẽ dùng crate thay vì tự implement — nhưng hiểu cách nó hoạt động bên trong giúp bạn debug khi gặp vấn đề.

## Tóm tắt

- Merkle Tree = binary tree of hashes. Root hash là "dấu vân tay" của toàn bộ dữ liệu.
- Thay đổi bất kỳ dữ liệu nào → root hash thay đổi → phát hiện ngay.
- Proof chỉ cần O(log n) hash — cực kỳ hiệu quả cho hệ thống lớn.
- Dùng khắp nơi: Git, blockchain, P2P, distributed database.
- Chọn hash function phù hợp: nhanh (xxHash) cho detect lỗi, an toàn (SHA-256) cho bảo mật.
- Merkle Patricia Trie (Ethereum) = Trie + Merkle — biến tấu cho bài toán cụ thể.

---

---

[← LFU Cache](./05-lfu-cache.md) | [Design Structures →](./07-design-structures.md)
