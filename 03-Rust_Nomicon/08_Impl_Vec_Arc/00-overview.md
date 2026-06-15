# 08 · Example: Implementing Vec · 本章定位

← [本章目录](./README.md) · [全书笔记](../notes.md) · 上一章：[07 Concurrency](../07_Concurrency_Atomic/README.md) · 下一章：[09 FFI](../09_FFI/README.md)

---

官方标题 **Example: Implementing Vec**。全书使用 **stable Rust** 从零实现标准库 `Vec` 的实战章，展示如何用 unsafe 原语构建安全、高性能底层结构。

> 同目录 **Arc** 见 [01-arc-overview.md](./01-arc-overview.md)；**Mutex** 为同章延伸，待建。

| 对照 | 路径 |
|------|------|
| 未初始化 / ptr::write | [05_Uninit_Mem](../05_Uninit_Mem/README.md) |
| OBRM / Drain 泄漏放大 | [06_OBRM_RAII](../06_OBRM_RAII/README.md) |
| ZST | [02_Data_Layout](../02_Data_Layout/00-overview.md) |
| Send/Sync | [07_Concurrency_Atomic](../07_Concurrency_Atomic/00-overview.md) |

**读完应能回答**：MyVec 三字段布局、为何用 `ptr::write/read`、`RawVec` 为何存在、ZST 为何要特判。

---

## 1. 数据布局 (Layout)

`Vec` = **指针 + cap + len**。stable 路径用 **`NonNull<T>`**（协变、非空保证、支持 niche）。因非 `Unique`，`T: Send/Sync` 时需 **`unsafe impl Send/Sync`**。

→ 源码：[src/my_vec.rs](./src/my_vec.rs)

---

## 2. 内存分配 (Allocating)

| 要点 | 做法 |
|------|------|
| 延迟分配 | 空 Vec 不 alloc；`NonNull::dangling()` + `cap == 0` |
| OOM | `handle_alloc_error` |
| 容量上限 | 分配字节 ≤ `isize::MAX`（防 GEP inbounds 溢出） |

→ 源码：[src/raw_vec.rs](./src/raw_vec.rs)

---

## 3. Push 与 Pop

| 操作 | API | 原因 |
|------|-----|------|
| **push** | `ptr::write` | 普通 `*ptr = x` 会先 Drop **未初始化**旧比特 → UB |
| **pop** | `ptr::read` | 不能直接 move 出（会留下逻辑未初始化槽位） |

---

## 4. 内存释放 (Dealloc)

`Drop`：循环 `pop` 清理元素（无 Drop 的类型可被优化掉）→ `dealloc`；`cap == 0` 或未 alloc 则跳过。

---

## 5. 切片解引用 (Deref)

`Deref` / `DerefMut` → `slice::from_raw_parts(_mut)(ptr, len)`。

---

## 6. Insert 与 Remove

移动元素用 **`ptr::copy`**（memmove，允许重叠）。

---

## 7. IntoIter、Drain 与 RawVec

| 组件 | 职责 |
|------|------|
| **`RawVec`** | 指针 + cap；分配/释放逻辑复用 |
| **`IntoIter`** | 双指针 `start`/`end`；`ptr::read` + 推进 |
| **`Drain`** | 借用 Vec；**初始化时 `len = 0`** 防泄漏放大 / panic 不安全 |

→ 源码：[src/raw_vec.rs](./src/raw_vec.rs) · [src/into_iter.rs](./src/into_iter.rs) · [src/drain.rs](./src/drain.rs)

---

## 8. 零大小类型 (ZST)

| 问题 | 对策 |
|------|------|
| alloc(0) → UB | 不调用分配器；`cap = usize::MAX` |
| `ptr.offset` 为 no-op | 迭代用 **usize 计数**，指针 cast 推进 |
| 读 ZST | 从 **`NonNull::dangling()`** 对齐地址 read |
| Drop RawVec | ZST 跳过 `dealloc` |

→ 源码：[src/raw_vec.rs](./src/raw_vec.rs) · [src/zst.rs](./src/zst.rs)
