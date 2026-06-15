# Rustonomicon 学习仓库

配套 **[The Rustonomicon](https://doc.rust-lang.org/nomicon/)** — 围绕 *The Dark Arts of Unsafe Rust* 的 11 个主题板块。

## 基准 Edition

**2018**（与 Nomicon 主体一致；后续可按需升级单章 crate）

## 全书大纲

| 目录 | 官方主题 |
|------|----------|
| [01_Safe_Unsafe/](./01_Safe_Unsafe/) | Meet Safe and Unsafe |
| [02_Data_Layout/](./02_Data_Layout/) | Data Representation in Rust |
| [03_Lifetime_Variance/](./03_Lifetime_Variance/) | Ownership and Lifetimes |
| [04_Type_Cast/](./04_Type_Cast/) | Type Conversions |
| [05_Uninit_Mem/](./05_Uninit_Mem/) | Working With Uninitialized Memory |
| [06_OBRM_RAII/](./06_OBRM_RAII/) | The Perils Of OBRM |
| [07_Panic_Safety/](./07_Panic_Safety/) | （OBRM 延伸：panic / 异常安全） |
| [08_Concurrency_Atomic/](./08_Concurrency_Atomic/) | Concurrency and Parallelism |
| [09_Impl_Vec_Arc/](./09_Impl_Vec_Arc/) | Implementing Vec / Arc and Mutex |
| [10_FFI/](./10_FFI/) | Foreign Function Interface |
| [11_NoStd/](./11_NoStd/) | Beneath std |

| 资料 | 路径 |
|------|------|
| 读书笔记 | [notes.md](./notes.md) |
| 各章源码 | 各章目录下 `Cargo.toml`（待补齐） |

## 编译示例

```bash
cd 03-Rust_Nomicon/09_Impl_Vec_Arc
rustup run nightly cargo build   # 待 Cargo.toml 补齐
```

与本仓库其它板块对照：`atomic/`（Ch08 并发）、`02-RFR/`（类型与 unsafe 进阶）、`rust_network_programming/stage08`（TLS）。
