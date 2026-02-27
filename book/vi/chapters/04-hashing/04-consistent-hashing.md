# Consistent Hashing

## Đây là gì?

Tưởng tượng 3 bạn sinh viên ở cùng khu trọ, chia nhau **trông xe** cho cả xóm. Cách đơn giản: đánh số xe từ 1 đến 100, xe số 1-33 giao bạn A, 34-66 giao bạn B, 67-100 giao bạn C. Ổn.

Nhưng giờ bạn D mới dọn vào, muốn phụ trông xe. Nếu chia lại thành 4 phần đều (1-25, 26-50, 51-75, 76-100)? **Gần như tất cả xe phải đổi chỗ trông.** Chủ xe nào cũng bị ảnh hưởng. Rối loạn.

Cách thông minh hơn: xếp các bạn **quanh một vòng tròn**. Mỗi chiếc xe được gán vào vòng, rồi "đi theo chiều kim đồng hồ" cho đến khi gặp bạn nào thì bạn đó trông. Khi bạn D chen vào vòng, chỉ những xe nằm ngay trước D cần đổi người trông. Phần còn lại giữ nguyên.

Đó chính là **consistent hashing** -- kỹ thuật phân phối dữ liệu trên nhiều server sao cho khi thêm/bớt server, **chỉ một phần nhỏ dữ liệu** cần di chuyển.

### Tại sao cần? Bài toán cũ có gì sai?

Cách ngây thơ: `server = hash(key) % số_server`

```
3 server:  hash("user:1001") % 3 = 1  → server 1
           hash("user:1002") % 3 = 0  → server 0

Thêm 1 server (giờ có 4):
           hash("user:1001") % 4 = 1  → server 1  (giữ nguyên, may mắn)
           hash("user:1002") % 4 = 2  → server 2  (PHẢI DI CHUYỂN!)
```

Khi đổi số server, **gần như tất cả** key bị gán lại server khác. Với hàng triệu key trong cache? Toàn bộ cache bị mất hiệu lực. Server database quá tải. Thảm họa.

Consistent hashing giải quyết: thêm 1 server chỉ di chuyển khoảng **1/N** tổng số key (N = số server). Phần còn lại giữ nguyên.

### Ai dùng?

Amazon DynamoDB, Apache Cassandra, Memcached, CDN (Content Delivery Network). Những hệ thống phân tán lớn đều cần consistent hashing.

## Hoạt động như thế nào?

### Hash ring -- vòng tròn hash

Tưởng tượng output của hash function trải trên một vòng tròn, từ 0 đến giá trị tối đa, rồi quay ngược về 0:

```
                        0 / max
                          │
                  ┌───────●───────┐
                 ╱                 ╲
               ╱                     ╲
              ●  Server A              │
             ╱                         │
            │                          │
3/4 max     │                          │  1/4 max
            │                          │
             ╲                         │
              ╲                       ●  Server B
               ╲                     ╱
                 ╲                 ╱
                  └───────●───────┘
                     Server C
                     1/2 max
```

Mỗi server được hash vào 1 vị trí trên vòng. Để tìm server cho 1 key:

1. Hash key → ra 1 vị trí trên vòng
2. Đi **theo chiều kim đồng hồ** cho đến khi gặp server đầu tiên

```
                          0
                          │
                  ┌───────●───────┐
                 ╱    key1 ↘       ╲
               ╱           ↘        ╲
              ●  Server A   ↘        │
             ╱               ↘       │
            │            key1 đi     │
            │            tới Server B│
             ╲                       │
              ╲                     ●  Server B  ← key1 nằm đây
               ╲                     ╱
                 ╲                 ╱
                  └───────●───────┘
                     Server C
```

### Thêm server -- chỉ di chuyển ít

Khi server D được thêm giữa A và B, chỉ những key nằm giữa A và D cần chuyển từ B sang D. Tất cả key khác giữ nguyên:

```
Trước:                               Sau khi thêm Server D:

    0                                    0
    │                                    │
    ●  Server A                          ●  Server A
   ╱ ╲                                  ╱ ╲
  ╱   ╲  key k1,k2,k3                 ╱   ╲  k1 ở lại Server B
 ╱     ╲  đều tới B                  ╱     ●  Server D (mới)
╱       ╲                           ╱       ╲  k2,k3 chuyển sang D
●        ●  Server B                ●        ●  Server B
 Server C                            Server C

Chỉ k2 và k3 di chuyển! k1 và tất cả key khác không bị ảnh hưởng.
```

### Xóa server -- tương tự

Khi 1 server bị xóa, chỉ key của nó di chuyển tới server kế tiếp (theo chiều kim đồng hồ). Key khác không ảnh hưởng.

### Virtual node -- giải quyết mất cân bằng

Vấn đề: với ít server, mỗi server "sở hữu" một cung trên vòng tròn. Có server cung lớn (nhiều key), có server cung nhỏ (ít key). Mất cân bằng.

Giải pháp: mỗi server physical tạo nhiều **virtual node** (điểm ảo) trên vòng:

```
3 server, mỗi server 3 virtual node:

    0
    │
    ●  A-1
   ╱ ╲
  B-2  C-1
 ╱     ╲
A-3     B-1
 ╲     ╱
  C-3  A-2
   ╲ ╱
    ●  C-2
    │
    B-3
```

Server A có 3 điểm trên vòng: A-1, A-2, A-3. Key hash vào vùng nào thì thuộc server tương ứng. Nhiều điểm hơn = phân phối đều hơn.

| Virtual node mỗi server | Độ lệch tải |
|--------------------------|-------------|
| 1 | ~50% lệch |
| 10 | ~15% lệch |
| 100 | ~5% lệch |
| 200 | ~3% lệch |

100-200 virtual node mỗi server là con số phổ biến trong thực tế.

### Tóm tắt di chuyển key

| Sự kiện | Key phải di chuyển | Key giữ nguyên |
|---------|-------------------|----------------|
| Thêm 1 server (tổng N) | ~K/N (≈ 1/N tổng) | ~K*(N-1)/N |
| Xóa 1 server | Key trên server đó | Tất cả còn lại |
| Naive modulo thêm server | ~K*(N-1)/N (gần hết!) | ~K/N |

## Code Rust

Đây là khái niệm hệ thống phân tán, nên code mang tính minh họa. Cài đặt đầy đủ hash ring với virtual node:

```rust
use std::collections::BTreeMap;

/// Hash ring với virtual node
struct ConsistentHashRing {
    ring: BTreeMap<u64, String>,  // vị trí → tên server
    replicas: usize,              // số virtual node mỗi server
}

/// Hash function đơn giản (FNV-1a) để minh họa
fn fnv1a_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

impl ConsistentHashRing {
    fn new(replicas: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            replicas,
        }
    }

    /// Thêm server vào ring (tạo nhiều virtual node)
    fn add_server(&mut self, server: &str) {
        for i in 0..self.replicas {
            let vnode_key = format!("{server}-vnode-{i}");
            let pos = fnv1a_hash(vnode_key.as_bytes());
            self.ring.insert(pos, server.to_string());
        }
    }

    /// Xóa server và tất cả virtual node của nó
    fn remove_server(&mut self, server: &str) {
        for i in 0..self.replicas {
            let vnode_key = format!("{server}-vnode-{i}");
            let pos = fnv1a_hash(vnode_key.as_bytes());
            self.ring.remove(&pos);
        }
    }

    /// Tìm server nào giữ key này
    ///
    /// Hash key, rồi đi theo chiều kim đồng hồ (tìm vị trí >= hash).
    /// Nếu vượt quá max, quay về đầu (vòng tròn mà).
    fn get_server(&self, key: &str) -> Option<&str> {
        if self.ring.is_empty() {
            return None;
        }
        let hash = fnv1a_hash(key.as_bytes());
        // range(hash..) → tất cả vị trí >= hash
        let server = self.ring.range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())  // quay vòng
            .map(|(_, s)| s.as_str());
        server
    }
}
```

### Cách lookup hoạt động

`BTreeMap` giữ các vị trí **đã sắp xếp**. Tìm server kế tiếp theo chiều kim đồng hồ chỉ là range query:

```
Vị trí trên ring (đã sắp xếp):

  102  → cache-01
  347  → cache-03
  512  → cache-02      ← key hash ra 400, đi theo chiều kim đồng hồ
  789  → cache-01          gặp cache-02 ở vị trí 512
  901  → cache-03

get_server("mykey"):
  hash("mykey") = 400
  range(400..) → đầu tiên là (512, "cache-02")
  Kết quả: "cache-02"
```

## Độ phức tạp

| Thao tác | Thời gian | Giải thích |
|----------|-----------|------------|
| `add_server` | O(R log N) | R = số virtual node, N = tổng vị trí trên ring |
| `remove_server` | O(R log N) | Tương tự |
| `get_server` | O(log N) | Binary search trên BTreeMap |
| **Bộ nhớ** | O(S * R) | S = số server, R = virtual node mỗi server |

**Nôm na:** Thêm/xóa server hơi tốn (phải tạo/xóa R virtual node). Nhưng tìm server cho key thì rất nhanh (O(log N)).

## Ví dụ

### Sử dụng cơ bản

```rust
fn main() {
    let mut ring = ConsistentHashRing::new(150);  // 150 virtual node mỗi server

    ring.add_server("cache-01");
    ring.add_server("cache-02");
    ring.add_server("cache-03");

    // Phân phối key
    let keys = ["user:1001", "user:1002", "session:abc", "order:xyz"];
    for key in &keys {
        let server = ring.get_server(key).unwrap();
        println!("{key} → {server}");
    }

    // Thêm server mới — hầu hết key giữ nguyên server!
    ring.add_server("cache-04");
    println!("\nSau khi thêm cache-04:");
    for key in &keys {
        let server = ring.get_server(key).unwrap();
        println!("{key} → {server}");
    }
}
```

### Đo lường key di chuyển

```rust
let mut ring = ConsistentHashRing::new(150);
ring.add_server("A");
ring.add_server("B");
ring.add_server("C");

// Ghi nhận vị trí ban đầu của 10,000 key
let assignments: Vec<_> = (0..10_000)
    .map(|i| ring.get_server(&format!("key-{i}")).unwrap().to_string())
    .collect();

// Thêm server D
ring.add_server("D");

// Đếm bao nhiêu key phải di chuyển
let new_assignments: Vec<_> = (0..10_000)
    .map(|i| ring.get_server(&format!("key-{i}")).unwrap().to_string())
    .collect();

let moved = assignments.iter().zip(&new_assignments)
    .filter(|(a, b)| a != b)
    .count();

// Lý tưởng: ~25% key di chuyển (10000/4 = 2500)
// Thực tế sẽ gần con số đó
println!("Key di chuyển: {moved} / 10000");
```

Với naive modulo (`hash % num_servers`), con số di chuyển sẽ là ~7500 (75%). Consistent hashing giảm xuống ~2500 (25%). Càng nhiều server, lợi ích càng rõ.
