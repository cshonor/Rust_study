# 3.3.2 · UnsafeCell 与标准库容器速查

> 所属：**Borrowing and Lifetimes · 内部可变性** · [← 07 hub](./07-interior-mutability.md)

← [07.1 外部 vs 内部](./07-1-external-vs-interior.md) · 下一节 [07.3 Cell/RefCell](./07-3-cell-vs-refcell.md)

---

## 一、`UnsafeCell<T>` 底层基石

所有内部可变容器的唯一底层原语 — **opt-out** 普通 `&T` 的「永不修改」假设：

```text
普通 &T     → LLVM 可假定只读（load 消除、寄存器缓存）
UnsafeCell  → 共享路径下也可能写 → 禁止错误优化
```

用户**不直接使用** `UnsafeCell`；`Cell` / `RefCell` / `Mutex` 封装并承担安全契约。

→ Nomicon [五种 unsafe 能力](../../03-Rust_Nomicon/01_Safe_Unsafe/03-five-powers.md) · [05 §LLVM](./05-shared-references.md)

---

## 二、标准库容器速查

| 容器 | 场景 | 读写方式 | 违规 |
|------|------|----------|------|
| `Cell<T>` | 单线程 · **`Copy`** | `get`/`set`，无内部引用 | 类型约束保证安全 |
| `RefCell<T>` | 单线程 · 任意 `T` | `borrow`/`borrow_mut` | **panic** |
| `Mutex<T>` | 多线程写 | `lock()` | 死锁 / **中毒** |
| `RwLock<T>` | 多线程读多写少 | `read`/`write` | 同上 |

→ 单线程详解 [07.3 Cell vs RefCell](./07-3-cell-vs-refcell.md)  
→ 多线程 [RFR 第 10 章](../Chapter-10-Concurrency-and-Parallelism/README.md) · Nomicon [Send/Sync](../../03-Rust_Nomicon/07_Concurrency_Atomic/02-send-sync.md)

---

## 三、`RefCell` 结构概要

```text
RefCell<T> { borrow_flag, value: UnsafeCell<T> }
```

| API | 规则 |
|-----|------|
| `.borrow()` | 多读可共存 |
| `.borrow_mut()` | 读计数=0 且无写 |

**`let` 约束**：绑定不换；盒内 `borrow_mut`。栈上 `RefCell::new(10)` 常见；`Rc<RefCell<T>>` 时盒在堆。

```rust
let x = RefCell::new(10);
let r1 = x.borrow();
// x.borrow_mut();  // panic
drop(r1);
*x.borrow_mut() = 20;
```

C++ 类比：`int* const` — 指针不可改指向，内容可改；`RefCell` 多运行时互斥。

→ API / 选型 / 对照表见 [07.3](./07-3-cell-vs-refcell.md)

---

## 四、速记

1. **一切内部可变底层都是 `UnsafeCell`。**  
2. **单线程：`Cell`（Copy）或 `RefCell`（通用）。**  
3. **多线程：放弃 cell，用 `Mutex`/`RwLock`。**

→ 下一节：[07.3 Cell 与 RefCell](./07-3-cell-vs-refcell.md)
