# 第 13 章 · I/O 系统

> 所属：[03 DeepRustStdLib](../README.md) · 前：[第 12 章 文件系统](../chapter12_filesystem/README.md) · 后：[第 14 章 异步编程](../chapter14_async/README.md) · 原书目录：[本书目录 § 第 13 章](../本书目录.md#第-13-章--io-系统)

**本章定位**：`Stdin`/`Stdout`、`Read`/`Write`、向量读写、**网络 I/O** — `std::io` 与 `std::net` 的标准库实现栈。

**阅读顺序**：**13.1 → 13.2 → 13.3**

---

## 子节索引

### 13.1 标准输入 `Stdin`

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **13.1** | 标准输入 `Stdin` 类型分析 | 📝 规划 |
| **13.1.1** | `Read` Trait | 📝 规划 |
| **13.1.2** | 向量读/写类型分析 | 📝 规划 |
| **13.1.3** | 对外接口层 | 📝 规划 |

### 13.2～13.3

| 节 | 主题 | 笔记 |
|:---:|------|------|
| **13.2** | 标准输出 `Stdout` 类型分析 | 📝 规划 |
| **13.3** | 网络 I/O | 📝 规划 |

---

## 与主线对照

| 本章 | 本仓库延伸 |
|------|------------|
| `Read` / `Write` | [4.3 基本 Trait](../chapter04_primitive_types/README.md) |
| TCP/UDP 实战 | [05-rust_network stage03](../../05-Async-Concurrency-Network/03-rust_network_programming/stage03_std_tcp_udp/README.md) |
| 文件 I/O | [第 12 章 文件系统](../chapter12_filesystem/README.md) |
| async 网络 | [第 14 章 异步](../chapter14_async/README.md) · [05-async](../../05-Async-Concurrency-Network/02-async/) |

---

## HFT 阅读提示

| 节 | 实盘关联 |
|----|----------|
| **13.1.1～13.1.2** | 协议帧 `read_exact` / `BufReader` 解包 |
| **13.3** | 阻塞 `TcpStream` vs `tokio::net`；行情 feed 接入层选型 |
