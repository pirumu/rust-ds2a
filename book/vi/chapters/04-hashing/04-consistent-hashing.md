# Consistent Hashing

## Đây là gì?

> **Đừng lo!** Consistent Hashing nghe tên "hàn lâm" nhưng ý tưởng core cực kỳ đơn giản: đặt server và key lên **vòng tròn**, key đi theo chiều kim đồng hồ gặp server nào thì thuộc server đó. Thêm server? Chỉ key gần nó bị ảnh hưởng. Xóa server? Chỉ key của nó chuyển sang server kế tiếp. Bạn đã hiểu `BTreeMap::range()` từ chương B-Tree -- đó chính là tool cần cho implementation. Code ngắn hơn AVL tree nhiều! Đây là kiến thức system design **bắt buộc** -- Amazon, Google, Meta đều hỏi trong interview.

Tưởng tượng 3 bạn sinh viên ở cùng khu trọ, chia nhau **trông xe** cho cả xóm. Cách đơn giản: đánh số xe từ 1 đến 100, xe số 1-33 giao bạn A, 34-66 giao bạn B, 67-100 giao bạn C. Ổn.

Nhưng giờ bạn D mới dọn vào, muốn phụ trông xe. Nếu chia lại thành 4 phần đều (1-25, 26-50, 51-75, 76-100)? **Gần như tất cả xe phải đổi chỗ trông.** Chủ xe nào cũng bị ảnh hưởng. Rối loạn.

Cách thông minh hơn: xếp các bạn **quanh một vòng tròn**. Mỗi chiếc xe được gán vào vòng, rồi "đi theo chiều kim đồng hồ" cho đến khi gặp bạn nào thì bạn đó trông. Khi bạn D chen vào vòng, chỉ những xe nằm ngay trước D cần đổi người trông. Phần còn lại giữ nguyên.

Đó chính là **consistent hashing** -- kỹ thuật phân phối dữ liệu trên nhiều server sao cho khi thêm/bớt server, **chỉ một phần nhỏ dữ liệu** cần di chuyển.

## Từ HashMap đến Distributed HashMap

### Bài toán: data không fit trong 1 máy

HashMap cho O(1) lookup trên **1 máy**. Nhưng khi data không fit trong 1 máy thì sao?

```
Tình huống thực tế:
  100TB session data
  1 máy chỉ có 64GB RAM
  → Cần 100TB / 64GB ≈ 1,563 máy

Hoặc:
  500M user, mỗi user 1KB cache
  → 500GB cache data
  → 1 máy Redis max ~25GB RAM hữu dụng
  → Cần ~20 máy Redis
```

Giải pháp: chia data ra nhiều máy -- gọi là **sharding**. Câu hỏi: key nào nằm trên máy nào?

### Cách naive: modulo

```
server = hash(key) % N    (N = số server)

3 servers:
  hash("user:1") % 3 = 0  → Server 0
  hash("user:2") % 3 = 1  → Server 1
  hash("user:3") % 3 = 2  → Server 2

Hoạt động tốt... cho đến khi thêm/bớt server.
```

### Thảm họa khi thêm server

Quay lại khu trọ: bạn D dọn vào, chia lại theo modulo. Trace cụ thể:

```
Trước (3 người trông xe):        Sau (4 người):
  hash(xe-1) % 3 = 0 → A        hash(xe-1) % 4 = 1 → B   ← ĐỔI!
  hash(xe-2) % 3 = 1 → B        hash(xe-2) % 4 = 2 → C   ← ĐỔI!
  hash(xe-3) % 3 = 2 → C        hash(xe-3) % 4 = 3 → D   ← ĐỔI!
  hash(xe-4) % 3 = 1 → B        hash(xe-4) % 4 = 0 → A   ← ĐỔI!
  hash(xe-5) % 3 = 2 → C        hash(xe-5) % 4 = 1 → B   ← ĐỔI!

5/5 xe phải đổi người trông!
```

Tương tự trong hệ thống thực:

```
Trước (3 cache servers):         Sau (4 cache servers):
  hash("user:1001") % 3 = 0     hash("user:1001") % 4 = 1  ← MOVED!
  hash("user:1002") % 3 = 1     hash("user:1002") % 4 = 2  ← MOVED!
  hash("user:1003") % 3 = 2     hash("user:1003") % 4 = 3  ← MOVED!
  hash("user:1004") % 3 = 1     hash("user:1004") % 4 = 0  ← MOVED!
  hash("user:1005") % 3 = 2     hash("user:1005") % 4 = 1  ← MOVED!

5/5 key bị di chuyển!
Với 1M key trong cache → cache miss storm → database quá tải → hệ thống sập.
```

Đây là **cascade failure**: cache miss hàng loạt → tất cả request đổ về database → database overload → timeout → user thấy lỗi → retry → càng nhiều request → chết hẳn.

### So sánh

| | Naive Modulo | Consistent Hashing |
|---|-------------|-------------------|
| Thêm 1 server (N→N+1) | ~**(N-1)/N** key di chuyển (gần hết!) | ~**1/N** key di chuyển |
| Ví dụ: 100 servers, thêm 1 | ~**99%** key di chuyển | ~**1%** key di chuyển |
| Cache miss storm | Có, nghiêm trọng | Không đáng kể |

Consistent hashing giải quyết: thêm 1 server chỉ di chuyển khoảng **1/N** tổng số key (N = số server). Phần còn lại giữ nguyên.

## Hash ring -- vòng tròn hash

### Ý tưởng

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

Quay lại khu trọ: bạn A đứng cổng chính, bạn B đứng cổng phụ, bạn C đứng bãi sau. Xe nào vào gặp bạn nào trước thì bạn đó trông.

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

## Thêm/xóa server -- chỉ ảnh hưởng cục bộ

### Thêm server

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

Trong khu trọ: bạn D dọn vào phòng giữa A và B. Chỉ mấy chiếc xe đậu gần chỗ D mới đổi người trông. Xe ở phía A, B, C không cần đổi gì.

### Xóa server

Khi 1 server bị xóa, chỉ key của nó di chuyển tới server kế tiếp (theo chiều kim đồng hồ). Key khác không ảnh hưởng.

```
Trước:                               Sau khi xóa Server B:

    0                                    0
    │                                    │
    ●  Server A                          ●  Server A
   ╱ ╲                                  ╱ ╲
  ╱   ╲                               ╱   ╲
 ╱     ╲                              ╱     ╲  key của B chuyển sang C
╱       ╲                            ╱       ╲
●        ●  Server B                 ●        (B đã xóa)
 Server C                            Server C ← nhận key cũ của B

Key thuộc A: giữ nguyên
Key thuộc C: giữ nguyên
Key thuộc B: chuyển sang C (server kế tiếp theo chiều kim đồng hồ)
```

Bạn B dọn đi -- xe của B giao cho bạn kế bên (C). Xe của A, C không ảnh hưởng.

### Tóm tắt di chuyển key

| Sự kiện | Key phải di chuyển | Key giữ nguyên |
|---------|-------------------|----------------|
| Thêm 1 server (tổng N) | ~K/N (≈ 1/N tổng) | ~K*(N-1)/N |
| Xóa 1 server | Key trên server đó | Tất cả còn lại |
| Naive modulo thêm server | ~K*(N-1)/N (gần hết!) | ~K/N |

## Virtual node -- giải quyết mất cân bằng

### Tại sao cần?

Với ít server, mỗi server "sở hữu" một cung trên vòng tròn. Nhưng vị trí hash **không đều**:

```
3 servers, KHÔNG có virtual node:

Hash ring [0...999]:
  Server A ở vị trí 100
  Server B ở vị trí 400
  Server C ở vị trí 900

Phân bổ:
  A sở hữu: 901-100 = 200 slots (20%)
  B sở hữu: 101-400 = 300 slots (30%)
  C sở hữu: 401-900 = 500 slots (50%)  ← gấp 2.5 lần A!

Server C nhận 50% traffic trong khi A chỉ nhận 20%.
```

Trong khu trọ: bạn C đứng ở khu vực rộng nhất, phải trông gấp 2.5 lần xe so với bạn A. Nếu C là bạn "yếu" nhất (phòng nhỏ, ít chỗ) → quá tải → bỏ cuộc → xe C chuyển sang A → A cũng quá tải → **cascading failure**!

### Giải pháp: nhiều "phân thân" trên vòng

Mỗi server physical tạo nhiều **virtual node** (điểm ảo) trên vòng:

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

Trong khu trọ: thay vì mỗi bạn đứng 1 chỗ, mỗi bạn **chia ca** ở 100 vị trí khác nhau quanh xóm. Kết quả: ai cũng nhận lượng xe gần bằng nhau.

```
KHÔNG có virtual node:           Có virtual node (100 per server):
  A: 20% traffic                   A: ~33% ± 5% traffic
  B: 30% traffic                   B: ~33% ± 5% traffic
  C: 50% traffic ← quá tải!       C: ~33% ± 5% traffic
                                   → Cân bằng!
```

### Bao nhiêu virtual node là đủ?

| Virtual node mỗi server | Độ lệch tải |
|--------------------------|-------------|
| 1 | ~50% lệch |
| 10 | ~15% lệch |
| 100 | ~5% lệch |
| 200 | ~3% lệch |

100-200 virtual node mỗi server là con số phổ biến trong thực tế.

## Replication + Fault Tolerance

Consistent hashing giải quyết **distribution** (chia data đều). Nhưng nếu server chết thì sao? Data trên server đó **mất luôn** nếu không có bản sao. Cần **replication**.

### Cách hoạt động

```
Replication factor = 3:
  Key X hash vào vị trí P trên ring
  Copy 1: server kế tiếp theo chiều kim đồng hồ (primary)
  Copy 2: server tiếp theo (secondary)
  Copy 3: server tiếp theo nữa (tertiary)

Ring: ... → A → [key X ở đây] → B(copy1) → C(copy2) → D(copy3) → ...
```

Trong khu trọ: mỗi chiếc xe có **3 bạn biết mật khẩu khóa xe**. Bạn chính (B) giữ chìa, bạn phụ (C, D) giữ chìa dự phòng. Nếu B đi vắng, C hoặc D vẫn lấy được xe.

### Khi server chết

```
Nếu B chết:
  - Read: đọc từ C hoặc D (vẫn có data!)
  - Write: ghi vào C và D, khi B recovery thì sync lại
  - Key X KHÔNG cần di chuyển, chỉ cần failover

Nếu B chết VÀ C chết:
  - Vẫn có D giữ copy → hệ thống vẫn hoạt động
  - Replication factor 3 chịu được 2 server chết cùng lúc

Nếu cả B, C, D đều chết:
  - Data mất. Nhưng 3 server chết cùng lúc là cực hiếm.
  - Trong practice: B, C, D nằm ở 3 availability zone khác nhau
    → cả 3 AZ chết = thiên tai cấp toàn cầu
```

Đây chính là cách **DynamoDB** và **Cassandra** hoạt động. Consistent hashing + replication = hệ thống vừa scalable vừa fault-tolerant.

## Weighted Nodes -- server không cùng "sức mạnh"

Server không phải lúc nào cũng cùng spec. Máy mạnh nên nhận nhiều hơn:

```
Server A: 64GB RAM, 8 cores   → 300 virtual nodes
Server B: 32GB RAM, 4 cores   → 150 virtual nodes
Server C: 16GB RAM, 2 cores   → 75 virtual nodes

A nhận ~57% traffic, B ~29%, C ~14%
Tỷ lệ gần đúng tỷ lệ resources: 4:2:1
```

Trong khu trọ: bạn A khỏe, phòng rộng → nhận nhiều ca trông xe hơn. Bạn C phòng nhỏ → ít ca hơn. Công bằng theo năng lực.

Nhiều virtual node hơn = nhận nhiều key hơn. Đây là cách balance load theo capacity thực tế của mỗi server.

## Code Rust

Đây là khái niệm hệ thống phân tán, nên code mang tính minh họa. Cài đặt đầy đủ hash ring với virtual node:

### BTreeMap -- vũ khí bí mật

Tại sao dùng `BTreeMap` mà không phải `HashMap` cho hash ring?

```rust
// HashMap: KHÔNG có range(). Muốn tìm "vị trí >= hash value"?
//   Phải scan TẤT CẢ key → O(n). Chậm.

// BTreeMap: range() = O(log n). Perfect cho hash ring.
//   BTreeMap::range(hash..) = "tìm tất cả vị trí >= hash"
//   = "đi theo chiều kim đồng hồ gặp server đầu tiên"
```

Đây là ví dụ hoàn hảo cho `BTreeMap` range query mà chương B-Tree đã giới thiệu. `HashMap` **không thể** implement hash ring hiệu quả.

### Implementation

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

        // BTreeMap::range(hash..) → tất cả vị trí >= hash
        // = "đi theo chiều kim đồng hồ gặp server đầu tiên"
        let server = self.ring.range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())  // wrap around: quay về đầu vòng
            .map(|(_, s)| s.as_str());
        server
    }
}
```

### Cách lookup hoạt động

`BTreeMap` giữ các vị trí **đã sắp xếp**. Tìm server kế tiếp theo chiều kim đồng hồ chỉ là range query:

```
Vị trí trên ring (đã sắp xếp trong BTreeMap):

  102  → cache-01
  347  → cache-03
  512  → cache-02      ← key hash ra 400, đi theo chiều kim đồng hồ
  789  → cache-01          gặp cache-02 ở vị trí 512
  901  → cache-03

get_server("mykey"):
  hash("mykey") = 400
  range(400..) → đầu tiên là (512, "cache-02")
  Kết quả: "cache-02"

get_server("another-key"):
  hash("another-key") = 950
  range(950..) → không có gì (max là 901)
  → quay vòng: iter().next() → (102, "cache-01")
  Kết quả: "cache-01"   ← wrap around!
```

## Consistent Hashing trong thực tế

### Amazon DynamoDB

DynamoDB là cơ sở dữ liệu NoSQL của Amazon, dùng consistent hashing làm **xương sống**:

- Mỗi partition là 1 virtual node trên ring
- Replication factor 3 across 3 Availability Zones (AZ) -- data luôn có 3 bản sao ở 3 data center khác nhau
- Khi thêm node: chỉ transfer partition ranges cần thiết, không phải rebuild toàn bộ
- Paper gốc: Amazon's Dynamo (2007) -- đặt nền tảng cho cả thế hệ distributed database

### Apache Cassandra

Cassandra dùng **token ring** -- tên gọi khác của consistent hash ring:

- Mỗi node sở hữu 1 range trên ring (hoặc multiple vnodes)
- `CREATE KEYSPACE` với replication strategy sử dụng consistent hashing để quyết định data nằm ở node nào
- Gossip protocol để các node biết ring topology -- khi node mới join, các node khác tự biết và chia data

### Memcached / Redis Cluster

- **Client-side consistent hashing**: client tự tính server nào giữ key (không cần coordinator trung tâm)
- Khi server chết: chỉ key trên server đó bị cache miss, phần còn lại hoạt động bình thường
- Redis Cluster dùng 16,384 hash slots -- biến thể của consistent hashing
- Không cần coordination central → đơn giản, nhanh

### CDN (Content Delivery Network)

- URL content được hash vào ring → quyết định edge server nào cache content đó
- Edge server gần user nhận request → giảm latency
- Khi edge server overload: thêm server vào ring, chỉ subset content cần redistribute
- CloudFlare, Akamai đều dùng consistent hashing cho content routing

### Kafka / KaCrab

```
Kafka partition assignment cũng dùng concept tương tự:
  - Partitions = "key" trên ring
  - Consumers = "server" trên ring
  - Rebalance khi consumer join/leave = add/remove server

Kafka dùng range/round-robin/sticky assignor, nhưng concept
"minimize data movement khi group thay đổi" giống consistent hashing.
```

KaCrab có thể implement custom assignor dùng consistent hashing cho consumer group rebalance -- minimize partition migration khi consumer join/leave group.

## Độ phức tạp

| Thao tác | Thời gian | Giải thích |
|----------|-----------|------------|
| `add_server` | O(R log N) | R = số virtual node, N = tổng vị trí trên ring |
| `remove_server` | O(R log N) | Tương tự |
| `get_server` | O(log N) | Binary search trên BTreeMap |
| **Bộ nhớ** | O(S * R) | S = số server, R = virtual node mỗi server |

**Nôm na:** Thêm/xóa server hơi tốn (phải tạo/xóa R virtual node). Nhưng tìm server cho key thì rất nhanh (O(log N)). Trong thực tế, thêm/xóa server là sự kiện hiếm, còn lookup key xảy ra hàng triệu lần mỗi giây -- nên O(log N) cho lookup mới là con số quan trọng.

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

## Những cái bẫy hay gặp

### a) Quên virtual node

❌ Chỉ dùng 1 điểm per server → load imbalance nghiêm trọng. Server "may mắn" nhận 50%+ traffic.

✅ Luôn dùng ≥100 virtual node per server.

💡 Trong khu trọ: 1 bạn chỉ đứng 1 chỗ → ai gần bạn đó thì bạn trông hết, bạn khác rảnh rỗi. 100 "phân thân" rải đều quanh xóm → ai cũng bận đều.

### b) Hash function không uniform

❌ Hash function phân bổ không đều → vùng đặc, vùng thưa trên ring → imbalance dù có virtual node.

✅ Dùng hash function tốt: FNV-1a, xxHash, MurmurHash. MD5/SHA cũng OK cho distribution (dù overkill cho non-crypto use).

💡 Hash function tệ giống kiểu: 100 bạn chia ca nhưng 80 bạn đều đứng cổng chính, chỉ 20 bạn rải quanh xóm. Vẫn mất cân bằng.

### c) Quên replication khi thiết kế

❌ Consistent hashing chỉ giải quyết **distribution**, KHÔNG giải quyết **fault tolerance**. Server chết → data mất nếu không replicate.

✅ Luôn kết hợp với replication (thường factor 3). Mỗi key lưu ở 3 server liên tiếp trên ring.

💡 Trông xe mà chỉ 1 bạn biết mật khẩu khóa xe. Bạn ốm → xe kẹt. Phải có ít nhất 2-3 bạn biết mật khẩu dự phòng.

### d) Nhầm consistent hashing với sharding strategy

❌ Consistent hashing là **cơ chế** phân phối key→server. Nó KHÔNG quyết định shard key (partition key) nào tốt.

✅ Chọn shard key tốt TRƯỚC, rồi mới dùng consistent hashing để phân phối. Ví dụ: shard theo `user_id` thường tốt hơn shard theo `timestamp`.

💡 Chọn shard key tệ (timestamp) → tất cả request cùng giây đổ vào 1 server → hot partition dù dùng consistent hashing. Giống như tất cả xe đều đậu cùng 1 chỗ -- chia ca kiểu gì cũng kẹt.

## Khi nào dùng / không nên dùng

| Tình huống | Consistent Hashing? | Thay bằng gì? | Tại sao? |
|------------|-------------------|---------------|----------|
| Distributed cache (Memcached, Redis) | ✅ Dùng | -- | Scale out/in smooth, minimize cache miss |
| Database sharding (DynamoDB, Cassandra) | ✅ Dùng | -- | Minimize rebalance khi thêm/bớt node |
| CDN content routing | ✅ Dùng | -- | Server failure graceful, chỉ subset content ảnh hưởng |
| Load balancer | ✅ Dùng | -- | Sticky sessions + scale theo traffic |
| Consumer group rebalance | ✅ Dùng | -- | Minimize partition migration |
| Single machine HashMap | ❌ Không | HashMap | Overkill, không cần distribution |
| Fixed number of shards (never change) | ❌ Không | Modulo | Đơn giản hơn, overhead consistent hashing không cần |
| Data < 1 server capacity | ❌ Không | Single server | Distributed overhead không đáng |
| Cần ordered processing | ⚠️ Cẩn thận | Range-based sharding | Consistent hashing scatter order -- key liền nhau có thể nằm khác server |

## Luyện nhận diện Pattern

### Bài 1: Design Distributed Cache

**Đề bài:** 5 cache servers, 10M keys. Server thêm/bớt mỗi tuần (scale theo traffic peak giờ cao điểm). Design key→server mapping sao cho minimize cache miss khi scale.

**Gợi ý:** Consistent hashing + 150 virtual nodes per server + replication factor 2. Khi thêm server thứ 6: chỉ ~17% key di chuyển (1/6) thay vì ~83% với modulo.

### Bài 2: Design URL Shortener (scaling)

**Đề bài:** Hệ thống short URL ban đầu 1 server. Traffic tăng 10x, cần scale lên 10 servers. Data (`short_code → original_url`) đang trên 1 DB. Làm sao shard mà không downtime?

**Gợi ý:** Consistent hashing trên `short_code`. Thêm từng server một. Mỗi lần thêm, chỉ migrate ~1/(N+1) data. Từ 1→2 server: migrate 50%. Từ 9→10 server: migrate chỉ 10%. Không cần downtime vì key cũ vẫn ở server cũ.

### Bài 3: Tính toán

**Đề bài:** 4 servers, 100 virtual nodes mỗi server (400 points trên ring). Thêm server thứ 5 (thêm 100 virtual nodes → 500 points).

- Ước tính % key di chuyển với consistent hashing?
- Nếu dùng naive modulo thì %?

**Đáp án:**
- Consistent hashing: ~1/5 = **20%** key di chuyển
- Naive modulo: `hash % 4` → `hash % 5`, khoảng **80%** key di chuyển ((5-1)/5)
- Chênh lệch: 4x ít hơn. Với 10M key: 2M vs 8M key phải migrate.

## Rust ecosystem

Muốn dùng consistent hashing trong project Rust thật (không cần tự implement):

- **`hashring`** crate -- hỗ trợ virtual nodes, weighted nodes, API đơn giản
- **`consistent-hash-ring`** crate -- implementation cơ bản
- **`conhash`** crate -- thread-safe consistent hashing, dùng được trong multi-threaded server

Và tất nhiên, `BTreeMap::range()` từ standard library -- tool chính cho tự implement. Không cần crate ngoài nếu chỉ cần basic hash ring.

**Liên hệ KaCrab:**
- Consumer rebalance: khi consumer join/leave group, dùng consistent hashing để minimize partition reassignment
- Broker discovery: hash `topic-partition` → broker (nếu implement custom partition assignment)
- Connection pooling: hash `broker_id` → connection pool slot, tránh tạo connection mới khi broker thay đổi

## Chương tiếp theo

Consistent Hashing giải quyết bài toán phân phối data trên nhiều server. Nhưng tất cả data structures đến giờ đều lưu data dạng key-value hoặc tree. Có những bài toán mà data là **quan hệ**: bạn bè trên mạng xã hội, đường đi giữa thành phố, dependency giữa package. Chương tiếp theo giới thiệu **Graph** -- cấu trúc biểu diễn quan hệ nhiều-nhiều, nền tảng của BFS, DFS, shortest path, topological sort.

---

---

[← Bloom Filter](./03-bloom-filter.md) | [Biểu diễn đồ thị →](../05-graphs/01-graph-representations.md)
