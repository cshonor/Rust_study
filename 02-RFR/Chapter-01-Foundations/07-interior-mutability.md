# 3.3 Interior Mutability（内部可变性）

> 所属：**Borrowing and Lifetimes** · [← 章索引](./README.md)

前置 → [05 共享引用](./05-shared-references.md) · [06 可变引用](./06-mutable-references.md)

---

## 一、核心定义

在对外仅暴露**共享不可变引用 `&T`**、外层类型本身不标记 `mut` 的前提下，通过特殊容器实现**受控修改内部数据**的能力。

| 层面 | 普通借用 | 内部可变性 |
|------|----------|------------|
| 表层 | `&T` 只读、`&mut T` 独占修改 | 外表常是 `&Container<T>`（共享） |
| 校验 | **编译期**借用检查 | **运行时** / 容器内部逻辑 |
| 优化假设 | `&T` → 不可变；`&mut` → noalias | `UnsafeCell` → opt-out 不可变假设 |

**大白话**：外面拿着「只读通行证」（`&T`），盒子内部在规则内偷偷改 — 安全由容器负责，不是编译器默认路径。

---

## 二、底层基石：`UnsafeCell<T>`

所有内部可变容器的底层唯一基础，语言层面用于**主动退出（opt-out）普通共享引用的不可变、无别名优化假设**：

1. **普通 `&T`**：编译器默认内部永远不会被修改，可做缓存、常量传播等优化。
2. **`UnsafeCell<T>`**：向编译器声明 — 本块内存即便通过共享引用访问，也可能发生修改，禁止激进的别名优化。
3. **安全契约**：`UnsafeCell` 本身是 unsafe 原语，不对外直接使用，必须被上层安全容器封装，由容器自行保证多别名下的可变安全。

```rust
// 标准库示意（简化）：RefCell 内部持有 UnsafeCell<T>
// 用户永远不应直接 unwrap UnsafeCell 到 Safe API 外
```

→ Nomicon [五种 unsafe 能力](../../03-Rust_Nomicon/01_Safe_Unsafe/03-five-powers.md) · [05 共享引用 §LLVM](./05-shared-references.md)

---

## 三、标准库安全封装容器对比

| 容器 | 使用场景 | 核心特性 | 违规行为后果 |
|------|----------|----------|--------------|
| `Cell<T>` | 单线程、适合 `Copy` 小型数据 | 不产生内部引用，仅 `get()`/`set()` 整体替换值，无借用计数 | 无运行时借用检查，类型约束保证安全 |
| `RefCell<T>` | 单线程、需要借用内部数据 | 运行时维护读写借用计数器，模拟编译期借用规则 | 同时存在活跃冲突借用 → **panic** |
| `Mutex<T>` / `RwLock<T>` | 多线程并发共享修改 | OS 互斥锁/读写锁同步，跨线程安全 | 死锁、阻塞；**中毒锁**（持锁 panic 后锁永久不可用） |

> 联动：`Mutex`/`RwLock` → [RFR 第 10 章 并发](../Chapter-10-Concurrency-and-Parallelism/10-并发与并行-Concurrency-and-Parallelism-深度解析.md)

---

## 四、内部可变性的设计必要性

1. **业务模型需求**  
   缓存、图节点、观察者模式、全局共享配置：对外大量只读句柄（`&T`），内部需自主更新。若只用普通 `&mut T` 独占可变，许多 API 形状无法实现。

2. **规避编译器错误优化**  
   普通 `&T` 会让 LLVM 假定内存永不变化（load 消除、寄存器缓存）。内部可变容器依托 `UnsafeCell` 关闭该假设，防止读写不一致的 UB。

3. **补充编译期借用检查短板**  
   编译期无法分析复杂运行时逻辑（动态数量的共享访问）。内部可变性将校验下沉运行时，拓展共享状态的表达能力。

---

## 五、代码示例

### 1. `Cell` — 单线程、无内部引用

```rust
use std::cell::Cell;

let data = Cell::new(0);
let shared = &data;   // 外层是不可变引用
shared.set(10);
println!("{}", shared.get());  // 10
```

`Cell` 不提供 `&T` 指向内部 — 只有整体 `get`/`set`，故无 borrow 冲突。

### 2. `RefCell` — 运行时借用校验

```rust
use std::cell::RefCell;

let cell = RefCell::new(vec![1, 2, 3]);
let s = &cell;   // 多个 &RefCell 可以共存

let mut w = s.borrow_mut();
w.push(4);
// 同一作用域再获取可变借用 → 运行时 panic
// let w2 = s.borrow_mut();
```

与 [05 §RefCell](./05-shared-references.md) 示例一致 — 编译期允许多个 `&`，运行时 `borrow`/`borrow_mut` 仍互斥。

### 3. 多句柄共享同一 `RefCell`

```rust
let data = RefCell::new(5);
let r1 = &data;
let r2 = &data;

{
    let _a = r1.borrow_mut();
    // let _b = r2.borrow_mut(); // ❌ 运行时 panic
}
let _c = r2.borrow(); // ✅ 上一段 mut borrow 已结束
```

---

## 六、关键区分：可变引用 vs 内部可变性

| | `&mut T` | 内部可变性（`RefCell` / `Mutex` 等） |
|---|----------|--------------------------------------|
| 校验时机 | **编译期** | **运行时**（或锁） |
| 表层借用 | 独占 `&mut` | 常是共享 `&Container` |
| 运行开销 | 零（仅编译检查） | borrow 计数 / 锁 / 阻塞 |
| 违反规则 | 编译失败 | `RefCell` panic；`Mutex` 死锁/中毒 |
| 典型场景 | 单路径清晰修改 | 多共享句柄、图、缓存、并发 |

→ 速记对照：[06-07-mutability-cheat-sheet.md](./06-07-mutability-cheat-sheet.md)

---

## 七、常见误区

| 误区 | 纠正 |
|------|------|
| 「有 `&T` 就完全不能改」 | **直接改**不行；**内部可变性**是显式例外 |
| 「`RefCell` 可以绕过所有借用规则」 | 只是**推迟到运行时**；规则仍在，违反仍 panic |
| 「`Cell` 和 `RefCell` 一样」 | `Cell` 无内部引用、仅 `Copy` 替换；`RefCell` 可 `borrow` 内部 |
| 「`Mutex` 是 `RefCell` 的多线程版」 | 语义类似（运行时互斥），但涉及 **Send/Sync**、中毒、阻塞 |
| 「内部可变 = 不需要 `UnsafeCell`」 | 所有此类容器底层都依赖 `UnsafeCell` opt-out |

---

## 八、下一节

→ [08 生命周期](./08-lifetimes.md)

---

## 对照阅读

- 前置 → [05 共享引用](./05-shared-references.md) · [06 可变引用](./06-mutable-references.md)
- Book → [15.5 RefCell 与内部可变性](../../00-Book/15-smart-pointers/15.5-RefCell与内部可变性.md)
- ER → [Item 17 共享状态并行](../../01-ER/Chapter-03-Concepts/Item-17-shared-state-parallelism/README.md)
- Nomicon → [07 Concurrency Send/Sync](../../03-Rust_Nomicon/07_Concurrency_Atomic/02-send-sync.md)
