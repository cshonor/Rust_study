# 08 · Implementing Vec and Arc

> **The Rustonomicon** · [03 Rust Nomicon](../README.md) · [全书笔记](../notes.md)

## 状态

- [x] Vec 已读（笔记 + `MyVec` 实现）
- [x] Arc 已读（笔记 + `MyArc` 实现，无 Weak）
- [ ] Mutex（Nomicon 同章延伸，待建）

---

## 一句话

**Vec + Arc 实战** — stable 徒手 `MyVec` 与简化 `MyArc`：NonNull/PhantomData、原子 refcount、Release/Acquire fence。

---

## 专项笔记

| 小节 | 主题 | 阅读 |
|------|------|------|
| — | Vec 全章要点 | [00-overview.md](./00-overview.md) |
| — | Arc 实现要点 | [01-arc-overview.md](./01-arc-overview.md) |

---

## 示例源码

| 文件 | 演示 |
|------|------|
| [src/my_vec.rs](./src/my_vec.rs) | Vec：push/pop/Deref/… |
| [src/my_arc.rs](./src/my_arc.rs) | Arc：Clone Relaxed / Drop Release+Acquire |
| [src/raw_vec.rs](./src/raw_vec.rs) | RawVec 分配 |
| [src/into_iter.rs](./src/into_iter.rs) | IntoIter |
| [src/drain.rs](./src/drain.rs) | Drain |
| [src/zst.rs](./src/zst.rs) | ZST 辅助 |
| [src/main.rs](./src/main.rs) | 运行入口 |

```bash
cd 03-Rust_Nomicon/08_Impl_Vec_Arc
cargo run
cargo test
```

---

## 与仓库其他部分

| 主题 | 对照 |
|------|------|
| 原子内存序 | [07_Concurrency_Atomic](../07_Concurrency_Atomic/README.md) |
| forget / Rc | [06_OBRM_RAII](../06_OBRM_RAII/README.md) |
| 上一章 | [07_Concurrency_Atomic](../07_Concurrency_Atomic/README.md) |
| 下一章 | [09_FFI](../09_FFI/README.md) |

---

## 逻辑脉络

MyVec 全链路 → MyArc 布局/refcount/屏障 →（待）Mutex → FFI。
