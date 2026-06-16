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

| 节 | 主题 | 阅读 |
|:--:|------|------|
| — | 本章定位 | [00-overview.md](./00-overview.md) |
| 1 | 数据布局 | [01-layout.md](./01-layout.md) |
| 2 | 内存分配 | [02-allocating.md](./02-allocating.md) |
| 3 | Push 与 Pop | [03-push-pop.md](./03-push-pop.md) |
| 4 | 内存释放 | [04-dealloc.md](./04-dealloc.md) |
| 5 | 切片解引用 | [05-deref.md](./05-deref.md) |
| 6 | Insert 与 Remove | [06-insert-remove.md](./06-insert-remove.md) |
| 7 | IntoIter / Drain / RawVec | [07-iterators.md](./07-iterators.md) |
| 8 | 零大小类型 | [08-zst.md](./08-zst.md) |
| — | Arc 实现要点 | [01-arc-overview.md](./01-arc-overview.md) |
| — | 速记 · 自测 | [cheat-sheet.md](./cheat-sheet.md) |

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

MyVec 布局 → 分配 → push/pop → dealloc → deref → insert/remove → 迭代器/Drain → ZST → MyArc →（待）Mutex → FFI。
