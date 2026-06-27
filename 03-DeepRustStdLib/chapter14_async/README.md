# 第 14 章 · 异步编程

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 13 章 I/O 系统](../chapter13_io/README.md) · 原书目录：[本书目录 § 第 14 章](../本书目录.md#第-14-章--异步编程)

**本章定位**：全书**压轴**（p.421+ 正文待刷书）— **`async` 状态机 · I/O 多路复用 · `Future`/`Poll`/`Waker`/`Context`** · **`Pin` 闭环（Ch7）**；Executor 在 **tokio** 等生态，非 `std` 内置。

**原书主线（脉络已写入）**：14.1 协程+epoll → 14.2 Future 轮询与唤醒 → 接 [05-async](../../05-Async-Concurrency-Network/02-async/README.md)。

**阅读顺序**：**14.1 → 14.2**

---

<!-- AUTO:SECTION-INDEX -->

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **14.1** | Rust 协程框架简析 | [笔记](./14.1-coroutine-framework.md) |
| **14.1.1** | 协程概述 | [笔记](./14.1.1-coroutine-overview.md) |
| **14.1.2** | Rust 的 I/O 多路复用 | [笔记](./14.1.2-io-multiplexing.md) |
| **14.2** | Rust 协程支持类型简析 | [笔记](./14.2-coroutine-types.md) |
| **14.2.1** | Rust 协程管理 | [笔记](./14.2.1-coroutine-management.md) |
| **14.2.2** | Future Trait 分析 | [笔记](./14.2.2-future-trait.md) |

<!-- /AUTO:SECTION-INDEX -->
## 子节索引

### 14.1 Rust 协程框架简析

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **14.1** | Rust 协程框架简析 | ✅ |
| **14.1.1** | 协程概述 | ✅ |
| **14.1.2** | Rust 的 I/O 多路复用 | ✅ 脉络 |

### 14.2 Rust 协程支持类型简析

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **14.2** | Rust 协程支持类型简析 | ✅ |
| **14.2.1** | Rust 协程管理 | ✅ 脉络 |
| **14.2.2** | `Future` Trait 分析 | ✅ |

---

## 全书 14 章脉络（宏观）

| 段 | 章 | 主题 |
|:--:|:---:|------|
| **底座** | 1～2 | core/alloc/std · Trait · 解封装 |
| **内存** | 3 | 裸指针 · MaybeUninit · 堆 · 型变 |
| **语言根基** | 4～6 | 类型 · 迭代 · NonZero · str/slice |
| **抽象** | 7～8 | 内部可变 · Pin · Box/Vec/Rc/Arc |
| **OS** | 9～10 | FFI · SYSCALL · fd · 进程 |
| **并发 I/O** | 11～13 | 锁 · MPSC · fs · Read/Write/net |
| **终局** | **14** | **Future · async · 多路复用** |

---

## 与主线对照

| 本章 | 本仓库延伸 |
|------|------------|
| `std` RUNTIME | [11.11 RUNTIME](../chapter11_concurrency/11.11-runtime.md) |
| 阻塞 I/O | [第 13 章 §13.3](../chapter13_io/README.md) |
| RFR 异步原理 | [RFR Ch10](../../02-RFR/Chapter-10-Asynchronous-Programming/README.md) |
| `tokio` 实战 | [05-async](../../05-Async-Concurrency-Network/02-async/) |
| **Pin** | [7.4 Pin/Unpin](../chapter07_interior_mutability/7.4-pin-unpin.md) |

---

## HFT 阅读提示

| 节 | 实盘关联 |
|----|----------|
| **14.1.2** | `epoll`/`kqueue` 与单线程多连接行情 |
| **14.2.2** | `Future` 状态机与 `.await` 编译产物；与低延时「少 poll」策略 |
| 全书衔接 | 原书 std 深挖 → 实盘常接 **05-async / tokio** 做高并发 I/O |
