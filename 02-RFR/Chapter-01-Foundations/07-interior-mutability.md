# 3.3 Interior Mutability（内部可变性）

> 所属：**Borrowing and Lifetimes** · [← 章索引](./README.md)

在**外表仍是共享借用**（`&T`）或固定地址语义下，仍允许**受控地**改变内存。

## 核心：`UnsafeCell`

- 语言内建的「_opt-out_ 共享引用不可变假设」的单元。
- 安全封装告诉编译器与 API 使用者：**这里的别名规则由运行时或类型逻辑保证**。

## 常见安全封装

| 类型 | 场景 | 要点 |
|------|------|------|
| `Cell<T>` | 单线程、`Copy` 或小值替换 | 无引用对外，只有 `get`/`set` |
| `RefCell<T>` | 单线程、运行时 borrow 计数 | 违反规则 **panic** |
| `Mutex<T>` / `RwLock<T>` | 多线程 | 与 [RFR 第 10 章](../Chapter-10-Concurrency-and-Parallelism/10-并发与并行-Concurrency-and-Parallelism-深度解析.md) 衔接 |

## 为什么需要它

- 图、缓存、观察者模式：逻辑上「只读访问句柄」，物理上需更新内部状态。
- 与 [05 共享引用](./05-shared-references.md) 的优化假设**刻意分离**——否则 LLVM 会按普通 `&T` 做错误优化。

Book → [15.5 RefCell 与内部可变性](../../00-Book/15-smart-pointers/15.5-RefCell与内部可变性.md) · ER → [Item 17 共享状态并行](../../01-ER/Chapter-03-Concepts/Item-17-shared-state-parallelism/README.md)
