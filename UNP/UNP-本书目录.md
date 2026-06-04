# UNIX 网络编程（UNP）— 本书目录

> **原书**：*UNIX Network Programming, Volume 1*（W. Richard Stevens / Bill Fenner / Andrew M. Rudoff）  
> **本仓库**：精读笔记，**HFT / 网关**向批注；与 Rust 主线 [Book](../Book/Book-本书目录.md)、[ER](../ER/ER-本书目录.md) 并行。

---

## 卷 1 笔记

| 章 | 主题 | 笔记 |
|----|------|------|
| 1 | 简介 | [1.6 客户/服务器示例索引表](./Vol1/1.6-客户服务器示例索引表.md) |

其余章节按需补充。

---

## 与 Rust 学习对照（HFT）

| UNP 主题 | Rust / 本仓库 |
|----------|----------------|
| epoll / 非阻塞 IO | 后续可对照 `mio` / `tokio`；IO 多路复用优先读 UNP 再映射 |
| UDP 组播行情 | [ER Item 20 TLV](../ER/Chapter-03-Concepts/Item-20-avoid-over-optimize.md)、二进制协议 |
| 线程池 | [Book 16 并发](../Book/16-fearless-concurrency/) |
| `getaddrinfo` 双栈 | 与部署、运维 DNS 相关；Rust `std::net` / `socket2` |

工具链概念（与网络无关）：根 [README § Rust 工具链](../README.md#rust-工具链stablenightly--edition)
