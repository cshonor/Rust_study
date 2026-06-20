# 04 · Async · Concurrency · Network

> 所属：Rust 主线 **`00-Book` → `02-RFR` → `01-ER` → `03-Rust_Nomicon`** 之后的**实战专题**  
> 下一专题：[05 · Compilers & LLVM Learning](../05_Compilers-and-LLVM-Learning/README.md)

**Rust 基础 → 内存黑魔法（Nomicon）→ 并发 / 异步 / 网络 → LLVM / IR** — 本目录对应中间三段实战链路。

---

## 学习链路（推荐顺序）

```text
03-Rust_Nomicon 通读（unsafe / 布局 / Send·Sync 边界）
        ↓
① atomic/          狗熊书 · 原子、锁、内存序、无锁（底层「知其然」）
        ↓
② async_tokio/     Async Rust · Future / 运行时 / 设计模式（理论 + 手写 executor）
        ↓
③ rust_network_programming/   网络书 · 阻塞 Socket → Tokio 网络 → HTTP / 安全
        ↓
05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17   用上面写过的代码反查 IR / 优化
```

| 阶段 | 目录 | 书目 / 定位 | 与前后衔接 |
|:---:|------|-------------|------------|
| **①** | [`atomic/`](./atomic/README-学习区.md) | *Rust Atomics and Locks* | Nomicon Ch07 · RFR Ch10 · 为网络/异步中的 `Arc`/`Mutex`/原子打底 |
| **②** | [`async_tokio/`](./async_tokio/README.md) | *Async Rust* | RFR Ch08 · Book Ch16 · 与 ③ 的 stage07 对照 |
| **③** | [`rust_network_programming/`](./rust_network_programming/README.md) | *Network Programming with Rust* | 先 stage03 阻塞 TCP，再 stage07 + `async_tokio`；TLS 对照 Nomicon / stage08 |

**不必**按书目顺序死磕三本书；可按上表 **atomic → async → network** 递进，LLVM 专题放在最后做 IR 对照（见 [Learn LLVM 17 学习取舍](../05_Compilers-and-LLVM-Learning/04_Learn-LLVM-17/Learn-LLVM-17-学习取舍.md)）。

---

## 三块内容速览

| 目录 | 结构约定 | 运行 demo |
|------|----------|-----------|
| **atomic/** | `Chapter-01`～`10` + `study_atomic` crate | `cd 04-Async-Concurrency-Network/atomic && cargo build` |
| **async_tokio/** | `ch01`～`ch11` · 每节 `X.Y-slug.md` + `*-demo.rs` | 多数需 Tokio；见 [async_tokio/README.md](./async_tokio/README.md) |
| **rust_network_programming/** | `stage01`～`09` · 与 atomic 同套小节规范 | `cargo run --manifest-path 04-Async-Concurrency-Network/rust_network_programming/stage03_std_tcp_udp/Cargo.toml --bin …` |

共用规范：[atomic/小节笔记与Demo规范.md](./atomic/小节笔记与Demo规范.md) · [rust_network_programming/小节笔记与Demo规范.md](./rust_network_programming/小节笔记与Demo规范.md)

---

## 与 RFR / Nomicon / LLVM 对照

| 本目录 | RFR | Nomicon | LLVM（05） |
|--------|-----|---------|------------|
| atomic Ch02–03 | [Ch10 并发](../02-RFR/Chapter-10-Concurrency-and-Parallelism/README.md) | [Ch07 并发原子](../03-Rust_Nomicon/07_Concurrency_Atomic/README.md) | ch07 优化 · `ir_samples/atomic_ir/` |
| async_tokio | [Ch08 异步](../02-RFR/Chapter-08-Asynchronous-Programming/README.md) | Pin / Send 边界 | `ir_samples/async_tokio_ir/` |
| rust_network stage07 | Ch08 + Ch10 | — | `ir_samples/network_ir/` |

章节级对照表 → [02-RFR/学习路径与章节对照.md](../02-RFR/学习路径与章节对照.md)

---

## 速记

**顺序** atomic → async_tokio → rust_network → **05** Learn LLVM IR  
**demo** 从仓库根 `--manifest-path 04-Async-Concurrency-Network/…`  
**规范** 小节 `X.Y-slug.md` + 同名校验 demo 目录
