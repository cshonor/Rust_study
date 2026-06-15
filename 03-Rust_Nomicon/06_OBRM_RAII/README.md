# 06 · The Perils Of OBRM

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] 已读（笔记整理）
- [x] 示例 crate（Drop / forget / proxy / guard / poison）

---

## 一句话

**OBRM 风险章** — 极简构造与递归 Drop、`mem::forget` 与泄漏、代理类型陷阱、panic Guard、Mutex 投毒。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | 本章定位与要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/construct_drop.rs](./src/construct_drop.rs) | 递归 Drop、`ManuallyDrop`、`Option::take` |
| [src/forget_leak.rs](./src/forget_leak.rs) | `mem::forget` 泄漏但内存安全 |
| [src/proxy_types.rs](./src/proxy_types.rs) | `Drain` + leak amplification |
| [src/panic_guard.rs](./src/panic_guard.rs) | RAII Guard 恢复中间状态 |
| [src/poisoning.rs](./src/poisoning.rs) | `Mutex` poison |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/06_OBRM_RAII
cargo run
cargo run -- --drop-order    # 观察递归 Drop 顺序
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| Drop 基础 | [Book 15.3](../../00-Book/15-smart-pointers/15.3-使用Drop运行清理代码.md) |
| 未初始化 / set_len | [05_Uninit_Mem](../05_Uninit_Mem/README.md) |
| 异常安全延伸 | [07_Panic_Safety](../07_Panic_Safety/README.md) |
| 上一章 | [05_Uninit_Mem](../05_Uninit_Mem/README.md) |
| 下一章 | [07_Panic_Safety](../07_Panic_Safety/README.md) |

---

## 逻辑脉络

构造析构模型 → forget 与代理危险 → panic Guard → 投毒护栏 → 进入 Panic Safety 深入。
