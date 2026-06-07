# Item 8: Familiarize yourself with reference and pointer types

> **Effective Rust** · [Chapter 1 — Types](../ER-本书目录.md)  
> **中文**：熟悉引用与指针类型  
> 原文：[effective-rust.com](https://www.effective-rust.com/print.html)

## 状态

- [x] 已读（笔记整理）
- [ ] demo（见 [ER-demos 索引](../ER-demos/README.md) / Book demo）

---

## 与 The Book 对照

| 主题 | 本仓库 |
|------|--------|
| `&T` / `&mut T`、借用 | [4.2 引用与借用](../../Book/04-ownership/4.2-引用与借用.md) |
| 切片 `&[T]` | [4.3 切片](../../Book/04-ownership/4.3-切片slice.md) |
| `Box` | [15.1 Box](../../Book/15-smart-pointers/15.1-使用Box指向堆上的数据.md) |
| `Deref` | [15.2 Deref](../../Book/15-smart-pointers/15.2-通过Deref将智能指针当作引用.md) · [15.2.1 嵌套/坑点](../../Book/15-smart-pointers/15.2.1-Deref嵌套可变与编译坑.md) |
| `Rc` / `RefCell` / `Weak` | [15.4 Rc](../../Book/15-smart-pointers/15.4-Rc引用计数智能指针.md)、[15.5 RefCell](../../Book/15-smart-pointers/15.5-RefCell与内部可变性.md)、[15.6 循环引用](../../Book/15-smart-pointers/15.6-引用循环与Weak.md) |
| `Arc` / `Mutex` | [16.3 共享状态](../../Book/16-fearless-concurrency/16.3-共享状态并发.md) |
| `dyn Trait` | [17.2 trait 对象](../../Book/17-oop/17.2-为使用不同类型的值而设计的trait对象.md) |
| `AsRef` / 转换 | [Item 5](./Item-05-type-conversions.md) |

---

## 1. 核心知识点与关键定义

### 引用 `&T` / `&mut T`

| | |
|--|--|
| 监督 | 借用检查器 + 生命周期 |
| 保证 | 指向合法、对齐内存；无悬垂（安全 Rust） |
| 64 位大小 | **8 字节**（单一地址） |

### 胖指针（Fat Pointers，通常 16 字节）

| 类型 | 组成 |
|------|------|
| **`&[T]`** | 数据指针 + **length** |
| **`&str`** | 同上（DST 视图） |
| **`&dyn Trait`** | 数据指针 + **vtable** 指针 |

### 智能指针（Smart Pointers）

| 类型 | 角色 |
|------|------|
| **`Box<T>`** | 堆分配，**独占**所有权 |
| **`Rc<T>` / `Arc<T>`** | 引用计数共享；`Arc` 原子、可跨线程 |
| **`Weak<T>`** | 弱引用，**不**增 strong count |
| **`RefCell<T>`** | 内部可变性，**运行时**借用检查 |
| **`Cell<T>`** | 内部可变性，限 **`Copy`**，按值改 |
| **`Mutex<T>` / `RwLock<T>`** | 线程安全内部可变 + 锁 |
| **`Cow<'a, T>`** | 借或拥有；改时才 **clone** |

### 裸指针 `*const T` / `*mut T`

- **无**借用检查；仅 **`unsafe`** / FFI 底层使用。

---

## 2. 逻辑脉络与知识点关联

### Deref 强制转换

- 智能指针实现 **`Deref` / `DerefMut`** → 调用处可像用 `&T` 一样用 `Box` / `Rc` 等。
- **`Deref::Target` 唯一**（与 `AsRef` 多目标不同）→ 避免 coercion 歧义。

### `AsRef` / `Borrow` / `ToOwned`

| Trait | 作用 |
|-------|------|
| **`AsRef<T>`** | 显式转成多种引用视图（如 `String` → `&str`、`&[u8]`） |
| **`Borrow<T>`** | 容器键查找：`HashMap::get` 同时收 `&K` 和 `K`（有 `Borrow` blanket） |
| **`ToOwned`** | 在 `Borrow` 之上泛化「从借用得到 owned」 |

---

## 3. 重点结论与实用要点

1. **安全 Rust 无悬垂引用**——生命周期在编译期拦截；裸指针隔离在 `unsafe`。
2. **复杂共享结构**——组合智能指针，例如：
   - `Rc<RefCell<Vec<T>>>`：共享列表、整体可变；
   - `Rc<Vec<RefCell<T>>>`：共享容器、**逐项**可变。
3. **`Rc` 单线程，`Arc` 多线程**；跨 `.await` 常需 `Arc` + `Send`（见 §6）。
4. **图/树回边**——父指针用 **`Weak`**，避免 `Rc` 环泄漏。

---

## 4. 案例与代码要点

### 切片胖指针

```rust
let v = vec![10, 20, 30, 40];
let slice: &[i32] = &v[1..3]; // ptr 指向 v[1]，len = 2
```

`Range` → `SliceIndex` → `Index` → 构建 ptr + len。

### Trait 对象与 vtable

```rust
trait Calculate { fn add(&self, x: i32) -> i32; }

let tobj: &dyn Calculate = &concrete_impl;
// 胖指针：数据地址 + vtable（含 add 等函数指针）
```

### Deref 链（示意）

```rust
let b = Box::new(42);
let r: &i32 = &*b; // Deref: Box<i32> -> i32
```

---

## 5. 易错细节

### `RefCell` 运行时 panic

| 违规 | 结果 |
|------|------|
| 已有 `borrow`，再 `borrow_mut` | **panic**（编译不拦） |
| 多个 `borrow_mut` | **panic** |

→ 用 **`try_borrow` / `try_borrow_mut`** 若需优雅失败。

### `Rc` 强引用环

```text
Parent --Rc--> Child
Child  --Rc--> Parent   // strong count 永不为 0 → 泄漏
```

→ 回指用 **`Weak`**，需要时再 `upgrade()`。

### `Deref` vs `AsRef`

- **`Deref`**：编译器自动 coercion，**一个** `Target`。
- **`AsRef`**：手动、可多个 impl，API 设计用（Item 5 待补）。

---

## 6. 后续拓展

> 展开版：[ER-拓展索引 § Item 08](../ER-拓展索引.md#item-08)

详见索引中各条目的完成度 `[x]` / `[ ]` 与 Book demo 链接。

---

## 记忆卡片

| 要点 | 一句 |
|------|------|
| `&T` | 8B 瘦指针；借用监督 |
| 胖指针 | 切片/str、dyn = ptr + 元数据 |
| `Box` | 堆 + 独占 |
| `Rc/Arc` | 共享所有权；环用 `Weak` |
| `RefCell` | 内部可变；违反规则 runtime panic |
| 裸指针 | 仅 unsafe / FFI |
