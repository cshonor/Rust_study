# 第 14 章 · 异步编程

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 13 章 I/O 系统](../chapter13_io/README.md) · 原书目录：[本书目录 § 第 14 章](../本书目录.md#第-14-章--异步编程)

**本章定位**：协程概述、**I/O 多路复用**、`Future` trait 与协程管理 — 从 `std` 阻塞模型过渡到 Rust 异步生态的理论底座（原书收束章）。

**阅读顺序**：**14.1 → 14.2**

---

## 子节索引

### 14.1 Rust 协程框架简析

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **14.1** | Rust 协程框架简析 | 📝 规划 |
| **14.1.1** | 协程概述 | 📝 规划 |
| **14.1.2** | Rust 的 I/O 多路复用 | 📝 规划 |

### 14.2 Rust 协程支持类型简析

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **14.2** | Rust 协程支持类型简析 | 📝 规划 |
| **14.2.1** | Rust 协程管理 | 📝 规划 |
| **14.2.2** | `Future` Trait 分析 | 📝 规划 |

---

## 与主线对照

| 本章 | 本仓库延伸 |
|------|------------|
| `std` RUNTIME | [11.11 RUNTIME](../chapter11_concurrency/README.md) |
| 阻塞 I/O | [第 13 章 §13.3](../chapter13_io/README.md) |
| RFR 异步原理 | [RFR Ch10](../../02-RFR/Chapter-10-Asynchronous-Programming/README.md) |
| `tokio` 实战 | [05-async](../../05-Async-Concurrency-Network/02-async/) |

---

## HFT 阅读提示

| 节 | 实盘关联 |
|----|----------|
| **14.1.2** | `epoll`/`kqueue` 与单线程多连接行情 |
| **14.2.2** | `Future` 状态机与 `.await` 编译产物；与低延时「少 poll」策略 |
| 全书衔接 | 原书 std 深挖 → 实盘常接 **05-async / tokio** 做高并发 I/O |
