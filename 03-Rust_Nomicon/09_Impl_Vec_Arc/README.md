# 09 · Implementing Vec (and Arc)

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] Vec 已读（笔记 + `MyVec` 实现）
- [ ] Arc/Mutex（Nomicon 下一章，待建）

---

## 一句话

**Vec 实战章** — stable 徒手 `MyVec`：NonNull 布局、alloc/write/read、Deref、insert/remove、RawVec/IntoIter/Drain、ZST 特判。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | Vec 全章要点 | [00-overview.md](./00-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/raw_vec.rs](./src/raw_vec.rs) | 分配/释放、ZST cap |
| [src/my_vec.rs](./src/my_vec.rs) | push/pop/Drop/Deref/insert/remove |
| [src/into_iter.rs](./src/into_iter.rs) | 消费迭代器 |
| [src/drain.rs](./src/drain.rs) | Drain（len=0 异常安全） |
| [src/zst.rs](./src/zst.rs) | ZST 辅助 |
| [src/lib.rs](./src/lib.rs) | 模块导出 |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/09_Impl_Vec_Arc
cargo run
cargo test
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| MaybeUninit / set_len | [05_Uninit_Mem](../05_Uninit_Mem/README.md) |
| forget Drain | [06_OBRM_RAII](../06_OBRM_RAII/README.md) |
| Arc 原子 refcount | [08_Concurrency_Atomic](../08_Concurrency_Atomic/README.md) |
| 上一章 | [08_Concurrency_Atomic](../08_Concurrency_Atomic/README.md) |
| 下一章 | [10_FFI](../10_FFI/README.md) |

---

## 逻辑脉络

Layout → alloc → push/pop → Drop/Deref → 高级迭代器 → ZST →（待）Arc。
