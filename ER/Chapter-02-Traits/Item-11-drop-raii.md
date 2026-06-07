# Item 11: Implement the Drop trait for RAII patterns

> **Effective Rust** · [Chapter 2 — Traits](../ER-本书目录.md)  
> **中文**：为 RAII 模式实现 Drop trait  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [x] demo：[15.3-drop-demo](../../Book/15-smart-pointers/15.3-drop-demo/)（`cargo run` · `-- early` · `-- guard` · `-- manual`）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| RAII、`Drop` | [15.3 使用 Drop 运行清理代码](../../Book/15-smart-pointers/15.3-使用Drop运行清理代码.md) · [15.3.1 顺序/进阶](../../Book/15-smart-pointers/15.3.1-Drop顺序与进阶场景.md) |
| `MutexGuard` 示例 | [16.3 共享状态并发](../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| 所有权、作用域 | [4.1 所有权](../../Book/04-ownership/4.1-什么是所有权.md) |
| FFI 资源 | [Item 34](../Chapter-06-Beyond-Standard-Rust/Item-34-ffi-boundaries.md)（ER） |

---

## 1. 核心知识点与关键定义

### RAII（Resource Acquisition Is Initialization）

- **资源获取即初始化**：对象**创建**时拿到资源，**销毁**时释放资源。
- **不变量**：只有该类型的**实例存在**时，才合法访问底层资源；实例离开作用域 → 资源必释放。

### `Drop` trait

```rust
trait Drop {
    fn drop(&mut self);
}
```

- 内存回收**前**由编译器**自动**调用（离开 `{}`、变量被 move 走且旧值 drop 等）。
- 与 C++ 析构函数同类思想；Rust 用**所有权 + 作用域**强制配对。

---

## 2. 逻辑脉络

```text
C/C++ 手动 lock/unlock、malloc/free
  → 早退 / panic 易漏释放
Rust：构造拿资源 + Drop 放资源（词法作用域）
  → MutexGuard、File、Box 等 RAII 包装
```

---

## 3. 重点结论与实用要点

### 何时实现 `Drop`

类型掌管**非纯 Rust 堆**或**系统资源**时，封装成 RAII：

| 资源 | 示例 |
|------|------|
| OS | 文件描述符、socket |
| 锁 | 文件锁、DB 锁；**`MutexGuard`** |
| FFI | C 分配、需手动 free 的内存 |

### 缩小作用域，尽早 `Drop`

```rust
{
    let _guard = self.lock();
    // 仅此处持锁
} // guard drop → 解锁，后续耗时逻辑不阻塞别人
```

---

## 4. 案例：`MutexGuard`

```rust
{
    let mut v = self.value.lock().unwrap(); // RAII guard
    *v += delta;
} // drop(&mut guard) → 自动 unlock
```

- 不能「忘了 unlock」；
- 不能「没锁就改数据」（类型系统 + guard 代理）。

---

## 5. 易错细节

### 不要写 `x.drop()`

```rust
// x.drop(); // ❌ explicit use of destructor method

std::mem::drop(x); // ✅ 按所有权消费 x，再 drop
```

- `Drop::drop` 是 **`&mut self`**：若允许手动调，对象还在作用域里却已被清理 → **僵尸对象**。
- **`mem::drop`** 先 **move** 再析构，生命周期正确结束。

### `Drop` 里不能报告失败

- `drop` **无返回值**，不能 `Result`。
- 可能失败的释放（如 DB 优雅断连）→ 提供 **`release() -> Result`** 让用户主动调；`Drop` 只做**防泄漏兜底**（best-effort）。

### 其它（待展开 §6）

- `Drop` 中再 **panic** → 可能 **double panic → abort**。
- **Async**：`Drop` 是**同步**的，**不能 `.await`** → 异步关闭需专门模式。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 11](../ER-拓展索引.md#item-11)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| RAII | 构造拿、析构放；实例在才可用资源 |
| `Drop` | 编译器自动调；别手写 `x.drop()` |
| 提前结束 | `std::mem::drop(x)` |
| 失败释放 | 用 `release()`；Drop 兜底 |
| 锁 | 小作用域 `{}` 尽早解锁 |
