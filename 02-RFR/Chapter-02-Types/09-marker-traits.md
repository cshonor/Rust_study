# 2.5 Marker Traits（标记 Trait）

> 所属：**Traits and Trait Bounds** · [← 章索引](./README.md)

如 **`Send`**、**`Sync`**、**`Copy`**、**`Sized`** 等：**无（或几乎无）可调用方法**，主要向编译器声明**额外语义**，参与类型检查与优化假设。

| Trait | 编译器契约（直觉） |
|-------|-------------------|
| **`Sized`** | 编译期大小已知；`?Sized` 允许 DST |
| **`Copy`** | 按位复制合法；无 move 语义上的「旧位失效」 |
| **`Send`** | 所有权可安全转移到另一线程 |
| **`Sync`** | `&T` 可安全跨线程共享（多线程读） |

## 自动实现与 unsafe impl

- 多数 marker 由编译器 **auto trait** 推导。
- **`Send` / `Sync` unsafe impl**：封装 FFI、自旋锁等时，作者手动承诺线程安全。

## 与其它章节的衔接

- 并发 → [RFR 第 10 章](../Chapter-10-Concurrency-and-Parallelism/10-并发与并行-Concurrency-and-Parallelism-深度解析.md)
- `Unpin`（async）→ 第 8 章

Book → [16.4 Send 与 Sync](../../00-Book/16-fearless-concurrency/16.4-Send与Sync.md) · ER → [Item 10 标准 trait](../../01-ER/Chapter-02-Traits/Item-10-standard-traits/README.md)
